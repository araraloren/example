pub mod bindings {
    wasmtime::component::bindgen!({
        world: "example",
        path: "wit",
        tracing: true,
        with: {
            "sni:cli/language/language": crate::Language,
            "sni:cli/compiler/compiler": crate::Compiler,
        }
    });
}

pub use bindings::*;

use bindings::sni::cli::compiler::Mode;

pub struct Language;

impl Language {
    pub fn name(&self) -> String {
        "Rust".to_string()
    }

    pub fn select_compiler(&self, complier: Vec<&Compiler>) -> Option<usize> {
        todo!()
    }

    pub fn run(&self, args: Vec<String>, complier: Compiler) -> Result<(), String> {
        todo!()
    }
}

pub struct Compiler;

impl Compiler {
    pub fn name(&self) -> String {
        todo!()
    }

    pub fn langs(&self) -> Vec<String> {
        todo!()
    }

    pub fn set_lang(&mut self, lang: String) -> bool {
        todo!()
    }

    pub fn set_mode(&mut self, mode: Mode) -> bool {
        todo!()
    }
}
