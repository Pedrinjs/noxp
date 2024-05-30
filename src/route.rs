use std::collections::BTreeMap;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};

use super::http::{Method, Request, Response, StatusCode};

/// Handler Function type
pub type HandlerFunc = fn(Request, Response) -> Response;

/// Middleware type (same as the handler)
pub type Middleware = fn(Request, Response) -> Response;

/// The router type for composing handlers
pub struct Router {
    routes: BTreeMap<(Method, String), HandlerFunc>,
    middlewares: Vec<Middleware>,
}

impl Router {
    /// Create a new `Router`
    pub fn new() -> Self {
        Self {
            routes: BTreeMap::new(),
            middlewares: Vec::new(),
        }
    }

    /// Add a middleware to the router
    pub fn r#use(&mut self, middleware: Middleware) {
        self.middlewares.push(middleware);
    }

    /// Add a handler function to the router
    /// The key is a string tuple that contains the method and the path
    pub fn handle(&mut self, key: (String, String), handler: HandlerFunc) {
        let method = Method::from_str(&key.0);
        let path = key.1.to_string();

        self.routes.insert((method, path), handler);
    }

    /// Handle the route function when receiving a request
    /// If the path or the method aren't registered, it returns `404 Not Found`
    pub fn route(&self, mut stream: TcpStream, addr: SocketAddr) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        
        let request = Request::from(&buffer, addr);

        let key = (request.method(), request.path());

        if self.routes.contains_key(&key) {
            let handler = self.routes.get(&key).unwrap();
            let mut response = Response::new();

            for middleware in &self.middlewares {
                response = middleware(request.clone(), response);
            }

            response = handler(request, response);

            response.write(stream);
        } else {
            let response = Response::new().set_status(StatusCode::NotFound);
            response.write(stream);
        }
    }
}
