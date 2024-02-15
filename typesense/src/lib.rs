#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Typesense
//!
//! Rust client library for Typesense
//!
//! # Examples
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use typesense::document::Document as DocumentTrait;
//! use typesense::Document;
//! use typesense::openapi::apis::collections_api;
//! use typesense::openapi::apis::configuration::{ApiKey, Configuration};
//!
//! #[derive(Document, Serialize, Deserialize)]
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
//! ```

pub use typesense_codegen as openapi;

pub mod collection_schema;
pub mod document;
pub mod field;
pub mod keys;

#[cfg(feature = "typesense_derive")]
#[doc(hidden)]
pub use typesense_derive::*;
