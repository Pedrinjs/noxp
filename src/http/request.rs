use super::method::Method;
//use super::query_string::QueryString;

use std::str::FromStr;
use std::str;

pub struct Request<'a> {
    path: &'a str,
    //query_string: Option<QueryString>,
    method: Method,
}

impl<'a> Request<'a> {
    pub fn new(path: &'a str, method: &'a str) -> Request<'a> {
        Request {
            method: Method::from_str(method).unwrap(),
            path,
        }
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    /*pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }*/
}
