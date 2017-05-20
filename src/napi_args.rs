use napi;

pub trait FromNapiArgs: Sized {
    fn from_napi_args(napi::NapiEnv, &[napi::NapiValue]) -> Result<Self, napi::NapiError>;
}

pub trait ToNapiArgs {
    fn to_napi_args(&self, env: napi::NapiEnv) -> Result<napi::NapiValue, napi::NapiError> {
        napi::get_undefined(env)
    }
}

impl<T> ToNapiArgs for T
    where T: AsRef<str>
{
    fn to_napi_args(&self, env: napi::NapiEnv) -> Result<napi::NapiValue, napi::NapiError> {
        napi::create_string_utf8(env, self)
    }
}
