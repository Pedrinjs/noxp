use std::fmt;

use noxp::{
    Server,
    http::{Response, Request, StatusCode} };

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
    let mut server = Server::default();

    server.r#use(|_: Request, res: Response| -> Response {
       res.header("Content-Security-Policy", r#"default-src 'self';base-uri 'self'"#)
           .header("Cross-Origin-Embedder-Policy", "require-corp")
           .header("Cross-Origin-Opener-Policy", "same-origin")
           .header("Cross-Origin-Resource-Policy", "same-origin")
    });

    server.handle_func(("GET", "/"), index);
    server.handle_func(("POST", "/"), post);
    server.handle_func(("GET", "/file"), file);
    server.handle_func(("GET", "/json"), json);

    server.listen_and_serve("127.0.0.1:8080")
}

fn index(_: Request, res: Response) -> Response {
    res.set_status(StatusCode::OK)
        .set_text("Hello, World!")
}

fn post(req: Request, res: Response) -> Response {
    match req.get_header("Authorization") {
        Some(_) => {
            req.print_body();

            res.set_status(StatusCode::Created)
                .set_json(req.body())
        },
        None => {
            res.set_status(StatusCode::Unauthorized)
                .header("WWW-Authenticate", r#"Basic realm="User Visible Realm", charset="UTF-8""#)
                .set_text("you need authorization!")
        }
    }
}

fn file(_: Request, res: Response) -> Response {
    res.set_status(StatusCode::OK)
        .set_html("hello.html")
}

fn json(_: Request, res: Response) -> Response {
    let person = Person {
        name: String::from("Menezes"),
        age: 15,
    };

    res.set_status(StatusCode::OK)
        .set_json(person)
}
