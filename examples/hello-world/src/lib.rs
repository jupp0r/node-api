#![feature(link_args)]
#[macro_use(napi_module)]
extern crate node_api;

napi_module!("testmod", register);

use node_api::{NapiEnv, NapiValue, FromNapiValues, ToNapiValue, NapiError};
use node_api::{create_function, set_named_property, create_object};

#[no_mangle]
pub extern "C" fn register(env: NapiEnv,
                           exports: NapiValue,
                           _module: NapiValue,
                           _priv: *mut std::os::raw::c_void) {
    let function = create_function(env, "foo", |_: NapiEnv, _: HelloArgs| {
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
    fn from_napi_args(_: NapiEnv, _: &[NapiValue]) -> Result<Self, NapiError> {
        Ok(HelloArgs {})
    }
}

struct HelloReturn {
    pub foo: String,
    pub bar: u64,
}
impl ToNapiValue for HelloReturn {
    fn to_napi_value(&self, env: NapiEnv) -> node_api::Result<NapiValue> {
        let object = create_object(env)?;
        let foo = self.foo.to_napi_value(env)?;
        let bar = self.bar.to_napi_value(env)?;
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
