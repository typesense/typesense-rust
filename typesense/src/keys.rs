//! Module containing everything related to Keys API.
//!
//! More info [here](https://typesense.org/docs/0.20.0/api/api-keys.html).

use base64::{engine::general_purpose::STANDARD as Base64Engine, Engine};
use core::fmt;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use typesense_codegen::models::ScopedKeyParameters;

/// Generate a scoped search API key that can have embedded search parameters in them.
///
/// More info [here](https://typesense.org/docs/0.20.0/api/api-keys.html#generate-scoped-search-key).
pub async fn generate_scoped_search_key(
    key: impl AsRef<str>,
    filter_by: impl Into<String>,
    expires_at: f32,
) -> anyhow::Result<String> {
    let generate_scoped_search_key = ScopedKeyParameters {
        filter_by: Some(filter_by.into()),
        expires_at: Some(expires_at),
    };
    let params = serde_json::to_string(&generate_scoped_search_key)?;

    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_ref().as_bytes())?;
    mac.update(params.as_bytes());
    let result = mac.finalize();
    let digest = Base64Engine.encode(result.into_bytes());

    let key_prefix = &key.as_ref()[0..4];
    let raw_scoped_key = format!("{}{}{}", digest, key_prefix, params);

    Ok(Base64Engine.encode(raw_scoped_key.as_bytes()))
}

/// Enum over the possible list of Actions.
///
/// More info [here](https://typesense.org/docs/0.25.2/api/api-keys.html#sample-actions).
#[derive(Serialize, Deserialize)]
pub enum Actions {
    /// Allows only search requests.
    #[serde(rename = "documents:search")]
    DocumentsSearch,

    /// Allows fetching a single document.
    #[serde(rename = "documents:get")]
    DocumentsGet,

    /// Allow all kinds of collection related operations.
    #[serde(rename = "documents:*")]
    DocumentsAll,

    /// Allows a collection to be deleted.
    #[serde(rename = "collections:delete")]
    CollectionsDelete,

    /// Allows a collection to be created.
    #[serde(rename = "collections:create")]
    CollectionsCreate,

    /// Allow all kinds of collection related operations.
    #[serde(rename = "collections:*")]
    CollectionsAll,

    /// Allows all operations.
    #[serde(rename = "*")]
    All,
}

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DocumentsAll => write!(f, "documents:*"),
            Self::DocumentsSearch => write!(f, "documents:search"),
            Self::DocumentsGet => write!(f, "documents:get"),
            Self::CollectionsAll => write!(f, "collections:*"),
            Self::CollectionsDelete => write!(f, "collections:delete"),
            Self::CollectionsCreate => write!(f, "collections:create"),
            Self::All => write!(f, "*"),
        }
    }
}
