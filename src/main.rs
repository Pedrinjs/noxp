use std::fmt;
use std::net::TcpStream;

use noxp::{ Server, http::{Response, Request, StatusCode} };

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
    Response::new(StatusCode::OK)
        .set_text("Hello, World!")
        .write(stream);
}

fn post(req: Request, stream: TcpStream) {
    match req.get_header("Authorization") {
        Some(_) => {
            req.print_body();

            Response::new(StatusCode::Created)
                .set_json(req.get_body())
                .write(stream);
        },
        None => {
            Response::new(StatusCode::Unauthorized)
                .header(
                    "WWW-Authenticate",
                    r#"Basic realm="User Visible Realm", charset="UTF-8""#
                ).set_text("you need authorization!").write(stream);
        }
    };
}

fn file(_req: Request, stream: TcpStream) {
    Response::new(StatusCode::OK)
        .set_html("hello.html")
        .write(stream);
}

fn json(_req: Request, stream: TcpStream) {
    let person = Person {
        name: String::from("Menezes"),
        age: 15,
    };

    Response::new(StatusCode::OK)
        .set_json(person)
        .write(stream);
}
