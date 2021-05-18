#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Typesense
//!
//! Welcome to typesense, the rust library for the Typesense API.

pub mod collection;
pub mod document;
mod error;
pub mod field;
pub mod transport;

pub use error::{Result, TypesenseError};

#[cfg(feature = "typesense_derive")]
#[doc(hidden)]
pub use typesense_derive::*;
