#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Typesense
//!
//! Welcome to typesense, the rust library for the Typesense API.

mod error;
pub mod transport;

pub use error::{Result, TypesenseError};
