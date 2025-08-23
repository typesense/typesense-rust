//! Provides access to the API endpoints for managing stemming.
//!
//! An instance of `Stemming` is created via the `Client::stemming()` method.

pub mod dictionaries;
pub mod dictionary;

use super::Client;
use dictionaries::Dictionaries;
use dictionary::Dictionary;

/// Provides methods for managing Typesense stemming.
///
/// This struct is created by calling `client.stemming()`.
pub struct Stemming<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Stemming<'a> {
    /// Creates a new `Stemming` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Provides access to endpoints for managing the collection of dictionaries.
    pub fn dictionaries(&self) -> Dictionaries<'a> {
        Dictionaries::new(self.client)
    }

    /// Provides access to endpoints for managing a single dictionary.
    ///
    /// # Arguments
    /// * `dictionary_id` - The ID of the dictionary to manage.
    pub fn dictionary(&self, dictionary_id: &'a str) -> Dictionary<'a> {
        Dictionary::new(self.client, dictionary_id)
    }
}
