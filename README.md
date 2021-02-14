# libcoap-sys

Raw Rust FFI bindings for [libcoap](https://libcoap.net/).

## How to build

### Requirements

Make sure the following programs are installed:

- `CMake` version 3.10 or greater.
- `Clang` version 3.9 or greater.

CMake is used to generate the necessary build configuration and build the static `libcoap-2.a` library.
Clang is used by bindgen to generate the bindings.

### Steps

```
git submodule update --init --recursive
cargo build -vv
cargo test
```
