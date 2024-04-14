#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Method {
    GET,
    POST,
}

impl Method {
    pub fn from_str(s: &str) -> Self {
        match s {
            "GET" => Self::GET,
            "POST" => Self::POST,
            _ => panic!("onlt GET/POST available"),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
        }
    }
}
