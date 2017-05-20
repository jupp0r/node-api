#![feature(link_args)]
#[macro_use(napi_module)]
extern crate node_api;

napi_module!("tests", register);

use node_api::{NapiEnv, NapiValue, FromNapiValues, ToNapiValue, NapiError};
use node_api::{create_function, set_named_property, create_object};

#[no_mangle]
pub extern "C" fn register(env: NapiEnv,
                           exports: NapiValue,
                           _module: NapiValue,
                           _priv: *mut std::os::raw::c_void) {
    let returns_objects_test =
        create_function(env, "returns_objects", &returns_objects).expect("error creating function");
    set_named_property(env, exports, "returns_objects", returns_objects_test).expect("error attaching function");
}

fn returns_objects(_: NapiEnv, _: ReturnsObjectsArgs) -> ReturnsObjectsReturn {
    ReturnsObjectsReturn {
        foo: "hello".to_string(),
        bar: 42,
    }
}

struct ReturnsObjectsArgs {}
impl FromNapiValues for ReturnsObjectsArgs {
    fn from_napi_args(_: NapiEnv, _: &[NapiValue]) -> Result<Self, NapiError> {
        Ok(ReturnsObjectsArgs {})
    }
}

struct ReturnsObjectsReturn {
    pub foo: String,
    pub bar: u64,
}
impl ToNapiValue for ReturnsObjectsReturn {
    fn to_napi_value(&self, env: NapiEnv) -> node_api::Result<NapiValue> {
        let object = create_object(env)?;
        let foo = self.foo.to_napi_value(env)?;
        let bar = self.bar.to_napi_value(env)?;
        set_named_property(env, object, "foo", foo)?;
        set_named_property(env, object, "bar", bar)?;
        Ok(object)
    }
}
