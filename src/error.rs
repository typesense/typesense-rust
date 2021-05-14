use http::StatusCode;
use thiserror::Error;

/// [`Result`](std::result::Result) type that is returned from
/// functions with error as [`TypesenseError`].
pub type Result<T> = std::result::Result<T, TypesenseError>;

/// Represents an error that can occur while using the library.
#[derive(Error, Debug)]
pub enum TypesenseError {
    /// TypesenseClientError
    #[error("typesense client error")]
    TypesenseClientError,

    /// Config error.
    #[error("config error")]
    ConfigError,

    /// Timeout.
    #[error("timeout")]
    Timeout,

    /// Request malformed.
    #[error("request malformed")]
    RequestMalformed,

    /// Request unauthorized.
    #[error("request unauthorized")]
    RequestUnauthorized,

    /// Request forbidden.
    #[error("request forbidden")]
    RequestForbidden,

    /// Object not found.
    #[error("object not found")]
    ObjectNotFound,

    /// Object already exists.
    #[error("object already exists")]
    ObjectAlreadyExists,

    /// Object unprocessable.
    #[error("object unprocessable")]
    ObjectUnprocessable,

    /// Server error.
    #[error("server error")]
    ServerError,

    /// Service unavailable.
    #[error("service unavailable")]
    ServiceUnavailable,

    /// HTTP status error.
    #[error("HTTP status error")]
    HttpStatusError,

    /// HTTP error.
    #[error("http error: {0}")]
    HttpError(#[from] http::Error),

    /// Hyper error.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
    #[error("hyper error: {0}")]
    HyperError(#[from] hyper::Error),

    /// WASM error.
    #[cfg(target_arch = "wasm32")]
    #[cfg_attr(docsrs, doc(cfg(target_arch = "wasm32")))]
    #[error("wasm client error: {0:?}")]
    WasmError(String),
}

impl From<StatusCode> for TypesenseError {
    fn from(status: StatusCode) -> Self {
        match status {
            // 400
            StatusCode::BAD_REQUEST => Self::RequestMalformed,
            // 401
            StatusCode::UNAUTHORIZED => Self::RequestUnauthorized,
            // 403
            StatusCode::FORBIDDEN => Self::RequestForbidden,
            // 404
            StatusCode::NOT_FOUND => Self::ObjectNotFound,
            // 409
            StatusCode::CONFLICT => Self::ObjectAlreadyExists,
            // 422
            StatusCode::UNPROCESSABLE_ENTITY => Self::ObjectUnprocessable,
            // 500
            StatusCode::INTERNAL_SERVER_ERROR => Self::ServerError,
            // 503
            StatusCode::SERVICE_UNAVAILABLE => Self::ServiceUnavailable,
            _ => Self::TypesenseClientError,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl From<wasm_bindgen::JsValue> for TypesenseError {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Self::WasmError(format!("{:?}", value))
    }
}
