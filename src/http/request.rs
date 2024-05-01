use std::collections::BTreeMap;

use super::method::Method;

#[derive(Clone)]
pub struct Request {
    headers: BTreeMap<String, String>,
    method: Method,
    path: String,
    body: String,
}

impl Request {
    pub fn from<'a>(buffer: &'a [u8]) -> Self {
        let buf = String::from_utf8_lossy(buffer);
        let mut buf_iter = buf.lines().enumerate().peekable();
        
        let (mut first, mut last) = (String::new(), String::new());
        let mut headers: BTreeMap<String, String> = BTreeMap::new();

        while let Some((index, line)) = buf_iter.next() {
            if index == 0 {
                first = line.to_string();
                continue;
            }
            if let None = buf_iter.peek() {
                last = line.to_string();
                continue;
            }
            if let Some(&(_, "")) = buf_iter.peek() {
                continue;
            }

            let Some((k, v)) = line.split_once(": ") else {
                continue
            };
            headers.insert(k.into(), v.into());
        }

        let mut first_line = first.split(' ');
        let method = first_line.next().unwrap();
        let path = first_line.next().unwrap();

        Self {
            headers,
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

    pub fn headers(&self) {
        for (key, value) in &self.headers {
            println!("{key}: {value}");
        }
    }

    pub fn print_body(&self) {
        println!("{}", self.body);
    }
}
