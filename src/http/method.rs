/// The method type to handle HTTP method
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl Method {
    /// Convert a `&str` to a `Method`
    pub fn from_str(s: &str) -> Self {
        match s {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "HEAD" => Self::HEAD,
            "CONNECT" => Self::CONNECT,
            "OPTIONS" => Self::OPTIONS,
            "TRACE" => Self::TRACE,
            "PATCH" => Self::PATCH,
            _ => panic!("If you're seeing this, congratulations!\nYou somehow created a new HTTP method!"),
        }
    }

    /// Convert a `Method` into a `&str`
    pub fn to_str(&self) -> &str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::HEAD => "HEAD",
            Method::CONNECT => "CONNECT",
            Method::OPTIONS => "OPTIONS",
            Method::TRACE => "TRACE",
            Method::PATCH => "PATCH",
        }
    }
}
