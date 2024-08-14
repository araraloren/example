use wasmtime::{component::*, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!( {
    world: "example",
    path: "../snippet-cpp/wit/"
});

pub struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

fn main() -> wasmtime::Result<()> {
    // Instantiate the engine and store
    let engine = wasmtime::Engine::default();

    // Create a linker
    let mut linker = Linker::<MyState>::new(&engine);

    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    // Create a WasiCtx
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();

    // Create a store
    let mut store = Store::new(
        &engine,
        MyState {
            ctx: wasi_ctx,
            table: ResourceTable::new(),
        },
    );

    // Load component
    let component = wasmtime::component::Component::from_file(
        &engine,
        "target/wasm32-wasip1/release/snippet_cpp.wasm",
    )?;

    // Instantiate the component
    let bindings = Example::instantiate(&mut store, &component, &linker)?;

    // Call the `greet` function
    let result = bindings.call_hello_world(&mut store)?;

    // This should print out `Greeting: [String("Hello, Alice!")]`
    println!("Greeting: {:?}", result);

    Ok(())
}
