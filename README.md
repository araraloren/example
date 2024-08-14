
# A example of how using wit

## Running

install cargo-component

`cargo install cargo-component`

build snippet-cpp

`cargo component build --manifest-path .\packages\snippet-cpp\Cargo.toml --release` 

run snippet-cpp

`cargo r --bin snippet`

## Help

* snippet-cpp/wit/deps is copied from directory wasmtime@23.0.0/crates/wasi/wit

* need add meta for cargo-component to \[package.metadata.component.target.dependencies\]