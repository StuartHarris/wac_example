wit_bindgen::generate!({
    world: "example:http/host",
    path: [
        "../wit/deps/io",
        "../wit/deps/clocks",
        "../wit/deps/filesystem",
        "../wit/deps/sockets",
        "../wit/deps/random",
        "../wit/deps/cli",
        "../wit/deps/http",
        "wit",
    ],
    generate_all,
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

struct Component;

impl Guest for Component {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        let headers = Fields::new();
        let response = OutgoingResponse::new(headers);
        let body = response.body().expect("outgoing response");

        ResponseOutparam::set(response_out, Ok(response));

        let greeting = greet();

        let out = body.write().expect("outgoing stream");
        out.blocking_write_and_flush(greeting.as_bytes())
            .expect("writing response");
        drop(out);

        OutgoingBody::finish(body, None).unwrap();
    }
}

export!(Component);
