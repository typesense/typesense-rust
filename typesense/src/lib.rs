#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Typesense
//!
//! A Rust client library for the Typesense API.
//!
//! This library provides a modern, ergonomic, and async-native interface for
//! interacting with Typesense, with special attention to multi-node clusters and
//! platform-specific environments like WebAssembly.
//!
//! # Examples
//!
//! The following examples demonstrate how to define a collection schema using
//! the Typesense derive macro and create it on the server.
//!
//! ---
//!
//! ### Native (Tokio)
//!
//! This example shows the typical setup for a server-side application using the
//! Tokio runtime. It includes features like connection timeouts and automatic
//! request retries.
//!
//! ```no_run
//! #[cfg(not(target_family = "wasm"))]
//! {
//!     use serde::{Deserialize, Serialize};
//!     use typesense::{Client, Typesense, prelude::*};
//!     use reqwest::Url;
//!     use reqwest_retry::policies::ExponentialBackoff;
//!     use std::time::Duration;
//!
//!     /// A struct representing a company document.
//!     #[derive(Typesense, Serialize, Deserialize, Debug)]
//!     #[typesense(
//!         collection_name = "companies",
//!         default_sorting_field = "num_employees"
//!     )]
//!     struct Company {
//!         company_name: String,
//!         num_employees: i32,
//!         #[typesense(facet)]
//!         country: String,
//!     }
//!
//!     #[tokio::main]
//!     async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!         let client = Client::builder()
//!             .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
//!             .api_key("xyz")
//!             .healthcheck_interval(Duration::from_secs(60))
//!             .retry_policy(ExponentialBackoff::builder().build_with_max_retries(3))
//!             .connection_timeout(Duration::from_secs(5))
//!             .build()?;
//!
//!         // Create the collection in Typesense
//!         let collection = client
//!             .collections()
//!             .create(Company::collection_schema())
//!             .await?;
//!
//!         println!("Created collection: {:?}", collection);
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ---
//!
//! ### WebAssembly (Wasm)
//!
//! This example is tailored for a WebAssembly target.
//! Key differences:
//! - The `main` function is synchronous and uses `spawn_local` to run async code.
//! - Tokio-dependent features like `.retry_policy()` and `.connection_timeout()`
//!   are disabled. You can still set them in the client builder but it will do nothing.
//!
//! ```no_run
//! #[cfg(target_family = "wasm")]
//! {
//!     use serde::{Deserialize, Serialize};
//!     use typesense::{Client, Typesense, prelude::*};
//!     use reqwest::Url;
//!     use std::time::Duration;
//!     use wasm_bindgen_futures::spawn_local;
//!
//!     /// A struct representing a company document.
//!     #[derive(Typesense, Serialize, Deserialize, Debug)]
//!     #[typesense(
//!         collection_name = "companies",
//!         default_sorting_field = "num_employees"
//!     )]
//!     struct Company {
//!         company_name: String,
//!         num_employees: i32,
//!         #[typesense(facet)]
//!         country: String,
//!     }
//!
//!     fn main() {
//!         spawn_local(async {
//!             let client = Client::builder()
//!                 .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
//!                 .api_key("xyz")
//!                 .healthcheck_interval(Duration::from_secs(60))
//!                 // .retry_policy(...)       <-- disabled in Wasm
//!                 // .connection_timeout(...) <-- disabled in Wasm
//!                 .build()
//!                 .unwrap();
//!
//!             // Create the collection in Typesense
//!             match client.collections().create(Company::collection_schema()).await {
//!                 Ok(collection) => println!("Created collection: {:?}", collection),
//!                 Err(e) => eprintln!("Error creating collection: {}", e),
//!             }
//!         });
//!     }
//! }
//! ```

mod client;
mod traits;

pub mod builders;
pub mod error;
pub mod models;
pub mod prelude;

pub use builders::*;
pub use client::Client;
pub use error::*;
pub use models::*;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use typesense_derive::*;
