use std::fs;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt::Display;
use std::io::Write;
use std::net::TcpStream;

use super::status_code::StatusCode;

/// The response type for handling responses
#[derive(Clone)]
pub struct Response {
    status: Option<StatusCode>,
    headers: BTreeMap<String, String>,
    body: String,
}

impl Response {
    /// Create a new `Response`
    pub fn new() -> Self {
        Self {
            status: None,
            headers: BTreeMap::new(),
            body: String::new(),
        }
    }

    /// Set the response status
    pub fn set_status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }

    /// Add another header to the response
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Remove a header from the response
    pub fn remove_header(mut self, name: &str) -> Self {
        if let Entry::Occupied(o) = self.headers.entry(name.to_string()) {
            o.remove_entry();
        }

        self
    }

    /// Set a text/plain response with a text
    pub fn set_text(mut self, body: &str) -> Self {
        self.body = body.into();
        self.headers.insert("Content-Length".into(), body.len().to_string());
        self.headers.insert("Content-Type".into(), "text/plain".into());
        self
    }

    /// Set a text/html body with the file path.
    /// Files should be located at `views/`
    pub fn set_html(mut self, path: &str) -> Self {
        let path = format!("views/{}", path);
        let body = fs::read_to_string(path).unwrap();

        self.body = body.clone().into();
        self.headers.insert("Content-Length".into(), body.len().to_string());
        self.headers.insert("Content-Type".into(), "text/html".into());
        self
    }

    /// Set a application/json body with the struct.
    /// The struct should implement the `Display` trait
    pub fn set_json(mut self, body: impl Display) -> Self {
        let body = format!("{body}");

        self.body = body.clone().into();
        self.headers.insert("Content-Length".into(), body.len().to_string());
        self.headers.insert("Content-Type".into(), "application/json".into());
        self
    }
    
    /// Write the response to the user with a `TcpStream`
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
