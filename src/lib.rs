#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Typesense
//!
//! Welcome to typesense, the rust library for the Typesense API.

mod client;
mod error;
pub mod transport;

pub use client::{keys, Client, ClientBuilder};
pub use error::{Result, TypesenseError};
