wit_bindgen::generate!({
    world: "name",
    generate_all,
});

struct Component;

impl Guest for Component {
    fn name() -> String {
        wasi::cli::environment::get_environment()
            .into_iter()
            .find(|(k, _)| k == "NAME")
            .map(|(_, v)| v)
            .unwrap_or("World".to_string())
    }
}

export!(Component);
