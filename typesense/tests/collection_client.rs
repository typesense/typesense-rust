#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use typesense::document::Document as DocumentTrait;
use typesense::Document;
use typesense_codegen::apis::collections_api;
use typesense_codegen::apis::configuration::{ApiKey, Configuration};
use typesense_codegen::models::CollectionResponse;
use typesense_codegen::models::CollectionSchema;

#[allow(dead_code)]
#[derive(Document, Serialize, Deserialize)]
#[typesense(collection_name = "companies", default_sorting_field = "num_employees")]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    country: String,
}

fn schema_to_resp(schema: CollectionSchema, resp: &CollectionResponse) -> CollectionResponse {
    CollectionResponse {
        name: schema.name,
        fields: schema.fields,
        default_sorting_field: schema.default_sorting_field,
        token_separators: schema.token_separators,
        enable_nested_fields: schema.enable_nested_fields,
        symbols_to_index: schema.symbols_to_index,
        num_documents: resp.num_documents,
        created_at: resp.created_at,
    }
}

async fn create_collection(host: impl Into<String>, api_key: impl Into<String>) {
    let config = Configuration {
        base_path: host.into(),
        api_key: Some(ApiKey {
            prefix: None,
            key: api_key.into(),
        }),
        ..Default::default()
    };

    let collection_schema_response =
        collections_api::create_collection(&config, Company::collection_schema())
            .await
            .unwrap();

    assert_eq!(collection_schema_response.num_documents, 0);
    assert_eq!(
        schema_to_resp(Company::collection_schema(), &collection_schema_response),
        collection_schema_response
    );
}

async fn get_collection(host: impl Into<String>, api_key: impl Into<String>) {
    let config = Configuration {
        base_path: host.into(),
        api_key: Some(ApiKey {
            prefix: None,
            key: api_key.into(),
        }),
        ..Default::default()
    };

    let collection_schema_response = collections_api::get_collection(&config, "companies")
        .await
        .unwrap();

    assert_eq!(collection_schema_response.num_documents, 1250);
    assert_eq!(
        schema_to_resp(Company::collection_schema(), &collection_schema_response),
        collection_schema_response
    );
}

async fn delete_collection(host: impl Into<String>, api_key: impl Into<String>) {
    let config = Configuration {
        base_path: host.into(),
        api_key: Some(ApiKey {
            prefix: None,
            key: api_key.into(),
        }),
        ..Default::default()
    };

    let collection_schema_response = collections_api::delete_collection(&config, "companies")
        .await
        .unwrap();

    assert_eq!(collection_schema_response.num_documents, 1200);
    assert_eq!(
        schema_to_resp(Company::collection_schema(), &collection_schema_response),
        collection_schema_response
    );
}

async fn get_collections(host: impl Into<String>, api_key: impl Into<String>) {
    let config = Configuration {
        base_path: host.into(),
        api_key: Some(ApiKey {
            prefix: None,
            key: api_key.into(),
        }),
        ..Default::default()
    };

    let collection_schema_response = collections_api::get_collections(&config).await.unwrap();

    assert_eq!(collection_schema_response.len(), 2);
}

#[cfg(all(feature = "tokio-rt", not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn create_collection_tokio() {
        let _ = dotenvy::dotenv();

        let host = std::env::var("URL").expect("URL must be present in .env");
        let api_key = std::env::var("API_KEY").expect("API_KEY must be present in .env");

        create_collection(host, api_key).await
    }

    #[tokio::test]
    async fn get_collection_tokio() {
        let _ = dotenvy::dotenv();

        let host = std::env::var("URL").expect("URL must be present in .env");
        let api_key = std::env::var("API_KEY").expect("API_KEY must be present in .env");

        get_collection(host, api_key).await
    }

    #[tokio::test]
    async fn delete_collection_tokio() {
        let _ = dotenvy::dotenv();

        let host = std::env::var("URL").expect("URL must be present in .env");
        let api_key = std::env::var("API_KEY").expect("API_KEY must be present in .env");

        delete_collection(host, api_key).await
    }

    #[tokio::test]
    async fn get_collections_tokio() {
        let _ = dotenvy::dotenv();

        let host = std::env::var("URL").expect("URL must be present in .env");
        let api_key = std::env::var("API_KEY").expect("API_KEY must be present in .env");

        get_collections(host, api_key).await
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    const HOST: &str = "http://localhost:5000";
    const API_KEY: &str = "VerySecretKey";

    #[wasm_bindgen_test]
    async fn create_collection_wasm() {
        console_error_panic_hook::set_once();

        create_collection(HOST, API_KEY).await
    }

    #[wasm_bindgen_test]
    async fn get_collection_wasm() {
        console_error_panic_hook::set_once();

        get_collection(HOST, API_KEY).await
    }

    #[wasm_bindgen_test]
    async fn delete_collection_wasm() {
        console_error_panic_hook::set_once();

        delete_collection(HOST, API_KEY).await
    }

    #[wasm_bindgen_test]
    async fn get_collections_wasm() {
        console_error_panic_hook::set_once();

        get_collections(HOST, API_KEY).await
    }
}
