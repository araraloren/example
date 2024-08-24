cargo build --target wasm32-wasip1 --manifest-path ./packages/compiler-c/Cargo.toml 
wasm-tools component new .\target\wasm32-wasip1\debug\compiler_c.wasm -o compiler_c.wasm --adapt .\packages\wasi_snapshot_preview1.wasm
cargo build --target wasm32-wasip1 --manifest-path ./packages/language-c/Cargo.toml 
wasm-tools component new .\target\wasm32-wasip1\debug\language_c.wasm -o language_c.wasm --adapt .\packages\wasi_snapshot_preview1.wasm