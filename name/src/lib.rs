wit_bindgen::generate!({
    world: "example:name/name",
    path: [
        "../wit/deps/io",
        "../wit/deps/clocks",
        "../wit/deps/filesystem",
        "../wit/deps/sockets",
        "../wit/deps/random",
        "../wit/deps/cli",
        "wit",
    ],
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
