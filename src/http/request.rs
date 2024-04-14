use super::method::Method;
//use super::query_string::QueryString;

pub struct Request<'a> {
    method: Method,
    path: &'a str,
    //query_string: Option<QueryString>,
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

    /*pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }*/
}
