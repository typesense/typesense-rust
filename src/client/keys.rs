use serde::{Deserialize, Serialize};

use super::Client;
use crate::transport::HttpLowLevel;

/// To interact with the Keys API.
pub struct ClientKeys<'a, T> {
    pub(super) client: Client<'a, T>,
}

impl<'a, T> ClientKeys<'a, T>
where
    T: HttpLowLevel,
{
    /// Create an API Key.
    pub async fn create(
        &self,
        actions: Vec<String>,
        collections: Vec<String>,
        description: impl Into<Option<String>>,
        expires_at: impl Into<Option<usize>>,
    ) -> crate::Result<ClientKeyCreate> {
        let create = Create {
            actions,
            collections,
            description: description.into(),
            expires_at: expires_at.into(),
        };

        let response = self
            .client
            .post("/keys", serde_json::to_vec(&create)?)
            .await?;

        let body = response.into_body();
        Ok(serde_json::from_slice(&body)?)
    }

    /// Retrieve (metadata about) a key.
    pub async fn retrieve(&self, n: usize) -> crate::Result<ClientKeyRetrieve> {
        let response = self.client.get(format!("/keys/{}", n).as_str()).await?;

        let body = response.into_body();
        Ok(serde_json::from_slice(&body)?)
    }

    /// Retrieve (metadata about) all keys.
    pub async fn retrieve_all(&self) -> crate::Result<ClientKeyRetrieveAll> {
        let response = self.client.get("/keys").await?;

        let body = response.into_body();
        Ok(serde_json::from_slice(&body)?)
    }

    /// Delete an API key given its ID.
    pub async fn delete(&self, n: usize) -> crate::Result<ClientKeyDelete> {
        let response = self.client.delete(format!("/keys/{}", n).as_str()).await?;

        let body = response.into_body();
        Ok(serde_json::from_slice(&body)?)
    }

    // /// Generate a scoped search API key that can have embedded search parameters in them.
    // pub async fn generate_scoped_search_key(
    //     key: String,
    //     filter_by: String,
    //     expires_at: usize,
    // ) -> crate::Result<String> {
    //     todo!()
    // }
}

#[derive(Deserialize)]
pub struct ClientKeyCreate {
    pub id: usize,
    pub actions: Vec<String>,
    pub collections: Vec<String>,
    pub value: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ClientKeyRetrieve {
    pub actions: Vec<String>,
    pub collections: Vec<String>,
    pub description: String,
    pub id: usize,
    pub value_prefix: String,
}

#[derive(Deserialize)]
pub struct ClientKeyRetrieveAll {
    pub keys: Vec<ClientKeyRetrieve>,
}

#[derive(Deserialize)]
pub struct ClientKeyDelete {
    pub id: usize,
}

#[derive(Serialize)]
struct Create {
    actions: Vec<String>,
    collections: Vec<String>,
    description: Option<String>,
    expires_at: Option<usize>,
}
