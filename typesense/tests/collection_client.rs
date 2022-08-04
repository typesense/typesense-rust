use serde::{Deserialize, Serialize};
use typesense::Document;

#[cfg(all(test, feature = "tokio-rt", not(target_arch = "wasm32")))]
mod hyper_tests {
    use super::*;
    use typesense::document::Document as DocumentTrait;
    use typesense::ClientBuilder;

    #[tokio::test]
    async fn collection_create() {
        let host = "http://localhost:5000";
        let api_key = "VerySecretKey";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        let collection_client = client.collection();

        let collection_schema_response = collection_client.create::<Company>().await.unwrap();

        assert_eq!(collection_schema_response.num_documents, 0);
        assert_eq!(
            collection_schema_response.schema,
            Company::collection_schema()
        );
    }

    #[tokio::test]
    async fn collection_retrieve() {
        let host = "http://localhost:5000";
        let api_key = "VerySecretKey";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        let collection_client = client.collection();

        let collection_schema_response = collection_client.retrieve("companies").await.unwrap();

        assert_eq!(collection_schema_response.num_documents, 1250);
        assert_eq!(
            collection_schema_response.schema,
            Company::collection_schema()
        );
    }

    #[tokio::test]
    async fn collection_delete() {
        let host = "http://localhost:5000";
        let api_key = "VerySecretKey";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        let collection_client = client.collection();

        let collection_schema_response = collection_client.delete("companies").await.unwrap();

        assert_eq!(collection_schema_response.num_documents, 1200);
        assert_eq!(
            collection_schema_response.schema,
            Company::collection_schema()
        );
    }

    #[tokio::test]
    async fn collection_retrieve_all() {
        let host = "http://localhost:5000";
        let api_key = "VerySecretKey";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        let collection_client = client.collection();

        let collection_schema_response = collection_client.retrieve_all().await.unwrap();

        assert_eq!(collection_schema_response.len(), 2);
    }
}

#[allow(dead_code)]
#[derive(Document, Serialize, Deserialize)]
#[typesense(default_sorting_field = "num_employees")]
#[typesense(collection_name = "companies")]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    country: String,
}
