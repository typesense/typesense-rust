//! Provides access to the API endpoints for managing a single stopwords set.
//!
//! An instance of `Stopword` is created via the `client.stopword("set_id")` method.

use crate::{Client, Error, execute_wrapper};
use typesense_codegen::{apis::stopwords_api, models};

/// Provides methods for interacting with a specific stopwords set.
///
/// This struct is created by calling `client.stopword("set_id")`.
pub struct Stopword<'a> {
    pub(super) client: &'a Client,
    pub(super) set_id: &'a str,
}

impl<'a> Stopword<'a> {
    /// Creates a new `Stopword` instance for a specific set ID.
    #[inline]
    pub(super) fn new(client: &'a Client, set_id: &'a str) -> Self {
        Self { client, set_id }
    }

    /// Retrieves the details of this specific stopwords set.
    pub async fn retrieve(
        &self,
    ) -> Result<
        models::StopwordsSetRetrieveSchema<'static>,
        Error<stopwords_api::RetrieveStopwordsSetError>,
    > {
        let params = stopwords_api::RetrieveStopwordsSetParams {
            set_id: self.set_id.into(),
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, stopwords_api::retrieve_stopwords_set, params)
    }

    /// Permanently deletes this specific stopwords set.
    pub async fn delete(
        &self,
    ) -> Result<
        models::DeleteStopwordsSet200Response<'static>,
        Error<stopwords_api::DeleteStopwordsSetError>,
    > {
        let params = stopwords_api::DeleteStopwordsSetParams {
            set_id: self.set_id.into(),
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, stopwords_api::delete_stopwords_set, params)
    }
}
