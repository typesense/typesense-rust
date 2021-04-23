use serde::Serialize;

/// HTTP Methods used by this library.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum HttpMethod<T: Serialize> {
    Get,
    Post(T),
    Put(T),
    Delete,
}
