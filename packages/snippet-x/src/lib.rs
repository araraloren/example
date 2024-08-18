wit_bindgen::generate!({
    world: "example",
    path: "wit",
    with: {
        "snippet:c/language@0.1.0": generate,
        "snippet:c/compiler@0.1.0": generate,
        "snippet:plugin/plugin@0.1.0": generate,
        "snippet:plugin/types@0.1.0": generate,
    }
});

use exports::snippet::c::language::Guest as GuestLanguage;
use exports::snippet::plugin::plugin::Guest as GuestPlugin;
use exports::snippet::plugin::plugin::PluginKind;
use snippet::c::compiler::Compiler;
use snippet::plugin::types::ErrorType;
use snippet::plugin::types::Optset;

pub struct Language;

impl GuestLanguage for Language {
    fn initialize_optset(optset: Optset) -> Result<(), ErrorType> {
        Ok(())
    }

    fn run(optset: Optset, compiler: Compiler) -> Result<(), ErrorType> {
        Ok(())
    }
}

impl GuestPlugin for Language {
    fn name() -> String {
        "c".to_string()
    }

    fn kind() -> PluginKind {
        PluginKind::Language
    }
}

export!(Language);
