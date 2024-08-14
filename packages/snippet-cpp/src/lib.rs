#[allow(warnings)]
mod bindings;

use bindings::{wasi::cli::environment::get_arguments, Guest};

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        let args = get_arguments();

        if args.len() > 1 {
            format!("Hello {}!", args[1])
        } else {
            "Nobody need hello!".to_string()
        }
    }
}

bindings::export!(Component with_types_in bindings);
