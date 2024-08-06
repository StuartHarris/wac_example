#[allow(warnings)]
mod bindings;

use bindings::{wasi::cli::environment, Guest};

struct Component;

impl Guest for Component {
    /// Say hello!
    fn name() -> String {
        environment::get_environment()
            .into_iter()
            .find(|(k, _)| k == "NAME")
            .map(|(_, v)| v)
            .unwrap_or("World".to_string())
    }
}

bindings::export!(Component with_types_in bindings);
