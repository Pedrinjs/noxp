use std::collections::BTreeMap;
use std::io::prelude::*;
use std::net::TcpStream;

use super::http::{Method,Request};

pub type HandlerFunc = fn(Request, TcpStream);

pub struct Router {
    routes: BTreeMap<(Method, String), HandlerFunc>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: BTreeMap::new(),
        }
    }

    pub fn handle(&mut self, key: (Method, &str), value: HandlerFunc) {
        self.routes.insert((key.0, key.1.to_string()), value);
    }

    pub fn route(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        
        let request = Request::from(&buffer);

        let key = (request.method(), request.path());

        if self.routes.contains_key(&key) {
            let handler = self.routes.get(&key).unwrap();
            handler(request, stream);
        }
    }
}
