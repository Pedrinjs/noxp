use super::method::Method;

#[derive(Clone)]
pub struct Request<'a> {
    method: Method,
    path: &'a str,
}

impl Request<'_> {
    pub fn new(method: Method, path: &str) -> Request {
        Request { method, path }
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn path(&self) -> String {
        self.path.to_string()
    }
}
