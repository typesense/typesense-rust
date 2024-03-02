#![allow(dead_code)]

use super::Config;
use serde::{Deserialize, Serialize};
use typesense::document::Document;
use typesense::Typesense;
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

#[cfg(all(feature = "tokio_test", not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn import_documents_tokio() {
        import_documents().await
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
}
