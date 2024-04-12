use std::fs;
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
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            status.get_status(),
            contents.len(),
            contents,
        );

        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn write_file(&mut self, status: StatusCode, path: &str) {
        let path = format!("views/{path}");
        let contents = fs::read_to_string(path).unwrap();
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            status.get_status(),
            contents.len(),
            contents
        );

        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}
