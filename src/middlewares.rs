use super::http::{Request, Response};

pub fn logger(req: Request, res: Response) -> Response {
    println!(
        "LOG {} - HTTP/1.1 {} {}",
        req.remote(),
        req.method().to_str(),
        req.path(),
    );

    res
}

pub fn helmet(_: Request, res: Response) -> Response {
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
        upgrade-insecure-requests")
        .header("Cross-Origin-Embedder-Policy", "require-corp")
        .header("Cross-Origin-Opener-Policy", "same-origin")
        .header("Cross-Origin-Resource-Policy", "same-origin")
        .header("Origin-Agent-Cluster", "?1")
        .header("Referrer-Policy", "no-referrer")
        .header("Strict-Transport-Security", "max-age=15552000; includeSubDomains")
        .header("X-Content-Type-Options", "nosniff")
        .header("X-DNS-Prefetch-Control", "off")
        .header("X-Download-Options", "noopen")
        .header("X-Frame-Options", "SAMEORIGIN")
        .header("X-Permitted-Cross-Domain-Policies", "none")
        .header("X-XSS-Protection", "0")
        .remove_header("Powered-By")
}

pub fn cors(_: Request, res: Response) -> Response {
    res.header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, HEAD, CONNECT, OPTIONS, TRACE, PATCH")
        .header("", "")
}
