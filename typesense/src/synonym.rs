//! # Synonyms
//!
//! The synonyms feature allows you to define search terms that should be considered
//! equivalent. For eg: when you define a synonym for `sneaker` as `shoe`, searching
//! for `sneaker` will now return all records with the word `shoe` in them, in addition
//! to records with the word `sneaker`.
//! 
//! Typesense supports two types of synonyms:
//! 1. **One-way synonyms:** Defining the words `iphone` and `android` as one-way synonyms
//! of `smart phone` will cause searches for `smart phone` to return documents containing
//! `iphone` or `android` or both.
//! 2. **Multi-way synonyms:** Defining the words `blazer`, `coat` and `jacket` as multi-way
//! synonyms will cause searches for any one of those words (eg: `coat`) to return documents
//! containing at least one of the words in the synonym set (eg: records with `blazer` or
//! `coat` or `jacket` are returned).
//!

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::document::Document;
use crate::transport::HttpLowLevel;
use crate::Result;

/// Client for the Typesense SynonymAPI
pub struct SynonymClient<T> {
    pub(crate) client: Client<T>,
}

impl<T> SynonymClient<T>
where
    T: HttpLowLevel,
{
    /// Create a collection in Typesense for a [`Document`] type.
    pub async fn create<D: Document>(&self) -> Result<SynonymResponse> {
        let schema = D::synonym_schema();
        self.create_from_schema(schema).await
    }

    /// Create a Synonym in Typesense given a ['SynonymSchema`]
    pub async fn create_from_schema(&self, schema: SynonymSchema) -> Result<SynonymResponse> {
        let body = serde_json::to_vec(&schema)?;

        let response_body = self
            .client
            .post("/synonyms", body)
            .await?
            .into_body();

        let response: SynonymResponse = serde_json::from_slice(&response_body)?;

        Ok(response)
    }

    /// Retrieve a single synonym
    pub async fn retrieve(&self, synonym_name: &str) -> Result<SynonymResponse> {
        let path = format!("/synonyms/{}", synonym_name);

        let response_body = self.client.get(&path).await?.into_body();

        let response: SynonymResponse = serde_json::from_slice(&response_body)?;

        Ok(response)
    }

    /// List all synonyms associated with a given collection
    pub async fn list_all(&self) -> Result<SynonymListResponse> {
        let response_body = self.client.get("/synonyms").await?.into_body();
        let response: SynonymListResponse = serde_json::from_slice(&response_body)?;

        Ok(response)
    }

    /// Delete a synonym associated with a collection.
    pub async fn delete(&self, synonym_name: &str) -> Result<SynonymResponse> {
        let path = format!("/synonyms/{}", synonym_name);

        let response_body = self.client.delete(&path).await?.into_body();
        let response: SynonymResponse = serde_json::from_slice(&response_body)?;

        Ok(response)
    }
}

/// Schema to create Synonyms in the Typesense Synonym API.
#[derive(Deserialize, Serialize)]
pub struct SynonymSchema {
    /// root word associated with a One-way Synonym
    /// NOTE: Left as None in the case of Multi-way synonym
    pub root: Option<String>,
    /// List of synonyms
    pub synonyms: Vec<String>
}

/// Represents a Response from the Typesense Synonym API.
#[derive(Deserialize, Serialize)]
pub struct SynonymResponse {
    /// id associated with the synonym stored in Typesense
    pub id: String,
    /// root of a one-way synonym stored in Typesense
    /// NOTE: Will be None in case of multi-way synonyms
    pub root: Option<String>,
    /// List of associated synonyms stored in Typesense
    pub synonyms: Vec<String>,
}

/// Represents a List format Response from the Typesense Synonym API.
#[derive(Deserialize, Serialize)]
pub struct SynonymListResponse {
    /// List of synonyms
    pub synonyms: Vec<SynonymResponse>
}
