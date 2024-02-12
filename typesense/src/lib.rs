#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Typesense
//!
//! Welcome to typesense, the rust library for the Typesense API.

#[cfg(feature = "openapi_client")]
pub use typesense_codegen as openapi_client;

mod client;
pub mod collection;
pub mod document;
mod error;
pub mod field;
pub mod transport;
pub mod synonym;

pub use client::{keys, Client, ClientBuilder};
pub use error::{Result, TypesenseError};

#[cfg(feature = "typesense_derive")]
#[doc(hidden)]
pub use typesense_derive::*;
