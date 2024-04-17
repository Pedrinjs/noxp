use std::net::TcpStream;

use noxp::http::{
    Method,
    Request,
    response::{JSON, ResponseWriter},
    StatusCode
};
use noxp::thread::ThreadPool;
use noxp::Server;

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl JSON for Person {}

fn main() -> std::io::Result<()> {
    let pool = ThreadPool::new(4);
    let mut server = Server::default().set_pool(pool).build();

    server.handle_func((Method::GET, "/"), index);
    server.handle_func((Method::GET, "/file"), file);
    server.handle_func((Method::GET, "/json"), json);

    server.listen_and_serve(6969)
}

fn index(res: ResponseWriter, _req: Request, stream: TcpStream) {
    res.set_status(StatusCode::OK)
        .set_text("Hello, World!")
        .build()
        .write(stream);
}

fn file(res: ResponseWriter, _req: Request, stream: TcpStream) {
    res.set_status(StatusCode::OK)
        .set_html("hello.html")
        .build()
        .write(stream);
}

fn json(res: ResponseWriter, _req: Request, stream: TcpStream) {
    let person = Person {
        name: String::from("Menezes"),
        age: 15,
    };

    res.set_status(StatusCode::OK)
        .set_json(person)
        .build()
        .write(stream);
}
