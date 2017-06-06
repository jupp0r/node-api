use std::ffi::CString;
use std::{ptr,mem,f64,usize,os};
use std::result::Result as StdResult;
use node_api_sys::*;

use napi_value::{FromNapiValues, IntoNapiValue};
use error::*;

pub type NapiEnv = napi_env;
pub type NapiRef = napi_ref;
pub type NapiValue = napi_value;

#[derive(Debug, Clone)]
pub struct NapiModule {
    pub version: i32,
    pub flags: u32,
    pub filename: String,
    pub register_func: napi_addon_register_func,
    pub modname: String,
}


#[derive(Debug, Clone, PartialEq)]
pub enum NapiValueType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Symbol,
    Object,
    Function,
    External,
}

impl From<napi_valuetype> for NapiValueType {
    fn from(s: napi_valuetype) -> Self {
        match s {
            napi_valuetype::napi_undefined => NapiValueType::Undefined,
            napi_valuetype::napi_null => NapiValueType::Null,
            napi_valuetype::napi_boolean => NapiValueType::Boolean,
            napi_valuetype::napi_number => NapiValueType::Number,
            napi_valuetype::napi_string => NapiValueType::String,
            napi_valuetype::napi_symbol => NapiValueType::Symbol,
            napi_valuetype::napi_object => NapiValueType::Object,
            napi_valuetype::napi_function => NapiValueType::Function,
            napi_valuetype::napi_external => NapiValueType::External,
        }
    }
}

pub fn napi_either<T>(env: NapiEnv, status: napi_status, val: T) -> Result<T> {
    match status {
        napi_status::napi_ok => Ok(val),
        _err => Err(get_last_napi_error(env).expect("error fetching last napi error")),
    }
}

fn get_last_error_info(env: napi_env) -> StdResult<napi_extended_error_info, napi_status> {
    unsafe {
        let mut info: *const napi_extended_error_info =
            Box::into_raw(Box::new(mem::uninitialized()));
        let status = napi_get_last_error_info(env, &mut info);
        match status {
            napi_status::napi_ok => Ok(*info),
            _ => Err(status),
        }
    }
}

fn get_last_napi_error(env: NapiEnv) -> StdResult<NapiError, NapiErrorType> {
    get_last_error_info(env)
        .map(|res| NapiError::from(res))
        .map_err(|err| NapiErrorType::from(err))
}

pub fn module_register(mod_: NapiModule) -> StdResult<(), NapiError> {
    let module = &mut napi_module {
                          nm_version: mod_.version,
                          nm_flags: mod_.flags,
                          nm_filename: CString::new(mod_.filename)?.as_ptr(),
                          nm_register_func: mod_.register_func,
                          nm_modname: try!(CString::new(mod_.modname)).as_ptr(),
                          nm_priv: ptr::null_mut(),
                          reserved: [ptr::null_mut(),
                                     ptr::null_mut(),
                                     ptr::null_mut(),
                                     ptr::null_mut()],
                      };
    unsafe {
        napi_module_register(module);
    }
    Ok(())
}

pub fn get_undefined(env: NapiEnv) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = mem::uninitialized();
        let status = napi_get_undefined(env, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn get_null(env: NapiEnv) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = mem::uninitialized();
        let status = napi_get_null(env, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn get_global(env: NapiEnv) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = mem::uninitialized();
        let status = napi_get_global(env, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn get_boolean(env: NapiEnv, value: bool) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = mem::uninitialized();
        let status = napi_get_boolean(env, value, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn create_object(env: NapiEnv) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = mem::uninitialized();
        let status = napi_create_object(env, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn create_array(env: NapiEnv) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = mem::uninitialized();
        let status = napi_create_array(env, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn array_with_length(env: NapiEnv, size: usize) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = mem::uninitialized();
        let status = napi_create_array_with_length(env, size, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn create_number(env: NapiEnv, value: f64) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = mem::uninitialized();
        let status = napi_create_number(env, value, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn create_string_utf8<T>(env: NapiEnv, val: T) -> Result<NapiValue>
    where T: AsRef<str>
{
    let mut napi_val: NapiValue = 0;
    let converted_value = CString::new(val.as_ref())?;
    let status = unsafe {
        napi_create_string_utf8(env,
                                converted_value.as_ptr(),
                                usize::MAX, // indicates 0-terminated string
                                &mut napi_val)
    };
    napi_either(env, status, napi_val)
}

//     pub fn napi_create_symbol(env: napi_env, description: napi_value,
//                               result: *mut napi_value) -> napi_status;

pub fn create_function<F, T, R>(env: NapiEnv, utf8name: &str, f: F) -> Result<NapiValue>
    where F: Fn(NapiEnv, NapiValue, T) -> R,
          T: FromNapiValues,
          R: IntoNapiValue
{
    unsafe extern "C" fn wrapper<F, T, R>(env: NapiEnv, cbinfo: napi_callback_info) -> NapiValue
        where F: Fn(NapiEnv, NapiValue, T) -> R,
              T: FromNapiValues,
              R: IntoNapiValue
    {
        let mut argc: usize = 16;
        let mut argv: [NapiValue; 16] = mem::uninitialized();
        let mut user_data = ptr::null_mut();
        let mut this: NapiValue = 0;
        let status = napi_get_cb_info(env,
                                      cbinfo,
                                      &mut argc,
                                      argv.as_mut_ptr(),
                                      &mut this,
                                      &mut user_data);
        assert!(status == napi_status::napi_ok);
        assert!(user_data != ptr::null_mut());

        let args =
            T::from_napi_values(env, this, &argv[0..argc]).expect("cannot convert arguments");

        let callback: Box<Option<F>> = Box::from_raw(user_data as *mut Option<F>);

        let return_value = callback.expect("no callback found")(env, this, args);
        return_value
            .into_napi_value(env)
            .unwrap_or(get_undefined(env).unwrap())
    }

    let boxed_f = Box::new(Some(f));
    let user_data = Box::into_raw(boxed_f) as *mut os::raw::c_void;
    let mut napi_val: NapiValue = 0;
    let name = CString::new(utf8name)?;
    let status = unsafe {
        napi_create_function(env,
                             name.into_raw(),
                             Some(wrapper::<F, T, R>),
                             user_data,
                             &mut napi_val)
    };
    napi_either(env, status, napi_val)
}


//     pub fn napi_create_error(env: napi_env, msg: napi_value,
//                              result: *mut napi_value) -> napi_status;


//     pub fn napi_create_type_error(env: napi_env, msg: napi_value,
//                                   result: *mut napi_value) -> napi_status;


//     pub fn napi_create_range_error(env: napi_env, msg: napi_value,
//                                    result: *mut napi_value) -> napi_status;


pub fn type_of(env: NapiEnv, napi_value: NapiValue) -> Result<NapiValueType> {
    let mut napi_value_type = napi_valuetype::napi_undefined;
    let status = unsafe { napi_typeof(env, napi_value, &mut napi_value_type) };
    napi_either(env, status, NapiValueType::from(napi_value_type))
}

pub fn get_value_double(env: NapiEnv, value: NapiValue) -> Result<f64> {
    let mut result: f64 = f64::NAN;
    let status = unsafe { napi_get_value_double(env, value, &mut result) };
    napi_either(env, status, result)
}

//     pub fn napi_get_value_int32(env: napi_env, value: napi_value,
//                                 result: *mut i32) -> napi_status;


pub fn get_value_uint32(env: NapiEnv, value: NapiValue) -> Result<u32> {
    let mut result: u32 = 0;
    let status = unsafe { napi_get_value_uint32(env, value, &mut result) };
    napi_either(env, status, result)
}

pub fn get_value_int64(env: NapiEnv, value: NapiValue) -> Result<i64> {
    let mut result: i64 = 0;
    let status = unsafe { napi_get_value_int64(env, value, &mut result) };
    napi_either(env, status, result)
}



pub fn get_value_bool(env: NapiEnv, value: NapiValue) -> Result<bool> {
    let mut result = false;
    let status = unsafe { napi_get_value_bool(env, value, &mut result) };
    napi_either(env, status, result)
}

//     pub fn napi_get_value_string_latin1(env: napi_env, value: napi_value,
//                                         buf: *mut ::std::os::raw::c_char,
//                                         bufsize: usize, result: *mut usize)
//      -> napi_status;


pub fn get_value_string_utf8(env: NapiEnv, value: NapiValue) -> Result<String> {
    let mut size: usize = 0;
    // obtain string length in bytes to determine buffer size
    let size_status =
        unsafe { napi_get_value_string_utf8(env, value, ptr::null_mut(), 0, &mut size) };
    napi_either(env, size_status, size)?;
    let mut buffer: Vec<u8> = Vec::with_capacity(size + 1);
    let mut written: usize = 0;
    let status = unsafe {
        napi_get_value_string_utf8(env,
                                   value,
                                   buffer.as_mut_ptr() as *mut i8,
                                   size + 1,
                                   &mut written)
    };
    match written == size {
        true => {
            napi_either(env, status, unsafe {
                buffer.set_len(size);
                String::from_utf8(buffer)?
            })
        }
        false => {
            Err(NapiError {
                    error_message: format!("buffer size mismatch, expected {}, got {}",
                                           size,
                                           written),
                    engine_error_code: 0,
                    error_code: NapiErrorType::GenericFailure,
                })
        }
    }
}



//     pub fn napi_get_value_string_utf16(env: napi_env, value: napi_value,
//                                        buf: *mut char16_t, bufsize: usize,
//                                        result: *mut usize) -> napi_status;


//     pub fn napi_coerce_to_bool(env: napi_env, value: napi_value,
//                                result: *mut napi_value) -> napi_status;


//     pub fn napi_coerce_to_number(env: napi_env, value: napi_value,
//                                  result: *mut napi_value) -> napi_status;


//     pub fn napi_coerce_to_object(env: napi_env, value: napi_value,
//                                  result: *mut napi_value) -> napi_status;


//     pub fn napi_coerce_to_string(env: napi_env, value: napi_value,
//                                  result: *mut napi_value) -> napi_status;


//     pub fn napi_get_prototype(env: napi_env, object: napi_value,
//                               result: *mut napi_value) -> napi_status;


//     pub fn napi_get_property_names(env: napi_env, object: napi_value,
//                                    result: *mut napi_value) -> napi_status;


//     pub fn napi_set_property(env: napi_env, object: napi_value,
//                              key: napi_value, value: napi_value)
//      -> napi_status;


//     pub fn napi_has_property(env: napi_env, object: napi_value,
//                              key: napi_value, result: *mut bool)
//      -> napi_status;


//     pub fn napi_get_property(env: napi_env, object: napi_value,
//                              key: napi_value, result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_set_named_property(env: napi_env, object: napi_value,
//                                    utf8name: *const ::std::os::raw::c_char,
//                                    value: napi_value) -> napi_status;
pub fn set_named_property(env: NapiEnv,
                          target: NapiValue,
                          name: &str,
                          value: NapiValue)
                          -> Result<()> {
    let status =
        unsafe { napi_set_named_property(env, target, CString::new(name)?.as_ptr(), value) };
    napi_either(env, status, ())
}

//     pub fn napi_has_named_property(env: napi_env, object: napi_value,
//                                    utf8name: *const ::std::os::raw::c_char,
//                                    result: *mut bool) -> napi_status;

pub fn get_named_property(env: NapiEnv, object: NapiValue, name: &str) -> Result<NapiValue> {
    let mut result: NapiValue = 0;
    let status =
        unsafe { napi_get_named_property(env, object, CString::new(name)?.as_ptr(), &mut result) };
    napi_either(env, status, result)
}

pub fn set_element(env: NapiEnv, array: NapiValue, index: usize, value: NapiValue) -> Result<()> {
    let status = unsafe { napi_set_element(env, array, index as u32, value) };
    napi_either(env, status, ())
}

//     pub fn napi_has_element(env: napi_env, object: napi_value, index: u32,
//                             result: *mut bool) -> napi_status;


pub fn get_element(env: NapiEnv, array: NapiValue, index: usize) -> Result<NapiValue> {
    let mut result: NapiValue = 0;
    let status = unsafe { napi_get_element(env, array, index as u32, &mut result) };
    napi_either(env, status, result)
}

//     pub fn napi_define_properties(env: napi_env, object: napi_value,
//                                   property_count: usize,
//                                   properties: *const napi_property_descriptor)
//      -> napi_status;


pub fn is_array(env: NapiEnv, value: NapiValue) -> Result<bool> {
    let mut result: bool = false;
    let status = unsafe { napi_is_array(env, value, &mut result) };
    napi_either(env, status, result)
}

pub fn get_array_length(env: NapiEnv, value: NapiValue) -> Result<usize> {
    let mut result: u32 = 0;
    let status = unsafe { napi_get_array_length(env, value, &mut result) };
    napi_either(env, status, result as usize)
}

//     pub fn napi_strict_equals(env: napi_env, lhs: napi_value, rhs: napi_value,
//                               result: *mut bool) -> napi_status;


//     pub fn napi_call_function(env: napi_env, recv: napi_value,
//                               func: napi_value, argc: usize,
//                               argv: *const napi_value,
//                               result: *mut napi_value) -> napi_status;
pub fn call_function(env: NapiEnv,
                     recv: NapiValue,
                     func: NapiValue,
                     args: &[NapiValue])
                     -> Result<NapiValue> {
    let mut result: NapiValue = 0;
    let status =
        unsafe { napi_call_function(env, recv, func, args.len(), args.as_ptr(), &mut result) };
    napi_either(env, status, result)
}



//     pub fn napi_new_instance(env: napi_env, constructor: napi_value,
//                              argc: usize, argv: *const napi_value,
//                              result: *mut napi_value) -> napi_status;


//     pub fn napi_instanceof(env: napi_env, object: napi_value,
//                            constructor: napi_value, result: *mut bool)
//      -> napi_status;


//     pub fn napi_make_callback(env: napi_env, recv: napi_value,
//                               func: napi_value, argc: usize,
//                               argv: *const napi_value,
//                               result: *mut napi_value) -> napi_status;


//     pub fn napi_get_cb_info(env: napi_env, cbinfo: napi_callback_info,
//                             argc: *mut usize, argv: *mut napi_value,
//                             this_arg: *mut napi_value,
//                             data: *mut *mut ::std::os::raw::c_void)
//      -> napi_status;


//     pub fn napi_is_construct_call(env: napi_env, cbinfo: napi_callback_info,
//                                   result: *mut bool) -> napi_status;


//     pub fn napi_define_class(env: napi_env,
//                              utf8name: *const ::std::os::raw::c_char,
//                              constructor: napi_callback,
//                              data: *mut ::std::os::raw::c_void,
//                              property_count: usize,
//                              properties: *const napi_property_descriptor,
//                              result: *mut napi_value) -> napi_status;


//     pub fn napi_wrap(env: napi_env, js_object: napi_value,
//                      native_object: *mut ::std::os::raw::c_void,
//                      finalize_cb: napi_finalize,
//                      finalize_hint: *mut ::std::os::raw::c_void,
//                      result: *mut napi_ref) -> napi_status;
pub fn wrap<T>(env: NapiEnv, js_object: NapiValue, native_object: Box<T>) -> Result<NapiRef> {
    let mut result: NapiRef = unsafe { mem::uninitialized() };
    let status = unsafe {
        napi_wrap(env,
                  js_object,
                  Box::into_raw(native_object) as *mut ::std::os::raw::c_void,
                  Some(finalize_box::<T>),
                  ptr::null_mut(),
                  &mut result)
    };
    napi_either(env, status, result)
}

//     pub fn napi_unwrap(env: napi_env, js_object: napi_value,
//                        result: *mut *mut ::std::os::raw::c_void)
//      -> napi_status;
pub fn unwrap<T>(env: NapiEnv, js_object: NapiValue) -> Result<Box<T>> {
    let mut result = ptr::null_mut();
    let status = unsafe { napi_unwrap(env, js_object, &mut result) };
    napi_either(env, status, unsafe { Box::<T>::from_raw(result as *mut T) })
}

//     pub fn napi_create_external(env: napi_env,
//                                 data: *mut ::std::os::raw::c_void,
//                                 finalize_cb: napi_finalize,
//                                 finalize_hint: *mut ::std::os::raw::c_void,
//                                 result: *mut napi_value) -> napi_status;
pub fn create_external<T>(env: NapiEnv, t: Box<T>) -> Result<NapiValue> {

    let mut result: NapiValue = 0;
    let status = unsafe {
        napi_create_external(env,
                             Box::into_raw(t) as *mut ::std::os::raw::c_void,
                             Some(finalize_box::<T>),
                             ptr::null_mut(),
                             &mut result)
    };
    napi_either(env, status, result)
}

unsafe extern "C" fn finalize_box<T>(_env: NapiEnv,
                                     finalize_data: *mut ::std::os::raw::c_void,
                                     _finalize_hint: *mut ::std::os::raw::c_void) {
    // move ownership into transient box in order to handle Drop, etc
    Box::from_raw(finalize_data as *mut T);
}

//     pub fn napi_get_value_external(env: napi_env, value: napi_value,
//                                    result: *mut *mut ::std::os::raw::c_void)
//      -> napi_status;
pub fn get_value_external<T>(env: NapiEnv, value: NapiValue) -> Result<Box<T>> {
    let mut result = ptr::null_mut();
    let status = unsafe { napi_get_value_external(env, value, &mut result) };
    napi_either(env, status, unsafe { Box::<T>::from_raw(result as *mut T) })
}


//     pub fn napi_create_reference(env: napi_env, value: napi_value,
//                                  initial_refcount: u32, result: *mut napi_ref)
//      -> napi_status;


//     pub fn napi_delete_reference(env: napi_env, ref_: napi_ref)
//      -> napi_status;


//     pub fn napi_reference_ref(env: napi_env, ref_: napi_ref, result: *mut u32)
//      -> napi_status;


//     pub fn napi_reference_unref(env: napi_env, ref_: napi_ref,
//                                 result: *mut u32) -> napi_status;


//     pub fn napi_get_reference_value(env: napi_env, ref_: napi_ref,
//                                     result: *mut napi_value) -> napi_status;


//     pub fn napi_open_handle_scope(env: napi_env,
//                                   result: *mut napi_handle_scope)
//      -> napi_status;


//     pub fn napi_close_handle_scope(env: napi_env, scope: napi_handle_scope)
//      -> napi_status;


//     pub fn napi_open_escapable_handle_scope(env: napi_env,
//                                             result:
//                                                 *mut napi_escapable_handle_scope)
//      -> napi_status;


//     pub fn napi_close_escapable_handle_scope(env: napi_env,
//                                              scope:
//                                                  napi_escapable_handle_scope)
//      -> napi_status;


//     pub fn napi_escape_handle(env: napi_env,
//                               scope: napi_escapable_handle_scope,
//                               escapee: napi_value, result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_throw(env: napi_env, error: napi_value) -> napi_status;


//     pub fn napi_throw_error(env: napi_env, msg: *const ::std::os::raw::c_char)
//      -> napi_status;


//     pub fn napi_throw_type_error(env: napi_env,
//                                  msg: *const ::std::os::raw::c_char)
//      -> napi_status;


//     pub fn napi_throw_range_error(env: napi_env,
//                                   msg: *const ::std::os::raw::c_char)
//      -> napi_status;


//     pub fn napi_is_error(env: napi_env, value: napi_value, result: *mut bool)
//      -> napi_status;


//     pub fn napi_is_exception_pending(env: napi_env, result: *mut bool)
//      -> napi_status;


//     pub fn napi_get_and_clear_last_exception(env: napi_env,
//                                              result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_create_buffer(env: napi_env, length: usize,
//                               data: *mut *mut ::std::os::raw::c_void,
//                               result: *mut napi_value) -> napi_status;


//     pub fn napi_create_external_buffer(env: napi_env, length: usize,
//                                        data: *mut ::std::os::raw::c_void,
//                                        finalize_cb: napi_finalize,
//                                        finalize_hint:
//                                            *mut ::std::os::raw::c_void,
//                                        result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_create_buffer_copy(env: napi_env, length: usize,
//                                    data: *const ::std::os::raw::c_void,
//                                    result_data:
//                                        *mut *mut ::std::os::raw::c_void,
//                                    result: *mut napi_value) -> napi_status;


//     pub fn napi_is_buffer(env: napi_env, value: napi_value, result: *mut bool)
//      -> napi_status;


//     pub fn napi_get_buffer_info(env: napi_env, value: napi_value,
//                                 data: *mut *mut ::std::os::raw::c_void,
//                                 length: *mut usize) -> napi_status;


//     pub fn napi_is_arraybuffer(env: napi_env, value: napi_value,
//                                result: *mut bool) -> napi_status;


//     pub fn napi_create_arraybuffer(env: napi_env, byte_length: usize,
//                                    data: *mut *mut ::std::os::raw::c_void,
//                                    result: *mut napi_value) -> napi_status;


//     pub fn napi_create_external_arraybuffer(env: napi_env,
//                                             external_data:
//                                                 *mut ::std::os::raw::c_void,
//                                             byte_length: usize,
//                                             finalize_cb: napi_finalize,
//                                             finalize_hint:
//                                                 *mut ::std::os::raw::c_void,
//                                             result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_get_arraybuffer_info(env: napi_env, arraybuffer: napi_value,
//                                      data: *mut *mut ::std::os::raw::c_void,
//                                      byte_length: *mut usize) -> napi_status;


//     pub fn napi_is_typedarray(env: napi_env, value: napi_value,
//                               result: *mut bool) -> napi_status;


//     pub fn napi_create_typedarray(env: napi_env, type_: napi_typedarray_type,
//                                   length: usize, arraybuffer: napi_value,
//                                   byte_offset: usize, result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_get_typedarray_info(env: napi_env, typedarray: napi_value,
//                                     type_: *mut napi_typedarray_type,
//                                     length: *mut usize,
//                                     data: *mut *mut ::std::os::raw::c_void,
//                                     arraybuffer: *mut napi_value,
//                                     byte_offset: *mut usize) -> napi_status;


//     pub fn napi_create_async_work(env: napi_env,
//                                   execute: napi_async_execute_callback,
//                                   complete: napi_async_complete_callback,
//                                   data: *mut ::std::os::raw::c_void,
//                                   result: *mut napi_async_work)
//      -> napi_status;


//     pub fn napi_delete_async_work(env: napi_env, work: napi_async_work)
//      -> napi_status;


//     pub fn napi_queue_async_work(env: napi_env, work: napi_async_work)
//      -> napi_status;


//     pub fn napi_cancel_async_work(env: napi_env, work: napi_async_work)
//      -> napi_status;
