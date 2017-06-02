use napi;
use futures::future;
use futures::future::Future;

use napi::NapiValueType;
use error::{Result, NapiError, NapiErrorType};
use napi_futures;

pub trait FromNapiValues: Sized {
    fn from_napi_values(napi::NapiEnv, napi::NapiValue, &[napi::NapiValue]) -> Result<Self>;
}

impl FromNapiValues for () {
    fn from_napi_values(_: napi::NapiEnv, _: napi::NapiValue, _: &[napi::NapiValue]) -> Result<Self> {
        Ok(())
    }
}

impl FromNapiValues for u64 {
    fn from_napi_values(env: napi::NapiEnv, _: napi::NapiValue, napi_values: &[napi::NapiValue]) -> Result<Self> {
        check_napi_args_length(env, napi_values, 1)?;
        let value = napi_values[0];
        check_napi_type(env, NapiValueType::Number, value)?;
        napi::get_value_uint32(env, value).map(|x| x as u64)
    }
}

impl FromNapiValues for i64 {
    fn from_napi_values(env: napi::NapiEnv, _: napi::NapiValue, napi_values: &[napi::NapiValue]) -> Result<Self> {
        check_napi_args_length(env, napi_values, 1)?;
        let value = napi_values[0];
        check_napi_type(env, NapiValueType::Number, value)?;
        napi::get_value_int64(env, value)
    }
}

impl FromNapiValues for String {
    fn from_napi_values(env: napi::NapiEnv, _: napi::NapiValue, napi_values: &[napi::NapiValue]) -> Result<Self> {
        check_napi_args_length(env, napi_values, 1)?;
        let value = napi_values[0];
        check_napi_type(env, NapiValueType::String, value)?;
        napi::get_value_string_utf8(env, value)
    }
}

impl FromNapiValues for bool {
    fn from_napi_values(env: napi::NapiEnv, _: napi::NapiValue, napi_values: &[napi::NapiValue]) -> Result<Self> {
        check_napi_args_length(env, napi_values, 1)?;
        let value = napi_values[0];
        check_napi_type(env, NapiValueType::Boolean, value)?;
        napi::get_value_bool(env, value)
    }
}

impl FromNapiValues for f64 {
    fn from_napi_values(env: napi::NapiEnv, _: napi::NapiValue, napi_values: &[napi::NapiValue]) -> Result<Self> {
        check_napi_args_length(env, napi_values, 1)?;
        let value = napi_values[0];
        check_napi_type(env, NapiValueType::Number, value)?;
        napi::get_value_double(env, value)
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

impl IntoNapiValue for () {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::get_undefined(env)
    }
}

impl IntoNapiValue for String {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_string_utf8(env, self)
    }
}

impl<'a> IntoNapiValue for &'a str {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_string_utf8(env, self)
    }
}

impl IntoNapiValue for u8 {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, self as f64)
    }
}

impl IntoNapiValue for u16 {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, self as f64)
    }
}

impl IntoNapiValue for u32 {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, self as f64)
    }
}

impl IntoNapiValue for u64 {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, self as f64)
    }
}

impl IntoNapiValue for i8 {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, self as f64)
    }
}

impl IntoNapiValue for i16 {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, self as f64)
    }
}

impl IntoNapiValue for i32 {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, self as f64)
    }
}

impl IntoNapiValue for i64 {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, self as f64)
    }
}

impl IntoNapiValue for f32 {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, self as f64)
    }
}

impl IntoNapiValue for f64 {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::create_number(env, self as f64)
    }
}

impl IntoNapiValue for bool {
    fn into_napi_value(self, env: napi::NapiEnv) -> Result<napi::NapiValue> {
        napi::get_boolean(env, self)
    }
}

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
                future::result::<(),()>(Ok(())).boxed()
            }).boxed().wait().unwrap();
        })?;
        napi::set_named_property(env, obj, "then", then)?;
        napi::set_named_property(env, obj, "state", state)?;
        Ok(obj)
    }
}
