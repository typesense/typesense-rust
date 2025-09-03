//! Provides access to the collection and alias-related API endpoints.
//!
//! A `Collections` instance is created via the main `client.collections()` method.

use crate::{Client, Error};
use typesense_codegen::{
    apis::{
        collections_api::{self, GetCollectionsParams},
        configuration,
    },
    models::{self, GetCollectionsParameters},
};

/// Provides methods for interacting with Typesense collections and aliases.
///
/// This struct is created by calling `client.collections()`.
pub struct Collections<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Collections<'a> {
    /// Creates a new `Collection` instance
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates a new collection with the given schema.
    ///
    /// # Arguments
    /// * `schema` - A `CollectionSchema` object describing the collection to be created.
    pub async fn create(
        &self,
        schema: models::CollectionSchema,
    ) -> Result<models::CollectionResponse, Error<collections_api::CreateCollectionError>> {
        let params = collections_api::CreateCollectionParams {
            collection_schema: schema,
        };

        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = params.clone();
                async move { collections_api::create_collection(&config, params_for_move).await }
            })
            .await
    }

    /// List the existing Typesense collections.
    pub async fn retrieve(
        &self,
        params: GetCollectionsParameters,
    ) -> Result<Vec<models::CollectionResponse>, Error<collections_api::GetCollectionsError>> {
        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = GetCollectionsParams {
                    exclude_fields: params.exclude_fields.clone(),
                    limit: params.limit,
                    offset: params.offset,
                };
                async move { collections_api::get_collections(&config, params_for_move).await }
            })
            .await
    }
}
