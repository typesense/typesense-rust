use crate::vendor_attributes::VendorAttributes;
use serde_yaml::Mapping;

pub fn add_vendor_attributes(doc_root: &mut Mapping) -> Result<(), String> {
    println!("Adding custom x-* vendor attributes...");
    let mut attrs = VendorAttributes::new(doc_root);

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
            ("hits", "Option<Vec<models::SearchResultHit<'a, D>>>"),
            (
                "grouped_hits",
                "Option<Vec<models::SearchGroupedHit<'a, D>>>",
            ),
        ],
    )?;
    attrs.schema_field_type_overrides(
        "SearchGroupedHit",
        [("hits", "Vec<models::SearchResultHit<'a, D>>")],
    )?;
    attrs.schema_field_type_overrides("SearchResultHit", [("document", "Option<D>")])?;
    attrs.schema_field_type_overrides(
        "MultiSearchResult",
        [("results", "Vec<models::MultiSearchResultItem<'a, D>>")],
    )?;

    // Operations
    attrs
        .operation("/collections/{collectionName}/documents/search", "get")
        .generic_parameter("D: for<'de> serde::Deserialize<'de> + Serialize")
        .return_type("models::SearchResult<'static, D>")
        .done()?;

    attrs
        .operation("/multi_search", "post")
        .return_type("serde_json::Value")
        .done()?;

    // The endpoint return `null` if no schema changes are in progress
    attrs
        .operation("/operations/schema_changes", "get")
        .return_type("Option<Vec<models::SchemaChangeStatus<'static>>>")
        .done()?;

    // The documents /import endpoint expects a text/plain body and response
    attrs
        .operation("/collections/{collectionName}/documents/import", "post")
        .body_is_raw_text()
        .supports_plain_text()
        .done()?;

    // The stemming /import endpoint also expects a text/plain body and response
    attrs
        .operation("/stemming/dictionaries/import", "post")
        .body_is_raw_text()
        .supports_plain_text()
        .done()?;

    attrs
        .operation("/collections/{collectionName}/documents/export", "get")
        .supports_plain_text()
        .done()?;

    attrs
        .operation("/collections/{collectionName}/documents", "patch")
        .generic_parameter("B: Serialize")
        .params_generic_parameter("B")
        .request_type("B")
        .done()?;

    attrs
        .operation(
            "/collections/{collectionName}/documents/{documentId}",
            "patch",
        )
        .generic_parameter("B: Serialize")
        .params_generic_parameter("B")
        .request_type("B")
        .done()?;

    attrs.schemas_mark_owned_data()?;

    Ok(())
}
