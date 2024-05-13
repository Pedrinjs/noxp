use std::fs;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::io::Write;
use std::net::TcpStream;

use super::status_code::StatusCode;

#[derive(Clone)]
pub struct Response {
    status: Option<StatusCode>,
    headers: BTreeMap<String, String>,
    body: String,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: None,
            headers: BTreeMap::new(),
            body: String::new(),
        }
    }

    pub fn set_status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }

    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn set_text(mut self, body: &str) -> Self {
        self.body = body.into();
        self.headers.insert("Content-Length".into(), body.len().to_string());
        self.headers.insert("Content-Type".into(), "text/plain".into());
        self
    }

    pub fn set_html(mut self, path: &str) -> Self {
        let path = format!("views/{}", path);
        let body = fs::read_to_string(path).unwrap();

        self.body = body.clone().into();
        self.headers.insert("Content-Length".into(), body.len().to_string());
        self.headers.insert("Content-Type".into(), "text/html".into());
        self
    }

    pub fn set_json(mut self, body: impl Display) -> Self {
        let body = format!("{body}");

        self.body = body.clone().into();
        self.headers.insert("Content-Length".into(), body.len().to_string());
        self.headers.insert("Content-Type".into(), "application/json".into());
        self
    }
    
    pub fn write(&self, mut stream: TcpStream) {
        let mut response = format!(
            "HTTP/1.1 {}\r\n",
            self.status.as_ref().unwrap().get_status()
        );

        for (key, value) in &self.headers {
            let kv = format!("{key}: {value}\r\n");
            response.push_str(&kv);
        }

        let body = format!("\r\n{}", self.body);
        response.push_str(&body);

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
