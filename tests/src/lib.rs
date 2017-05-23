#![feature(link_args)]
#[macro_use(napi_module)]
extern crate node_api;

use node_api::{NapiEnv, NapiValue, FromNapiValues, ToNapiValue, NapiError, NapiErrorType};
use node_api::{create_function, get_named_property, set_named_property, create_object};

napi_module!("tests", register);

#[no_mangle]
pub extern "C" fn register(env: NapiEnv,
                           exports: NapiValue,
                           _module: NapiValue,
                           _priv: *mut std::os::raw::c_void) {
    register_test(env, "returns_objects", exports, &returns_objects);
    register_test(env, "returns_strings", exports, &returns_strings);
    register_test(env, "returns_numbers", exports, &returns_numbers);
    register_test(env, "returns_booleans", exports, &returns_booleans);
    register_test(env, "returns_arrays", exports, &returns_arrays);
    register_test(env, "receives_objects", exports, &receives_objects);
    register_test(env, "receives_strings", exports, &receives_strings);
    register_test(env, "receives_booleans", exports, &receives_booleans);
    register_test(env, "receives_f64", exports, &receives_f64);
    register_test(env, "receives_u64", exports, &receives_u64);
    register_test(env, "receives_i64", exports, &receives_i64);
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
fn returns_objects(_: NapiEnv, _: ()) -> Object {
    Object {
        foo: "hello".to_string(),
        bar: 42,
    }
}

#[derive(Debug)]
struct Object {
    pub foo: String,
    pub bar: u64,
}

impl ToNapiValue for Object {
    fn to_napi_value(&self, env: NapiEnv) -> node_api::Result<NapiValue> {
        let object = create_object(env)?;
        let foo = self.foo.to_napi_value(env)?;
        let bar = self.bar.to_napi_value(env)?;
        set_named_property(env, object, "foo", foo)?;
        set_named_property(env, object, "bar", bar)?;
        Ok(object)
    }
}

impl FromNapiValues for Object {
    fn from_napi_values(env: NapiEnv, napi_values: &[NapiValue]) -> node_api::Result<Object> {
        match napi_values.len() {
            1 => {
                let object = napi_values[0];
                let foo_property = get_named_property(env, object, "foo")?;
                let bar_property = get_named_property(env, object, "bar")?;
                Ok(Object {
                       foo: FromNapiValues::from_napi_values(env, &[foo_property])?,
                       bar: FromNapiValues::from_napi_values(env, &[bar_property])?,
                   })
            }
            n => {
                Err(NapiError {
                        error_message: "expected one argument, got ".to_string() + &n.to_string(),
                        engine_error_code: 0,
                        error_code: NapiErrorType::InvalidArg,
                    })
            }
        }
    }
}

#[derive(Debug)]
struct ReceivesObjectsArgs {
    pub arg0: Object,
}

impl FromNapiValues for ReceivesObjectsArgs {
    fn from_napi_values(env: NapiEnv,
                        napi_values: &[NapiValue])
                        -> node_api::Result<ReceivesObjectsArgs> {
        let arg0 = Object::from_napi_values(env, napi_values)?;
        Ok(ReceivesObjectsArgs { arg0: arg0 })
    }
}

fn returns_strings(_: NapiEnv, _: ()) -> String {
    "returned_string".to_string()
}

fn returns_numbers(_: NapiEnv, _: ()) -> u64 {
    42
}

fn returns_booleans(_: NapiEnv, _: ()) -> bool {
    true
}

fn returns_arrays(_: NapiEnv, _: ()) -> Vec<&'static str> {
    vec!["one", "two", "three"]
}

fn receives_objects(_: NapiEnv, args: ReceivesObjectsArgs) -> Object {
    args.arg0
}

fn receives_strings(_: NapiEnv, arg: String) -> String {
    arg
}

fn receives_booleans(_: NapiEnv, arg: bool) -> bool {
    arg
}

fn receives_f64(_: NapiEnv, arg: f64) -> f64 {
    arg
}

fn receives_u64(_: NapiEnv, arg: u64) -> u64 {
    arg
}

fn receives_i64(_: NapiEnv, arg: i64) -> i64 {
    arg
}
