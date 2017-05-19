use std::vec::Vec;
use napi;

pub trait FromNapiArgs: Sized {
    fn from_napi_args(&[napi::NapiValue]) -> Option<Self>;
}

pub trait ToNapiArgs {
    fn to_napi_args(self) -> Option<Vec<napi::NapiValue>>;
}
