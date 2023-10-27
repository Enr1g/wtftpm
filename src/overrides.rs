#![allow(non_snake_case)]

use crate::bindings::*;
use crate::config::Config;
use libloading::Library;
use secstr::SecUtf8;
use std::ops::Deref;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    sync::{OnceLock, RwLock},
};

/// Fair to say, this code is super unsafe.
/// It expects that symbol pointers outlive library.
/// I guess, it's true only if library is 'static
/// or LibraryWrapper lives long enough ('static too)
pub struct LibraryWrapper {
    pins_for_slots: HashMap<CK_SLOT_ID, SecUtf8>,
    slots_for_sessions: HashMap<CK_SESSION_HANDLE, CK_SLOT_ID>,
    overrides: HashMap<Vec<u8>, fn()>,
    library: Library,
    config: Config,
}

pub fn get_lw() -> &'static RwLock<LibraryWrapper> {
    static LW: OnceLock<RwLock<LibraryWrapper>> = OnceLock::new();

    LW.get_or_init(|| unsafe {
        let config = Config::default();

        config
            .log_file
            .as_ref()
            .map(|log_file| init_logging(&log_file));

        unsafe fn add_overrides(hm: &mut HashMap<Vec<u8>, fn()>, symbols: &[(&[u8], *const ())]) {
            for (symbol, f) in symbols {
                // Transmuting between raw and function pointers is fine (not equal to automatic UB)
                hm.insert(symbol.to_vec(), std::mem::transmute::<*const (), fn()>(*f));
            }
        }

        let mut overrides: HashMap<Vec<u8>, fn()> = HashMap::new();

        add_overrides(
            &mut overrides,
            &[
                (
                    b"C_Initialize",
                    LibraryWrapper::wrap_C_Initialize as *const (),
                ),
                (
                    b"C_GetTokenInfo",
                    LibraryWrapper::wrap_C_GetTokenInfo as *const (),
                ),
                (
                    b"C_OpenSession",
                    LibraryWrapper::wrap_C_OpenSession as *const (),
                ),
                (b"C_Login", LibraryWrapper::wrap_C_Login as *const ()),
            ],
        );

        log::debug!("overrides registred");

        RwLock::new(LibraryWrapper {
            pins_for_slots: HashMap::new(),
            slots_for_sessions: HashMap::new(),
            overrides,
            library: Library::new(&config.libpath).expect("couln't init shared library"),
            config,
        })
    })
}

impl LibraryWrapper {
    pub unsafe fn get<T: Copy>(&self, symbol: &[u8]) -> Result<T, Box<dyn std::error::Error>> {
        log::debug!(
            "trying to get wrapper for {}",
            String::from_utf8_lossy(symbol)
        );

        let func = self
            .overrides
            .get(symbol)
            .map(|f| *std::mem::transmute::<&fn(), &T>(f))
            .unwrap_or(self.get_original::<T>(symbol)?);

        Ok(func)
    }

    pub unsafe fn get_original<T: Copy>(
        &self,
        symbol: &[u8],
    ) -> Result<T, Box<dyn std::error::Error>> {
        let orig = unsafe { self.library.get::<T>(symbol)? };

        Ok(*orig.deref())
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    fn map_pin_to_slot_id(&mut self, slot_id: CK_SLOT_ID, info: CK_TOKEN_INFO_PTR) -> bool {
        if info.is_null() {
            return false;
        }

        if self.pins_for_slots.contains_key(&slot_id) {
            return true;
        }

        unsafe {
            match std::str::from_utf8(&(*info).label) {
                Ok(slot_label) => match self.config.get_secret_for_slot_label(slot_label.trim()) {
                    Some(sv) => {
                        if let Some(pin) = sv.get_value() {
                            self.pins_for_slots.insert(slot_id, pin);
                            return true;
                        }

                        return false;
                    }
                    None => return false,
                },
                Err(_) => return false,
            }
        }
    }

    pub fn get_pin_for_slot_id(&self, slot_id: CK_SLOT_ID) -> Option<&SecUtf8> {
        self.pins_for_slots.get(&slot_id)
    }

    pub fn slot_is_known(&self, slot_id: CK_SLOT_ID) -> bool {
        self.pins_for_slots.contains_key(&slot_id)
    }

    pub fn map_slot_id_to_session(&mut self, session: CK_SESSION_HANDLE, slot_id: CK_SLOT_ID) {
        self.slots_for_sessions.insert(session, slot_id);
    }

    pub fn get_slot_id_for_session(&self, session: CK_SESSION_HANDLE) -> Option<&CK_SLOT_ID> {
        self.slots_for_sessions.get(&session)
    }

    #[no_mangle]
    pub extern "C" fn wrap_C_Initialize(init_args: *mut ::std::os::raw::c_void) -> CK_RV {
        log::debug!("call wrap_C_Initialize");

        let orig: C_Initialize_type = unsafe {
            get_lw()
                .read()
                .unwrap()
                .get_original(b"C_Initialize")
                .unwrap()
        };

        let ret: CK_RV = unsafe { orig(init_args) };

        if ret != (CKR_OK as CK_RV) {
            return ret;
        }

        unsafe {
            let mut ulCount: std::os::raw::c_ulong = 0;

            let ret = C_GetSlotList(0, std::ptr::null_mut(), &mut ulCount);

            if ret != (CKR_OK as CK_RV) {
                return ret;
            }

            let mut slot_list: Vec<CK_SLOT_ID> = Vec::with_capacity(ulCount as usize);
            let ret = C_GetSlotList(0, slot_list.as_mut_ptr(), &mut ulCount);

            if ret != (CKR_OK as CK_RV) {
                return ret;
            }

            slot_list.set_len(ulCount as usize);

            log::debug!("got {} slots: {:?}", ulCount, slot_list);

            let mut info: _CK_TOKEN_INFO = std::mem::zeroed();

            for slotID in slot_list {
                let ret = C_GetTokenInfo(slotID, &mut info);

                if ret != (CKR_OK as CK_RV) {
                    return CKR_GENERAL_ERROR as CK_RV;
                }

                log::debug!(
                    "slot {}, raw label = {:?}",
                    slotID,
                    String::from_utf8_lossy(&info.label)
                );
            }
        }

        CKR_OK as CK_RV
    }

    #[no_mangle]
    pub extern "C" fn wrap_C_GetTokenInfo(slotID: CK_SLOT_ID, info: *mut _CK_TOKEN_INFO) -> CK_RV {
        log::debug!("call wrap_C_GetTokenInfo");

        let orig: C_GetTokenInfo_type = unsafe {
            get_lw()
                .read()
                .unwrap()
                .get_original(b"C_GetTokenInfo")
                .unwrap()
        };

        let ret: CK_RV = unsafe { orig(slotID, info) };

        if ret != (CKR_OK as CK_RV) {
            return ret;
        }

        if get_lw().write().unwrap().map_pin_to_slot_id(slotID, info) {
            // If there is saved PIN don't ask for it
            unsafe { (*info).flags |= CKF_PROTECTED_AUTHENTICATION_PATH as CK_FLAGS }
        }

        CKR_OK as CK_RV
    }

    #[no_mangle]
    pub extern "C" fn wrap_C_OpenSession(
        slotID: CK_SLOT_ID,
        flags: CK_FLAGS,
        application: *mut ::std::os::raw::c_void,
        notify: CK_NOTIFY,
        session: *mut CK_SESSION_HANDLE,
    ) -> CK_RV {
        log::debug!("call wrap_C_OpenSession");

        let orig: C_OpenSession_type = unsafe {
            get_lw()
                .read()
                .unwrap()
                .get_original(b"C_OpenSession")
                .unwrap()
        };

        let ret = unsafe { orig(slotID, flags, application, notify, session) };

        if ret != (CKR_OK as CK_RV) {
            return ret;
        }

        if get_lw().read().unwrap().slot_is_known(slotID) {
            unsafe {
                get_lw()
                    .write()
                    .unwrap()
                    .map_slot_id_to_session(*session, slotID);

                Self::wrap_C_Login(
                    *session,
                    CKU_USER as CK_USER_TYPE,
                    std::ptr::null_mut() as *mut std::os::raw::c_uchar,
                    0,
                )
            }
        } else {
            ret
        }
    }

    pub extern "C" fn wrap_C_Login(
        session: CK_SESSION_HANDLE,
        user_type: CK_USER_TYPE,
        pin: *mut ::std::os::raw::c_uchar,
        pin_len: ::std::os::raw::c_ulong,
    ) -> CK_RV {
        log::debug!("call wrap_C_Login");

        let orig: C_Login_type =
            unsafe { get_lw().read().unwrap().get_original(b"C_Login").unwrap() };

        // Slot was seen before
        if let Some(slot_id) = get_lw().read().unwrap().get_slot_id_for_session(session) {
            // Pin is cached
            if let Some(cached_pin) = get_lw().read().unwrap().get_pin_for_slot_id(*slot_id) {
                if !pin.is_null() {
                    log::warn!("original pin is not null!");
                }

                let mut pin = cached_pin.clone();

                return unsafe {
                    orig(
                        session,
                        user_type,
                        pin.unsecure_mut().as_bytes_mut().as_mut_ptr()
                            as *mut std::os::raw::c_uchar,
                        pin.unsecure().len() as std::os::raw::c_ulong,
                    )
                };
            }
        }

        unsafe { orig(session, user_type, pin, pin_len) }
    }
}

pub fn init_logging(log_file: &PathBuf) {
    simplelog::WriteLogger::init(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
            .unwrap(),
    )
    .unwrap();
}
