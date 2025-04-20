wit_bindgen::generate!({
    world: "ccompiler",
    path: "wit",
    with: {
        "snippet:plugin/types@0.1.0": generate,
        "snippet:plugin/compiler@0.1.0": generate,
    }
});

use exports::snippet::plugin::compiler::GuestCompiler;
use snippet::plugin::types::ErrorType;
use snippet::plugin::types::Lang;
use snippet::plugin::types::Mode;

pub struct Compiler;

impl GuestCompiler for Compiler {
    fn new() -> Self {
        todo!()
    }

    #[doc = " Current arguments."]
    fn args(&self) -> Vec<String> {
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
    fn set_standard(&self, std: String) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Add a macro."]
    fn add_macro(&self, macro_: String, value: Option<String>) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Add include path."]
    fn add_include_path(&self, path: String) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Add library path."]
    fn add_library_path(&self, path: String) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Link a library."]
    fn link_library_path(&self, library: String) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Add an argument."]
    fn add_arg(&self, arg: String) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Append arguments."]
    fn add_args(&self, args: Vec<String>) -> Result<(), ErrorType> {
        todo!()
    }

    #[doc = " Compile the code"]
    fn compile(&self, source: Vec<String>, out: String) -> Result<String, ErrorType> {
        todo!()
    }

    #[doc = " Compile the file"]
    fn compile_file(&self, path: String, out: String) -> Result<String, ErrorType> {
        todo!()
    }

    #[doc = " Link the object into executable"]
    fn link_object(&self, objs: Vec<String>, out: String) -> Result<String, ErrorType> {
        todo!()
    }
}

impl exports::snippet::plugin::compiler::Guest for Compiler {
    type Compiler = Compiler;

    fn bin() -> std::string::String {
        "gcc".to_string()
    }

    fn support_langs() -> _rt::Vec<Lang> {
        vec![Lang::C, Lang::Cxx]
    }
}

export!(Compiler with_types_in crate);
