use std::boxed::Box;

use napi::{NapiEnv, NapiValue, Result};
use napi_value::{ToNapiValue, FromNapiValues};

struct FutureContext {}

pub struct ThenArgs<T, E> {
    on_fulfilled: Box<FnOnce(NapiEnv, T)>,
    on_rejected: Box<FnOnce(NapiEnv, E)>,
}

impl<T, E> FromNapiValues for ThenArgs<T, E> {
    fn from_napi_values(env: NapiEnv, values: &[NapiValue]) -> Result<Self> {

        Ok(ThenArgs {
               on_fulfilled: Box::new(|_, _| {}),
               on_rejected: Box::new(|_, _| {}),
           })
    }
}
