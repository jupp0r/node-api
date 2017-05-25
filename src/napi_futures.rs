use std::boxed::Box;

use napi::{NapiEnv, NapiValue, Result};
use napi_value::{IntoNapiValue, FromNapiValues};

struct FutureContext {}

pub struct ThenArgs<T, E> {
    pub on_fulfilled: Box<Fn(NapiEnv, T) + Send>,
    pub on_rejected: Box<Fn(NapiEnv, E) + Send>,
}

impl<T, E> FromNapiValues for ThenArgs<T, E>
    where T: IntoNapiValue
{
    fn from_napi_values(env: NapiEnv, this: NapiValue, values: &[NapiValue]) -> Result<Self> {

        Ok(ThenArgs {
               on_fulfilled: Box::new(|_, _| {}),
               on_rejected: Box::new(|_, _| {}),
           })
    }
}
