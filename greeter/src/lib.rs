#[allow(warnings)]
mod bindings;

use bindings::{name, Guest};

struct Component;

impl Guest for Component {
    fn greet() -> String {
        let name = name();
        format!("Hello, {name}!")
    }
}

bindings::export!(Component with_types_in bindings);
