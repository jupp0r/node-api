#![feature(link_args)]
#[macro_use(napi_module)]
extern crate node_api;
extern crate futures;
extern crate tokio_core;

use node_api::{NapiEnv, NapiValue, FromNapiValues, IntoNapiValue};
use node_api::error::*;
use node_api::{
    create_function,
    get_named_property,
    set_named_property,
    create_object,
    create_external
};

use futures::future;
use futures::Future;

use tokio_core::reactor::Core;

napi_module!("tests", register);

#[no_mangle]
pub unsafe extern "C" fn register(env: NapiEnv, exports: NapiValue) -> usize {
    // create_and_attach_event_loop(env, exports);

    register_test(env, "returns_objects",   &returns_objects);
    register_test(env, "returns_strings",   &returns_strings);
    register_test(env, "returns_numbers",   &returns_numbers);
    register_test(env, "returns_booleans",  &returns_booleans);
    register_test(env, "returns_arrays",    &returns_arrays);

    register_test(env, "receives_objects",  &receives_objects);
    register_test(env, "receives_strings",  &receives_strings);
    register_test(env, "receives_booleans", &receives_booleans);
    register_test(env, "receives_f64",      &receives_f64);
    register_test(env, "receives_i64",      &receives_i64);
    register_test(env, "receives_arrays",   &receives_arrays);

    register_test(env, "returns_promises",  &returns_promises);

    exports
}

// fn create_and_attach_event_loop(env: NapiEnv, module: NapiValue) {
//    let core = Box::new(Core::new().unwrap());
//    let core_js = node_api::create_external(env, core).unwrap();
//}

fn register_test<F, A, R>(env: NapiEnv, name: &str, f: F)
    where F: Fn(NapiEnv, NapiValue, A) -> R,
          A: FromNapiValues,
          R: IntoNapiValue
{
    let test = create_function(env, name, f).unwrap();
    let exports = create_object(env).unwrap();
    set_named_property(env, exports, name, test).unwrap();
}

// returns objects
fn returns_objects(_: NapiEnv, _: NapiValue, _: ()) -> Object {
    Object {
        foo: "hello".to_string(),
        bar: 42,
    }
}

#[derive(Debug)]
struct Object {
    pub foo: String,
    pub bar: i64,
}

impl IntoNapiValue for Object {
    fn into_napi_value(self, env: NapiEnv) -> Result<NapiValue> {
        let object = create_object(env)?;
        let foo = self.foo.into_napi_value(env)?;
        let bar = self.bar.into_napi_value(env)?;
        set_named_property(env, object, "foo", foo)?;
        set_named_property(env, object, "bar", bar)?;
        Ok(object)
    }
}

impl FromNapiValues for Object {
    fn from_napi_values(env: NapiEnv,
                        this: NapiValue,
                        napi_values: &[NapiValue])
                        -> Result<Object> {
        match napi_values.len() {
            1 => {
                let object = napi_values[0];
                let foo_property = get_named_property(env, object, "foo")?;
                let bar_property = get_named_property(env, object, "bar")?;
                Ok(Object {
                       foo: FromNapiValues::from_napi_values(env, this, &[foo_property])?,
                       bar: FromNapiValues::from_napi_values(env, this, &[bar_property])?,
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
                        this: NapiValue,
                        napi_values: &[NapiValue])
                        -> Result<ReceivesObjectsArgs> {
        let arg0 = Object::from_napi_values(env, this, napi_values)?;
        Ok(ReceivesObjectsArgs { arg0: arg0 })
    }
}

fn returns_strings(_: NapiEnv, _: NapiValue, _: ()) -> String {
    "returned_string".to_string()
}

fn returns_numbers(_: NapiEnv, _: NapiValue, _: ()) -> f64 {
    42.1337
}

fn returns_booleans(_: NapiEnv, _: NapiValue, _: ()) -> bool {
    true
}

fn returns_arrays(_: NapiEnv, _: NapiValue, _: ()) -> Vec<&'static str> {
    vec!["one", "two", "three"]
}

fn receives_objects(_: NapiEnv, _: NapiValue, args: ReceivesObjectsArgs) -> Object {
    args.arg0
}

fn receives_strings(_: NapiEnv, _: NapiValue, arg: String) -> String {
    arg
}

fn receives_booleans(_: NapiEnv, _: NapiValue, arg: bool) -> bool {
    arg
}

fn receives_f64(_: NapiEnv, _: NapiValue, arg: f64) -> f64 {
    arg
}

fn receives_i64(_: NapiEnv, _: NapiValue, arg: i64) -> i64 {
    arg
}

fn receives_arrays(_: NapiEnv, _: NapiValue, arg: Vec<String>) -> Vec<String> {
    arg
}

fn returns_promises(_: NapiEnv, _: NapiValue, _arg: ()) -> futures::BoxFuture<(), ()> {
    future::ok(()).boxed()
}
