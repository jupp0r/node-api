#![feature(link_args)]
#[macro_use(napi_module)]
extern crate node_api;

napi_module!("testmod", register);

use node_api::{NapiEnv, NapiValue, FromNapiValues, IntoNapiValue};
use node_api::{create_function, set_named_property, create_object};
use node_api::error::*;

#[no_mangle]
pub extern "C" fn register(env: NapiEnv, exports: NapiValue, _module: NapiValue, _priv: *mut std::os::raw::c_void) {
    let function = create_function(env, "foo", |_: NapiEnv, _: NapiValue, ()| {
        HelloReturn {
            foo: "hello".to_string(),
            bar: 42,
        }
    })
            .expect("error creating function");
    set_named_property(env, exports, "hello", function).expect("error attaching function");
}

struct HelloArgs {}
impl FromNapiValues for HelloArgs {
    fn from_napi_values(_: NapiEnv, _: NapiValue,  _: &[NapiValue]) -> Result<Self> {
        Ok(HelloArgs {})
    }
}

struct HelloReturn {
    pub foo: String,
    pub bar: u64,
}
impl IntoNapiValue for HelloReturn {
    fn into_napi_value(self, env: NapiEnv) -> Result<NapiValue> {
        let object = create_object(env)?;
        let foo = self.foo.into_napi_value(env)?;
        let bar = self.bar.into_napi_value(env)?;
        set_named_property(env, object, "foo", foo)?;
        set_named_property(env, object, "bar", bar)?;
        Ok(object)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
