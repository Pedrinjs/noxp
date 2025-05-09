use std::collections::BTreeMap;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;

use super::http::{Method, Request, Response, StatusCode};

/// Handler Function type
pub type HandlerFunc = Arc<dyn for<'a> Fn(Request, &'a mut Response)>;

/// Middleware type (same as the handler)
pub type Middleware = fn(HandlerFunc) -> HandlerFunc;

/// The router type for composing handlers
pub struct Router {
    routes: BTreeMap<(Method, String), HandlerFunc>,
    middlewares: BTreeMap<(Method, String), Vec<Middleware>>,
}

impl Router {
    /// Create a new `Router`
    pub fn new() -> Self {
        Self {
            routes: BTreeMap::new(),
            middlewares: BTreeMap::new(),
        }
    }

    /// Add a handler function to the router
    pub fn handle(&mut self, key: (String, String), handler: HandlerFunc) {
        let method = Method::from_str(&key.0);
        let path = key.1;

        self.routes.insert((method, path), handler);
    }

    /// Append a middleware into a vector related to a specific path
    pub fn r#use(&mut self, key: (String, String), middleware: Middleware) {
        let method = Method::from_str(&key.0);
        let path = key.1;
        let key: (Method, String) = (method, path);

        self.middlewares.entry(key).or_insert_with(Vec::new).push(middleware);
    }

    /// Handle the route function when receiving a request
    /// If the path or the method aren't registered, it returns `404 Not Found`
    pub fn route<'a>(&'a self, mut stream: TcpStream, addr: SocketAddr) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let mut response = Response::new();
        let request = Request::from(&buffer, addr);
        let key = (request.method(), request.path());

        if let Some(handler) = self.routes.get(&key) {
            if let Some(mids) = self.middlewares.get(&key) {
                let handler_func = Arc::clone(handler);
                let resulting = mids
                    .into_iter()
                    .rev()
                    .fold(handler_func, |acc: HandlerFunc, middleware: &Middleware| {
                        middleware(acc)
                    });
                resulting(request, &mut response);
                response.write(stream);
                return;
            }
            handler(request, &mut response);
            response.write(stream);
            return;
        }
        response.set_status(StatusCode::NotFound);
        response.write(stream);
    }
}
