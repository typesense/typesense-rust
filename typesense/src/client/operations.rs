//! Provides access to top-level, non-namespaced API endpoints.
//!
//! An `Operations` instance is created via the main `Client::operations()` method.

use crate::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, debug_api, health_api, operations_api},
    models,
};

/// Provides methods for top-level, non-namespaced Typesense operations.
///
/// This struct is created by calling `client.operations()`.
pub struct Operations<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Operations<'a> {
    /// Creates a new `Operations` instance
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Retrieves debugging information from a Typesense node.
    ///
    /// This method will try nodes in sequence according to the health policy
    /// until it gets a successful response. The returned information pertains
    /// to the specific node that responded successfully.
    pub async fn debug(&self) -> Result<models::Debug200Response, Error<debug_api::DebugError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                debug_api::debug(&config).await
            })
            .await
    }

    /// Get health information about a Typesense node.
    /// When a node is running out of memory / disk, the API response will have an additional resource_error field that's set to either `OUT_OF_DISK`` or `OUT_OF_MEMORY``.
    pub async fn health(&self) -> Result<models::HealthStatus, Error<health_api::HealthError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                health_api::health(&config).await
            })
            .await
    }

    /// Get current RAM, CPU, Disk & Network usage metrics.
    /// ### Example JSON response:
    /// ```json
    /// {
    ///  "system_cpu1_active_percentage": "0.00",
    ///  "system_cpu2_active_percentage": "0.00",
    ///  "system_cpu3_active_percentage": "0.00",
    ///  "system_cpu4_active_percentage": "0.00",
    ///  "system_cpu_active_percentage": "0.00",
    ///  "system_disk_total_bytes": "1043447808",
    ///  "system_disk_used_bytes": "561152",
    ///  "system_memory_total_bytes": "2086899712",
    ///  "system_memory_used_bytes": "1004507136",
    ///  "system_memory_total_swap_bytes": "1004507136",
    ///  "system_memory_used_swap_bytes": "0.00",
    ///  "system_network_received_bytes": "1466",
    ///  "system_network_sent_bytes": "182",
    ///  "typesense_memory_active_bytes": "29630464",
    ///  "typesense_memory_allocated_bytes": "27886840",
    ///  "typesense_memory_fragmentation_ratio": "0.06",
    ///  "typesense_memory_mapped_bytes": "69701632",
    ///  "typesense_memory_metadata_bytes": "4588768",
    ///  "typesense_memory_resident_bytes": "29630464",
    ///  "typesense_memory_retained_bytes": "25718784"
    /// }
    /// ```
    pub async fn retrieve_metrics(
        &self,
    ) -> Result<serde_json::Value, Error<operations_api::RetrieveMetricsError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                operations_api::retrieve_metrics(&config).await
            })
            .await
    }

    /// Get stats about API endpoints.
    /// This endpoint returns average requests per second and latencies for all requests in the last 10 seconds.
    /// Example JSON response:
    /// ```json
    /// {
    ///  "latency_ms": {
    ///    "GET /collections/products": 0.0,
    ///    "POST /collections": 4.0,
    ///    "POST /collections/products/documents/import": 1166.0
    ///  },
    ///  "requests_per_second": {
    ///    "GET /collections/products": 0.1,
    ///    "POST /collections": 0.1,
    ///    "POST /collections/products/documents/import": 0.1
    ///  }
    /// }
    /// ```
    pub async fn retrieve_api_stats(
        &self,
    ) -> Result<models::ApiStatsResponse, Error<operations_api::RetrieveApiStatsError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                operations_api::retrieve_api_stats(&config).await
            })
            .await
    }

    /// Creates a point-in-time snapshot of a Typesense node's state and data in the specified directory.
    /// You can then backup the snapshot directory that gets created and later restore it as a data directory, as needed.
    pub async fn take_snapshot(
        &self,
        params: operations_api::TakeSnapshotParams,
    ) -> Result<models::SuccessStatus, Error<operations_api::TakeSnapshotError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { operations_api::take_snapshot(&config, params_for_move).await }
            })
            .await
    }

    /// Triggers a follower node to initiate the raft voting process, which triggers leader re-election.
    /// The follower node that you run this operation against will become the new leader, once this command succeeds.
    pub async fn vote(&self) -> Result<models::SuccessStatus, Error<operations_api::VoteError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                operations_api::vote(&config).await
            })
            .await
    }

    /// You can check the status of in-progress schema change operations by using the schema changes endpoint.
    pub async fn get_schema_changes(
        &self,
    ) -> Result<Vec<models::SchemaChangeStatus>, Error<operations_api::GetSchemaChangesError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                operations_api::get_schema_changes(&config).await
            })
            .await
    }
}
