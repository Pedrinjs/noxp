# NOXP
***
![Badge em Desenvolvimento](http://img.shields.io/static/v1?label=STATUS&message=EM%20DESENVOLVIMENTO&color=GREEN&style=for-the-badge)
### 🦀 NOXP is a simple web framework for rust inspired by golang's net/http package
NOXP uses only the standard library

#### 🚧 What's next:
- [x] Make the user choose if the server single or multithreaded
- [ ] Use the Request struct for some usefull purpose
- [ ] Use a better routing system
- [ ] Add middleware and other status codes
- [ ] Use query strings
- [ ] Add dynamic routing
- [ ] Publish to crates.io
- [ ] Teach how to install noxp

#### Usage
```rust
use noxp::http::{Request, Response, StatusCode};
use noxp::server::Server;
use noxp::thread::ThreadPool;

fn main() -> std::io::Result<()> {
  let pool = ThreadPool::new(4); // threadpool with a finite number of threads (4)
  let mut server = Server::new(pool); // create the server with the threadpool

  // we are not using the request for now
  // the function is similar to golang's net/http
  server.handle_func("GET /", |mut res: Response, _req: Request| {
    res.write_string(StatusCode::OK, "Hello, World!");
  });

  // you can also send html (only in the views folder)
  server.handle_func("GET /hello", |mut res: Response, _req: Request| {
    res.write_file(StatusCode::OK, "hello.html");
  });

  // it will listen at the the local addres 127.0.0.1:8080
  // for incoming TCP streams
  server.listen_and_serve(8080);
}
```
