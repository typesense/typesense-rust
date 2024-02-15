#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Typesense
//!
//! Welcome to typesense, the rust library for the Typesense API.

pub use typesense_codegen as openapi;

pub mod collection_schema;
pub mod document;
pub mod field;
pub mod keys;

#[cfg(feature = "typesense_derive")]
#[doc(hidden)]
pub use typesense_derive::*;
