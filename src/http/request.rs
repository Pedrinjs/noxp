use std::collections::BTreeMap;

use super::method::Method;
use super::query_string::QueryString;

#[derive(Clone)]
pub struct Request {
    headers: BTreeMap<String, String>,
    query_string: Option<QueryString>,
    method: Method,
    path: String,
    body: String,
}

impl Request {
    pub fn from<'a>(buffer: &'a [u8]) -> Self {
        let buf = String::from_utf8_lossy(buffer);
        let mut buf_iter = buf.lines().enumerate().peekable();
        
        let (mut first, mut last) = (String::new(), String::new());
        let mut headers = BTreeMap::new();

        while let Some((index, line)) = buf_iter.next() {
            if index == 0 {
                first = line.into();
                continue;
            }
            if let None = buf_iter.peek() {
                last = line.into();
                continue;
            }
            if let Some(&(_, "")) = buf_iter.peek() {
                continue;
            }

            let Some((k, v)) = line.split_once(": ") else {
                continue
            };
            headers.insert(k.to_string(), v.to_string());
        }

        let mut first_line = first.split(' ');
        let method = first_line.next().unwrap();
        let mut path = first_line.next().unwrap();

        let mut query_string: Option<QueryString> = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Self {
            headers,
            query_string,
            method: Method::from_str(method),
            path: path.to_string(),
            body: last,
        }
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers.get(key).cloned()
    }

    pub fn print_body(&self) {
        println!("{}", self.body);
    }

    pub fn print_self(&self) {
        println!("{} {} HTTP/1.1", self.method.to_str(), self.path);

        for (key, value) in &self.headers {
            println!("{key}: {value}");
        }

        if let Some(qs) = &self.query_string {
            println!("{:?}", qs);
        }

        println!("{}", self.body);
    }
}
