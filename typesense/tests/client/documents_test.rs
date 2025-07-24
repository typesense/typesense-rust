use serde_json::json;
use typesense::models::IndexAction;
use typesense_codegen::models::{
    CollectionSchema, DeleteDocumentsParameters, ExportDocumentsParameters, Field,
    ImportDocumentsParameters, SearchParameters, UpdateDocumentsParameters,
};

use super::{get_client, new_id};

#[tokio::test]
async fn test_document_lifecycle() {
    let client = get_client();
    let collection_name = new_id("books");

    // --- 1. Setup: Create a Collection ---
    let schema = CollectionSchema {
        name: collection_name.clone(),
        fields: vec![
            Field {
                name: "title".to_string(),
                r#type: "string".to_string(),
                ..Default::default()
            },
            Field {
                name: "author".to_string(),
                r#type: "string".to_string(),
                facet: Some(true),
                ..Default::default()
            },
            Field {
                name: "publication_year".to_string(),
                r#type: "int32".to_string(),
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
    let collection_client = client.collection(&collection_name);
    let documents_client = collection_client.documents();

    // --- 2. Create a document (via `documents().create()`) ---
    let create_res = documents_client.create(book_1.clone()).await;
    assert!(create_res.is_ok(), "Failed to create document 1");

    // --- 3. Upsert a document (via `documents().upsert()`) ---
    let upsert_res = documents_client.upsert(book_2.clone()).await;
    assert!(upsert_res.is_ok(), "Failed to upsert document 2");

    // --- 4. Retrieve a single document (via `document(id).retrieve()`) ---
    let retrieve_res = client
        .collection(&collection_name)
        .document(book_1_id)
        .retrieve()
        .await;
    assert!(retrieve_res.is_ok(), "Failed to retrieve document 1");
    assert_eq!(retrieve_res.unwrap(), book_1);

    // --- 5. Search for documents ---
    let search_params = SearchParameters {
        q: Some("the".to_string()),
        query_by: Some("title".to_string()),
        ..Default::default()
    };
    let search_res = documents_client.search(search_params).await;
    assert!(search_res.is_ok(), "Search failed");
    assert_eq!(search_res.unwrap().found, Some(2));

    // --- 6. Update a single document ---
    let partial_update = json!({ "publication_year": 1980 });
    let update_res = client
        .collection(&collection_name)
        .document(book_1_id)
        .update(partial_update)
        .await;
    assert!(update_res.is_ok(), "Failed to update document 1");

    // --- 7. Verify the single update ---
    let retrieve_after_update_res = client
        .collection(&collection_name)
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
        .collection(&collection_name)
        .document(book_1_id)
        .delete()
        .await;
    assert!(delete_res.is_ok(), "Failed to delete document 1");

    // --- 9. Verify single deletion ---
    let retrieve_after_delete_res = client
        .collection(&collection_name)
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
        .import(new_books_jsonl, import_params)
        .await;
    assert!(import_res.is_ok(), "Bulk import failed");

    // Give Typesense a moment to index
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // --- 11. Verify Import via Search ---
    let search_after_import_params = SearchParameters {
        q: Some("*".to_string()),
        query_by: Some("title".to_string()),
        ..Default::default()
    };
    let search_after_import_res = documents_client.search(search_after_import_params).await;
    let search_results = search_after_import_res.unwrap();
    // 1 remaining (book_2) + 2 new imports = 3
    assert_eq!(search_results.found, Some(3));

    // --- 12. Bulk Update (via `documents().update()`) ---
    let bulk_update_params = UpdateDocumentsParameters {
        filter_by: Some("publication_year:<1960".to_string()),
    };
    let bulk_update_payload = json!({ "author": "Sci-Fi Pioneer" });
    let bulk_update_res = documents_client
        .update(bulk_update_payload, bulk_update_params)
        .await;
    assert!(bulk_update_res.is_ok(), "Bulk update failed");
    // Should update Lord of the Rings (1954) and Foundation (1951)
    assert_eq!(bulk_update_res.unwrap().num_updated, 2);

    // Give Typesense a moment to index
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // --- 13. Export documents (via `documents().export()`) ---
    let export_params = ExportDocumentsParameters {
        filter_by: Some("author:\"Sci-Fi Pioneer\"".to_string()),
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
        filter_by: "publication_year:>1960".to_string(),
        ..Default::default()
    };
    let bulk_delete_res = documents_client.delete(delete_params).await;
    assert!(bulk_delete_res.is_ok(), "Bulk delete failed");
    // Only "Dune" (1965) should be deleted
    assert_eq!(bulk_delete_res.unwrap().num_deleted, 1);

    // --- 15. Teardown: Delete the collection ---
    let delete_collection_result = client.collection(&collection_name).delete().await;
    assert!(
        delete_collection_result.is_ok(),
        "Failed to delete collection"
    );
}
