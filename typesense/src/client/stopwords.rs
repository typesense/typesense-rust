//! Provides access to the API endpoints for managing stopwords sets.
//!
//! A `Stopwords` instance is created via the main `client.stopwords()` method.

use crate::{Client, Error, execute_wrapper};
use ::std::borrow::Cow;
use typesense_codegen::{apis::stopwords_api, models};

/// Provides methods for managing Typesense stopwords sets.
///
/// This struct is created by calling `client.stopwords()`.
pub struct Stopwords<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Stopwords<'a> {
    /// Creates a new `Stopwords` instance.
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates or updates an existing stopwords set.
    ///
    /// # Arguments
    /// * `set_id` - The ID of the stopwords set to create or update.
    /// * `schema` - A `StopwordsSetUpsertSchema` object with the stopwords to upsert.
    pub async fn upsert(
        &self,
        set_id: impl Into<Cow<'_, str>>,
        schema: models::StopwordsSetUpsertSchema<'_>,
    ) -> Result<models::StopwordsSetSchema, Error<stopwords_api::UpsertStopwordsSetError>> {
        let params = stopwords_api::UpsertStopwordsSetParams {
            set_id: set_id.into(),
            stopwords_set_upsert_schema: schema,
        };
        execute_wrapper!(self, stopwords_api::upsert_stopwords_set, params)
    }

    /// Retrieves the details of all stopwords sets.
    pub async fn retrieve(
        &self,
    ) -> Result<
        models::StopwordsSetsRetrieveAllSchema,
        Error<stopwords_api::RetrieveStopwordsSetsError>,
    > {
        execute_wrapper!(self, stopwords_api::retrieve_stopwords_sets)
    }
}
