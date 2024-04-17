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
use std::net::TcpStream;

use noxp::http::{
  Method,
  Request,
  response::{JSON, ResponseWriter},
  StatusCode
};
use noxp::thread::ThreadPool;
use noxp::Server;

#[derive(Debug)]
struct Person {
  name: String,
  age: i32,
}

impl JSON for Person {}

fn main() -> std::io::Result<()> {
  // threadpool with a finite number of threads(4)
  let pool = ThreadPool::new(4);
  
  // create the server with the threadpool (it's optional)
  let mut server = Server::default().set_pool(pool).build();

  // pay attention for the tuple (Method, &str)
  server.handle_func((Method::GET, "/"), index);

  // you can also send html (only in the views folder)
  server.handle_func((Method::GET, "/hello"), file);

  // and send json (only structs which implement JSON and Debug)
  server.handle_func((Method::GET, "/person"), json);

  // listening at localhost:8080
  server.listen_and_serve(8080)
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
```
