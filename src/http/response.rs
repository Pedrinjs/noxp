use std::fs;
use std::fmt::Debug;
use std::io::Write;
use std::net::TcpStream;

use super::status_code::StatusCode;

pub trait JSON {
    fn to_string(&self) -> String
    where Self: Debug {
        let temp = format!("{:?}", self);
        let mut parts = temp.as_str().split(' ');
        if let Some("{") = parts.next() {
            panic!("You should use only structs!");
        }

        let contents: String = parts.collect();
        return contents;
    }
}

pub struct ResponseWriter {
    status: StatusCode,
    body: String,
    kind: String,
}

impl Default for ResponseWriter {
    fn default() -> Self {
        Self {
            status: StatusCode::NotFound,
            body: "".to_string(),
            kind: "".to_string(),
        }
    }
}

impl ResponseWriter {
    pub fn set_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn set_text(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self.kind = "text/plain".to_string();
        self
    }

    pub fn set_html(mut self, path: impl Into<String>) -> Self {
        let path = format!("views/{}", path.into());
        self.body = fs::read_to_string(path).unwrap();
        self.kind = "text/html".to_string();
        self
    }

    pub fn set_json<S: JSON + Debug>(mut self, body: S) -> Self {
        self.body = body.to_string();
        self.kind = "application/json".to_string();
        self
    }

    pub fn build(self) -> Response {
        Response::new(self.status, self.body, self.kind)
    }
}

pub struct Response {
    status: StatusCode,
    body: String,
    kind: String,
}

impl Response {
    pub fn new(status: StatusCode, body: String, kind: String) -> Self {
        Self { status, body, kind }
    }

    pub fn write(&self, mut stream: TcpStream) {
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
            self.status.get_status(),
            self.body.len(),
            self.kind,
            self.body,
        );
        
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
