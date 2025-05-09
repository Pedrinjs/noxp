//! # NOXP
//!
//! noxp is a web application framework

pub mod http;
pub mod route;
pub mod middlewares;

use crate::route::{HandlerFunc, Router, Middleware};

use std::net::{ToSocketAddrs, TcpListener};
use std::sync::Arc;

/// Server struct
pub struct Server {
    router: Router,
}

impl Server {
    /// Create a new `Server`
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    /// Add a middleware to the router
    pub fn r#use(&mut self, key: (&str, &str), middleware: Middleware) {
        let method = key.0.to_string();
        let path = key.1.to_string();
        self.router.r#use((method, path), middleware);
    }

    /// Add a route to the router.
    /// The key is a tuple that receives, respectively, the method and the path
    pub fn handle_func(&mut self, key: (&str, &str), handler: HandlerFunc) {
        let method = key.0.to_string();
        let path = key.1.to_string();
        self.router.handle((method, path), handler);
    }

    /// Start the server and bind it to a socket address
    pub fn listen_and_serve<A>(self, addr: A) -> std::io::Result<()>
        where A: ToSocketAddrs
    {
        let listener = TcpListener::bind(addr)?;
        let router = Arc::new(self.router);
        while let Ok((stream, addr)) = listener.accept() {
            let router = Arc::clone(&router);
            router.route(stream, addr);
        }
        Ok(())
    }
}
