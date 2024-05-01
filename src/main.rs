use std::fmt;
use std::net::TcpStream;

use noxp::http::{
    response::ResponseWriter,
    Request,
    StatusCode,
};
use noxp::Server;

struct Person {
    name: String,
    age: i32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"name": "{}", "age": {}}}"#, self.name, self.age)
    }
}

fn main() -> std::io::Result<()> {
    let mut server = Server::default().set_pool(4).build();

    server.handle_func(("GET", "/"), index);
    server.handle_func(("POST", "/"), post);
    server.handle_func(("GET", "/file"), file);
    server.handle_func(("GET", "/json"), json);

    server.listen_and_serve("127.0.0.1:8080")
}

fn index(_req: Request, stream: TcpStream) {
    ResponseWriter::default()
        .set_status(StatusCode::OK)
        .set_text("Hello, World!")
        .build()
        .write(stream);
}

fn post(req: Request, stream: TcpStream) {
    req.print_body();

    ResponseWriter::default()
        .set_status(StatusCode::OK)
        .set_json(req.body())
        .build()
        .write(stream);
}

fn file(_req: Request, stream: TcpStream) {
    ResponseWriter::default()
        .set_status(StatusCode::OK)
        .set_html("hello.html")
        .build()
        .write(stream);
}

fn json(_req: Request, stream: TcpStream) {
    let person = Person {
        name: String::from("Menezes"),
        age: 15,
    };

    ResponseWriter::default()
        .set_status(StatusCode::OK)
        .set_json(person)
        .build()
        .write(stream);
}
