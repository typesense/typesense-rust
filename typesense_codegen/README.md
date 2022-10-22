# Rust API client for openapi

An open source search engine for building delightful search experiences.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.23.0
- Package version: 0.23.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CollectionsApi* | [**create_collection**](docs/CollectionsApi.md#create_collection) | **POST** /collections | Create a new collection
*CollectionsApi* | [**delete_alias**](docs/CollectionsApi.md#delete_alias) | **DELETE** /aliases/{aliasName} | Delete an alias
*CollectionsApi* | [**delete_collection**](docs/CollectionsApi.md#delete_collection) | **DELETE** /collections/{collectionName} | Delete a collection
*CollectionsApi* | [**get_alias**](docs/CollectionsApi.md#get_alias) | **GET** /aliases/{aliasName} | Retrieve an alias
*CollectionsApi* | [**get_aliases**](docs/CollectionsApi.md#get_aliases) | **GET** /aliases | List all aliases
*CollectionsApi* | [**get_collection**](docs/CollectionsApi.md#get_collection) | **GET** /collections/{collectionName} | Retrieve a single collection
*CollectionsApi* | [**get_collections**](docs/CollectionsApi.md#get_collections) | **GET** /collections | List all collections
*CollectionsApi* | [**update_collection**](docs/CollectionsApi.md#update_collection) | **PATCH** /collections/{collectionName} | Update a collection
*CollectionsApi* | [**upsert_alias**](docs/CollectionsApi.md#upsert_alias) | **PUT** /aliases/{aliasName} | Create or update a collection alias
*DebugApi* | [**debug**](docs/DebugApi.md#debug) | **GET** /debug | Print debugging information
*DocumentsApi* | [**delete_document**](docs/DocumentsApi.md#delete_document) | **DELETE** /collections/{collectionName}/documents/{documentId} | Delete a document
*DocumentsApi* | [**delete_documents**](docs/DocumentsApi.md#delete_documents) | **DELETE** /collections/{collectionName}/documents | Delete a bunch of documents
*DocumentsApi* | [**delete_search_override**](docs/DocumentsApi.md#delete_search_override) | **DELETE** /collections/{collectionName}/overrides/{overrideId} | Delete an override associated with a collection
*DocumentsApi* | [**delete_search_synonym**](docs/DocumentsApi.md#delete_search_synonym) | **DELETE** /collections/{collectionName}/synonyms/{synonymId} | Delete a synonym associated with a collection
*DocumentsApi* | [**export_documents**](docs/DocumentsApi.md#export_documents) | **GET** /collections/{collectionName}/documents/export | Export all documents in a collection
*DocumentsApi* | [**get_document**](docs/DocumentsApi.md#get_document) | **GET** /collections/{collectionName}/documents/{documentId} | Retreive a document
*DocumentsApi* | [**get_search_override**](docs/DocumentsApi.md#get_search_override) | **GET** /collections/{collectionName}/overrides/{overrideId} | Retrieve a single search override
*DocumentsApi* | [**get_search_overrides**](docs/DocumentsApi.md#get_search_overrides) | **GET** /collections/{collectionName}/overrides | List all collection overrides
*DocumentsApi* | [**get_search_synonym**](docs/DocumentsApi.md#get_search_synonym) | **GET** /collections/{collectionName}/synonyms/{synonymId} | Retrieve a single search synonym
*DocumentsApi* | [**get_search_synonyms**](docs/DocumentsApi.md#get_search_synonyms) | **GET** /collections/{collectionName}/synonyms | List all collection synonyms
*DocumentsApi* | [**import_documents**](docs/DocumentsApi.md#import_documents) | **POST** /collections/{collectionName}/documents/import | Import documents into a collection
*DocumentsApi* | [**index_document**](docs/DocumentsApi.md#index_document) | **POST** /collections/{collectionName}/documents | Index a document
*DocumentsApi* | [**multi_search**](docs/DocumentsApi.md#multi_search) | **POST** /multi_search | send multiple search requests in a single HTTP request
*DocumentsApi* | [**search_collection**](docs/DocumentsApi.md#search_collection) | **GET** /collections/{collectionName}/documents/search | Search for documents in a collection
*DocumentsApi* | [**update_document**](docs/DocumentsApi.md#update_document) | **PATCH** /collections/{collectionName}/documents/{documentId} | Update a document
*DocumentsApi* | [**upsert_search_override**](docs/DocumentsApi.md#upsert_search_override) | **PUT** /collections/{collectionName}/overrides/{overrideId} | Create or update an override to promote certain documents over others
*DocumentsApi* | [**upsert_search_synonym**](docs/DocumentsApi.md#upsert_search_synonym) | **PUT** /collections/{collectionName}/synonyms/{synonymId} | Create or update a synonym
*HealthApi* | [**health**](docs/HealthApi.md#health) | **GET** /health | Checks if Typesense server is ready to accept requests.
*KeysApi* | [**create_key**](docs/KeysApi.md#create_key) | **POST** /keys | Create an API Key
*KeysApi* | [**delete_key**](docs/KeysApi.md#delete_key) | **DELETE** /keys/{keyId} | Delete an API key given its ID.
*KeysApi* | [**get_key**](docs/KeysApi.md#get_key) | **GET** /keys/{keyId} | Retrieve (metadata about) a key
*KeysApi* | [**get_keys**](docs/KeysApi.md#get_keys) | **GET** /keys | Retrieve (metadata about) all keys.
*OperationsApi* | [**take_snapshot**](docs/OperationsApi.md#take_snapshot) | **POST** /operations/snapshot | Creates a point-in-time snapshot of a Typesense node's state and data in the specified directory.
*OperationsApi* | [**vote**](docs/OperationsApi.md#vote) | **POST** /operations/vote | Triggers a follower node to initiate the raft voting process, which triggers leader re-election.
*OverrideApi* | [**get_search_override**](docs/OverrideApi.md#get_search_override) | **GET** /collections/{collectionName}/overrides/{overrideId} | Retrieve a single search override
*PromoteApi* | [**delete_search_override**](docs/PromoteApi.md#delete_search_override) | **DELETE** /collections/{collectionName}/overrides/{overrideId} | Delete an override associated with a collection
*PromoteApi* | [**get_search_overrides**](docs/PromoteApi.md#get_search_overrides) | **GET** /collections/{collectionName}/overrides | List all collection overrides
*PromoteApi* | [**upsert_search_override**](docs/PromoteApi.md#upsert_search_override) | **PUT** /collections/{collectionName}/overrides/{overrideId} | Create or update an override to promote certain documents over others


## Documentation For Models

 - [ApiKey](docs/ApiKey.md)
 - [ApiKeyAllOf](docs/ApiKeyAllOf.md)
 - [ApiKeySchema](docs/ApiKeySchema.md)
 - [ApiKeysResponse](docs/ApiKeysResponse.md)
 - [ApiResponse](docs/ApiResponse.md)
 - [CollectionAlias](docs/CollectionAlias.md)
 - [CollectionAliasSchema](docs/CollectionAliasSchema.md)
 - [CollectionAliasesResponse](docs/CollectionAliasesResponse.md)
 - [CollectionResponse](docs/CollectionResponse.md)
 - [CollectionResponseAllOf](docs/CollectionResponseAllOf.md)
 - [CollectionSchema](docs/CollectionSchema.md)
 - [CollectionUpdateSchema](docs/CollectionUpdateSchema.md)
 - [Debug200Response](docs/Debug200Response.md)
 - [DeleteDocuments200Response](docs/DeleteDocuments200Response.md)
 - [DeleteDocumentsDeleteDocumentsParametersParameter](docs/DeleteDocumentsDeleteDocumentsParametersParameter.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [ExportDocumentsExportDocumentsParametersParameter](docs/ExportDocumentsExportDocumentsParametersParameter.md)
 - [FacetCounts](docs/FacetCounts.md)
 - [FacetCountsCountsInner](docs/FacetCountsCountsInner.md)
 - [FacetCountsStats](docs/FacetCountsStats.md)
 - [Field](docs/Field.md)
 - [HealthStatus](docs/HealthStatus.md)
 - [ImportDocumentsImportDocumentsParametersParameter](docs/ImportDocumentsImportDocumentsParametersParameter.md)
 - [MultiSearchCollectionParameters](docs/MultiSearchCollectionParameters.md)
 - [MultiSearchCollectionParametersAllOf](docs/MultiSearchCollectionParametersAllOf.md)
 - [MultiSearchParameters](docs/MultiSearchParameters.md)
 - [MultiSearchResult](docs/MultiSearchResult.md)
 - [MultiSearchSearchesParameter](docs/MultiSearchSearchesParameter.md)
 - [ScopedKeyParameters](docs/ScopedKeyParameters.md)
 - [SearchGroupedHit](docs/SearchGroupedHit.md)
 - [SearchHighlight](docs/SearchHighlight.md)
 - [SearchOverride](docs/SearchOverride.md)
 - [SearchOverrideAllOf](docs/SearchOverrideAllOf.md)
 - [SearchOverrideExclude](docs/SearchOverrideExclude.md)
 - [SearchOverrideInclude](docs/SearchOverrideInclude.md)
 - [SearchOverrideRule](docs/SearchOverrideRule.md)
 - [SearchOverrideSchema](docs/SearchOverrideSchema.md)
 - [SearchOverridesResponse](docs/SearchOverridesResponse.md)
 - [SearchParameters](docs/SearchParameters.md)
 - [SearchResult](docs/SearchResult.md)
 - [SearchResultHit](docs/SearchResultHit.md)
 - [SearchResultRequestParams](docs/SearchResultRequestParams.md)
 - [SearchSynonym](docs/SearchSynonym.md)
 - [SearchSynonymSchema](docs/SearchSynonymSchema.md)
 - [SearchSynonymsResponse](docs/SearchSynonymsResponse.md)
 - [SnapshotParameters](docs/SnapshotParameters.md)
 - [SuccessStatus](docs/SuccessStatus.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


