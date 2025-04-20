wit_bindgen::generate!({
    world: "clang",
    path: "wit",
    with: {
        "snippet:plugin/language@0.1.0": generate,
        "snippet:plugin/compiler@0.1.0": generate,
        "snippet:plugin/types@0.1.0": generate,
    }
});

use exports::snippet::plugin::language::Compiler;
use exports::snippet::plugin::language::Guest as GuestLanguage;
use snippet::plugin::types::ErrorType;
use snippet::plugin::types::Optset;

pub struct Language;

impl GuestLanguage for Language {
    fn initialize_optset(optset: Optset) -> Result<(), ErrorType> {
        todo!()
    }

    fn compile(optset: Optset, compiler: Compiler) -> Result<(), ErrorType> {
        todo!()
    }

    fn name() -> _rt::String {
        "c".to_string()
    }
}

export!(Language with_types_in crate);
