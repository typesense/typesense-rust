//! Provides access to the analytics API endpoints for managing rules and posting events.
//!
//! An `Analytics` instance is created via the main `client.analytics()` method.
mod events;
mod rule;
mod rules;
use crate::Client;
use events::Events;
use rule::Rule;
use rules::Rules;

/// Provides methods for interacting with Typesense analytics rules and events.
///
/// This struct is created by calling `client.analytics()`.
pub struct Analytics<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Analytics<'a> {
    /// Creates a new `Analytics` instance.
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Provides access to endpoints for managing a collection of analytics rules.
    #[inline]
    pub fn rules(&self) -> Rules<'a> {
        Rules::new(self.client)
    }

    /// Provides access to endpoints for managing a single analytics rule.
    ///
    /// # Arguments
    /// * `rule_name` - The name of the analytics rule to manage.
    #[inline]
    pub fn rule(&self, rule_name: &'a str) -> Rule<'a> {
        Rule::new(self.client, rule_name)
    }

    /// Provides access to the endpoint for creating analytics events.
    #[inline]
    pub fn events(&self) -> Events<'a> {
        Events::new(self.client)
    }
}
