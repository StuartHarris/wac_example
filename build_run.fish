#!/usr/bin/env fish

pushd name
cargo component build --release
cd ./target/wasm32-wasip1/release/
wasi-virt --allow-env=NAME name.wasm --out virt.wasm
popd

pushd greeter
cargo component build --release
cd ./target/wasm32-wasip1/release/
wasi-virt greeter.wasm --out virt.wasm
popd

pushd cli
cargo component build --release
popd

wac compose example.wac -o output.wasm \
    --dep example:cli=cli/target/wasm32-wasip1/release/cli.wasm \
    --dep example:name=name/target/wasm32-wasip1/release/virt.wasm \
    --dep example:greeter=greeter/target/wasm32-wasip1/release/virt.wasm

wasmtime run --wasi cli=y --env NAME output.wasm
