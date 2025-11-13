use crate::{preprocess_openapi::OpenAPI, vendor_attributes::VendorAttributes};

pub fn add_vendor_attributes(doc: &mut OpenAPI) -> Result<(), String> {
    println!("Adding custom x-* vendor attributes...");
    let mut attrs = VendorAttributes::new(doc);

    // Schemas
    attrs.schema_builder([
        "CollectionSchema",
        "Field",
        "SearchParameters",
        "MultiSearchParameters",
        "MultiSearchCollectionParameters",
    ])?;

    attrs.schema_generic_parameter([
        ("SearchResult", "D"),
        ("SearchGroupedHit", "D"),
        ("SearchResultHit", "D"),
        ("MultiSearchResult", "D"),
        ("MultiSearchResultItem", "D"),
    ])?;

    attrs.schema_field_type_overrides(
        "SearchResult",
        [
            ("hits", "Option<Vec<models::SearchResultHit<D>>>"),
            ("grouped_hits", "Option<Vec<models::SearchGroupedHit<D>>>"),
        ],
    )?;
    attrs.schema_field_type_overrides(
        "SearchGroupedHit",
        [("hits", "Vec<models::SearchResultHit<D>>")],
    )?;
    attrs.schema_field_type_overrides("SearchResultHit", [("document", "Option<D>")])?;
    attrs.schema_field_type_overrides(
        "MultiSearchResult",
        [("results", "Vec<models::MultiSearchResultItem<D>>")],
    )?;

    // Operations
    attrs
        .operation("/collections/{collectionName}/documents/search", "get")
        .generic_parameter("D: for<'de> serde::Deserialize<'de> + Serialize")?
        .return_type("models::SearchResult<D>")?;

    attrs
        .operation("/multi_search", "post")
        .return_type("serde_json::Value")?;

    // The endpoint return `null` if no schema changes are in progress
    attrs
        .operation("/operations/schema_changes", "get")
        .return_type("Option<Vec<models::SchemaChangeStatus>>")?;

    // The documents /import endpoint expects a text/plain body and response
    attrs
        .operation("/collections/{collectionName}/documents/import", "post")
        .body_is_raw_text()?
        .supports_plain_text()?;

    // The stemming /import endpoint also expects a text/plain body and response
    attrs
        .operation("/stemming/dictionaries/import", "post")
        .body_is_raw_text()?
        .supports_plain_text()?;

    attrs
        .operation("/collections/{collectionName}/documents/export", "get")
        .supports_plain_text()?;

    attrs
        .operation("/collections/{collectionName}/documents", "patch")
        .generic_parameter("B: Serialize")?
        .params_generic_parameter("B")?
        .request_type("B")?;

    attrs
        .operation(
            "/collections/{collectionName}/documents/{documentId}",
            "patch",
        )
        .generic_parameter("B: Serialize")?
        .params_generic_parameter("B")?
        .request_type("B")?;

    Ok(())
}
