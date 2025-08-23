//! Contains the error types for the Typesense client

use thiserror::Error;
pub use typesense_codegen::apis::Error as ApiError;

/// The primary error type for the Typesense client.
///
/// This enum encapsulates all possible failures, from network issues to API errors
/// returned by the Typesense server, to client-side data handling problems.
///
/// The generic parameter `E` represents the specific error type associated with a
/// particular API operation (e.g., `SearchCollectionError`, `GetDocumentError`).
#[derive(Debug, Error)]
pub enum Error<E>
where
    E: std::fmt::Debug + 'static,
    ApiError<E>: std::error::Error + 'static,
{
    /// Occurs when an operation fails against all configured Typesense nodes.
    ///
    /// This error is only returned when using a client configured with multiple nodes.
    /// It signifies that the client attempted the operation against each node in turn,
    /// and every attempt failed. The user should check the health and connectivity
    /// of all their Typesense nodes.
    ///
    /// The `source` field contains the error from the *last* node that was attempted.
    #[error("All configured Typesense nodes failed to respond. Last error: {source}")]
    AllNodesFailed {
        /// The underlying API or network error from the last node attempt.
        #[source]
        source: ApiError<E>,
    },

    /// Wraps an error returned by the Typesense API or the underlying network stack.
    ///
    /// This can be due to:
    /// - A server-side issue (e.g., HTTP 5xx errors).
    /// - A client-side mistake (e.g., HTTP 4xx errors like `404 Not Found` or `401 Unauthorized`).
    /// - A network connectivity problem (e.g., connection refused, timeout, DNS failure).
    ///
    /// You should inspect the wrapped error to get specific details about the HTTP status code and response body.
    #[error("An API or network error occurred: {0}")]
    Api(#[from] ApiError<E>),

    /// Occurs when the JSON response from Typesense cannot be deserialized into the target Rust struct.
    ///
    /// This typically signifies a mismatch between the data in your Typesense collection
    /// and the fields or data types defined in your Rust struct (`T`).
    ///
    /// **To debug this, check for:**
    /// - A field that exists in Typesense but not in your struct (unless your struct ignores unknown fields).
    /// - A field in your struct that doesn't exist in the Typesense document and is not wrapped in an `Option`.
    /// - A type mismatch (e.g., a Typesense `string` field that you are trying to deserialize into a `u64`).
    #[error("Failed to deserialize the API response into the target struct: {0}")]
    Deserialization(#[from] serde_json::Error),
}

/// Represents the possible errors that can occur when parsing a `multi_search` response.
///
/// This error enum is returned by the `MultiSearchResultExt::parse_at` method when it
/// fails to convert a raw search result into a strongly-typed `SearchResult<T>`.
#[derive(Debug, Error)]
pub enum MultiSearchParseError {
    /// Indicates that the requested index was outside the bounds of the results vector.
    ///
    /// For a `multi_search` request with `n` search queries, the valid indices for the
    /// results are `0` through `n-1`. This error occurs if the provided index is `n` or greater.
    ///
    /// # Fields
    /// * `0` - The invalid index that was requested.
    #[error("Search result index {0} is out of bounds.")]
    IndexOutOfBounds(usize),

    /// Indicates that the Typesense server returned an error for the specific search query at this index.
    ///
    // It's possible for a `multi_search` request to succeed overall, but for one or more
    // individual searches within it to fail (e.g., due to a typo in a collection name).
    ///
    /// # Fields
    /// * `index` - The index of the search query that failed.
    /// * `message` - The error message returned by the Typesense API for this specific search.
    #[error("The search at index {index} failed with an API error: {message}")]
    ApiError {
        /// The index of the search query that failed.
        index: usize,
        /// The error message returned by the Typesense API for this specific search.
        message: String,
    },

    /// Indicates a failure to deserialize a document's JSON into the target struct `T`.
    ///
    /// This typically happens when the fields in the document stored in Typesense do not
    /// match the fields defined in the target Rust struct `T`. Check for mismatches in
    /// field names or data types.
    ///
    /// # Fields
    /// * `index` - The index of the search query where the deserialization error occurred.
    /// * `source` - The underlying `serde_json::Error` that provides detailed information
    ///   about the deserialization failure.
    #[error("Failed to deserialize a document at index {index}: {source}")]
    Deserialization {
        /// The index of the search query where the deserialization error occurred.
        index: usize,
        /// The underlying `serde_json::Error` that provides detailed information
        /// about the deserialization failure.
        #[source]
        source: serde_json::Error,
    },
}
