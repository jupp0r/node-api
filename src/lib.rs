#![feature(link_args)]

extern crate node_api_sys;
extern crate futures;

mod napi;
mod napi_value;
mod napi_futures;
pub mod error;

pub use napi::*;
pub use napi_value::{FromNapiValues, IntoNapiValue};

#[macro_export]
macro_rules! napi_module {
    ($module:expr, $register_func:ident) => {
const NAPI_MODULE_VERSION: std::os::raw::c_int = 1;

#[cfg_attr(target_os = "macos", link_args = "-Wl,-undefined,dynamic_lookup")]
extern "C" {}

#[cfg_attr(target_os = "linux", link_section = ".ctors")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
pub static REGISTER_FOO: extern "C" fn() = {
    extern "C" fn __load_napi_module() {
        node_api::module_register(node_api::NapiModule {
                            version: NAPI_MODULE_VERSION,
                            flags: 0,
                            filename: $module.to_string(),
                            register_func: Some($register_func),
                            modname: $module.to_string(),
                        })
                .expect("error registering module");
    }
    __load_napi_module
};
}}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
