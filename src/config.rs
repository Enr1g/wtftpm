use glib::MainContext;
use secstr::SecUtf8;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

const DEFAULT_APP: &str = "com.enr1g.wtftpm";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Namespace {
    #[serde(rename = "plain")]
    PlainNS,
    #[serde(rename = "libsecret")]
    LibsecretNS { namespace: String },
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SecretValue {
    #[serde(rename = "plain")]
    PlainSV { slot_label: String, value: SecUtf8 },
    #[serde(rename = "libsecret")]
    LibsecretSV {
        slot_label: String,
        namespace: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub libpath: String,
    pub active_namespace: Namespace,
    pub log_file: Option<PathBuf>,
    secrets: Vec<SecretValue>,
}

impl Default for Namespace {
    fn default() -> Self {
        Self::LibsecretNS {
            namespace: SecretValue::default_libsecret_namespace(),
        }
    }
}

impl std::fmt::Debug for SecretValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlainSV { slot_label, .. } => {
                f.write_fmt(format_args!("PlainSV({}, REDACTED)", slot_label))
            }
            Self::LibsecretSV {
                slot_label,
                namespace,
            } => f.write_fmt(format_args!("LibsecretSV({}, {:?})", slot_label, namespace)),
        }
    }
}

impl SecretValue {
    pub fn default_libsecret_namespace() -> String {
        String::from("default")
    }

    pub fn make_attributes<'a>(
        namespace: &'a str,
        slot_label: &'a str,
    ) -> HashMap<&'a str, &'a str> {
        HashMap::from([
            ("namespace", namespace),
            ("slot_label", slot_label),
            ("app", DEFAULT_APP),
        ])
    }

    pub fn get_value(&self) -> Option<SecUtf8> {
        return match self {
            Self::PlainSV { value, .. } => Some(value.clone()),
            Self::LibsecretSV {
                slot_label,
                namespace,
            } => {
                let m = MainContext::new();
                let attributes = Self::make_attributes(namespace, slot_label);

                let req = libsecret::password_lookup_future(None, attributes);
                let res = m.block_on(req).ok().flatten();

                // Well, that sucks. GString into String method might differ:
                // - Native (just moves, nice)
                // - Foreign (clone + free the original, not nice)
                // - Inline (copies, not nice, impossible here because of fixed size)
                // I don't want to rely on internal specifics.
                // I'd rather destroy the contents myself.
                // Stupid and unsafe.
                res.map(|value| {
                    // The native String
                    let copy = value.as_str().to_owned();
                    // Destroying the memory contents of GString in a dirty way
                    unsafe {
                        let len = value.len();
                        let ptr = value.as_str().as_ptr() as *mut u8;
                        for idx in 0..len {
                            *ptr.add(idx) = b'X';
                        }
                        // log::debug!("Destroyed password: {}", value);
                    }
                    // Move from the native String
                    SecUtf8::from(copy)
                })
            }
        };
    }

    pub fn get_label(&self) -> &str {
        match self {
            Self::PlainSV { slot_label, .. } => slot_label,
            Self::LibsecretSV { slot_label, .. } => slot_label,
        }
    }

    pub fn is_in_namespace(&self, active_namespace: &Namespace) -> bool {
        match (self, active_namespace) {
            (Self::PlainSV { .. }, Namespace::PlainNS) => true,
            (
                Self::LibsecretSV {
                    namespace: left, ..
                },
                Namespace::LibsecretNS { namespace: right },
            ) => left == right,
            _ => false,
        }
    }

    pub fn save_into_libsecret(
        namespace: &str,
        slot_label: &str,
        pin: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let m = MainContext::new();

        let attributes = Self::make_attributes(namespace, slot_label);
        let label = format!("{}/{}/{}", DEFAULT_APP, namespace, slot_label);

        let res = libsecret::password_store_future(
            None,
            attributes,
            Some(libsecret::COLLECTION_DEFAULT),
            &label,
            pin,
        );

        Ok(m.block_on(res)?)
    }
}

impl Config {
    const CONFIG_NAME: &str = "wtftpm/config.yaml";

    pub fn new(
        libpath: &str,
        active_namespace: Namespace,
        secrets: Vec<SecretValue>,
        log_file: Option<PathBuf>,
    ) -> Config {
        Config {
            libpath: libpath.to_owned(),
            active_namespace,
            log_file,
            secrets,
        }
    }

    pub fn get_default_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        // You need a crate for everything this days
        let home = dirs::config_dir().ok_or(format!("couldn't get config dir"))?;

        Ok(home.join(Self::CONFIG_NAME))
    }

    pub fn parse(config_path: Option<PathBuf>) -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = config_path.unwrap_or(Self::get_default_config_path()?);
        let file = std::fs::File::open(&config_path)?;
        let config = serde_yaml::from_reader(file)?;

        Ok(config)
    }

    pub fn get_secret_for_slot_label(&self, slot_label: &str) -> Option<&SecretValue> {
        for sv in self.secrets.iter() {
            if sv.is_in_namespace(&self.active_namespace) && sv.get_label() == slot_label {
                return Some(sv);
            }
        }

        None
    }

    pub fn example_libsecret() -> Self {
        let ls1 = SecretValue::LibsecretSV {
            slot_label: "tpm-token".into(),
            namespace: SecretValue::default_libsecret_namespace(),
        };

        let ls2 = SecretValue::LibsecretSV {
            slot_label: "another-tpm-token".into(),
            namespace: SecretValue::default_libsecret_namespace(),
        };

        Config::new(
            "/path/to/pkcs11/lib.so",
            Namespace::default(),
            vec![ls1, ls2],
            Some("/tmp/wtftpm.log".into()),
        )
    }

    pub fn example_plain() -> Self {
        let plain1 = SecretValue::PlainSV {
            slot_label: "tpm-token".into(),
            value: "pin-in-plaintext".into(),
        };

        let plain2 = SecretValue::PlainSV {
            slot_label: "another-tpm-token".into(),
            value: "another-pin-in-plaintext".into(),
        };

        Config::new(
            "/path/to/pkcs11/lib.so",
            Namespace::PlainNS,
            vec![plain1, plain2],
            Some("/tmp/wtftpm.log".into()),
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::parse(None).unwrap()
    }
}
