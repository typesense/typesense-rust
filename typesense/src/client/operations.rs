//! Provides access to top-level, non-namespaced API endpoints.
//!
//! An `Operations` instance is created via the main `client.operations()` method.

use crate::{Client, Error, execute_wrapper};
use typesense_codegen::{
    apis::{debug_api, health_api, operations_api},
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
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Retrieves debugging information from a Typesense node.
    ///
    /// This method will try nodes in sequence according to the health policy
    /// until it gets a successful response. The returned information pertains
    /// to the specific node that responded successfully.
    ///
    /// Docs: <https://typesense.org/docs/latest/api/cluster-operations.html#debug>
    pub async fn debug(&self) -> Result<models::Debug200Response, Error<debug_api::DebugError>> {
        execute_wrapper!(self, debug_api::debug)
    }

    /// Get health information about a Typesense node.
    /// When a node is running out of memory / disk, the API response will have an additional resource_error field that's set to either `OUT_OF_DISK`` or `OUT_OF_MEMORY``.
    ///
    /// Docs: <https://typesense.org/docs/latest/api/cluster-operations.html#health>
    pub async fn health(&self) -> Result<models::HealthStatus, Error<health_api::HealthError>> {
        execute_wrapper!(self, health_api::health)
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
    ///
    /// Docs: <https://typesense.org/docs/latest/api/cluster-operations.html#cluster-metrics>
    pub async fn retrieve_metrics(
        &self,
    ) -> Result<serde_json::Value, Error<operations_api::RetrieveMetricsError>> {
        execute_wrapper!(self, operations_api::retrieve_metrics)
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
    ///
    /// Docs: <https://typesense.org/docs/latest/api/cluster-operations.html#api-stats>
    pub async fn retrieve_api_stats(
        &self,
    ) -> Result<models::ApiStatsResponse, Error<operations_api::RetrieveApiStatsError>> {
        execute_wrapper!(self, operations_api::retrieve_api_stats)
    }

    /// Creates a point-in-time snapshot of a Typesense node's state and data in the specified directory.
    /// You can then backup the snapshot directory that gets created and later restore it as a data directory, as needed.
    ///
    /// Docs: <https://typesense.org/docs/latest/api/cluster-operations.html#create-snapshot-for-backups>
    pub async fn take_snapshot(
        &self,
        params: operations_api::TakeSnapshotParams,
    ) -> Result<models::SuccessStatus, Error<operations_api::TakeSnapshotError>> {
        execute_wrapper!(self, operations_api::take_snapshot, params)
    }

    /// Triggers a follower node to initiate the raft voting process, which triggers leader re-election.
    /// The follower node that you run this operation against will become the new leader, once this command succeeds.
    ///
    /// Docs: <https://typesense.org/docs/latest/api/cluster-operations.html#re-elect-leader>
    pub async fn vote(&self) -> Result<models::SuccessStatus, Error<operations_api::VoteError>> {
        execute_wrapper!(self, operations_api::vote)
    }

    /// You can check the status of in-progress schema change operations by using the schema changes endpoint.
    ///
    /// Docs: <https://typesense.org/docs/latest/api/collections.html#get-schema-change-status>
    pub async fn get_schema_changes(
        &self,
    ) -> Result<Vec<models::SchemaChangeStatus>, Error<operations_api::GetSchemaChangesError>> {
        execute_wrapper!(self, operations_api::get_schema_changes)
    }

    /// Typesense uses RocksDB to store your documents on the disk. If you do frequent writes or updates, you could benefit from running a compaction of the underlying RocksDB database. This could reduce the size of the database and decrease read latency.
    /// While the database will not block during this operation, we recommend running it during off-peak hours.
    ///
    /// Docs: <https://typesense.org/docs/latest/api/cluster-operations.html#compacting-the-on-disk-database>
    pub async fn compact_db(
        &self,
    ) -> Result<models::SuccessStatus, Error<operations_api::CompactDbError>> {
        execute_wrapper!(self, operations_api::compact_db)
    }

    /// Responses of search requests that are sent with `use_cache` parameter are cached in a LRU cache. This operation will clear the cache completely.
    ///
    /// Docs: <https://typesense.org/docs/latest/api/cluster-operations.html#clear-cache>
    pub async fn clear_cache(
        &self,
    ) -> Result<models::SuccessStatus, Error<operations_api::ClearCacheError>> {
        execute_wrapper!(self, operations_api::clear_cache)
    }

    /// Enable logging of requests that take over a defined threshold of time.
    /// Default: `-1` which disables slow request logging.
    ///
    /// Docs: <https://typesense.org/docs/latest/api/cluster-operations.html#toggle-slow-request-log>
    pub async fn toggle_slow_request_log(
        &self,
        slow_requests_threshold_ms: i32,
    ) -> Result<models::SuccessStatus, Error<operations_api::ToggleSlowRequestLogError>> {
        let params = operations_api::ToggleSlowRequestLogParams {
            toggle_slow_request_log_request: Some(models::ToggleSlowRequestLogRequest {
                log_slow_requests_time_ms: slow_requests_threshold_ms,
            }),
        };
        execute_wrapper!(self, operations_api::toggle_slow_request_log, params)
    }
}
