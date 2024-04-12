use super::method::Method;

pub struct Request<'b> {
    pub path: &'b str,
    pub method: Method,
}

impl Request<'_> {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
