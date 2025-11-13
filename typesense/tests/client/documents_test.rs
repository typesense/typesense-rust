use serde::{Deserialize, Serialize};
use serde_json::json;
use typesense::models::{
    CollectionSchema, DeleteDocumentsParameters, DirtyValues, DocumentIndexParameters,
    ExportDocumentsParameters, Field, ImportDocumentsParameters, IndexAction, SearchParameters,
    UpdateDocumentsParameters,
};

use super::{get_client, new_id};

async fn run_test_schemaless_document_lifecycle() {
    let client = get_client();
    let collection_name = new_id("books");

    // ---  Setup: Create a Collection ---
    let schema = CollectionSchema {
        name: collection_name.as_str().into(),
        fields: vec![
            Field {
                name: "title".into(),
                r#type: "string".into(),
                ..Default::default()
            },
            Field {
                name: "author".into(),
                r#type: "string".into(),
                facet: Some(true),
                ..Default::default()
            },
            Field {
                name: "publication_year".into(),
                r#type: "int32".into(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let _create_collection_response = client
        .collections()
        .create(schema)
        .await
        .expect("Failed to create collection");

    let book_1_id = &new_id("document_1");
    let book_1 = json!({"id": book_1_id, "title": "The Hitchhiker's Guide to the Galaxy","author": "John","publication_year": 1979});
    let book_2 =
        json!({"title": "The Lord of the Rings","author": "John","publication_year": 1954});
    let book_3 = json!({"title": "Book 3","author": "John","publication_year": 2100});
    let collection_client = client.collection_schemaless(&collection_name);
    let documents_client = collection_client.documents();

    // ---  Bulk Import ---
    let new_books_jsonl = format!("{}\n{}\n{}", book_1, book_2, book_3);
    let import_params = ImportDocumentsParameters {
        action: Some(IndexAction::Create),
        ..Default::default()
    };
    let import_res = documents_client
        .import_jsonl(new_books_jsonl, import_params)
        .await;

    assert!(
        !import_res.unwrap().contains("success: false"),
        "Bulk import failed"
    );

    // --- Retrieve a single document (via `document(id).retrieve()`) ---
    let retrieved_book = client
        .collection_schemaless(&collection_name)
        .document(book_1_id)
        .retrieve()
        .await
        .expect("Failed to retrieve document 1");
    assert_eq!(retrieved_book, book_1);

    // ---  Search for documents ---
    let search_params = SearchParameters::builder()
        .q("the")
        .query_by("title")
        .build();
    let search_res = documents_client
        .search(search_params)
        .await
        .expect("Search failed");
    assert_eq!(search_res.found, Some(2));

    // ---  Delete a single document ---
    let _deleted_book = client
        .collection_schemaless(&collection_name)
        .document(book_1_id)
        .delete()
        .await
        .expect("Failed to delete document 1");

    // ---  Verify single deletion ---
    let retrieve_after_delete_res = client
        .collection_schemaless(&collection_name)
        .document(book_1_id)
        .retrieve()
        .await;
    assert!(
        retrieve_after_delete_res.is_err(),
        "Document should not exist after deletion"
    );

    // ---  Export documents (via `documents().export_jsonl()`) ---
    let export_params = ExportDocumentsParameters {
        filter_by: Some("author:John".into()),
        ..Default::default()
    };
    let exported_jsonl = documents_client
        .export_jsonl(export_params)
        .await
        .expect("Export failed");

    // Verify the exported content is a JSONL string with 2 lines.
    let lines: Vec<&str> = exported_jsonl.trim().split('\n').collect();
    assert_eq!(lines.len(), 2, "Exported JSONL should have 2 lines");
    let exported_doc_1: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(exported_doc_1["author"], "John");

    // --- Bulk Delete ---
    let delete_params = DeleteDocumentsParameters {
        filter_by: "publication_year:>1960".into(),
        ..Default::default()
    };
    let bulk_delete_response = documents_client
        .delete(delete_params)
        .await
        .expect("Bulk delete failed");
    // Only "The Hitchhiker's Guide to the Galaxy" (1979) should be deleted
    assert_eq!(bulk_delete_response.num_deleted, 1);
}

// --- TESTS FOR GENERIC FEATURES ---

/// A strongly-typed representation of a book document.
#[derive(typesense::Typesense, Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Book {
    id: String,
    title: String,
    author: String,
    publication_year: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    in_stock: Option<bool>,
}

async fn run_test_generic_document_lifecycle() {
    let client = get_client();
    let collection_name = new_id("generic_books");

    // --- 1. Setup: Create a Collection matching the Book struct ---
    let schema = CollectionSchema {
        name: collection_name.as_str().into(),
        fields: vec![
            Field {
                name: "title".into(),
                r#type: "string".into(),
                ..Default::default()
            },
            Field {
                name: "author".into(),
                r#type: "string".into(),
                facet: Some(true),
                ..Default::default()
            },
            Field {
                name: "publication_year".into(),
                r#type: "int32".into(),
                ..Default::default()
            },
            Field {
                name: "in_stock".into(),
                r#type: "bool".into(),
                optional: Some(true),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let _create_collection = client
        .collections()
        .create(schema)
        .await
        .expect("Failed to create collection for generic test");

    // Use the strongly-typed collection client
    let typed_collection = client.collection_named::<Book>(&collection_name);

    let book_1 = Book {
        id: new_id("book_1"),
        title: "Dune".to_owned(),
        author: "Frank Herbert".to_owned(),
        publication_year: 1965,
        in_stock: Some(true),
    };

    let book_2 = Book {
        id: new_id("book_2"),
        title: "Foundation".to_owned(),
        author: "Isaac Asimov".to_owned(),
        publication_year: 1951,
        in_stock: Some(false),
    };

    // --- 2. Create a document using a typed struct ---
    let create_book = typed_collection
        .documents()
        .create(&book_1, None)
        .await
        .expect("Failed to create typed document");
    // The created document should be returned and be equal to the input
    assert_eq!(create_book, book_1);

    // --- 3. Upsert a document using a typed struct ---
    let upsert_book = typed_collection
        .documents()
        .upsert(&book_2, None)
        .await
        .expect("Failed to upsert typed document");
    assert_eq!(upsert_book, book_2);

    // --- 4. Retrieve a single document and deserialize into a struct ---
    let retrieve_book = typed_collection
        .document(&book_1.id)
        .retrieve()
        .await
        .expect("Failed to retrieve typed document");
    assert_eq!(retrieve_book, book_1);

    // --- 5. Search for documents with strongly-typed results ---
    let search_params = SearchParameters {
        q: Some("dune".into()),
        query_by: Some("title".into()),
        ..Default::default()
    };
    let search_results = typed_collection
        .documents()
        .search(search_params)
        .await
        .expect("Typed search failed");

    assert_eq!(search_results.found, Some(1));
    let hits = search_results.hits.expect("Search should have hits");
    assert_eq!(hits.len(), 1);
    // The document within the hit should be the deserialized Book struct
    let hit_doc = hits[0]
        .document
        .as_ref()
        .expect("Hit should contain a document");
    assert_eq!(hit_doc, &book_1);

    // --- 6. Update a single document with a partial payload ---
    let partial_update_struct = BookPartial {
        publication_year: Some(1966),
        in_stock: Some(None),
        ..Default::default()
    };
    let index_params = DocumentIndexParameters {
        dirty_values: Some(DirtyValues::CoerceOrReject),
    };
    let updated_book = typed_collection
        .document(&book_1.id)
        .update(&partial_update_struct, Some(index_params))
        .await
        .expect("Failed to update typed document");

    // The returned document should be the full, updated Book struct
    assert_eq!(updated_book.publication_year, 1966);
    assert_eq!(
        updated_book.in_stock, None,
        "The updated 'in_stock' must be null"
    );
    assert_eq!(updated_book.title, book_1.title); // Other fields are preserved

    // --- 7. Bulk Update (via `documents().update()`) ---
    let bulk_update_params = UpdateDocumentsParameters {
        filter_by: Some("publication_year:>1965".into()),
    };
    let bulk_update_response = typed_collection
        .documents()
        .update(
            &BookPartial {
                publication_year: Some(2100),
                ..Default::default()
            },
            bulk_update_params,
        )
        .await
        .expect("Bulk update failed");
    // Should update book 1 (1966)
    assert_eq!(bulk_update_response.num_updated, 1);

    // --- 8. Delete a single document, receiving the typed struct back ---
    let deleted_book = typed_collection
        .document(&book_1.id)
        .delete()
        .await
        .expect("Failed to delete typed document");
    // The deleted document (in its state just before deletion) is returned
    assert_eq!(deleted_book.id, book_1.id);
    assert_eq!(deleted_book.publication_year, 2100); // It was the bulk updated version

    // --- 9. Verify single deletion ---
    let retrieve_after_delete_res = typed_collection.document(&book_1.id).retrieve().await;
    assert!(
        retrieve_after_delete_res.is_err(),
        "Typed document should not exist after deletion"
    );
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn test_schemaless_document_lifecycle() {
        run_test_schemaless_document_lifecycle().await;
    }
    #[tokio::test]
    async fn test_generic_document_lifecycle() {
        run_test_generic_document_lifecycle().await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_schemaless_document_lifecycle() {
        console_error_panic_hook::set_once();
        run_test_schemaless_document_lifecycle().await;
    }

    #[wasm_bindgen_test]
    async fn test_generic_document_lifecycle() {
        console_error_panic_hook::set_once();
        run_test_generic_document_lifecycle().await;
    }
}
