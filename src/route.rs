use std::collections::BTreeMap;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};

use super::http::{Method, Request, Response, StatusCode};

/// Handler Function type
pub type HandlerFunc = Box<dyn Fn(Request, &mut Response)>;

/*/// Middleware type (same as the handler)
pub type Middleware = fn(HandlerFunc) -> HandlerFunc;*/

/// The router type for composing handlers
pub struct Router {
    routes: BTreeMap<(Method, String), HandlerFunc>,
}

impl Router {
    /// Create a new `Router`
    pub fn new() -> Self {
        Self {
            routes: BTreeMap::new(),
        }
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

        let mut response = Response::new();
        let request = Request::from(&buffer, addr);
        let key = (request.method(), request.path());

        if self.routes.contains_key(&key) {
            let handler = self.routes.get(&key).unwrap();
            handler(request, &mut response);
            response.write(stream);
        } else {
            response.set_status(StatusCode::NotFound);
            response.write(stream);
        }
    }
}
