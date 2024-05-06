# NOXP
***
![Badge em Desenvolvimento](http://img.shields.io/static/v1?label=STATUS&message=EM%20DESENVOLVIMENTO&color=GREEN&style=for-the-badge)
### 🦀 NOXP is a simple web framework for rust inspired by golang's net/http package
NOXP uses only the standard library

#### 🚧 What's next:
- [x] Optional multithreading
- [x] Use the requests for some usefull purpose
- [x] Better routing system
- [x] More status codes
- [x] More http methods
- [x] Headers!
- [x] Use query strings
- [ ] Add middleware
- [x] Authentication (do it by yourself!)
- [ ] Publish to crates.io
- [ ] Add documentation

#### Usage
```rust
use std::fmt;
use std::net::TcpStream;

use noxp::Server;
use noxp::http::{ Response, Request, StatusCode };

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
  // define the size of the thread pool
  let size: usize = 4;
  // create the server with a thread pool of defined size 4 (it's optional)
  // it's also possible to use Server::default().set_pool(4)
  let mut server = Server::default().set_pool(size);

  // pay attention for the tuple (Method, &str)
  server.handle_func(("GET", "/"), index);
  server.handle_func(("POST", "/"), post);

  // you can also send html (only in the views folder)
  server.handle_func(("GET", "/hello"), file);

  // and send json (only structs which implement Display)
  server.handle_func(("GET", "/person"), json);

  // listening at localhost:8080
  server.listen_and_serve(8080)
}

fn index(_req: Request) -> Response {
  Response::default()
    .set_status(StatusCode::OK)
    .set_text("Hello, World!")
}

fn post(req: Request) -> Response {
  req.print_body();

  Response::default().set_status(StatusCode::OK).set_json(req.get_body())
}

fn file(_req: Request) -> Response {
  Response::default().set_status(StatusCode::OK).set_html("hello.html")
}

fn json(_req: Request) -> Response {
  let person = Person {
    name: String::from("Menezes"),
    age: 15,
  };

  Response::default().set_status(StatusCode::OK).set_json(person)
}
```
