//! # NOXP
//!
//! noxp is a web application framework

pub mod http;
pub mod middlewares;
pub mod route;
pub mod server;

pub use crate::{
    http::{Request, ResponseWriter, StatusCode},
    server::Server,
};
