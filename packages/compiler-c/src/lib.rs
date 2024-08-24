wit_bindgen::generate!({
    world: "ccompiler",
    path: "wit",
    with: {
        "snippet:plugin/types@0.1.0": generate,
        "snippet:plugin/plugin@0.1.0": generate,
        "snippet:c/compiler@0.1.0": generate,
    }
});

use exports::snippet::c::compiler::GuestCompiler;
use exports::snippet::c::compiler::Standard;
use exports::snippet::plugin::plugin::PluginKind;
use snippet::plugin::types::ErrorType;
use snippet::plugin::types::Mode;

pub struct Compiler;

impl GuestCompiler for Compiler {
    fn new() -> Self {
        todo!()
    }

    #[doc = " Current arguments."]
    fn args(&self) -> _rt::Vec<_rt::String> {
        todo!()
    }

    #[doc = " Is the compiler in debug mode?"]
    fn debug(&self) -> bool {
        todo!()
    }

    #[doc = " Current compile mode."]
    fn mode(&self) -> Mode {
        todo!()
    }

    #[doc = " Set debug mode."]
    fn set_debug(&self, debug: bool) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Set the compile mode."]
    fn set_mode(&self, mode: Mode) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Set the optimize level."]
    fn set_opt_level(&self, level: u8) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Set the c standard."]
    fn set_standard(&self, std: Standard) -> Result<(), ErrorType> {
        todo!()
    }
}

impl exports::snippet::c::compiler::Guest for Compiler {
    type Compiler = Compiler;
}

impl exports::snippet::plugin::plugin::Guest for Compiler {
    fn name() -> String {
        "gcc".to_string()
    }

    fn kind() -> PluginKind {
        PluginKind::Compiler
    }
}

export!(Compiler with_types_in crate);
