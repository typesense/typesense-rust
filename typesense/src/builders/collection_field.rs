//! Module with the common definitions for  the
//! [fields](https://github.com/typesense/typesense/blob/v0.19.0/include/field.)
//! available in Typesense.

use crate::traits::FieldType;
use bon::builder;
use typesense_codegen::models::{Field as TypesenseField, FieldEmbed};

/// Creates a new [`TypesenseField`] builder.
#[builder(
    // expose a public builder type named `FieldBuilder` and a public finish_fn `build()`
    builder_type(name = FieldBuilder, vis = "pub"),
    finish_fn(name = build, vis = "pub"),
    // allow passing &str into String params
    on(String, into)
)]
pub fn new_collection_field(
    #[builder(start_fn)] name: String,

    #[builder(start_fn)] typesense_type: FieldType,

    optional: Option<bool>,
    facet: Option<bool>,
    index: Option<bool>,
    locale: Option<String>,
    sort: Option<bool>,
    infix: Option<bool>,
    num_dim: Option<i32>,
    drop: Option<bool>,
    embed: Option<Box<FieldEmbed>>,
    store: Option<bool>,
    stem: Option<bool>,
    range_index: Option<bool>,
    vec_dist: Option<String>,
) -> TypesenseField {
    TypesenseField {
        name,
        r#type: typesense_type,
        optional,
        facet,
        index,
        locale,
        sort,
        infix,
        num_dim,
        drop,
        embed,
        store,
        stem,
        range_index,
        vec_dist,
        ..Default::default()
    }
}
