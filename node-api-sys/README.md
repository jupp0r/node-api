# Node-API-sys

This crate contains the bindings to the N-API.
These are generated from [node_api.h](https://github.com/nodejs/node/blob/master/src/node_api.h) [using](https://rust-lang-nursery.github.io/rust-bindgen/) [bindgen](https://github.com/rust-lang-nursery/rust-bindgen).

## Updating the bindings

Run [build.sh](build.sh) to get the latest version of the headers from master and run bindgen on them.
Manually remove a bunch of system constants :(
Manually change the following:

```rust
pub type napi_env = *mut napi_env__;
// to
pub type napi_env = u64;

// and
pub type napi_value = *mut napi_value__;
// to
pub type napi_value = u64;
```
