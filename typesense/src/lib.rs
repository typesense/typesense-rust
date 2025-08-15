#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Typesense
//!
//! Rust client library for Typesense
//!
//! # Examples
//!
//! ```
//! #[cfg(any(feature = "tokio_test", target_arch = "wasm32"))]
//! {
//! use serde::{Deserialize, Serialize};
//! use typesense::document::Document;
//! use typesense::Typesense;
//! use typesense::apis::collections_api;
//! use typesense::apis::configuration::{ApiKey, Configuration};
//!
//! #[derive(Typesense, Serialize, Deserialize)]
//! #[typesense(collection_name = "companies", default_sorting_field = "num_employees")]
//! struct Company {
//!     company_name: String,
//!     num_employees: i32,
//!     #[typesense(facet)]
//!     country: String,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = Configuration {
//!         base_path: "http://localhost:5000".to_owned(),
//!         api_key: Some(ApiKey {
//!             prefix: None,
//!             key: "VerySecretKey".to_owned(),
//!         }),
//!         ..Default::default()
//!     };
//!
//!     let collection = collections_api::create_collection(&config, Company::collection_schema())
//!         .await
//!         .unwrap();
//! }
//! }
//! ```

mod client;
mod traits;

pub mod builders;
pub mod error;
pub mod models;
pub mod prelude;

pub use builders::*;
pub use client::{Client, MultiNodeConfiguration};
pub use error::*;
pub use models::*;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use typesense_derive::*;
