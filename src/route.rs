use std::collections::BTreeMap;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

use super::http::{
    Method,
    response::ResponseWriter,
    Request
};

pub type HandlerFunc = fn(ResponseWriter, Request, TcpStream);

pub struct Router {
    routes: BTreeMap<(Method, String), HandlerFunc>,
    //middlewares: BTreeSet<HandlerFunc>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: BTreeMap::new(),
            //middlewares: BTreeSet::new(),
        }
    }

    pub fn handle(&mut self, key: (Method, &str), value: HandlerFunc) {
        self.routes.insert((key.0, key.1.to_string()), value);
    }

    /*pub fn r#use(&mut self, handler: HandlerFunc) {
        self.middlewares.insert(handler);
    }*/

    pub fn route(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        
        /*let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("{:?}", http_request);*/
        
        let line = buf_reader.lines().next().unwrap().unwrap();
        let line_slice = line.split(" ").collect::<Vec<&str>>();

        let (method, path) = match line_slice.as_slice() {
            [method, path, _] => (Method::from_str(method), path),
            _ => panic!("how did we get here?"),
        };

        let request = Request::new(method, path);
        let response = ResponseWriter::default();

        let key = (request.method(), request.path());

        if self.routes.contains_key(&key) {
            /*for mid in &self.middlewares {
                mid(response.clone(), request.clone());
            }*/
            
            let handler = self.routes.get(&key).unwrap();
            handler(response, request, stream);
        }
    }
}
