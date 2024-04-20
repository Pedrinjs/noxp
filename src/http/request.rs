use super::method::Method;

#[derive(Clone)]
pub struct Request {
    method: Method,
    path: String,
    body: String,
}

impl Request {
    pub fn from<'a>(buffer: &'a [u8]) -> Self {
        let buf = String::from_utf8_lossy(buffer);
        let mut buf_iter = buf.lines();
        let lines = buf_iter.next().unwrap();
        let first = lines.split(" ").collect::<Vec<&str>>();
        let last = buf_iter.last().unwrap();

        let (method, path) = match first.as_slice() {
            [method, path, _] => (Method::from_str(method), path.to_string()),
            _ => panic!("How did we get here?"),
        };

        Self {
            method,
            path,
            body: last.to_string(),
        }
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn print_body(&self) {
        println!("{}", self.body);
    }
}
