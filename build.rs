use quote::{format_ident, quote, ToTokens};
use std::io::Write;
use std::path::PathBuf;

const HEADER: &str = "include/pkcs11.h";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={}", HEADER);

    let bindings = bindgen::Builder::default()
        .header(HEADER)
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let content = bindings.to_string();
    let ast = syn::parse_file(&content)?;

    assert_eq!(
        ast.attrs.len(),
        0,
        "unexptected number of attributes in generated bindings"
    );

    let mut output = Vec::with_capacity(ast.items.len());
    let mut fields = Vec::new();

    for item in ast.items.iter() {
        match item {
            syn::Item::Struct(item_struct) => {
                if item_struct.ident == "_CK_FUNCTION_LIST" {
                    fields = Vec::with_capacity(item_struct.fields.len());

                    for field in item_struct.fields.iter() {
                        let field = field.ident.as_ref().unwrap();

                        if field != "version" {
                            fields.push(field.clone())
                        }
                    }
                }
            }
            _ => continue,
        }
    }

    assert_ne!(fields.len(), 0, "_CK_FUNCTION_LIST wasn't found");

    for item in ast.items {
        match item {
            syn::Item::ForeignMod(mut fmod) => {
                assert_eq!(fmod.items.len(), 1, "unexpected number of functions");
                let item = fmod.items.pop().unwrap();

                match item {
                    syn::ForeignItem::Fn(func) => {
                        let name = &func.sig.ident;
                        let type_name = format_ident!("{}_type", name);
                        let debug_msg = format!("call {}", name);
                        let symbol_name =
                            syn::LitByteStr::new(name.to_string().as_bytes(), name.span());
                        let err_no_symbol = format!("fatal error: no symbol {} found in lib", name);
                        let args = &func.sig.inputs;
                        let arg_names: syn::punctuated::Punctuated<
                            syn::PatIdent,
                            syn::token::Comma,
                        > = args
                            .iter()
                            .map(|arg| {
                                if let syn::FnArg::Typed(arg) = arg {
                                    if let syn::Pat::Ident(ident) = &*arg.pat {
                                        return ident.clone();
                                    }
                                }
                                panic!("unexpected arg type");
                            })
                            .collect();
                        let rv = &func.sig.output;
                        let body;

                        if name == "C_GetFunctionList" {
                            let flist = arg_names.iter().next().unwrap();
                            let new_flist =
                                format_ident!("STATIC_{}", flist.ident.to_string().to_uppercase());
                            body = quote! {
                                log::debug!(#debug_msg);

                                unsafe {
                                    static mut #new_flist: _CK_FUNCTION_LIST = _CK_FUNCTION_LIST {
                                        version: _CK_VERSION { major: 0, minor: 0 },
                                        #( #fields: Some(#fields), )*
                                    };

                                    *#flist = &mut #new_flist;

                                    CKR_OK as CK_RV
                                }
                            }
                        } else {
                            body = quote! {
                                log::debug!(#debug_msg);

                                unsafe {
                                    let func: #type_name = crate::overrides::get_lw()
                                        .read()
                                        .unwrap()
                                        .get(#symbol_name)
                                        .expect(#err_no_symbol);

                                    func(#arg_names)
                                }
                            }
                        }

                        let new_func = quote! {
                            pub type #type_name = unsafe extern "C" fn(
                                #args
                            ) #rv;

                            #[no_mangle]
                            pub extern "C" fn #name(
                                #args
                            ) #rv {
                                #body
                            }
                        };

                        output.push(new_func);
                    }
                    i => output.push(i.into_token_stream()),
                }
            }
            i => output.push(i.into_token_stream()),
        }
    }

    let code = quote! {
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]
        #![allow(dead_code)]

        #( #output )*
    };

    let file = syn::parse_file(&code.to_string())?;
    let code = prettyplease::unparse(&file);

    let out_path = PathBuf::from("src");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(out_path.join("bindings.rs"))?;

    file.write_all(code.as_bytes())?;

    Ok(())
}
