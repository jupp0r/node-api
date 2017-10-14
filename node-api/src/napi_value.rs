use napi;
use futures::future;
use futures::future::Future;

use napi::NapiValueType;
use error::{Result, NapiError, NapiErrorType};
use napi_futures;

pub trait FromNapiValue: Sized {
    fn from_napi_values(napi::NapiEnv, napi::NapiValue) -> Result<Self>;
}

pub trait FromNapiValues: Sized {
    fn from_napi_values(napi::NapiEnv, napi::NapiValue, &[napi::NapiValue]) -> Result<Self>;
}


macro_rules! impl_from_napi_values {
    ($t:ty, $from:expr, $get_value:expr) => {
        impl FromNapiValues for $t {
            fn from_napi_values(env: napi::NapiEnv, _: napi::NapiValue, napi_values: &[napi::NapiValue]) -> Result<$t> {
                check_napi_args_length(env, napi_values, 1)?;
                let value = napi_values[0];
                check_napi_type(env, $from, value)?;
                $get_value(env, value)
            }
        }
    }
}

impl_from_napi_values!(String, NapiValueType::String,  napi::get_value_string_utf8);
impl_from_napi_values!(i64,    NapiValueType::Number,  napi::get_value_int64);
impl_from_napi_values!(u64,    NapiValueType::Number,  get_value_uint64);
impl_from_napi_values!(bool,   NapiValueType::Boolean, napi::get_value_bool);
impl_from_napi_values!(f64,    NapiValueType::Number,  napi::get_value_double);


fn get_value_uint64(env: napi::NapiEnv, value: napi::NapiValue) -> Result<u64> {
    napi::get_value_uint32(env, value).map(|x| x as u64)
}

impl FromNapiValues for () {
    fn from_napi_values(_: napi::NapiEnv, _: napi::NapiValue, _: &[napi::NapiValue]) -> Result<Self> {
        Ok(())
    }
}

impl<T> FromNapiValues for Vec<T> where T: FromNapiValues {
    fn from_napi_values(env: napi::NapiEnv, this: napi::NapiValue, napi_values: &[napi::NapiValue]) -> Result<Self> {
        check_napi_args_length(env, napi_values, 1)?;
        let value = napi_values[0];

        if !napi::is_array(env, value)? {
            Err(NapiError{error_message: "expected array".to_string(),
                          engine_error_code: 0,
                          error_code: NapiErrorType::InvalidArg,
            })
        } else {
            let size = napi::get_array_length(env, value)?;
            let mut result = Vec::with_capacity(size);
            for i in 0..size {
                let ival = napi::get_element(env, value, i)?;
                result.push(FromNapiValues::from_napi_values(env, this, &[ival])?);
            }
            Ok(result)
        }
    }
}

fn check_napi_args_length(_env: napi::NapiEnv, napi_values: &[napi::NapiValue], expected_length: usize) -> Result<()> {
    let values_length = napi_values.len();
    if values_length == expected_length {
        Ok(())
    } else {
        Err(NapiError {
            error_message: format!("expected {} argument, got {}", expected_length, values_length),
            engine_error_code: 0,
            error_code: NapiErrorType::InvalidArg,
        })
    }
}

fn check_napi_type(env: napi::NapiEnv, expected_type: NapiValueType, value: napi::NapiValue) -> Result<()> {
    let value_type = napi::type_of(env, value)?;
    if expected_type == value_type {
        Ok(())
    } else {
        Err(NapiError {
                error_message: format!("expected argument to be of type {:?}, but found it to be of type {:?}", expected_type, value_type),
                engine_error_code: 0,
            error_code: NapiErrorType::InvalidArg,
        })
    }
}


pub trait IntoNapiValue {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue>;
}

macro_rules! impl_into_napi_values {
    ($t:ty, $get_value:expr) => {
        impl IntoNapiValue for $t {
            fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
                $get_value(env, self)
            }
        }
    }
}

impl_into_napi_values!((), |env, _| napi::get_undefined(env));
impl_into_napi_values!(String, napi::create_string_utf8);

impl<'a> IntoNapiValue for &'a str {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_string_utf8(env, self)
    }
}

impl_into_napi_values!(u8,  |env, s| napi::create_u32(env, s as u32));
impl_into_napi_values!(u16, |env, s| napi::create_u32(env, s as u32));
impl_into_napi_values!(u32, |env, s| napi::create_u32(env, s as u32));

impl_into_napi_values!(i8,  |env, s| napi::create_i64(env, s as i64));
impl_into_napi_values!(i16, |env, s| napi::create_i64(env, s as i64));
impl_into_napi_values!(i32, |env, s| napi::create_i64(env, s as i64));
impl_into_napi_values!(i64, |env, s| napi::create_i64(env, s as i64));

impl_into_napi_values!(f32, |env, s| napi::create_double(env, s as f64));
impl_into_napi_values!(f64, |env, s| napi::create_double(env, s));

impl_into_napi_values!(bool,  napi::get_boolean);


impl<'a, T> IntoNapiValue for &'a [T]
    where T: IntoNapiValue + Clone
{
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        let array = napi::array_with_length(env, self.len())?;

        let mut index: usize = 0;
        for item in self.into_iter() {
            let converted_item = item.clone().into_napi_value(env)?;
            napi::set_element(env, array, index, converted_item)?;
            index = index + 1;
        }
        Ok(array)
    }
}

impl<T> IntoNapiValue for Vec<T>
    where T: IntoNapiValue
{
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        let array = napi::array_with_length(env, self.len())?;

        let mut index: usize = 0;
        for item in self.into_iter() {
            let converted_item = item.into_napi_value(env)?;
            napi::set_element(env, array, index, converted_item)?;
            index = index + 1;
        }
        Ok(array)
    }
}

impl<T, E> IntoNapiValue for future::BoxFuture<T, E>
    where T: IntoNapiValue + 'static,
          E: IntoNapiValue + 'static,
{
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        let obj = napi::create_object(env)?;
        let state = napi::create_external(env, Box::new(self))?;
        let then = napi::create_function(env, "then", move |env, this, then_args: napi_futures::ThenArgs<T, E>| {
            let state = napi::get_named_property(env, this, "state").unwrap();
            let future: Box<future::BoxFuture<T,E>> = napi::get_value_external(env, state).unwrap();
            future.then(move |result| {
                match result {
                    Ok(val) => (then_args.on_fulfilled)(env, this, val),
                    Err(err) => (then_args.on_rejected)(env, this, err)
                }
                Box::new(future::result::<(),()>(Ok(())))
            }).boxed().wait().unwrap();
        })?;
        napi::set_named_property(env, obj, "then", then)?;
        napi::set_named_property(env, obj, "state", state)?;
        Ok(obj)
    }
}
