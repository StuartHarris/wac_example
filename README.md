# Example WebAssembly Component Model composition using `wac`

Updated to use the `wasm32-wasip2` target in rust nightly.

Composition...

```wit
package example:composition;

let n = new example:name { ... };

let g = new example:greeter {
  name: n.name,
  ...
};

let c = new example:cli {
  greet: g.greet,
  ...
};

export c.run;
```

... of 3 components:

### 1. `name`
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

### 2. `greeter`
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

### 3. `cli`
a `wasi:cli` component, which calls into the `greeter` component and prints the greeting.

```wit
world host {
  import greet: func() -> string;
}
```

```rust
fn main() {
    println!("{}", greet());
}
```

----

## Setup

* rust nightly — `rustup toolchain install nightly`
* [wac](https://github.com/bytecodealliance/wac) — `cargo binstall wac-cli`
* [wasmtime](https://github.com/bytecodealliance/wasmtime) — `brew install wasmtime`

----

## Run

```sh
NAME=Stu ./build_run.fish
```

outputs:
```txt
Hello, Stu!
```
