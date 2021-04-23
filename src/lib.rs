#![deny(missing_docs)]
//! # Typesense
//!
//! Welcome to typesense, the rust library for the Typesense API.

pub mod client;
mod error;
mod request;

pub use error::{Result, TypesenseError};
