pub mod bindings {
    wit_bindgen::generate!({
        world: "compiler",
        path: "wit",
        with: {
            "sni:cli/compiler@0.1.0": generate,
        }
    });
}

pub use bindings::*;

use exports::sni::cli::compiler::GuestCompiler;
use exports::sni::cli::compiler::Mode;

pub struct Compiler;

impl GuestCompiler for Compiler {
    #[doc = " The name of the compiler."]
    fn name(&self) -> String {
        todo!()
    }

    #[doc = " The language are supported by this compiler."]
    fn langs(&self) -> Vec<String> {
        todo!()
    }

    #[doc = " Set the language."]
    fn set_lang(&self, language: String) -> bool {
        todo!()
    }

    #[doc = " Set the compile mode."]
    fn set_mode(&self, mode: Mode) -> bool {
        todo!()
    }
}

impl bindings::exports::sni::cli::compiler::Guest for Compiler {
    type Compiler = Compiler;
}

impl Guest for Compiler {
    fn run() {
        todo!()
    }
}

bindings::export!(Compiler with_types_in bindings);

// use bindings::sni::cli::compiler::Mode;

// pub struct Language;

// impl bindings::sni::cli::language::HostLanguage for Language {
//     #[doc = " The name of the language interface."]
//     fn name(
//         &mut self,
//         self_: wasmtime::component::Resource<Language>,
//     ) -> wasmtime::component::__internal::String {
//         todo!()
//     }

//     #[doc = " Select a compiler for this language, return the index."]
//     fn select_compiler(
//         &mut self,
//         self_: wasmtime::component::Resource<Language>,
//         compiler: wasmtime::component::__internal::Vec<wasmtime::component::Resource<Compiler>>,
//     ) -> Option<u64> {
//         todo!()
//     }

//     #[doc = " Run the compiler."]
//     fn run(
//         &mut self,
//         self_: wasmtime::component::Resource<Language>,
//         args: wasmtime::component::__internal::Vec<wasmtime::component::__internal::String>,
//         compiler: wasmtime::component::Resource<Compiler>,
//     ) -> Result<(), wasmtime::component::__internal::String> {
//         todo!()
//     }

//     fn drop(&mut self, rep: wasmtime::component::Resource<Language>) -> wasmtime::Result<()> {
//         todo!()
//     }
// }

// pub struct Compiler;

// impl Compiler {
//     fn name(&self) -> String {
//         todo!()
//     }

//     fn langs(&self) -> Vec<String> {
//         todo!()
//     }

//     fn set_lang(&mut self, lang: String) -> bool {
//         todo!()
//     }

//     fn set_mode(&mut self, mode: Mode) -> bool {
//         todo!()
//     }
// }
