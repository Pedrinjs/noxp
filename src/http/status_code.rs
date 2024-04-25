#[derive(Clone)]
pub enum StatusCode {
    // Information Responses
    Continue,
    SwitchingProtocols,
    EarlyHints,

    // Successful Responses
    OK,
    Created,
    Accepted,
    NonAuthoritativeInformarion,
    NoContent,
    ResetContent,
    PartialContent,

    // Redirection Messages
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    TemporaryRedirect,
    PermanentRedirect,

    // Client Error Responses
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // Server Error Responses
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    NotExtended,
    NetworkAuthenticationRequired,
}

// just kill me
impl StatusCode {
    pub fn get_status(&self) -> &'static str {
        match self {
            StatusCode::Continue            => "100 Continue",
            StatusCode::SwitchingProtocols  => "101 SwitchingProtocols",
            StatusCode::EarlyHints          => "103 EarlyHints",
            
            StatusCode::OK                          => "200 OK",
            StatusCode::Created                     => "201 Created",
            StatusCode::Accepted                    => "202 Accepted",
            StatusCode::NonAuthoritativeInformarion => "203 Non-Authoritative Information",
            StatusCode::NoContent                   => "204 No Content",
            StatusCode::ResetContent                => "205 Reset Content",
            StatusCode::PartialContent              => "206 Partial Content",

            StatusCode::MultipleChoices     => "300 Multiple Choices",
            StatusCode::MovedPermanently    => "301 Moved Permanently",
            StatusCode::Found               => "302 Found",
            StatusCode::SeeOther            => "303 See Other",
            StatusCode::NotModified         => "304 Not Modified",
            StatusCode::TemporaryRedirect   => "307 Temporary Redirect",
            StatusCode::PermanentRedirect   => "308 Permanent Redirect",

            StatusCode::BadRequest                  => "400 Bad Request",
            StatusCode::Unauthorized                => "401 Unauthorized",
            StatusCode::Forbidden                   => "403 Forbidden",
            StatusCode::NotFound                    => "404 Not Found",
            StatusCode::MethodNotAllowed            => "405 Method Not Allowed",
            StatusCode::NotAcceptable               => "406 Not Acceptable",
            StatusCode::ProxyAuthenticationRequired => "407 Proxy Authentication Required",
            StatusCode::RequestTimeout              => "408 Request Timeout",
            StatusCode::Conflict                    => "409 Conflict",
            StatusCode::Gone                        => "410 Gone",
            StatusCode::LengthRequired              => "411 Length Required",
            StatusCode::PreconditionFailed          => "412 Precondition Failed",
            StatusCode::PayloadTooLarge             => "413 Payload Too Large",
            StatusCode::URITooLong                  => "414 URI Too Long",
            StatusCode::UnsupportedMediaType        => "415 Unsupported Media Type",
            StatusCode::RangeNotSatisfiable         => "416 Range Not Satisfiable",
            StatusCode::ExpectationFailed           => "417 Expectation Failed",
            StatusCode::ImATeapot                   => "418 I'm a teapot",
            StatusCode::MisdirectedRequest          => "421 Misdirected Request",
            StatusCode::UpgradeRequired             => "426 Upgrade Required",
            StatusCode::PreconditionRequired        => "428 Precondition Required",
            StatusCode::TooManyRequests             => "429 Too Many Requests",
            StatusCode::RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large",
            StatusCode::UnavailableForLegalReasons  => "451 Unavailable For Legal Reasons",

            StatusCode::InternalServerError             => "500 Internal Server Error",
            StatusCode::NotImplemented                  => "501 NotImplemented",
            StatusCode::BadGateway                      => "502 Bad Gateway",
            StatusCode::ServiceUnavailable              => "503 Service Unavailable",
            StatusCode::GatewayTimeout                  => "504 Gateway Timeout",
            StatusCode::HTTPVersionNotSupported         => "505 HTTP Version Not Supported",
            StatusCode::VariantAlsoNegotiates           => "506 Variant Also Negotiates",
            StatusCode::NotExtended                     => "510 Not Extended",
            StatusCode::NetworkAuthenticationRequired   => "511 Network Authentication Required",
        }
    }
}

// that took waaaaaay longer than I thought
