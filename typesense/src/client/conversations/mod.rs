//! Provides access to the API endpoints for managing conversation models.
//!
//! An `Conversations` instance is created via the main `Client::conversations()` method.

use super::Client;
use model::Model;
use models::Models;

mod model;
mod models;

/// Provides methods for managing Typesense conversation models.
///
/// This struct is created by calling `client.conversations()`.
pub struct Conversations<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Conversations<'a> {
    /// Creates a new `Conversations` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Provides access to endpoints for managing the collection of conversation models.
    ///
    /// Example: `client.conversations().models().list().await`
    pub fn models(&self) -> Models<'a> {
        Models::new(self.client)
    }

    /// Provides access to endpoints for managing a single conversation model.
    ///
    /// # Arguments
    /// * `model_id` - The ID of the conversation model to manage.
    ///
    /// Example: `client.conversations().model("...").get().await`
    pub fn model(&self, model_id: &'a str) -> Model<'a> {
        Model::new(self.client, model_id)
    }
}
