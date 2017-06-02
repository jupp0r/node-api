use std::boxed::Box;

use napi::{NapiEnv, NapiValue, call_function};
use napi_value::{IntoNapiValue, FromNapiValues};
use error::Result;

pub struct ThenArgs<T, E> {
    pub on_fulfilled: Box<Fn(NapiEnv, NapiValue, T) + Send>,
    pub on_rejected: Box<Fn(NapiEnv, NapiValue, E) + Send>,
}

#[allow(unused_variables, unused_must_use)]
impl<T, E> FromNapiValues for ThenArgs<T, E>
    where T: IntoNapiValue
{
    fn from_napi_values(env: NapiEnv, this: NapiValue, values: &[NapiValue]) -> Result<Self> {
        let fulfilled_function = values[0].clone();
        Ok(ThenArgs {
               on_fulfilled: Box::new(move |env, this, args| {
                                          let napi_args = [args.into_napi_value(env).unwrap()];
                                          call_function(env, this, fulfilled_function, &napi_args);
                                      }),
               on_rejected: Box::new(|_, _, _| {
                                         println!("onrejected called");
                                     }),
           })
    }
}
