use thiserror::Error;

/// [`Result`](std::result::Result) type that is returned from functions with error
/// as [`TypesenseError`](crate::error::TypesenseError).
pub type Result<T> = std::result::Result<T, TypesenseError>;

/// Represents an error that can occur while using the library.
#[derive(Error, Debug)]
pub enum TypesenseError {
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
}
