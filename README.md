
# A example of how using wit

## Running snippet-cpp

install cargo-component

`cargo install cargo-component`

build snippet-cpp

`cargo component build --manifest-path .\packages\snippet-cpp\Cargo.toml --release` 

run snippet-cpp

`cargo r --bin snippet`

## Running snippet-c

install cargo-component

`cargo install cargo-component`

install wasm-tools

`cargo install wasm-tools`

build snippet-c

`cargo build --target wasm32-wasip1 --manifest-path .\packages\snippet-c\Cargo.toml --release`

convert module to component

`wasm-tools component new .\target\wasm32-wasip1\release\snippet_c.wasm -o .\target\wasm32-wasip1\release\snippet_c1.wasm --adapt .\packages\wasi_snapshot_preview1.wasm`

run snippet-c

`cargo r --bin snippet1`

## Help

* snippet-cpp/wit/deps is copied from directory wasmtime@23.0.0/crates/wasi/wit

* need add meta for cargo-component to \[package.metadata.component.target.dependencies\]