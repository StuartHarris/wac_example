#[allow(warnings)]
mod bindings;

use bindings::greet;

fn main() {
    println!("{}", greet());
}
