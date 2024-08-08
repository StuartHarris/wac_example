# Example WebAssembly Component Model composition using `wac`

Demonstrates composing a `name` component and a `greeter` component with either:

1. a `cli` component
2. a `http` component

Updated to use the `wasm32-wasip2` target in Rust nightly.

## Setup

* Rust nightly — `rustup toolchain install nightly`
* [wac](https://github.com/bytecodealliance/wac) — `cargo binstall wac-cli`
* [wasmtime](https://github.com/bytecodealliance/wasmtime) — `brew install wasmtime`

----

### 1. The `name` component
reads the `NAME` environment variable and returns it.
```wit
world name {
    import wasi:cli/environment@0.2.0;
    export name: func() -> string;
}
```

```rust
impl Guest for Component {
    fn name() -> String {
        wasi::cli::environment::get_environment()
            .into_iter()
            .find(|(k, _)| k == "NAME")
            .map(|(_, v)| v)
            .unwrap_or("World".to_string())
    }
}
```

### 2. The `greeter` component
calls into the `name` component and returns a greeting.

```wit
world greeter {
  import name: func() -> string;
  export greet: func() -> string;
}
```

```rust
impl Guest for Component {
    fn greet() -> String {
        let name = name();
        format!("Hello, {name}!")
    }
}
```

----


## Run as a CLI

```sh
NAME=Stu ./cli.fish
```

outputs:
```txt
Hello, Stu!
```

----

## Run as an HTTP server

```sh
NAME=Stu ./http.fish
```

Then in a new terminal:

```sh
curl http://localhost:8080
```

outputs:
```txt
Hello, Stu!
```
