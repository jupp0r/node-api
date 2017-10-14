use std::{result, ffi, string};
use std::ffi::CStr;

use node_api_sys::*;
pub type Result<T> = result::Result<T, NapiError>;

fn make_generic_napi_error(message: &str) -> NapiError {
    NapiError {
        error_message: message.to_string(),
        engine_error_code: 0,
        error_code: NapiErrorType::GenericFailure,
    }
}

#[derive(Debug, Clone)]
pub struct NapiError {
    pub error_message: String,
    pub engine_error_code: u32,
    pub error_code: NapiErrorType,
}

impl From<napi_extended_error_info> for NapiError {
    fn from(error: napi_extended_error_info) -> Self {
        unsafe {
            Self {
                error_message: CStr::from_ptr(error.error_message)
                    .to_string_lossy()
                    .into_owned(),
                engine_error_code: error.engine_error_code,
                error_code: NapiErrorType::from(error.error_code),
            }
        }
    }
}

impl From<ffi::NulError> for NapiError {
    fn from(_: ffi::NulError) -> Self {
        make_generic_napi_error("string must not contain 0 byte")
    }
}

impl From<string::FromUtf8Error> for NapiError {
    fn from(err: string::FromUtf8Error) -> Self {
        make_generic_napi_error(&format!("{:?}", err))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NapiErrorType {
    InvalidArg,
    ObjectExpected,
    StringExpected,
    NameExpected,
    FunctionExpected,
    NumberExpected,
    BooleanExpected,
    ArrayExpected,
    GenericFailure,
    PendingException,
    Cancelled,
    StatusLast,
}

impl From<napi_status> for NapiErrorType {
    fn from(s: napi_status) -> Self {
        match s {
            napi_status::napi_invalid_arg => NapiErrorType::InvalidArg,
            napi_status::napi_object_expected => NapiErrorType::ObjectExpected,
            napi_status::napi_string_expected => NapiErrorType::StringExpected,
            napi_status::napi_name_expected => NapiErrorType::NameExpected,
            napi_status::napi_function_expected => NapiErrorType::FunctionExpected,
            napi_status::napi_number_expected => NapiErrorType::NumberExpected,
            napi_status::napi_boolean_expected => NapiErrorType::BooleanExpected,
            napi_status::napi_array_expected => NapiErrorType::ArrayExpected,
            napi_status::napi_generic_failure => NapiErrorType::GenericFailure,
            napi_status::napi_pending_exception => NapiErrorType::PendingException,
            napi_status::napi_cancelled => NapiErrorType::Cancelled,
            _ => NapiErrorType::GenericFailure,
        }
    }
}