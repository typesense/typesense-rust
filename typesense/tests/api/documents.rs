#![allow(dead_code)]

use super::Config;
use serde::{Deserialize, Serialize};
use typesense::Typesense;
use typesense::document::Document;
use typesense::models::SearchParameters;
use typesense_codegen::apis::documents_api;

#[derive(Typesense, Serialize, Deserialize)]
#[typesense(collection_name = "companies", default_sorting_field = "num_employees")]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    country: String,
}

async fn import_documents() {
    let documents = [
        Company {
            company_name: "test".to_owned(),
            num_employees: 1,
            country: "c1".to_owned(),
        },
        Company {
            company_name: "test2".to_owned(),
            num_employees: 2,
            country: "c2".to_owned(),
        },
    ]
    .map(|c| serde_json::to_string(&c).unwrap())
    .join("\n");

    let resp = documents_api::import_documents(
        Config::get(),
        &Company::collection_schema().name,
        documents,
        None,
    )
    .await
    .unwrap();

    assert_eq!(&resp, "{\"success\":true}\n{\"success\":true}");
}

async fn search_collection() {
    let search = SearchParameters {
        q: "test".to_owned(),
        query_by: "company_name".to_owned(),
        ..Default::default()
    };

    let resp = documents_api::search_collection::<Company>(
        Config::get(),
        &Company::collection_schema().name,
        search,
    )
    .await
    .unwrap();

    assert_eq!(resp.found, Some(2));
    assert_eq!(
        resp.hits
            .unwrap()
            .first()
            .unwrap()
            .document
            .as_ref()
            .unwrap()
            .company_name,
        "test".to_owned()
    );
}

#[cfg(all(feature = "tokio_test", not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn import_documents_tokio() {
        import_documents().await
    }

    #[tokio::test]
    async fn search_collection_tokio() {
        search_collection().await
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn import_documents_wasm() {
        console_error_panic_hook::set_once();

        import_documents().await
    }

    #[wasm_bindgen_test]
    async fn search_collection_wasm() {
        console_error_panic_hook::set_once();

        search_collection().await
    }
}
