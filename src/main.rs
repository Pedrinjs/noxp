use noxp::http::{Method, Request, Response, StatusCode};
use noxp::thread::ThreadPool;
use noxp::Server;

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

fn main() -> std::io::Result<()> {
    let pool = ThreadPool::new(4);
    let mut server = Server::default().set_pool(pool).build();

    server.handle_func((Method::GET, "/"), index);

    server.handle_func((Method::GET, "/file"), file);

    server.handle_func((Method::GET, "/json"), json);

    server.listen_and_serve(6969)
}

fn index(mut res: Response, _req: Request) {
    res.write_string(StatusCode::OK, "Hello, World!");
}

fn file(mut res: Response, _req: Request) {
    res.write_file(StatusCode::OK, "hello.html");
}

fn json(mut res: Response, _req: Request) {
    let person = Person::new("Menezes", 15);
    res.write_json(StatusCode::OK, person);
}
