//! Provides access to API endpoints for a single document within a Typesense collection.
//!
//! An instance of `Document` is scoped to a specific document and is created
//! via a parent `Collection` struct, for example:
//! `client.collection::<Book>("books").document("123")`

use crate::{Client, Error};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, documents_api},
    models,
};

/// Provides methods for interacting with a single document within a specific Typesense collection.
///
/// This struct is created by calling a method like `client.collection::<T>("collection_name").document("document_id")`.
/// The generic `T` represents the shape of the document and must implement `Serialize` and `DeserializeOwned`.
/// If `T` is not specified, it defaults to `serde_json::Value` for schemaless interactions.
pub struct Document<'a, T = serde_json::Value>
where
    T: DeserializeOwned + Serialize + Send + Sync,
{
    pub(super) client: &'a Client,
    pub(super) collection_name: &'a str,
    pub(super) document_id: &'a str,
    pub(super) _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> Document<'a, T>
where
    T: DeserializeOwned + Serialize + Send + Sync,
{
    /// Creates a new `Document` instance for a specific document ID.
    /// This is intended for internal use by the parent `Collection` struct.
    pub(super) fn new(client: &'a Client, collection_name: &'a str, document_id: &'a str) -> Self {
        Self {
            client,
            collection_name,
            document_id,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Fetches this individual document from the collection and deserializes it into `T`.
    ///
    /// # Returns
    /// A `Result` containing the strongly-typed document `T` if successful.
    pub async fn retrieve(&self) -> Result<T, Error<documents_api::GetDocumentError>> {
        let params = documents_api::GetDocumentParams {
            collection_name: self.collection_name.to_string(),
            document_id: self.document_id.to_string(),
        };

        let result_value = self
            .client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::get_document(&config, params_for_move).await }
            })
            .await?;

        // Deserialize the raw JSON value into the user's type T.
        serde_json::from_value(result_value).map_err(Error::from)
    }

    /// Updates this individual document. The update can be partial.
    /// The updated full document is returned.
    ///
    /// # Arguments
    /// * `partial_document` - A serializable struct or a `serde_json::Value` containing the fields to update.
    ///                        For example: `serde_json::json!({ "in_stock": false })`.
    /// * `params` - An optional `DocumentIndexParameters` struct to specify additional
    ///              parameters, such as `dirty_values` which determines what Typesense should do when the type of a particular field being indexed does not match the previously inferred type for that field, or the one defined in the collection's schema.
    ///
    /// # Returns
    /// A `Result` containing the full, updated document deserialized into `T`.
    ///
    /// # Example
    /// ```no_run
    /// # use serde::{Serialize, Deserialize};
    /// # use typesense::{Client, models};
    /// # use reqwest::Url;
    /// # #[derive(Serialize, Deserialize)]
    /// # struct Book { id: String, title: String, pages: i32 }
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let book_update = serde_json::json!({ "pages": 654 });
    ///
    /// // Simple update
    /// let updated_book = client.collection_of::<Book>("books").document("123")
    ///     .update(&book_update, None)
    ///     .await?;
    ///
    /// // Update with additional parameters
    /// let params = models::DocumentIndexParameters {
    ///     dirty_values: Some(models::DirtyValues::CoerceOrReject),
    /// };
    /// let updated_book_with_params = client.collection_of::<Book>("books").document("124")
    ///     .update(&book_update, Some(params))
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update<U: Serialize>(
        &self,
        partial_document: &U,
        params: Option<models::DocumentIndexParameters>,
    ) -> Result<T, Error<documents_api::UpdateDocumentError>> {
        let params = documents_api::UpdateDocumentParams {
            collection_name: self.collection_name.to_string(),
            document_id: self.document_id.to_string(),
            body: serde_json::to_value(partial_document)?,
            dirty_values: params.unwrap_or_default().dirty_values,
        };

        let result_value = self
            .client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::update_document(&config, params_for_move).await }
            })
            .await?;

        // Deserialize the raw JSON value of the updated document into T.
        serde_json::from_value(result_value).map_err(Error::from)
    }

    /// Deletes this individual document from the collection.
    /// The deleted document is returned.
    ///
    /// # Returns
    /// A `Result` containing the deleted document deserialized into `T`.
    pub async fn delete(&self) -> Result<T, Error<documents_api::DeleteDocumentError>> {
        let params = documents_api::DeleteDocumentParams {
            collection_name: self.collection_name.to_string(),
            document_id: self.document_id.to_string(),
        };

        let result_value = self
            .client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::delete_document(&config, params_for_move).await }
            })
            .await?;

        // Deserialize the raw JSON value of the deleted document into T.
        serde_json::from_value(result_value).map_err(Error::from)
    }
}
