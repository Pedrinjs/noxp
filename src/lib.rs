pub mod http;
pub mod thread;
mod route;

pub mod server {
    use super::thread::ThreadPool;
    use super::route::{HandlerFunc, Router};
    
    use std::{
        net::{SocketAddr, TcpListener},
        sync::Arc,
    };

    pub struct Server {
        router: Router,
        pool: ThreadPool,
    }

    impl Server {
        pub fn new(pool: ThreadPool) -> Server {
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
                let stream = stream?;

                let router = Arc::clone(&router);
                self.pool.execute(move || {
                    router.route(stream)
                });
            }
            Ok(())
        }
    }
}
