use super::http::{Request, Response};
use super::route::HandlerFunc;

use std::sync::Arc;

/// Logger middleware
pub fn logger(next: HandlerFunc) -> HandlerFunc {
    Arc::new(move |req: Request, res: &mut Response| {
        println!("LOG {} - HTTP/1.1 {} {}",
            req.remote(), req.method().to_str(), req.path());

        next(req, res);
    })
}

/// Middleware to add some protection to XSS attack
pub fn helmet(next: HandlerFunc) -> HandlerFunc {
    Arc::new(move |req: Request, res: &mut Response| {
        res.header(
            "Content-Security-Policy",
            "default-src 'self';
            base-uri 'self';
            font-src 'self' https: data:;
            form-action 'self';
            frame-ancestors 'self';
            img-src 'self' data:;
            object-src 'none';
            script-src 'self';
            script-src-attr 'none';
            style-src 'self' https: 'unsafe-inline';
            upgrade-insecure-requests");
        res.header("Cross-Origin-Embedder-Policy", "require-corp");
        res.header("Cross-Origin-Opener-Policy", "same-origin");
        res.header("Cross-Origin-Resource-Policy", "same-origin");
        res.header("Origin-Agent-Cluster", "?1");
        res.header("Referrer-Policy", "no-referrer");
        res.header("Strict-Transport-Security", "max-age=15552000; includeSubDomains");
        res.header("X-Content-Type-Options", "nosniff");
        res.header("X-DNS-Prefetch-Control", "off");
        res.header("X-Download-Options", "noopen");
        res.header("X-Frame-Options", "SAMEORIGIN");
        res.header("X-Permitted-Cross-Domain-Policies", "none");
        res.header("X-XSS-Protection", "0");
        res.remove_header("Powered-By");

        next(req, res)
    })
}

/*
/// Cross-Origin Resource Sharing (CORS) middleware
pub fn cors(_: Request, res: Response) -> Response {
    res.header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, HEAD, CONNECT, OPTIONS, TRACE, PATCH")
        .header("", "")
}
*/
