use std::boxed::Box;

use napi::{NapiEnv, NapiValue, Result};
use napi_value::{IntoNapiValue, FromNapiValues};

struct FutureContext {}

pub struct ThenArgs<T, E> {
    pub on_fulfilled: Box<FnOnce(NapiEnv, T)>,
    pub on_rejected: Box<FnOnce(NapiEnv, E)>,
}

impl<T, E> FromNapiValues for ThenArgs<T, E> {
    fn from_napi_values(env: NapiEnv, values: &[NapiValue]) -> Result<Self> {

        Ok(ThenArgs {
               on_fulfilled: Box::new(|_, _| {}),
               on_rejected: Box::new(|_, _| {}),
           })
    }
}
