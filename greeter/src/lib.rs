wit_bindgen::generate!({
    world: "greeter",
});

struct Component;

impl Guest for Component {
    fn greet() -> String {
        let name = name();
        format!("Hello, {name}!")
    }
}

export!(Component);
