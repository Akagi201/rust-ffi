# rust-call-c-pointer

## Generate Rust bindings for C functions

```sh
# install llvm, clang first
cargo install bindgen-cli
bindgen --use-core mytime.h > mytime.rs
```
