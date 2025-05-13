use noxp::*;
use noxp::middlewares::logger;
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let mut server = Server::new();
    server.handle_func(("GET", "/"), logger( Arc::new(index) ) );
    server.handle_func(("POST", "/"), Arc::new(post) );
    server.listen_and_serve("localhost:8080")
}

fn index(res: &mut ResponseWriter, _req: Request) {
    res.set_status(StatusCode::OK)
        .set_text("Hello, World!");
}

fn post(res: &mut ResponseWriter, req: Request) {
    req.print_body();

    res.set_status(StatusCode::OK)
        .set_json(req.body());
}
