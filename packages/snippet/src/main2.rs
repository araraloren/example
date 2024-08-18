use bindings::Example;
use std::path::PathBuf;
use wasmtime::component::*;
use wasmtime::Config;
use wasmtime::Store;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

mod bindings {
    wasmtime::component::bindgen!( {
        world: "example",
        path: "../snippet-x/wit",
        async: true,
        with: {
            "snippet:plugin/types/optset": crate::Optset,
            "snippet:plugin/types/services": crate::Services,
            "snippet:c/compiler/compiler": crate::Compiler,
        }
    });
}

use bindings::snippet::c::compiler::HostCompiler;
use bindings::snippet::plugin::types::CompileMode;
use bindings::snippet::plugin::types::ErrorType;
use bindings::snippet::plugin::types::HostOptset;
use bindings::snippet::plugin::types::HostServices;

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

#[async_trait::async_trait]
impl HostOptset for MyState {
    async fn add_opt(
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

#[async_trait::async_trait]
impl HostServices for MyState {
    #[doc = " Is the compiler in debug mode?"]
    async fn debug(&mut self, self_: wasmtime::component::Resource<Services>) -> bool {
        todo!()
    }

    #[doc = " Current language."]
    async fn lang(
        &mut self,
        self_: wasmtime::component::Resource<Services>,
    ) -> wasmtime::component::__internal::String {
        todo!()
    }

    #[doc = " Current arguments."]
    async fn args(
        &mut self,
        self_: wasmtime::component::Resource<Services>,
    ) -> wasmtime::component::__internal::Vec<wasmtime::component::__internal::String> {
        todo!()
    }

    #[doc = " Current compile mode."]
    async fn mode(&mut self, self_: wasmtime::component::Resource<Services>) -> CompileMode {
        todo!()
    }

    #[doc = " Set the language."]
    async fn set_lang(
        &mut self,
        self_: wasmtime::component::Resource<Services>,
        language: wasmtime::component::__internal::String,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Set debug mode."]
    async fn set_debug(
        &mut self,
        self_: wasmtime::component::Resource<Services>,
        debug: bool,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Set the compile mode."]
    async fn set_compile_mode(
        &mut self,
        self_: wasmtime::component::Resource<Services>,
        mode: CompileMode,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Add an argument."]
    async fn add_arg(
        &mut self,
        self_: wasmtime::component::Resource<Services>,
        arg: wasmtime::component::__internal::String,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Append arguments."]
    async fn add_args(
        &mut self,
        self_: wasmtime::component::Resource<Services>,
        args: wasmtime::component::__internal::Vec<wasmtime::component::__internal::String>,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Services>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl bindings::snippet::plugin::types::Host for MyState {}

#[async_trait::async_trait]
impl HostCompiler for MyState {
    #[doc = " Is the compiler in debug mode?"]
    async fn debug(&mut self, self_: wasmtime::component::Resource<Compiler>) -> bool {
        todo!()
    }

    #[doc = " Current language."]
    async fn lang(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
    ) -> wasmtime::component::__internal::String {
        todo!()
    }

    #[doc = " Current arguments."]
    async fn args(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
    ) -> wasmtime::component::__internal::Vec<wasmtime::component::__internal::String> {
        todo!()
    }

    #[doc = " Current compile mode."]
    async fn mode(&mut self, self_: wasmtime::component::Resource<Compiler>) -> CompileMode {
        todo!()
    }

    #[doc = " Set the language."]
    async fn set_lang(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        language: wasmtime::component::__internal::String,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Set debug mode."]
    async fn set_debug(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        debug: bool,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Set the compile mode."]
    async fn set_compile_mode(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        mode: CompileMode,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Set the optimize level."]
    async fn set_optimize_level(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        level: u8,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Set the c standard."]
    async fn set_standard(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        std: wasmtime::component::__internal::String,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Add a macro."]
    async fn set_macro(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        macro_: wasmtime::component::__internal::String,
        value: Option<wasmtime::component::__internal::String>,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Add include path."]
    async fn set_include_path(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        path: wasmtime::component::__internal::String,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Add library path."]
    async fn set_library_path(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        path: wasmtime::component::__internal::String,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Link a library."]
    async fn link_library_path(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        library: wasmtime::component::__internal::String,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Add an argument."]
    async fn add_arg(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        arg: wasmtime::component::__internal::String,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Append arguments."]
    async fn add_args(
        &mut self,
        self_: wasmtime::component::Resource<Compiler>,
        args: wasmtime::component::__internal::Vec<wasmtime::component::__internal::String>,
    ) -> Result<(), ErrorType> {
        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Compiler>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl bindings::snippet::c::compiler::Host for MyState {}

use cote::prelude::ConfigValue;
use cote::prelude::Cote;
use cote::prelude::Ctor;
use cote::prelude::SetExt;

#[derive(Debug, Cote)]
pub struct Cli {
    #[pos()]
    path: PathBuf,
}

pub async fn internal() -> wasmtime::Result<()> {
    let Cli { path } = Cli::parse_env()?;

    let mut config = Config::new();
    // Instantiate the engine and store
    let engine = wasmtime::Engine::new(config.async_support(true))?;

    // Create a linker
    let mut linker = Linker::<MyState>::new(&engine);

    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    bindings::snippet::plugin::types::add_to_linker(&mut linker, |a| a)?;
    bindings::snippet::c::compiler::add_to_linker(&mut linker, |a| a)?;

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
    let component = Component::from_file(&engine, &path)?;

    // Instantiate the component
    let bindings = Example::instantiate_async(&mut store, &component, &linker).await?;

    // Call the `greet` function
    let result = bindings
        .snippet_plugin_plugin()
        .call_name(&mut store)
        .await?;

    // This should print out `Greeting: [String("Hello, Alice!")]`
    println!("Greeting: {:?}", result);

    let optset = store.data_mut().table().push(Optset {})?;
    let compiler = store.data_mut().table().push(Compiler {})?;

    let result = bindings
        .snippet_c_language()
        .call_run(&mut store, optset, compiler)
        .await?;

    println!("--> {:?}", result);

    Ok(())
}

pub async fn internal2() -> wasmtime::Result<()> {
    let Cli { path } = Cli::parse_env()?;

    let mut config = Config::new();
    // Instantiate the engine and store
    let engine = wasmtime::Engine::new(config.async_support(true))?;

    // Create a linker
    let mut linker = Linker::<MyState>::new(&engine);

    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    bindings::snippet::plugin::types::add_to_linker(&mut linker, |a| a)?;
    bindings::snippet::c::compiler::add_to_linker(&mut linker, |a| a)?;

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
    let component = Component::from_file(&engine, &path)?;

    // Instantiate the component
    let bindings = Example::instantiate_async(&mut store, &component, &linker).await?;

    // Call the `greet` function
    let result = bindings
        .snippet_plugin_plugin()
        .call_name(&mut store)
        .await?;

    // This should print out `Greeting: [String("Hello, Alice!")]`
    println!("Greeting: {:?}", result);

    let optset = store.data_mut().table().push(Optset {})?;
    let compiler = store.data_mut().table().push(Compiler {})?;

    let result = bindings
        .snippet_c_language()
        .call_run(&mut store, optset, compiler)
        .await?;

    println!("--> {:?}", result);

    Ok(())
}

#[tokio::main]
async fn main() -> wasmtime::Result<()> {
    internal().await?;
    Ok(())
}
