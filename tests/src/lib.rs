#![feature(link_args)]
#[macro_use(napi_module)]
extern crate node_api;

napi_module!("tests", register);

use node_api::{NapiEnv, NapiValue, FromNapiValues, ToNapiValue};
use node_api::{create_function, set_named_property, create_object};

#[no_mangle]
pub extern "C" fn register(env: NapiEnv,
                           exports: NapiValue,
                           _module: NapiValue,
                           _priv: *mut std::os::raw::c_void) {
    register_test(env, "returns_objects", exports, &returns_objects);
    register_test(env, "returns_strings", exports, &returns_strings);
    register_test(env, "returns_numbers", exports, &returns_numbers);
    register_test(env, "returns_booleans", exports, &returns_booleans);
}

fn register_test<F, A, R>(env: NapiEnv, name: &str, exports: NapiValue, f: F)
    where F: Fn(NapiEnv, A) -> R,
          A: FromNapiValues,
          R: ToNapiValue
{
    let test = create_function(env, name, f).unwrap();
    set_named_property(env, exports, name, test).unwrap();
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

// returns numbers
fn returns_numbers(_: NapiEnv, _: ()) -> u64 {
    42
}

// returns booleans
fn returns_booleans(_: NapiEnv, _: ()) -> bool {
    true
}
