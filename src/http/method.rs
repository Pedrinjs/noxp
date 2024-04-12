use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            _ => Err("onlt GET/POST available".to_string()),
        }
    }
}

impl Method {
    pub fn to_str(&self) -> &str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
        }
    }
}
