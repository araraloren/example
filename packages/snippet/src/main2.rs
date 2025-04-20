// use bindings::Clang;
use bindings::snippet::plugin::types::Lang;
use bindings::Root;
use std::path::PathBuf;
use wasmtime::component::*;
use wasmtime::Config;
use wasmtime::Store;
use wasmtime_wasi::WasiImpl;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

mod bindings {
    wasmtime::component::bindgen!( {
        world: "root",
        path: "../compiler-c/wit",
        with: {
            "snippet:plugin/types/optset": crate::Optset,
            "snippet:plugin/types/services": crate::Services,
        },
        inline: "
            package root:component;

    world root {
      import snippet:plugin/types@0.1.0;

      export snippet:c/compiler@0.1.0;
      export snippet:plugin/plugin@0.1.0;
    }
            "
    });
}

use bindings::snippet::plugin::types::ErrorType;
use bindings::snippet::plugin::types::HostOptset;
use bindings::snippet::plugin::types::HostServices;
use bindings::snippet::plugin::types::Mode;

pub struct Optset {}

pub struct Services {}

pub struct Compiler {}

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

// #[async_trait::async_trait]
// impl HostOptset for MyState {
//     async fn add_opt(
//         &mut self,
//         self_: wasmtime::component::Resource<Optset>,
//         opt: wasmtime::component::__internal::String,
//     ) -> Result<u64, ErrorType> {
//         let optset = self.table().get(&self_).unwrap();

//         todo!()
//     }

//     fn drop(&mut self, rep: wasmtime::component::Resource<Optset>) -> wasmtime::Result<()> {
//         Ok(())
//     }
// }

impl<T: WasiView> HostOptset for WasiImpl<T> {
    fn add_opt(
        &mut self,
        self_: wasmtime::component::Resource<Optset>,
        opt: wasmtime::component::__internal::String,
    ) -> Result<u64, ErrorType> {
        let optset = self.table().get(&self_).unwrap();

        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Optset>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl<T: WasiView> HostServices for WasiImpl<T> {
    fn new(&mut self) -> wasmtime::component::Resource<Services> {
        todo!()
    }

    #[doc = " Is the compiler in debug mode?"]
    fn debug(&mut self, self_: wasmtime::component::Resource<Services>) -> Result<bool, ErrorType> {
        todo!()
    }

    #[doc = " Current language."]
    fn lang(&mut self, self_: wasmtime::component::Resource<Services>) -> Result<Lang, ErrorType> {
        todo!()
    }

    #[doc = " Current arguments."]
    fn args(
        &mut self,
        self_: wasmtime::component::Resource<Services>,
    ) -> Result<
        wasmtime::component::__internal::Vec<wasmtime::component::__internal::String>,
        ErrorType,
    > {
        todo!()
    }

    #[doc = " Current compile mode."]
    fn mode(&mut self, self_: wasmtime::component::Resource<Services>) -> Result<Mode, ErrorType> {
        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Services>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiView> bindings::snippet::plugin::types::Host for WasiImpl<T> {}

use cote::prelude::Cote;
use wac_graph::{types::Package, CompositionGraph, EncodeOptions};

#[derive(Debug, Cote)]
pub struct Cli {
    #[pos()]
    compiler: PathBuf,

    #[pos()]
    lang: PathBuf,
}

// NB: workaround some rustc inference - a future refactoring may make this
// obsolete.
fn type_annotate<T: WasiView, F>(val: F) -> F
where
    F: Fn(&mut T) -> WasiImpl<&mut T>,
{
    val
}

pub fn internal() -> wasmtime::Result<()> {
    let mut config = Config::new();
    // Instantiate the engine and store
    let engine = wasmtime::Engine::new(config.async_support(false))?;

    // Create a linker
    let mut linker = Linker::<MyState>::new(&engine);
    let closure = type_annotate::<MyState, _>(|t| WasiImpl(t));

    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    bindings::snippet::plugin::types::add_to_linker_get_host(&mut linker, closure)?;
    //bindings::snippet::c::compiler::add_to_linker(&mut linker, |a| a)?;

    // Create a store
    let mut store = Store::new(
        &engine,
        MyState {
            ctx: WasiCtxBuilder::new()
                .inherit_stdin()
                .inherit_stdout()
                .build(),
            table: ResourceTable::new(),
        },
    );

    // Load component
    let lang = Component::from_file(&engine, "out.wasm")?;
    //let compiler = Component::from_file(&engine, &compiler)?;

    // Instantiate the component
    let bindings = Root::instantiate(&mut store, &lang, &linker)?;

    // Call the `greet` function
    let result = bindings.snippet_plugin_plugin().call_name(&mut store)?;

    // This should print out `Greeting: [String("Hello, Alice!")]`
    println!("Greeting: {:?}", result);

    // let optset = store.data_mut().table().push(Optset {})?;
    // let compiler = store.data_mut().table().push(Compiler {})?;

    // let result = bindings
    //     .snippet_c_language()
    //     .call_run(&mut store, optset, compiler)
    //     .await?;

    // println!("--> {:?}", result);

    Ok(())
}

pub fn link_component(a: PathBuf, b: PathBuf) -> wasmtime::Result<()> {
    let mut graph = CompositionGraph::new();
    let compiler = Package::from_file("compiler_c", None, a, graph.types_mut())?;
    let compiler = graph.register_package(compiler)?;
    let language = Package::from_file("language_c", None, b, graph.types_mut())?;
    let language = graph.register_package(language)?;
    let compiler_ins = graph.instantiate(compiler);
    let language_ins = graph.instantiate(language);

    let comp_comp = graph.alias_instance_export(compiler_ins, "snippet:c/compiler@0.1.0")?;

    graph.set_instantiation_argument(language_ins, "snippet:c/compiler@0.1.0", comp_comp)?;

    let lang_plugin = graph.alias_instance_export(language_ins, "snippet:plugin/plugin@0.1.0")?;
    let lang_lang = graph.alias_instance_export(language_ins, "snippet:c/language@0.1.0")?;

    graph.export(lang_plugin, "snippet:plugin/plugin@0.1.0")?;
    graph.export(lang_lang, "snippet:c/language@0.1.0")?;
    graph.export(comp_comp, "snippet:c/compiler@0.1.0")?;

    let encoding = graph.encode(EncodeOptions::default())?;

    std::fs::write("out.wasm", encoding)?;

    Ok(())
}

fn main() -> wasmtime::Result<()> {
    let Cli { lang, compiler } = Cli::parse_env().unwrap();
    let engine = wasmtime::Engine::new(&Config::new())?;

    for x in Component::from_file(&engine, &lang)?
        .component_type()
        .exports(&engine)
    {
        dbg!(x);
    }

    link_component(compiler, lang)?;
    internal()?;
    Ok(())
}
