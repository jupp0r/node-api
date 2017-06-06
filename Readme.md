# Node-API [![Build Status](https://travis-ci.org/jupp0r/node-api.svg?branch=master)](https://travis-ci.org/jupp0r/node-api)

Using this crate, you'll be easily able to write native node modules in Rust!

## Design

I opted for using the
new
[stable, vm-agnostic node api](https://github.com/nodejs/abi-stable-node). It
is still in an experimental stage and will probably change quite a
bit. However, there are plenty of good reasons for using it:

* it's a plain C api, making the creation of Rust bindings a breeze
* once it's stable, ABI compatibility between node versions will make
  life for module maintainers easy
* the VM agnostic nature of the API enables use on more platforms (ChakraCore, SpiderMonkey)

Furthermore, this crate will make writing asynchronous modules much
easier. Due to the nature of NodeJS' event loop, there is almost no
point in providing synchronous native modules. Native modules either do

* compute-intensive work, in which case operations have to be performed in their own thread
* operations involving IO

In both cases, it's undesirable to block the node event loop.
Unfortunately, that's what most binding generators focus on. This
crate uses [futures-rs](https://github.com/alexcrichton/futures-rs) to
make writing asynchronous modules for compute or IO tasks much easier.

## Usage
This is a [cargo workspace](https://rust-lang.github.io/book/second-edition/ch14-03-cargo-workspaces.html). Simply run `cargo build --all`

## Status
pre-alpha, some parts work in a proof-of-concept way, but the crate cannot be consumed yet.

## License
MIT
