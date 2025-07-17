//! Provides access to the analytics API endpoints for managing rules and posting events.
//!
//! An `Analytics` instance is created via the main `Client::analytics()` method.
pub mod events;
pub mod rule;
pub mod rules;
use super::{Client, Error};
pub use events::Events;
pub use rule::Rule;
pub use rules::Rules;

/// Provides methods for interacting with Typesense analytics rules and events.
///
/// This struct is created by calling `client.analytics()`.
pub struct Analytics<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Analytics<'a> {
    /// Creates a new `Analytics` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Provides access to endpoints for managing a collection of analytics rules.
    pub fn rules(&self) -> Rules<'a> {
        Rules::new(self.client)
    }

    /// Provides access to endpoints for managing a single analytics rule.
    ///
    /// # Arguments
    /// * `rule_name` - The name of the analytics rule to manage.
    pub fn rule(&self, rule_name: &'a str) -> Rule<'a> {
        Rule::new(self.client, rule_name)
    }

    /// Provides access to the endpoint for creating analytics events.
    ///
    /// Example: `client.analytics().events().create(...).await`
    pub fn events(&self) -> Events<'a> {
        Events::new(self.client)
    }
}
