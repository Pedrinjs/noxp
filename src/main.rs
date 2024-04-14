use noxp::http::{Method, Request, Response, StatusCode};
use noxp::thread::ThreadPool;
use noxp::Server;

fn main() -> std::io::Result<()> {
    let pool = ThreadPool::new(4);
    let mut server = Server::default().set_pool(pool).build();

    server.handle_func((Method::GET, "/"), index);

    server.handle_func((Method::GET, "/file"), file);

    server.listen_and_serve(6969)
}

fn index(mut res: Response, _req: Request) {
    res.write_string(StatusCode::OK, "Hello, World!");
}

fn file(mut res: Response, _req: Request) {
    res.write_file(StatusCode::OK, "hello.html");
}
