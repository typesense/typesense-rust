//! Provides access to API endpoints for a single document within a Typesense collection.
//!
//! An instance of `Document` is scoped to a specific document and is created
//! via a parent `Collection` struct, for example:
//! `client.collection("collection_name").document("document_id")`

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::apis::{configuration, documents_api};

/// Provides methods for interacting with a single document within a specific Typesense collection.
///
/// This struct is created by calling a method like `collection.document("document_id")`.
pub struct Document<'a> {
    pub(super) client: &'a Client,
    pub(super) collection_name: &'a str,
    pub(super) document_id: &'a str,
}

impl<'a> Document<'a> {
    /// Creates a new `Document` instance for a specific document ID.
    /// This is intended for internal use by the parent `Documents` struct.
    pub(super) fn new(client: &'a Client, collection_name: &'a str, document_id: &'a str) -> Self {
        Self {
            client,
            collection_name,
            document_id,
        }
    }

    /// Fetches this individual document from the collection.
    pub async fn get(&self) -> Result<serde_json::Value, Error<documents_api::GetDocumentError>> {
        let params = documents_api::GetDocumentParams {
            collection_name: self.collection_name.to_string(),
            document_id: self.document_id.to_string(),
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::get_document(&config, params_for_move).await }
            })
            .await
    }

    /// Updates this individual document. The update can be partial.
    ///
    /// # Arguments
    /// * `document` - A `serde_json::Value` containing the fields to update.
    pub async fn update(
        &self,
        document: serde_json::Value,
    ) -> Result<serde_json::Value, Error<documents_api::UpdateDocumentError>> {
        let params = documents_api::UpdateDocumentParams {
            collection_name: self.collection_name.to_string(),
            document_id: self.document_id.to_string(),
            body: document,
            dirty_values: None,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::update_document(&config, params_for_move).await }
            })
            .await
    }

    /// Deletes this individual document from the collection.
    pub async fn delete(
        &self,
    ) -> Result<serde_json::Value, Error<documents_api::DeleteDocumentError>> {
        let params = documents_api::DeleteDocumentParams {
            collection_name: self.collection_name.to_string(),
            document_id: self.document_id.to_string(),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::delete_document(&config, params_for_move).await }
            })
            .await
    }
}
