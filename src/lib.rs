pub mod http;
pub mod thread;
mod route;

use crate::thread::ThreadPool;
use crate::route::{HandlerFunc, Router};

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

    pub fn build(self) -> RealServer {
        RealServer {
            router: self.router,
            pool: self.pool,
        }
    }
}

pub struct RealServer {
    router: Router,
    pool: Option<ThreadPool>,
}

impl RealServer {
    pub fn handle_func(&mut self, key: (&str, &str), handler: HandlerFunc) {
        let method = key.0.to_string();
        let path = key.1.to_string();
        self.router.handle((method, path), handler)
    }

    pub fn listen_and_serve<A>(self, addr: A) -> Result<(), std::io::Error>
        where A: ToSocketAddrs
    {
        let listener = TcpListener::bind(addr)?;

        let router = Arc::new(self.router);

        while let Ok((socket, _)) = listener.accept() {
            match self.pool {
                Some(ref pool) => {
                    let router = Arc::clone(&router);
                    pool.execute(move || {
                        router.route(socket);
                    });
                },
                None => {
                    let router = Arc::clone(&router);
                    router.route(socket);
                },
            }
        }

        Ok(())
    }
}
