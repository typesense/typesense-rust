//! Provides access to the API endpoints for managing a single stopwords set.
//!
//! An instance of `Stopword` is created via the `Client::stopword()` method.

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, stopwords_api},
    models,
};

/// Provides methods for interacting with a specific stopwords set.
///
/// This struct is created by calling `client.stopword("set_id")`.
pub struct Stopword<'a> {
    pub(super) client: &'a Client,
    pub(super) set_id: &'a str,
}

impl<'a> Stopword<'a> {
    /// Creates a new `Stopword` instance for a specific set ID.
    pub(super) fn new(client: &'a Client, set_id: &'a str) -> Self {
        Self { client, set_id }
    }

    /// Retrieves the details of this specific stopwords set.
    pub async fn retrieve(
        &self,
    ) -> Result<models::StopwordsSetRetrieveSchema, Error<stopwords_api::RetrieveStopwordsSetError>>
    {
        let params = stopwords_api::RetrieveStopwordsSetParams {
            set_id: self.set_id.to_string(),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { stopwords_api::retrieve_stopwords_set(&config, params_for_move).await }
            })
            .await
    }

    /// Permanently deletes this specific stopwords set.
    pub async fn delete(
        &self,
    ) -> Result<models::DeleteStopwordsSet200Response, Error<stopwords_api::DeleteStopwordsSetError>>
    {
        let params = stopwords_api::DeleteStopwordsSetParams {
            set_id: self.set_id.to_string(),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { stopwords_api::delete_stopwords_set(&config, params_for_move).await }
            })
            .await
    }
}
