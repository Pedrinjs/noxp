# NOXP
***
![Badge em Desenvolvimento](http://img.shields.io/static/v1?label=STATUS&message=EM%20DESENVOLVIMENTO&color=GREEN&style=for-the-badge)
### 🦀 NOXP is a simple web framework for rust inspired by golang's net/http package
NOXP uses only the standard library

#### 🚧 What's next:
- [x] Make the user choose if the server single or multithreaded
- [ ] Use the Request struct for some usefull purpose
- [x] Use a better routing system
- [ ] Add middleware
- [ ] More status codes
- [ ] More http methods
- [ ] Use query strings
- [ ] Publish to crates.io
- [ ] Teach how to install noxp

#### Usage
```rust
use noxp::http::{Method, Request, Response, StatusCode};
use noxp::thread::ThreadPool;
use noxp::Server;

#[derive(Debug)]
struct Person {
  name: String,
  age: i32,
}

fn main() -> std::io::Result<()> {
  let pool = ThreadPool::new(4); // threadpool with a finite number of threads (4)
  let mut server = Server::default().set_pool(pool).build(); // create the server with the threadpool

  // pay attention for the tuple (Method, &str)
  // the function is similar to golang's net/http
  server.handle_func((Method::GET, "/"), index);

  // you can also send html (only in the views folder)
  server.handle_func((Method::GET, "/hello"), |mut res: Response, _req: Request| {
    res.write_file(StatusCode::OK, "hello.html");
  });

  server.handle_func((Method::GET, "/person"), |mut res: Response, _req: Request| {
    let person = Person {
      name: String::from("John Doe"),
      age: 99,
    };

    // you can send json, but works only for structs that derive Debug
    res.write_json(StatusCode::OK, person);
  });

  // it will listen at the the local addres 127.0.0.1:8080
  // for incoming TCP streams
  server.listen_and_serve(8080)
}

fn index(mut res: Response, _req: Request) {
    res.write_string(StatusCode::OK, "Hello, World!");
}
```
