pub mod http;
pub mod thread;
pub mod route;
pub mod middlewares;

use crate::thread::ThreadPool;
use crate::route::{HandlerFunc, Middleware, Router};

use std::net::{ToSocketAddrs, TcpListener};
use std::sync::Arc;

pub struct Server {
    router: Router,
    pool: Option<ThreadPool>,
}

impl Server {
    pub fn default() -> Self {
        Self {
            router: Router::new(),
            pool: None,
        }
    }

    pub fn set_pool(mut self, size: usize) -> Self {
        let pool = ThreadPool::new(size);
        self.pool = Some(pool);
        self
    }

    pub fn r#use(&mut self, middleware: Middleware) {
        self.router.r#use(middleware);
    }

    pub fn handle_func(&mut self, key: (&str, &str), handler: HandlerFunc) {
        let method = key.0.to_string();
        let path = key.1.to_string();
        self.router.handle((method, path), handler)
    }

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
