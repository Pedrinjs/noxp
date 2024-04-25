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

        let mut first_line = lines.split(' ');
        let method = first_line.next().unwrap();
        let path = first_line.next().unwrap();
        
        let last = buf_iter.last().unwrap();

        Self {
            method: Method::from_str(method),
            path: path.to_string(),
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
