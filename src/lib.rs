#![feature(link_args)]

extern crate libc;
extern crate node_api_sys;

use std::ffi::CString;
use std::os::raw::c_void;
use std::io::Write;

use node_api_sys::{napi_set_named_property, napi_status, napi_has_named_property};
mod napi;
mod napi_args;

pub use napi::{NapiValue, NapiError, NapiEnv};
pub use napi::{get_null, get_undefined, get_global, get_boolean, create_object, create_array,
               array_with_length, create_number};

use napi::{module_register, NapiModule};

const NAPI_MODULE_VERSION: libc::c_int = 1;

struct HelloArgs {}
impl napi_args::FromNapiArgs for HelloArgs {
    fn from_napi_args(_: NapiEnv, _: &[napi::NapiValue]) -> Result<Self, NapiError> {
        Ok(HelloArgs {})
    }
}

#[no_mangle]
pub extern "C" fn register(env: NapiEnv,
                           exports: NapiValue,
                           _module: NapiValue,
                           _priv: *mut c_void) {
    std::io::stderr().write(b"register\n");
    let function = napi::create_function(env, "foo", |_: napi::NapiEnv, _: HelloArgs| "world")
        .unwrap();
    unsafe {
        let status = napi_set_named_property(env,
                                             exports,
                                             CString::new("hello").unwrap().as_ptr(),
                                             function);
        assert!(status == napi_status::napi_ok);
        let mut present: bool = false;
        let status2 = napi_has_named_property(env,
                                              exports,
                                              CString::new("hello").unwrap().as_ptr(),
                                              &mut present);
        assert!(status2 == napi_status::napi_ok);
        assert!(present);
        println!("register");
    }
}

#[cfg_attr(target_os = "macos", link_args = "-Wl,-undefined,dynamic_lookup")]
extern "C" {}

#[cfg_attr(target_os = "linux", link_section = ".ctors")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
pub static REGISTER_FOO: extern "C" fn() = {
    extern "C" fn __load_napi_module() {
        module_register(NapiModule {
                            version: NAPI_MODULE_VERSION,
                            flags: 0,
                            filename: "foo".to_string(),
                            register_func: Some(register),
                            modname: "foo".to_string(),
                        })
                .expect("error registering module");
        std::io::stderr().write(b"load\n");
    }
    __load_napi_module
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
