use std::fmt;
use std::net::TcpStream;

use noxp::http::{
    Method,
    Request,
    response::ResponseWriter,
    StatusCode
};
use noxp::Server;

struct Person {
    name: String,
    age: i32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ \"name\": \"{}\", \"age\": {} }}", self.name, self.age)
    }
}

fn main() -> std::io::Result<()> {
    let mut server = Server::default().set_pool(4).build();

    server.handle_func((Method::GET, "/"), index);
    server.handle_func((Method::GET, "/file"), file);
    server.handle_func((Method::GET, "/json"), json);

    server.listen_and_serve("127.0.0.1:6969")
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
