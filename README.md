# NOXP
***
### 🦀 NOXP is a simple web framework for rust inspired by golang's net/http package
NOXP uses only the standard library

#### 🚧 What's next:
- [x] Use the requests for some useful purpose
- [x] Better routing system
- [x] All status codes
- [x] All http methods
- [x] Headers!
- [x] Query strings support
- [ ] Dynamic routing
- [x] Add middleware
- [x] Fix middleware
- [ ] Authentication middleware
- [x] Add documentation
- [x] Publish to crates.io

#### Usage
```rust
use noxp::*;

use std::fmt;
use std::sync::Arc;

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
  // create the server
  let mut server = Server::new();

  // pay attention for the tuple (Method, &str)
  server.handle_func(("GET", "/"), Arc::new(index));
  server.handle_func(("POST", "/"), Arc::new(post));

  // you can also send html (only in the views folder)
  server.handle_func(("GET", "/hello"), Arc::new(file));

  // and send json (only structs which implement Display)
  server.handle_func(("GET", "/person"), Arc::new(json));

  // listening at localhost:8080
  server.listen_and_serve("localhost:8080")
}

fn index(res: &mut ResponseWriter, _req: Request) {
  res.set_status(StatusCode::OK)
      .set_text("Hello, World!");
}

fn post(res: &mut ResponseWriter, req: Request) {
  req.print_body();

  res.set_status(StatusCode::OK)
      .set_json(req.body());
}

fn file(res: &mut ResponseWriter, _req: Request) {
  res.set_status(StatusCode::OK)
      .set_html("hello.html");
}

fn json(res: &mut Response, _req: Request) {
  let person = Person {
    name: String::from("Menezes"),
    age: 16,
  };

  res.set_status(StatusCode::OK)
      .set_json(person);
}
```
