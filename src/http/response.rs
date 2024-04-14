use std::fs;
use std::fmt::Debug;
use std::io::Write;
use std::net::TcpStream;

use super::status_code::StatusCode;

pub struct Response {
    stream: TcpStream,
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        Response { stream }
    }

    pub fn write_string(&mut self, status: StatusCode, contents: &str) {
        let kind = "text/plain";

        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
            status.get_status(),
            contents.len(),
            kind,
            contents,
        );

        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn write_file(&mut self, status: StatusCode, path: &str) {
        let path = format!("views/{path}");
        let contents = fs::read_to_string(path).unwrap();
        let kind = "text/html";
        
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
            status.get_status(),
            contents.len(),
            kind,
            contents,
        );

        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn write_json<T: Debug>(&mut self, status: StatusCode, json: T) {
        let temp = format!("{:?}", json);
        let mut parts = temp.as_str().split(' ');
        parts.next();

        let contents = parts.collect::<String>();
        let kind = "application/json";
        
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
            status.get_status(),
            contents.len(),
            kind,
            contents,
        );

        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}
