#![allow(dead_code)]

use super::Config;
use serde::{Deserialize, Serialize};
use typesense::document::Document;
use typesense::Typesense;
use typesense_codegen::apis::collections_api;
use typesense_codegen::models::{CollectionResponse, CollectionSchema};

#[derive(Typesense, Serialize, Deserialize)]
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

async fn create_collection() {
    let collection_schema_response =
        collections_api::create_collection(Config::get(), Company::collection_schema())
            .await
            .unwrap();

    assert_eq!(collection_schema_response.num_documents, 0);
    assert_eq!(
        schema_to_resp(Company::collection_schema(), &collection_schema_response),
        collection_schema_response
    );
}

async fn get_collection() {
    let collection_schema_response = collections_api::get_collection(Config::get(), "companies")
        .await
        .unwrap();

    assert_eq!(collection_schema_response.num_documents, 1250);
    assert_eq!(
        schema_to_resp(Company::collection_schema(), &collection_schema_response),
        collection_schema_response
    );
}

async fn delete_collection() {
    let collection_schema_response = collections_api::delete_collection(Config::get(), "companies")
        .await
        .unwrap();

    assert_eq!(collection_schema_response.num_documents, 1200);
    assert_eq!(
        schema_to_resp(Company::collection_schema(), &collection_schema_response),
        collection_schema_response
    );
}

async fn get_collections() {
    let collection_schema_response = collections_api::get_collections(Config::get())
        .await
        .unwrap();

    assert_eq!(collection_schema_response.len(), 2);
}

#[cfg(all(feature = "tokio_test", not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn create_collection_tokio() {
        create_collection().await
    }

    #[tokio::test]
    async fn get_collection_tokio() {
        get_collection().await
    }

    #[tokio::test]
    async fn delete_collection_tokio() {
        delete_collection().await
    }

    #[tokio::test]
    async fn get_collections_tokio() {
        get_collections().await
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn create_collection_wasm() {
        console_error_panic_hook::set_once();

        create_collection().await
    }

    #[wasm_bindgen_test]
    async fn get_collection_wasm() {
        console_error_panic_hook::set_once();

        get_collection().await
    }

    #[wasm_bindgen_test]
    async fn delete_collection_wasm() {
        console_error_panic_hook::set_once();

        delete_collection().await
    }

    #[wasm_bindgen_test]
    async fn get_collections_wasm() {
        console_error_panic_hook::set_once();

        get_collections().await
    }
}
