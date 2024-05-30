//! # NOXP
//!
//! noxp is a web application framework

pub mod http;
pub mod thread;
pub mod route;
pub mod middlewares;

use crate::thread::ThreadPool;
use crate::route::{HandlerFunc, Middleware, Router};

use std::net::{ToSocketAddrs, TcpListener};
use std::sync::Arc;

/// Server struct
pub struct Server {
    router: Router,
    pool: Option<ThreadPool>,
}

impl Server {
    /// Create a new `Server`
    pub fn default() -> Self {
        Self {
            router: Router::new(),
            pool: None,
        }
    }

    /// Set the threadpool from its size.
    /// It's recommended to don't add too many threads, because it can waste resources, cost time
    /// to create unused threads and destroying too many threads requires more time!
    pub fn set_pool(mut self, size: usize) -> Self {
        let pool = ThreadPool::new(size);
        self.pool = Some(pool);
        self
    }

    /// Add a middleware to the router
    pub fn r#use(&mut self, middleware: Middleware) {
        self.router.r#use(middleware);
    }

    /// Add a route to the router.
    /// The key is a tuple that receives, respectively, the method and the path
    pub fn handle_func(&mut self, key: (&str, &str), handler: HandlerFunc) {
        let method = key.0.to_string();
        let path = key.1.to_string();
        self.router.handle((method, path), handler)
    }

    /// Start the server and bind it to a socket address
    pub fn listen_and_serve<A>(self, addr: A) -> std::io::Result<()>
        where A: ToSocketAddrs
    {
        let listener = TcpListener::bind(addr)?;

        let router = Arc::new(self.router);

        while let Ok((stream, addr)) = listener.accept() {
            match self.pool {
                Some(ref pool) => {
                    let router = Arc::clone(&router);
                    pool.execute(move || {
                        router.route(stream, addr);
                    });
                },
                None => {
                    let router = Arc::clone(&router);
                    router.route(stream, addr);
                },
            }
        }

        Ok(())
    }
}
