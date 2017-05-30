use std::boxed::Box;

use napi::{NapiEnv, NapiValue, Result};
use napi_value::{ToNapiValue, FromNapiValues};

struct FutureContext {}

struct ThenArgs<S, R, E> {
    on_fulfilled: Box<FnOnce(NapiEnv, S) -> R>,
    on_rejected: Box<FnOnce(NapiEnv, E)>,
}

impl<S, R, E> FromNapiValues for ThenArgs<S, R, E> {
    fn from_napi_values(env: NapiEnv, values: &[NapiValue]) -> Result<Self> {
        Ok(ThenArgs {
               on_fulfilled: Box::new(|| {}),
               on_rejected: Box::new(|| {}),
           })
    }
}
