//! Provides access to the document, search, and override-related API endpoints.
//!
//! An instance of `Documents` is scoped to a specific collection and is created
//! via the main `client.collection("collection_name").documents()` method.

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, documents_api},
    models,
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
    /// If the document has an 'id' field, it will be used as the document's ID.
    /// Otherwise, Typesense will auto-generate an ID.
    ///
    /// # Arguments
    /// * `document` - A `serde_json::Value` representing the document.
    /// * `action` - The indexing action to perform (e.g., "create", "upsert", "update").
    pub async fn index(
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

    /// Fetches an individual document from the collection by its ID.
    ///
    /// # Arguments
    /// * `document_id` - The ID of the document to retrieve.
    pub async fn retrieve(
        &self,
        document_id: &str,
    ) -> Result<serde_json::Value, Error<documents_api::GetDocumentError>> {
        let params = documents_api::GetDocumentParams {
            collection_name: self.collection_name.to_string(),
            document_id: document_id.to_string(),
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::get_document(&config, params_for_move).await }
            })
            .await
    }

    /// Updates an individual document from the collection by its ID. The update can be partial.
    ///
    /// # Arguments
    /// * `document_id` - The ID of the document to update.
    /// * `document` - A `serde_json::Value` containing the fields to update.
    pub async fn update(
        &self,
        document_id: &str,
        document: serde_json::Value,
    ) -> Result<serde_json::Value, Error<documents_api::UpdateDocumentError>> {
        let params = documents_api::UpdateDocumentParams {
            collection_name: self.collection_name.to_string(),
            document_id: document_id.to_string(),
            body: document,
            dirty_values: None,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::update_document(&config, params_for_move).await }
            })
            .await
    }

    /// Deletes an individual document from the collection by its ID.
    ///
    /// # Arguments
    /// * `document_id` - The ID of the document to delete.
    pub async fn delete(
        &self,
        document_id: &str,
    ) -> Result<serde_json::Value, Error<documents_api::DeleteDocumentError>> {
        let params = documents_api::DeleteDocumentParams {
            collection_name: self.collection_name.to_string(),
            document_id: document_id.to_string(),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::delete_document(&config, params_for_move).await }
            })
            .await
    }

    // --- Bulk Operation Methods ---

    /// Imports a batch of documents in JSONL format.
    ///
    /// The documents to be imported must be formatted as a newline-delimited JSON string.
    ///
    /// # Arguments
    /// * `documents_jsonl` - A string containing the documents in JSONL format.
    /// * `params` - An `ImportDocumentsParams` struct containing options like `action` and `batch_size`.
    ///              The `collection_name` field will be overwritten.
    pub async fn import(
        &self,
        documents_jsonl: String,
        mut params: documents_api::ImportDocumentsParams,
    ) -> Result<String, Error<documents_api::ImportDocumentsError>> {
        params.collection_name = self.collection_name.to_string();
        params.body = documents_jsonl;

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
    /// * `params` - An `ExportDocumentsParams` struct containing options like `filter_by` and `include_fields`.
    ///              The `collection_name` field will be overwritten.
    pub async fn export(
        &self,
        mut params: documents_api::ExportDocumentsParams,
    ) -> Result<String, Error<documents_api::ExportDocumentsError>> {
        params.collection_name = self.collection_name.to_string();

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
    /// * `filter_by` - The filter condition for deleting documents.
    /// * `batch_size` - The number of documents to delete at a time.
    pub async fn delete_by_filter(
        &self,
        filter_by: &str,
        batch_size: Option<i32>,
    ) -> Result<models::DeleteDocuments200Response, Error<documents_api::DeleteDocumentsError>>
    {
        let params = documents_api::DeleteDocumentsParams {
            collection_name: self.collection_name.to_string(),
            filter_by: Some(filter_by.to_string()),
            batch_size,
            ignore_not_found: None,
            truncate: None,
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
    /// * `filter_by` - The filter condition for updating documents.
    /// * `document` - A `serde_json::Value` containing the fields to update.
    pub async fn update_by_filter(
        &self,
        filter_by: &str,
        document: serde_json::Value,
    ) -> Result<models::UpdateDocuments200Response, Error<documents_api::UpdateDocumentsError>>
    {
        let params = documents_api::UpdateDocumentsParams {
            collection_name: self.collection_name.to_string(),
            filter_by: Some(filter_by.to_string()),
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
