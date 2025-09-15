//! Provides access to API endpoints for a single document within a Typesense collection.
//!
//! An instance of `Document` is scoped to a specific document and is created
//! via a parent `Collection` struct, for example:
//! `client.collection::<Book>().document("123")`

use crate::{Client, Error, execute_wrapper, traits};
use serde::{Serialize, de::DeserializeOwned};
use typesense_codegen::apis::documents_api;

/// Provides methods for interacting with a single document within a specific Typesense collection.
///
/// This struct is created by calling a method like `client.collection_schemaless("collection_name").document("document_id")`
/// or `client.collection::<MyType>().document("document_id")`.
/// The generic `D` represents the shape of the document and must implement `Serialize` and `DeserializeOwned`.
/// If `D` is not specified, it defaults to `serde_json::Value` for schemaless interactions.
pub struct Document<'c, 'n, D = serde_json::Value>
where
    D: DeserializeOwned + Serialize,
{
    client: &'c Client,
    collection_name: &'n str,
    document_id: String,
    _phantom: std::marker::PhantomData<D>,
}

impl<'c, 'n, D> Document<'c, 'n, D>
where
    D: DeserializeOwned + Serialize,
{
    /// Creates a new `Document` instance for a specific document ID.
    #[inline]
    pub(super) fn new(client: &'c Client, collection_name: &'n str, document_id: String) -> Self {
        Self {
            client,
            collection_name,
            document_id,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Fetches this individual document from the collection and deserializes it into `D`.
    ///
    /// # Returns
    /// A `Result` containing the strongly-typed document `D` if successful.
    pub async fn retrieve(&self) -> Result<D, Error<documents_api::GetDocumentError>> {
        let params = documents_api::GetDocumentParams {
            collection_name: self.collection_name.to_owned(),
            document_id: self.document_id.to_owned(),
        };

        let result_value = execute_wrapper!(self, documents_api::get_document, params)?;

        // Deserialize the raw JSON value into the user's type T.
        serde_json::from_value(result_value).map_err(Error::from)
    }

    /// Deletes this individual document from the collection.
    /// The deleted document is returned.
    ///
    /// # Returns
    /// A `Result` containing the deleted document deserialized into `D`.
    pub async fn delete(&self) -> Result<D, Error<documents_api::DeleteDocumentError>> {
        let params = documents_api::DeleteDocumentParams {
            collection_name: self.collection_name.to_owned(),
            document_id: self.document_id.to_owned(),
        };

        let result_value = execute_wrapper!(self, documents_api::delete_document, params)?;

        // Deserialize the raw JSON value of the deleted document into T.
        serde_json::from_value(result_value).map_err(Error::from)
    }
}

impl<'c, 'n, D> Document<'c, 'n, D>
where
    D: traits::Document,
{
    /// Updates this individual document. The update can be partial.
    /// The updated full document is returned.
    ///
    /// # Arguments
    /// * `partial_document` - A serializable struct or a `serde_json::Value` containing the fields to update. For example: `serde_json::json!({ "in_stock": false })`.
    /// * `params` - An optional `DocumentIndexParameters` struct to specify additional parameters, such as `dirty_values` which determines what Typesense should do when the type of a particular field being indexed does not match the previously inferred type for that field, or the one defined in the collection's schema.
    ///
    /// # Returns
    /// A `Result` containing the full, updated document deserialized into `D`.
    ///
    /// # Example
    /// ```no_run
    /// # use serde::{Serialize, Deserialize};
    /// # use typesense::{Client, Typesense, models};
    /// # use reqwest::Url;
    /// # #[derive(Typesense, Serialize, Deserialize)]
    /// # struct Book { id: String, title: String, pages: i32 }
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let book_update = BookPartial { pages: Some(654), ..Default::default() };
    ///
    /// // Simple update
    /// let updated_book = client.collection_named::<Book>("books").document("123")
    ///     .update(&book_update, None)
    ///     .await?;
    ///
    /// // Update with additional parameters
    /// let params = models::DocumentIndexParameters {
    ///     dirty_values: Some(models::DirtyValues::CoerceOrReject),
    /// };
    /// let updated_book_with_params = client.collection_named::<Book>("books").document("124")
    ///     .update(&book_update, Some(params))
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        partial_document: &D::Partial,
        params: Option<crate::models::DocumentIndexParameters>,
    ) -> Result<D, Error<documents_api::UpdateDocumentError>> {
        let params = documents_api::UpdateDocumentParams {
            collection_name: self.collection_name.to_owned(),
            document_id: self.document_id.to_owned(),
            body: partial_document,
            dirty_values: params.and_then(|d| d.dirty_values),
        };

        let result_value = execute_wrapper!(self, documents_api::update_document, params)?;

        // Deserialize the raw JSON value of the updated document into T.
        serde_json::from_value(result_value).map_err(Error::from)
    }
}
