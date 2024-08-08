#!/usr/bin/env fish

set DIR target/wasm32-wasip2/release/

cargo +nightly build --target wasm32-wasip2 --release

wac compose http.wac \
    -o $DIR/http-output.wasm \
    --dep example:name=$DIR/name.wasm \
    --dep example:greeter=$DIR/greeter.wasm \
    --dep example:http=$DIR/http.wasm

wasmtime serve --wasi cli=y --env NAME $DIR/http-output.wasm

# spin up --from $DIR/http-output.wasm
