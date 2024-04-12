pub enum StatusCode {
    OK,
    NotFound,
}

impl StatusCode {
    pub fn get_status(&self) -> &'static str {
        match self {
            StatusCode::OK => "200 OK",
            StatusCode::NotFound => "404 NOT FOUND",
        }
    }
}
