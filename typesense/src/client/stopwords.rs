//! Provides access to the API endpoints for managing stopwords sets.
//!
//! A `Stopwords` instance is created via the main `Client::stopwords()` method.

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, stopwords_api},
    models,
};

/// Provides methods for managing Typesense stopwords sets.
///
/// This struct is created by calling `client.stopwords()`.
pub struct Stopwords<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Stopwords<'a> {
    /// Creates a new `Stopwords` instance.
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
        set_id: &str,
        schema: models::StopwordsSetUpsertSchema,
    ) -> Result<models::StopwordsSetSchema, Error<stopwords_api::UpsertStopwordsSetError>> {
        let params = stopwords_api::UpsertStopwordsSetParams {
            set_id: set_id.to_string(),
            stopwords_set_upsert_schema: schema,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { stopwords_api::upsert_stopwords_set(&config, params_for_move).await }
            })
            .await
    }

    /// Retrieves the details of all stopwords sets.
    pub async fn retrieve(
        &self,
    ) -> Result<
        models::StopwordsSetsRetrieveAllSchema,
        Error<stopwords_api::RetrieveStopwordsSetsError>,
    > {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                stopwords_api::retrieve_stopwords_sets(&config).await
            })
            .await
    }
}
