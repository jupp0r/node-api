#![feature(link_args)]

extern crate libc;
use std::ffi::CString;

type NapiEnv = *const libc::c_void;
type NapiValue = *const libc::c_void;
type NapiCallbackInfo = *const libc::c_void;
type NapiRef = *const libc::c_void;
type NapiHandleScope = *const libc::c_void;
type NapiEscapableHandleScope = *const libc::c_void;

type NapiAddonRegisterFunc = extern "C" fn(NapiEnv, NapiValue, NapiValue, *const libc::c_void);
type NapiCallback = extern "C" fn(NapiEnv, NapiCallbackInfo);
type NapiFinalize = extern "C" fn(NapiEnv, *mut libc::c_void, *mut libc::c_void);

#[repr(C)]
pub struct NapiModule {
    nm_version: libc::c_int,
    nm_flags: libc::uint32_t,
    nm_filename: StaticCString,
    nm_register_func: NapiAddonRegisterFunc,
    nm_modname: StaticCString,
    nm_priv: *const libc::c_void,
    reserved: [libc::uint8_t; 4],
}
unsafe impl Sync for NapiModule {}

#[repr(C)]
enum NapiStatus {
    NapiOk,
    NapiInvalidArg,
    NapiObjectExpected,
    NapiStringExpected,
    NapiNameExpected,
    NapiFunctionExpected,
    NapiNumberExpected,
    NapiBooleanExpected,
    NapiArrayExpected,
    NapiGenericFailure,
    NapiPendingException,
    NapiCancelled,
    NapiStatusLast,
}

#[repr(C)]
struct NapiExtendedErrorInfo {
    error_message: CString,
    engine_reserved: *const libc::c_void,
    engine_error_code: libc::uint32_t,
    error_code: NapiStatus,
}

#[repr(C)]
struct NapiPropertyDescriptor {
    utf8name: CString,
    name: NapiValue,
    method: NapiCallback,
    getter: NapiCallback,
    setter: NapiCallback,
    value: NapiValue,
    attributes: NapiPropertyAttributes,
    data: *const libc::c_void,
}

#[repr(C)]
enum NapiPropertyAttributes {
    NapiDefault = 0,
    NapiWritable = 1 << 0,
    NapiEnumerable = 1 << 1,
    NapiConfigurable = 1 << 2,

    // Used with napi_define_class to distinguish static properties
    // from instance properties. Ignored by napi_define_properties.
    NapiStatic = 1 << 10,
}

#[link_args = "-Wl,-undefined,dynamic_lookup"]
extern {
    fn napi_module_register(module: *const NapiModule);
    fn napi_get_last_error_info(env: NapiEnv,
                                result: *const *mut NapiExtendedErrorInfo)
                                -> NapiStatus;
    fn napi_get_undefined(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;
    fn napi_get_null(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;
    fn napi_get_global(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;
    fn napi_get_boolean(env: NapiEnv, value: bool, result: *mut NapiValue) -> NapiStatus;

    fn napi_create_object(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;
    fn napi_create_array(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;
    fn napi_create_array_with_length(env: NapiEnv,
                                     length: libc::size_t,
                                     result: *mut NapiValue)
                                     -> NapiStatus;
    fn napi_create_number(env: NapiEnv,
                          value: libc::c_double,
                          result: *mut NapiValue)
                          -> NapiStatus;
    fn napi_create_string_latin1(env: NapiEnv,
                                 str: CString,
                                 length: libc::size_t,
                                 result: *mut NapiValue)
                                 -> NapiStatus;
    fn napi_create_string_utf8(env: NapiEnv,
                               str: CString,
                               length: libc::size_t,
                               result: *mut NapiValue)
                               -> NapiStatus;
    fn napi_create_string_utf16(env: NapiEnv,
                                str: CString,
                                length: libc::size_t,
                                result: *mut NapiValue)
                                -> NapiStatus;
    fn napi_create_symbol(env: NapiEnv,
                          description: NapiValue,
                          result: *mut NapiValue)
                          -> NapiStatus;
    fn napi_create_function(env: NapiEnv,
                            utf8name: CString,
                            cb: NapiCallback,
                            data: *const libc::c_void,
                            result: *mut NapiValue)
                            -> NapiStatus;
    fn napi_create_error(env: NapiEnv, msg: NapiValue, result: *mut NapiValue) -> NapiStatus;
    fn napi_create_type_error(env: NapiEnv, msg: NapiValue, result: *mut NapiValue) -> NapiStatus;
    fn napi_create_range_error(env: NapiEnv, msg: NapiValue, result: *mut NapiValue) -> NapiStatus;

    fn napi_typeof(env: NapiEnv, value: NapiValue, result: *mut NapiValue) -> NapiStatus;
    fn napi_get_value_double(env: NapiEnv,
                             value: NapiValue,
                             result: *mut libc::c_double)
                             -> NapiStatus;
    fn napi_get_value_int32(env: NapiEnv,
                            value: NapiValue,
                            result: *mut libc::int32_t)
                            -> NapiStatus;
    fn napi_get_value_uint32(env: NapiEnv,
                             value: NapiValue,
                             result: *mut libc::uint32_t)
                             -> NapiStatus;
    fn napi_get_value_int64(env: NapiEnv,
                            value: NapiValue,
                            result: *mut libc::int64_t)
                            -> NapiStatus;
    fn napi_get_value_bool(env: NapiEnv, value: NapiValue, result: *mut bool) -> NapiStatus;
    fn napi_get_value_string_latin1(env: NapiEnv,
                                    value: NapiValue,
                                    result: *mut CString)
                                    -> NapiStatus;
    fn napi_get_value_string_utf8(env: NapiEnv,
                                  value: NapiValue,
                                  result: *mut CString)
                                  -> NapiStatus;
    fn napi_get_value_string_utf16(env: NapiEnv,
                                   value: NapiValue,
                                   result: *mut CString)
                                   -> NapiStatus;

    fn napi_coerce_to_bool(env: NapiEnv, value: NapiValue, result: *mut NapiValue) -> NapiStatus;
    fn napi_coerce_to_number(env: NapiEnv, value: NapiValue, result: *mut NapiValue) -> NapiStatus;
    fn napi_coerce_to_object(env: NapiEnv, value: NapiValue, result: *mut NapiValue) -> NapiStatus;
    fn napi_coerce_to_string(env: NapiEnv, value: NapiValue, result: *mut NapiValue) -> NapiStatus;

    fn napi_get_prototype(env: NapiEnv, object: NapiValue, result: *mut NapiValue) -> NapiStatus;
    fn napi_get_property_names(env: NapiEnv,
                               object: NapiValue,
                               result: *mut NapiValue)
                               -> NapiStatus;
    fn napi_set_property(env: NapiEnv,
                         object: NapiValue,
                         key: NapiValue,
                         value: NapiValue)
                         -> NapiStatus;
    fn napi_has_property(env: NapiEnv,
                         object: NapiValue,
                         key: NapiValue,
                         result: *mut bool)
                         -> NapiStatus;
    fn napi_get_property(env: NapiEnv,
                         object: NapiValue,
                         key: NapiValue,
                         result: *mut NapiValue)
                         -> NapiStatus;
    fn napi_set_named_property(env: NapiEnv,
                               object: NapiValue,
                               utf8name: CString,
                               value: NapiValue)
                               -> NapiStatus;
    fn napi_has_named_property(env: NapiEnv,
                               object: NapiValue,
                               utf8name: CString,
                               result: *mut bool)
                               -> NapiStatus;
    fn napi_set_element(env: NapiEnv,
                        object: NapiValue,
                        index: libc::uint32_t,
                        value: NapiValue)
                        -> NapiStatus;
    fn napi_has_element(env: NapiEnv,
                        object: NapiValue,
                        index: libc::uint32_t,
                        result: *mut bool)
                        -> NapiStatus;
    fn napi_get_element(env: NapiEnv,
                        object: NapiValue,
                        index: libc::uint32_t,
                        result: *mut NapiValue)
                        -> NapiStatus;
    fn napi_define_properties(env: NapiEnv,
                              object: NapiValue,
                              property_count: libc::size_t,
                              properties: *const NapiPropertyDescriptor)
                              -> NapiStatus;

    // Methods to work with Arrays
    fn napi_is_array(env: NapiEnv, value: NapiValue, result: *mut bool) -> NapiStatus;
    fn napi_get_array_length(env: NapiEnv,
                             value: NapiValue,
                             result: *mut libc::uint32_t)
                             -> NapiStatus;

    // Methods to compare values
    fn napi_strict_equals(env: NapiEnv,
                          lhs: NapiValue,
                          rhs: NapiValue,
                          result: *mut bool)
                          -> NapiStatus;

    // Methods to work with Functions
    fn napi_call_function(env: NapiEnv,
                          recv: NapiValue,
                          func: NapiValue,
                          argc: libc::size_t,
                          argv: *const NapiValue,
                          result: *mut NapiValue)
                          -> NapiStatus;
    fn napi_new_instance(env: NapiEnv,
                         constructor: NapiValue,
                         argc: libc::size_t,
                         argv: *const NapiValue,
                         result: *mut NapiValue)
                         -> NapiStatus;
    fn napi_instanceof(env: NapiEnv,
                       object: NapiValue,
                       constructor: NapiValue,
                       result: *mut bool)
                       -> NapiStatus;

    // Napi version of node::MakeCallback(...)
    fn napi_make_callback(env: NapiEnv,
                          recv: NapiValue,
                          func: NapiValue,
                          argc: libc::size_t,
                          argv: *const NapiValue,
                          result: *mut NapiValue)
                          -> NapiStatus;

    // Methods to work with napi_callbacks

    // Gets all callback info in a single call. (Ugly, but faster.)
    fn napi_get_cb_info(env: NapiEnv,
                        cbinfo: NapiCallbackInfo,
                        argc: *mut libc::size_t,
                        argv: *mut NapiValue,
                        this_arg: *mut NapiValue,
                        result: *mut NapiValue)
                        -> NapiStatus;
    fn napi_is_construct_call(env: NapiEnv,
                              cbinfo: NapiCallbackInfo,
                              result: *mut bool)
                              -> NapiStatus;
    fn napi_define_class(env: NapiEnv,
                         utf8name: CString,
                         constructor: NapiCallback,
                         data: *mut libc::c_void,
                         property_count: libc::size_t,
                         properties: *const NapiPropertyDescriptor,
                         result: *mut NapiValue)
                         -> NapiStatus;

    // Methods to work with external data objects
    fn napi_wrap(env: NapiEnv,
                 js_object: NapiValue,
                 native_object: *mut libc::c_void,
                 finalize_cb: NapiFinalize,
                 finalize_hint: *mut libc::c_void,
                 result: *mut NapiRef)
                 -> NapiStatus;
    fn napi_unwrap(env: NapiEnv,
                   js_object: NapiValue,
                   result: *mut *mut libc::c_void)
                   -> NapiStatus;
    fn napi_create_external(env: NapiEnv,
                            data: *mut libc::c_void,
                            finalize_cb: NapiFinalize,
                            finalize_hint: *mut libc::c_void,
                            result: *mut NapiValue)
                            -> NapiStatus;
    fn napi_get_value_external(env: NapiEnv,
                               value: NapiValue,
                               result: *mut *mut libc::c_void)
                               -> NapiStatus;

    // Methods to control object lifespan

    // Set initial_refcount to 0 for a weak reference, >0 for a strong reference.
    fn napi_create_reference(env: NapiEnv,
                             value: NapiValue,
                             initial_refcount: libc::uint32_t,
                             result: *mut NapiRef)
                             -> NapiStatus;

    // Deletes a reference. The referenced value is released, and may
    // be GC'd unless there are other references to it.
    fn napi_delete_reference(env: NapiEnv, reff: NapiRef) -> NapiStatus;

    // Increments the reference count, optionally returning the resulting count.
    // After this call the  reference will be a strong reference because its
    // refcount is >0, and the referenced object is effectively "pinned".
    // Calling this when the refcount is 0 and the object is unavailable
    // results in an error.
    fn napi_reference_ref(env: NapiEnv, reff: NapiRef, result: *mut libc::uint32_t) -> NapiStatus;

    // Decrements the reference count, optionally returning the resulting count.
    // If the result is 0 the reference is now weak and the object may be GC'd
    // at any time if there are no other references. Calling this when the
    // refcount is already 0 results in an error.
    fn napi_reference_unref(env: NapiEnv,
                            reff: NapiRef,
                            result: *mut libc::uint32_t)
                            -> NapiStatus;

    // Attempts to get a referenced value. If the reference is weak,
    // the value might no longer be available, in that case the call
    // is still successful but the result is NULL.
    fn napi_get_reference_value(env: NapiEnv, reff: NapiRef, result: *mut NapiValue) -> NapiStatus;

    fn napi_open_handle_scope(env: NapiEnv, result: *mut NapiHandleScope) -> NapiStatus;
    fn napi_close_handle_scope(env: NapiEnv, result: *mut NapiHandleScope) -> NapiStatus;
    fn napi_open_escapable_handle_scope(env: NapiEnv, result: *mut NapiHandleScope) -> NapiStatus;
    fn napi_close_escapable_handle_scope(env: NapiEnv, result: *mut NapiHandleScope) -> NapiStatus;
    fn napi_escape_handle(env: NapiEnv,
                          scope: NapiEscapableHandleScope,
                          escape: NapiValue,
                          result: *mut NapiValue)
                          -> NapiStatus;

    // Methods to support error handling
    fn napi_throw(env: NapiEnv, error: NapiValue) -> NapiStatus;
    fn napi_throw_error(env: NapiEnv, msg: CString) -> NapiStatus;
    fn napi_throw_type_error(env: NapiEnv, msg: CString) -> NapiStatus;
    fn napi_throw_range_error(env: NapiEnv, msg: CString) -> NapiStatus;
    fn napi_is_error(env: NapiEnv, value: NapiValue, result: *mut bool) -> NapiStatus;

    // Methods to support catching exceptions
    fn napi_is_exception_pending(env: NapiEnv, result: *mut bool) -> NapiStatus;
    fn napi_get_and_clear_last_exception(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;

    // Methods to provide node::Buffer functionality with napi types
    fn napi_create_buffer(env: NapiEnv,
                          length: libc::size_t,
                          data: *const *mut libc::c_void,
                          result: *mut NapiValue)
                          -> NapiStatus;
    fn napi_create_external_buffer(env: NapiEnv,
                                   length: libc::size_t,
                                   data: *const *mut libc::c_void,
                                   finalize_cb: NapiFinalize,
                                   result: *mut NapiValue)
                                   -> NapiStatus;
}

const NAPI_MODULE_VERSION: libc::c_int = 1;

#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

pub const MODULE_NAME: StaticCString = StaticCString(b"foo\0" as *const u8);


pub static _module: NapiModule = NapiModule {
    nm_version: NAPI_MODULE_VERSION,
    nm_flags: 0,
    nm_filename: MODULE_NAME,
    nm_register_func: register,
    nm_modname: MODULE_NAME,
    nm_priv: 0 as *const libc::c_void,
    reserved: [9, 9, 9, 9],
};

#[no_mangle]
pub extern "C" fn register(env: NapiEnv,
                           exports: NapiValue,
                           module: NapiValue,
                           priv_: *const libc::c_void) {
}

#[allow(improper_ctypes)]
#[cfg_attr(target_os = "linux", link_section = ".ctors")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
pub static register_foo: extern "C" fn() = {
    extern "C" fn __load_napi_module() {
        unsafe {
            napi_module_register(&NapiModule {
                nm_version: NAPI_MODULE_VERSION,
                nm_flags: 0,
                nm_filename: MODULE_NAME,
                nm_register_func: register,
                nm_modname: MODULE_NAME,
                nm_priv: 0 as *const libc::c_void,
                reserved: [9, 9, 9, 9],
            });
        }
    }
    __load_napi_module
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
