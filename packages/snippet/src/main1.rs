use std::sync::mpsc::{channel, Sender};

use wasmtime::component::*;
use wasmtime::Config;
use wasmtime::Store;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

mod bindings {
    use wasmtime::component::*;

    bindgen!( {
        world: "example",
        path: "../snippet-c/wit/"
    });
}

use bindings::{component::snippet_c::send::Host, Example};

pub struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
    sender: Sender<String>,
}

impl WasiView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl Host for MyState {
    fn send(&mut self, val: String) -> Result<(), u32> {
        self.sender.send(val).map_err(|_| 1)
    }
}

fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    // Instantiate the engine and store
    let engine = wasmtime::Engine::new(config.async_support(false))?;

    // Create a linker
    let mut linker = Linker::<MyState>::new(&engine);

    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    bindings::component::snippet_c::send::add_to_linker(&mut linker, |a| a)?;

    let (tx, rx) = channel();

    // Create a store
    let mut store = Store::new(
        &engine,
        MyState {
            ctx: WasiCtxBuilder::new()
                .inherit_stdin()
                .inherit_stdout()
                .build(),
            table: ResourceTable::new(),
            sender: tx,
        },
    );

    // Load component
    let component = wasmtime::component::Component::from_file(
        &engine,
        "target/wasm32-wasip1/release/snippet_c1.wasm",
    )?;

    std::thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("Echo message: {}", msg);
        }
    });

    // Instantiate the component
    let bindings = Example::instantiate(&mut store, &component, &linker)?;

    // Call the `greet` function
    let result = bindings.call_echo(&mut store)?;

    // This should print out `Greeting: [String("Hello, Alice!")]`
    println!("Greeting: {:?}", result);

    Ok(())
}
