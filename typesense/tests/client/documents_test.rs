use serde::{Deserialize, Serialize};
use serde_json::json;
use typesense::models::{
    CollectionSchema, DeleteDocumentsParameters, DirtyValues, DocumentIndexParameters,
    ExportDocumentsParameters, Field, ImportDocumentsParameters, IndexAction, SearchParameters,
    UpdateDocumentsParameters,
};

use super::{get_client, new_id};

/* async fn run_test_document_lifecycle() {
    let client = get_client();
    let collection_name = new_id("books");

    // --- 1. Setup: Create a Collection ---
    let schema = CollectionSchema {
        name: collection_name.clone(),
        fields: vec![
            Field {
                name: "title".to_owned(),
                r#type: "string".to_owned(),
                ..Default::default()
            },
            Field {
                name: "author".to_owned(),
                r#type: "string".to_owned(),
                facet: Some(true),
                ..Default::default()
            },
            Field {
                name: "publication_year".to_owned(),
                r#type: "int32".to_owned(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let create_collection_result = client.collections().create(schema).await;
    assert!(
        create_collection_result.is_ok(),
        "Failed to create collection"
    );

    let book_1_id = &new_id("document_1");
    let book_1 = json!({
        "id": book_1_id,
        "title": "The Hitchhiker's Guide to the Galaxy",
        "author": "Douglas Adams",
        "publication_year": 1979
    });

    let book_2 = json!({
        "title": "The Lord of the Rings",
        "author": "J.R.R. Tolkien",
        "publication_year": 1954
    });
    let collection_client = client.collection_schemaless(&collection_name);
    let documents_client = collection_client.documents();

    // --- 2. Create a document (via `documents().create()`) ---
    let create_res = documents_client.create(&book_1, None).await;
    assert!(create_res.is_ok(), "Failed to create document 1");

    // --- 3. Upsert a document (via `documents().upsert()`) ---
    let upsert_res = documents_client.upsert(&book_2, None).await;
    assert!(upsert_res.is_ok(), "Failed to upsert document 2");

    // --- 4. Retrieve a single document (via `document(id).retrieve()`) ---
    let retrieve_res = client
        .collection_schemaless(&collection_name)
        .document(book_1_id)
        .retrieve()
        .await;
    assert!(retrieve_res.is_ok(), "Failed to retrieve document 1");
    assert_eq!(retrieve_res.unwrap(), book_1);

    // --- 5. Search for documents ---
    let search_params = SearchParameters::builder()
        .q("the")
        .query_by("title")
        .build();
    let search_res = documents_client.search(search_params).await;
    assert!(search_res.is_ok(), "Search failed");
    assert_eq!(search_res.unwrap().found, Some(2));

    // --- 6. Update a single document ---
    let partial_update = json!({ "publication_year": 1980 });
    let update_res = client
        .collection_schemaless(&collection_name)
        .document(book_1_id)
        .update(&partial_update, None)
        .await;
    assert!(update_res.is_ok(), "Failed to update document 1");

    // --- 7. Verify the single update ---
    let retrieve_after_update_res = client
        .collection_schemaless(&collection_name)
        .document(book_1_id)
        .retrieve()
        .await;
    let updated_doc = retrieve_after_update_res.unwrap();
    assert_eq!(
        updated_doc.get("publication_year").unwrap().as_i64(),
        Some(1980)
    );

    // --- 8. Delete a single document ---
    let delete_res = client
        .collection_schemaless(&collection_name)
        .document(book_1_id)
        .delete()
        .await;
    assert!(delete_res.is_ok(), "Failed to delete document 1");

    // --- 9. Verify single deletion ---
    let retrieve_after_delete_res = client
        .collection_schemaless(&collection_name)
        .document(book_1_id)
        .retrieve()
        .await;
    assert!(
        retrieve_after_delete_res.is_err(),
        "Document should not exist after deletion"
    );

    // --- 10. Bulk Import ---
    let new_books_jsonl = format!(
        "{}\n{}",
        json!({"title": "Foundation", "author": "Isaac Asimov", "publication_year": 1951}),
        json!({"title": "Dune", "author": "Frank Herbert", "publication_year": 1965})
    );

    let import_params = ImportDocumentsParameters {
        action: Some(IndexAction::Create),
        ..Default::default()
    };
    let import_res = documents_client
        .import_jsonl(new_books_jsonl, import_params)
        .await;
    assert!(import_res.is_ok(), "Bulk import failed");

    // --- 11. Verify Import via Search ---
    let search_after_import_params = SearchParameters {
        q: Some("*".to_owned()),
        query_by: Some("title".to_owned()),
        ..Default::default()
    };
    let search_after_import_res = documents_client.search(search_after_import_params).await;
    let search_results = search_after_import_res.unwrap();
    // 1 remaining (book_2) + 2 new imports = 3
    assert_eq!(search_results.found, Some(3));

    // --- 12. Bulk Update (via `documents().update()`) ---
    let bulk_update_params = UpdateDocumentsParameters {
        filter_by: Some("publication_year:<1960".to_owned()),
    };
    let bulk_update_payload = json!({ "author": "Sci-Fi Pioneer" });
    let bulk_update_res = documents_client
        .update(bulk_update_payload, bulk_update_params)
        .await;
    assert!(bulk_update_res.is_ok(), "Bulk update failed");
    // Should update Lord of the Rings (1954) and Foundation (1951)
    assert_eq!(bulk_update_res.unwrap().num_updated, 2);

    // --- 13. Export documents (via `documents().export()`) ---
    let export_params = ExportDocumentsParameters {
        filter_by: Some("author:\"Sci-Fi Pioneer\"".to_owned()),
        ..Default::default()
    };
    let export_res = documents_client.export(export_params).await;

    assert!(export_res.is_ok(), "Export failed");
    let exported_jsonl = export_res.unwrap();

    // Verify the exported content is a JSONL string with 2 lines.
    let lines: Vec<&str> = exported_jsonl.trim().split('\n').collect();
    assert_eq!(lines.len(), 2, "Exported JSONL should have 2 lines");
    let exported_doc_1: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(exported_doc_1["author"], "Sci-Fi Pioneer");

    // --- 14. Bulk Delete ---
    let delete_params = DeleteDocumentsParameters {
        filter_by: "publication_year:>1960".to_owned(),
        ..Default::default()
    };
    let bulk_delete_res = documents_client.delete(delete_params).await;
    assert!(bulk_delete_res.is_ok(), "Bulk delete failed");
    // Only "Dune" (1965) should be deleted
    assert_eq!(bulk_delete_res.unwrap().num_deleted, 1);
} */

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
        name: collection_name.clone(),
        fields: vec![
            Field {
                name: "title".to_owned(),
                r#type: "string".to_owned(),
                ..Default::default()
            },
            Field {
                name: "author".to_owned(),
                r#type: "string".to_owned(),
                facet: Some(true),
                ..Default::default()
            },
            Field {
                name: "publication_year".to_owned(),
                r#type: "int32".to_owned(),
                ..Default::default()
            },
            Field {
                name: "in_stock".to_owned(),
                r#type: "bool".to_owned(),
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
        q: Some("dune".to_owned()),
        query_by: Some("title".to_owned()),
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
        in_stock: Some(Some(false)),
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
    assert_eq!(updated_book.in_stock, Some(false));
    assert_eq!(updated_book.title, book_1.title); // Other fields are preserved

    // --- 7. Delete a single document, receiving the typed struct back ---
    let deleted_book = typed_collection
        .document(&book_1.id)
        .delete()
        .await
        .expect("Failed to delete typed document");
    // The deleted document (in its state just before deletion) is returned
    assert_eq!(deleted_book.id, book_1.id);
    assert_eq!(deleted_book.publication_year, 1966); // It was the updated version

    // --- 8. Verify single deletion ---
    let retrieve_after_delete_res = typed_collection.document(&book_1.id).retrieve().await;
    assert!(
        retrieve_after_delete_res.is_err(),
        "Typed document should not exist after deletion"
    );
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    /* #[tokio::test]
    async fn test_document_lifecycle() {
        run_test_document_lifecycle().await;
    } */
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

    /* #[wasm_bindgen_test]
    async fn test_document_lifecycle() {
        console_error_panic_hook::set_once();
        run_test_document_lifecycle().await;
    } */

    #[wasm_bindgen_test]
    async fn test_generic_document_lifecycle() {
        console_error_panic_hook::set_once();
        run_test_generic_document_lifecycle().await;
    }
}
