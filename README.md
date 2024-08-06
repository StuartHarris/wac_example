## WebAssembly Component Model composition using `wac`

### Setup

* [cargo-component](https://github.com/bytecodealliance/cargo-component) — `cargo binstall cargo-component`
* [wasi-virt](https://github.com/bytecodealliance/WASI-Virt) — 
  ```sh
  git clone https://github.com/bytecodealliance/WASI-Virt
  cd WASI-Virt
  cargo build --release
  cp ./target/release/wasi-virt ~/.cargo/bin
  ```
* [wac](https://github.com/bytecodealliance/wac) — `cargo binstall wac-cli`
* [wasmtime](https://github.com/bytecodealliance/wasmtime) — `brew install wasmtime`
* [fish](https://fishshell.com/) — `brew install fish`

### Run

```sh
NAME=stu ./build_run.fish
```
