//! # Collection
//!
//! In Typesense, a group of related documents is called a collection. A collection
//! is roughly equivalent to a table in a relational database.
//!

use serde::{Deserialize, Serialize};
mod schema;
pub use schema::{CollectionSchema, CollectionSchemaBuilder};

use crate::client::Client;
use crate::document::Document;
use crate::transport::HttpLowLevel;
use crate::Result;

/// Client for the Typesense CollectionAPI
pub struct CollectionClient<'c, T> {
    pub(crate) client: &'c Client<T>,
}

impl<'c, T> CollectionClient<'c, T>
where
    T: HttpLowLevel,
{
    /// Create a collection in Typesense for a [`Document`] type.
    pub async fn create<D: Document>(&self) -> Result<CollectionResponse> {
        let schema = D::collection_schema();
        self.create_from_schema(schema).await
    }

    /// Create a Collection in Typesense given a ['CollectionSchema`]
    pub async fn create_from_schema(&self, schema: CollectionSchema) -> Result<CollectionResponse> {
        let body = serde_json::to_vec(&schema).expect("unable to serialize ");

        let response_body = self
            .client
            .post("/collections", body)
            .await
            .unwrap()
            .into_body();

        let response: CollectionResponse = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }

    /// Retrieve the details of the collection given a collection name
    pub async fn retrieve(&self, collection_name: &str) -> Result<CollectionResponse> {
        let path = format!("/collections/{}", collection_name);

        let response_body = self.client.get(&path).await.unwrap().into_body();

        let response: CollectionResponse = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }

    /// Retrieve all the collections
    pub async fn retrieve_all(&self) -> Result<CollectionListResponse> {
        let response_body = self.client.get("/collections").await.unwrap().into_body();
        let response: CollectionListResponse = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }

    /// Permanently drops a collection. This action cannot be undone.
    /// For large collections, this might have an impact on read latencies.
    pub async fn delete(&self, collection_name: &str) -> Result<CollectionResponse> {
        let path = format!("/collections/{}", collection_name);

        let response_body = self.client.delete(&path).await.unwrap().into_body();
        let response: CollectionResponse = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }
}

/// Represents a Response from the Typesense Collection API.
#[derive(Deserialize, Serialize)]
pub struct CollectionResponse {
    /// schema of the collection stored in Typesense
    #[serde(flatten)]
    pub schema: CollectionSchema,
    /// current number of documents in Typesense
    pub num_documents: usize,
}

type CollectionListResponse = Vec<CollectionResponse>;
