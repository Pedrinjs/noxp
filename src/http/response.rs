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
    status: StatusCode,
    headers: BTreeMap<String, String>,
    body: String,
}

impl Response {
    pub fn builder() -> ResponseWriter {
        ResponseWriter::default()
    }

    /// Write the response to the user with a `TcpStream`
    pub fn write(&self, mut stream: TcpStream) {
        let mut response = format!("HTTP/1.1 {}\r\n", self.status.get_status());

        for (key, value) in &self.headers {
            let kv = format!("{}: {}\r\n", key, value);
            response.push_str(&kv);
        }

        let body = format!("\r\n{}", self.body);
        response.push_str(&body);

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

#[derive(Default)]
pub struct ResponseWriter {
    status: StatusCode,
    headers: BTreeMap<String, String>,
    body: String,
}

impl ResponseWriter {
    /// Set the response status
    pub fn set_status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status;
        self
    }

    /// Add another header to the response
    pub fn header(&mut self, name: &str, value: &str) -> &mut Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Remove a header from the response
    pub fn remove_header(&mut self, name: &str) -> &mut Self {
        if let Entry::Occupied(header) = self.headers.entry(name.to_string()) {
            header.remove_entry();
        }
        self
    }

    /// Set a text/plain response with a text
    pub fn set_text(&mut self, body: &str) -> &mut Self {
        self.body = body.into();
        self.headers.insert("Content-Length".into(), body.len().to_string());
        self.headers.insert("Content-Type".into(), "text/plain".into());
        self
    }

    /// Set a text/html body with the file path
    /// Files should be located at `views/`
    pub fn set_html(&mut self, path: &str) -> &mut Self {
        let path = format!("views/{}", path);
        let body = fs::read_to_string(path).unwrap();

        self.body = body.clone().into();
        self.headers.insert("Content-Length".into(), body.len().to_string());
        self.headers.insert("Content-Type".into(), "text/html".into());
        self
    }

    /// Set a application/json body with the struct
    /// The struct should implement the `Display` trait
    pub fn set_json(&mut self, body: impl Display) -> &mut Self {
        let body = format!("{}", body);

        self.body = body.clone().into();
        self.headers.insert("Content-Length".into(), body.len().to_string());
        self.headers.insert("Content-Type".into(), "application/json".into());
        self
    }

    /// Builds the Response from the ResponseWriter
    pub fn build(self) -> Response {
        Response {
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
}
