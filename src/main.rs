use noxp::http::{Request, Response, StatusCode};
use noxp::thread::ThreadPool;
use noxp::Server;

use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let pool = ThreadPool::new(4);
    let mut server = Server::new(Some(pool));

    server.handle_func("GET /", |mut res: Response, _req: Request| {
        res.write_string(StatusCode::OK, "Hello, World!");
    });

    server.handle_func("GET /file", |mut res: Response, _req: Request| {
        res.write_file(StatusCode::OK, "hello.html");
    });

    server.handle_func("GET /sleep", |mut res: Response, _req: Request| {
        thread::sleep(Duration::from_secs(5));
        res.write_string(StatusCode::OK, "Hello, sleepy");
    });

    server.listen_and_serve(6969)
}
