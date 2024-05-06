use std::fmt;

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
    let mut server = Server::default().set_pool(4);

    server.handle_func(("GET", "/"), index);
    server.handle_func(("POST", "/"), post);
    server.handle_func(("GET", "/file"), file);
    server.handle_func(("GET", "/json"), json);

    server.listen_and_serve("127.0.0.1:8080")
}

fn index(req: Request) -> Response {
    req.print_self();

    Response::new(StatusCode::OK).set_text("Hello, World!")
}

fn post(req: Request) -> Response {
    match req.get_header("Authorization") {
        Some(_) => {
            req.print_body();

            Response::new(StatusCode::Created).set_json(req.get_body())
        },
        None => {
            Response::new(StatusCode::Unauthorized)
                .header(
                    "WWW-Authenticate",
                    r#"Basic realm="User Visible Realm", charset="UTF-8""#
                ).set_text("you need authorization!")
        }
    }
}

fn file(_req: Request) -> Response {
    Response::new(StatusCode::OK).set_html("hello.html")
}

fn json(_req: Request) -> Response {
    let person = Person {
        name: String::from("Menezes"),
        age: 15,
    };

    Response::new(StatusCode::OK).set_json(person)
}
