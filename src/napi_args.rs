use std::vec::Vec;
use napi;

pub trait FromNapiArgs {
    fn from_napi_args(Vec<napi::NapiValue>) -> Self;
}

pub trait ToNapiArgs {
    fn to_napi_args(self) -> Vec<napi::NapiValue>;
}
