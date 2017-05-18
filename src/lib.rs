#![feature(link_args)]

extern crate libc;
extern crate node_api_sys;

use std::ffi::CString;
use std::os::raw::c_void;
use node_api_sys::{napi_env, napi_value, napi_create_function, napi_set_named_property,
                   napi_callback_info, napi_status, napi_create_string_utf8,
                   napi_has_named_property};
mod napi;
mod napi_args;

use napi::{module_register, NapiModule};

const NAPI_MODULE_VERSION: libc::c_int = 1;

#[no_mangle]
pub extern "C" fn hello(env: napi_env, _info: napi_callback_info) -> napi_value {
    unsafe {
        let mut w: napi_value = std::mem::uninitialized();
        let _status = napi_create_string_utf8(env,
                                              CString::new("World").unwrap().as_ptr(),
                                              (1 as libc::size_t).wrapping_neg(),
                                              &mut w as *mut _);
        println!("called hello");
        w
    }
}

#[no_mangle]
pub unsafe extern "C" fn register(env: napi_env,
                                  exports: napi_value,
                                  _module: napi_value,
                                  _priv: *mut c_void) {

    let mut function: napi_value = std::mem::uninitialized();
    napi_create_function(env,
                         std::ptr::null(),
                         Some(hello),
                         std::ptr::null_mut(),
                         &mut function);
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

#[cfg_attr(target_os = "macos", link_args = "-Wl,-undefined,dynamic_lookup")]
extern{}

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
        }).expect("error registering module");
        println!("load");
    }
    __load_napi_module
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
