#![feature(link_args)]
#[macro_use(napi_module)]
extern crate node_api;

napi_module!("tests", register);

use node_api::{NapiEnv, NapiValue, ToNapiValue};
use node_api::{create_function, set_named_property, create_object};

#[no_mangle]
pub extern "C" fn register(env: NapiEnv,
                           exports: NapiValue,
                           _module: NapiValue,
                           _priv: *mut std::os::raw::c_void) {
    let returns_objects_test =
        create_function(env, "returns_objects", &returns_objects).expect("error creating function");
    set_named_property(env, exports, "returns_objects", returns_objects_test).expect("error attaching function");
    let returns_strings_test =
        create_function(env, "returns_strings", &returns_strings).expect("error creating function");
    set_named_property(env, exports, "returns_strings", returns_strings_test).expect("error attaching function");
}

// returns objects
fn returns_objects(_: NapiEnv, _: ()) -> ReturnsObjectsReturn {
    ReturnsObjectsReturn {
        foo: "hello".to_string(),
        bar: 42,
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

// returns strings
fn returns_strings(_: NapiEnv, _: ()) -> String {
    "returned_string".to_string()
}
