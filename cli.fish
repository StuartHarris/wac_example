#!/usr/bin/env fish

set DIR target/wasm32-wasip2/release/

cargo +nightly build --target wasm32-wasip2 --release

wac compose cli.wac \
    -o $DIR/cli-output.wasm \
    --dep example:name=$DIR/name.wasm \
    --dep example:greeter=$DIR/greeter.wasm \
    --dep example:cli=$DIR/cli.wasm

wasmtime run --env NAME $DIR/cli-output.wasm
