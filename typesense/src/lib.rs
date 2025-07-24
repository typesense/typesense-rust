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

pub mod client;
pub mod collection_schema;
pub mod document;
pub mod field;
pub mod keys;

pub use typesense_codegen::*;

#[cfg(feature = "typesense_derive")]
#[doc(hidden)]
pub use typesense_derive::*;
