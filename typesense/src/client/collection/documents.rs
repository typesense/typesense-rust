//! Provides access to the document, search, and override-related API endpoints.
//!
//! An instance of `Documents` is scoped to a specific collection and is created
//! via the main `client.collection("collection_name").documents()` method.

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, documents_api},
    models::{
        self, DeleteDocumentsParameters, ExportDocumentsParameters, ImportDocumentsParameters,
        UpdateDocumentsParameters,
    },
};

/// Provides methods for interacting with documents within a specific Typesense collection.
///
/// This struct is created by calling `client.collection("collection_name").documents("collection_name")`.
pub struct Documents<'a> {
    pub(super) client: &'a Client,
    pub(super) collection_name: &'a str,
}

impl<'a> Documents<'a> {
    /// Creates a new `Documents` instance.
    ///
    /// This is typically called by `Client::documents()`.
    pub(super) fn new(client: &'a Client, collection_name: &'a str) -> Self {
        Self {
            client,
            collection_name,
        }
    }

    /// Indexes a document in the collection.
    ///

    ///
    /// # Arguments
    /// * `document` - A `serde_json::Value` representing the document.
    /// * `action` - The indexing action to perform (e.g., "create", "upsert").
    async fn index(
        &self,
        document: serde_json::Value,
        action: &str,
    ) -> Result<serde_json::Value, Error<documents_api::IndexDocumentError>> {
        let params = documents_api::IndexDocumentParams {
            collection_name: self.collection_name.to_string(),
            body: document,
            action: Some(action.to_string()),
            dirty_values: None, // Or expose this as an argument if needed
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::index_document(&config, params_for_move).await }
            })
            .await
    }

    /// Creates a new document in the collection.
    /// Fails if a document with the same id already exists
    ///
    /// If the document has an `id` field of type `string`, it will be used as the document's ID.
    /// Otherwise, Typesense will auto-generate an ID.
    ///
    /// # Arguments
    /// * `document` - A `serde_json::Value` representing the document to create.
    pub async fn create(
        &self,
        document: serde_json::Value,
    ) -> Result<serde_json::Value, Error<documents_api::IndexDocumentError>> {
        self.index(document, "create").await
    }

    /// Creates a new document or updates an existing document if a document with the same id already exists.
    /// Requires the whole document to be sent. For partial updates, use the `update()` action.
    ///
    /// # Arguments
    /// * `document` - A `serde_json::Value` representing the document to upsert.
    pub async fn upsert(
        &self,
        document: serde_json::Value,
    ) -> Result<serde_json::Value, Error<documents_api::IndexDocumentError>> {
        self.index(document, "upsert").await
    }

    // --- Bulk Operation Methods ---

    /// Imports a batch of documents in JSONL format.
    ///
    /// The documents to be imported must be formatted as a newline-delimited JSON string.
    ///
    /// # Arguments
    /// * `documents_jsonl` - A string containing the documents in JSONL format.
    /// * `params` - An `ImportDocumentsParameters` struct containing options like `action` and `batch_size`.
    pub async fn import(
        &self,
        documents_jsonl: String,
        params: ImportDocumentsParameters,
    ) -> Result<String, Error<documents_api::ImportDocumentsError>> {
        let params = documents_api::ImportDocumentsParams {
            body: documents_jsonl,
            collection_name: self.collection_name.to_string(),

            action: params.action,
            batch_size: params.batch_size,
            dirty_values: params.dirty_values,
            remote_embedding_batch_size: params.remote_embedding_batch_size,
            return_doc: params.return_doc,
            return_id: params.return_id,
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::import_documents(&config, params_for_move).await }
            })
            .await
    }

    /// Exports all documents in a collection in JSONL format.
    ///
    /// # Arguments
    /// * `params` - An `ExportDocumentsParameters` struct containing options like `filter_by` and `include_fields`.
    pub async fn export(
        &self,
        params: ExportDocumentsParameters,
    ) -> Result<String, Error<documents_api::ExportDocumentsError>> {
        let params = documents_api::ExportDocumentsParams {
            collection_name: self.collection_name.to_string(),
            exclude_fields: params.exclude_fields,
            filter_by: params.filter_by,
            include_fields: params.include_fields,
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::export_documents(&config, params_for_move).await }
            })
            .await
    }

    /// Deletes a batch of documents matching a specific filter condition.
    ///
    /// # Arguments
    /// * `params` - A `DeleteDocumentsParameters` describing the conditions for deleting documents.
    pub async fn delete(
        &self,
        params: DeleteDocumentsParameters,
    ) -> Result<models::DeleteDocuments200Response, Error<documents_api::DeleteDocumentsError>>
    {
        let params = documents_api::DeleteDocumentsParams {
            collection_name: self.collection_name.to_string(),
            filter_by: Some(params.filter_by),
            batch_size: params.batch_size,
            ignore_not_found: params.ignore_not_found,
            truncate: params.truncate,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::delete_documents(&config, params_for_move).await }
            })
            .await
    }

    /// Updates a batch of documents matching a specific filter condition.
    ///
    /// # Arguments
    /// * `document` - A `serde_json::Value` containing the fields to update.
    /// * `params` - A `UpdateDocumentsParameters` describing the conditions for updating documents.
    pub async fn update(
        &self,
        document: serde_json::Value,
        params: UpdateDocumentsParameters,
    ) -> Result<models::UpdateDocuments200Response, Error<documents_api::UpdateDocumentsError>>
    {
        let params = documents_api::UpdateDocumentsParams {
            collection_name: self.collection_name.to_string(),
            filter_by: params.filter_by,
            body: document,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::update_documents(&config, params_for_move).await }
            })
            .await
    }

    /// Searches for documents in the collection that match the given criteria.
    ///
    /// # Arguments
    /// * `params` - A `SearchParameters` struct containing all search parameters.
    ///              you can construct it like this:
    ///              `SearchParameters { q: Some("...".into()), query_by: Some("...".into()), ..Default::default() }`
    pub async fn search(
        &self,
        params: models::SearchParameters,
    ) -> Result<models::SearchResult, Error<documents_api::SearchCollectionError>> {
        let search_params = documents_api::SearchCollectionParams {
            collection_name: self.collection_name.to_string(),

            // Map all corresponding fields directly.
            cache_ttl: params.cache_ttl,
            conversation: params.conversation,
            conversation_id: params.conversation_id,
            conversation_model_id: params.conversation_model_id,
            drop_tokens_mode: params.drop_tokens_mode,
            drop_tokens_threshold: params.drop_tokens_threshold,
            enable_highlight_v1: params.enable_highlight_v1,
            enable_overrides: params.enable_overrides,
            enable_synonyms: params.enable_synonyms,
            enable_typos_for_alpha_numerical_tokens: params.enable_typos_for_alpha_numerical_tokens,
            enable_typos_for_numerical_tokens: params.enable_typos_for_numerical_tokens,
            exclude_fields: params.exclude_fields,
            exhaustive_search: params.exhaustive_search,
            facet_by: params.facet_by,
            facet_query: params.facet_query,
            facet_return_parent: params.facet_return_parent,
            facet_strategy: params.facet_strategy,
            filter_by: params.filter_by,
            filter_curated_hits: params.filter_curated_hits,
            group_by: params.group_by,
            group_limit: params.group_limit,
            group_missing_values: params.group_missing_values,
            hidden_hits: params.hidden_hits,
            highlight_affix_num_tokens: params.highlight_affix_num_tokens,
            highlight_end_tag: params.highlight_end_tag,
            highlight_fields: params.highlight_fields,
            highlight_full_fields: params.highlight_full_fields,
            highlight_start_tag: params.highlight_start_tag,
            include_fields: params.include_fields,
            infix: params.infix,
            limit: params.limit,
            max_candidates: params.max_candidates,
            max_extra_prefix: params.max_extra_prefix,
            max_extra_suffix: params.max_extra_suffix,
            max_facet_values: params.max_facet_values,
            max_filter_by_candidates: params.max_filter_by_candidates,
            min_len_1typo: params.min_len_1typo,
            min_len_2typo: params.min_len_2typo,
            num_typos: params.num_typos,
            offset: params.offset,
            override_tags: params.override_tags,
            page: params.page,
            per_page: params.per_page,
            pinned_hits: params.pinned_hits,
            pre_segmented_query: params.pre_segmented_query,
            prefix: params.prefix,
            preset: params.preset,
            prioritize_exact_match: params.prioritize_exact_match,
            prioritize_num_matching_fields: params.prioritize_num_matching_fields,
            prioritize_token_position: params.prioritize_token_position,
            q: params.q,
            query_by: params.query_by,
            query_by_weights: params.query_by_weights,
            remote_embedding_num_tries: params.remote_embedding_num_tries,
            remote_embedding_timeout_ms: params.remote_embedding_timeout_ms,
            search_cutoff_ms: params.search_cutoff_ms,
            snippet_threshold: params.snippet_threshold,
            sort_by: params.sort_by,
            split_join_tokens: params.split_join_tokens,
            stopwords: params.stopwords,
            synonym_num_typos: params.synonym_num_typos,
            synonym_prefix: params.synonym_prefix,
            text_match_type: params.text_match_type,
            typo_tokens_threshold: params.typo_tokens_threshold,
            use_cache: params.use_cache,
            vector_query: params.vector_query,
            voice_query: params.voice_query,
            nl_model_id: params.nl_model_id,
            nl_query: params.nl_query,
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = search_params.clone();
                async move { documents_api::search_collection(&config, params_for_move).await }
            })
            .await
    }
}
