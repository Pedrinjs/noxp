pub mod http;
pub mod thread;
mod route;

use crate::thread::ThreadPool;
use crate::route::{HandlerFunc, Router};

use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;

pub struct Server {
    router: Router,
    pool: Option<ThreadPool>,
}

impl Server {
    pub fn new(pool: Option<ThreadPool>) -> Server {
        Server {
            router: Router::new(),
            pool,
        }
    }

    pub fn handle_func(&mut self, path: &str, handler: HandlerFunc) {
        self.router.handle(path, handler)
    }

    pub fn listen_and_serve(self, port: u16) -> Result<(), std::io::Error> {
        // localhost:port (e.g. localhost:8080, localhost:6969)
        let ip = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = TcpListener::bind(ip)?;

        let router = Arc::new(self.router);
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            /*if self.pool.is_none() {
                let router = Arc::clone(&router);
                router.route(stream);
            } else {
                let router = Arc::clone(&router);
                self.pool.unwrap().execute(move || {
                    router.route(stream)
                });
            }*/

            match self.pool {
                Some(ref pool) => {
                    let router = Arc::clone(&router);
                    pool.execute(move || {
                        router.route(stream);
                    });
                },
                None => {
                    let router = Arc::clone(&router);
                    router.route(stream);
                },
            }
        }
        Ok(())
    }
}
