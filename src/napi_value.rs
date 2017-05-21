use napi;

use std::iter::Iterator;
use napi::Result;

pub trait FromNapiValues: Sized {
    fn from_napi_values(napi::NapiEnv, &[napi::NapiValue]) -> Result<Self>;
}

impl FromNapiValues for () {
    fn from_napi_values(_: napi::NapiEnv, _: &[napi::NapiValue]) -> Result<Self> {
        Ok(())
    }
}

pub trait ToNapiValue {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue>;
}

impl ToNapiValue for () {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::get_undefined(env)
    }
}

impl ToNapiValue for String {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_string_utf8(env, self)
    }
}

impl<'a> ToNapiValue for &'a str {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_string_utf8(env, self)
    }
}

impl ToNapiValue for u8 {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, *self as f64)
    }
}

impl ToNapiValue for u16 {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, *self as f64)
    }
}

impl ToNapiValue for u32 {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, *self as f64)
    }
}

impl ToNapiValue for u64 {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, *self as f64)
    }
}

impl ToNapiValue for i8 {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, *self as f64)
    }
}

impl ToNapiValue for i16 {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, *self as f64)
    }
}

impl ToNapiValue for i32 {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, *self as f64)
    }
}

impl ToNapiValue for i64 {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, *self as f64)
    }
}

impl ToNapiValue for f32 {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, *self as f64)
    }
}

impl ToNapiValue for f64 {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, *self as f64)
    }
}

impl ToNapiValue for bool {
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::get_boolean(env, *self)
    }
}

impl<T> ToNapiValue for [T]
    where T: ToNapiValue
{
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        let set_item_in_array = |env, array, index, item: &T| {
            item.to_napi_value(env)
                .and_then(|converted_item| napi::set_element(env, array, index, converted_item))
        };

        let fill_array_with_values = |array| {
            self.into_iter()
                .enumerate()
                .map(|(i, item)| set_item_in_array(env, array, i, item))
                .collect::<Result<Vec<()>>>()
                .map(|_| array)
        };

        napi::array_with_length(env, self.len()).and_then(fill_array_with_values)
    }
}

impl<T> ToNapiValue for Vec<T>
    where T: ToNapiValue
{
    fn to_napi_value(&self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        let set_item_in_array = |env, array, index, item: &T| {
            item.to_napi_value(env)
                .and_then(|converted_item| napi::set_element(env, array, index, converted_item))
        };

        let fill_array_with_values = |array| {
            self.into_iter()
                .enumerate()
                .map(|(i, item)| set_item_in_array(env, array, i, item))
                .collect::<Result<Vec<()>>>()
                .map(|_| array)
        };

        napi::array_with_length(env, self.len()).and_then(fill_array_with_values)
    }
}
