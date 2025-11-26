# Typesense (rust library)

[![Crate](https://img.shields.io/crates/v/typesense.svg)](https://crates.io/crates/typesense)
[![API](https://docs.rs/typesense/badge.svg)](https://docs.rs/typesense)

Community-maintained Rust client library for Typesense | Work In Progress &amp; Help Wanted!

## Collection schema derive

Apply `#[derive(Typesense)]` to any struct you want to index in Typesense. The macro generates:

- a `collection_schema()` definition based on your struct fields and attributes.
- a `{struct_name}Partial` struct for partial updates of Typesense documents.

### Quick example

```rust
#[derive(Typesense, Serialize, Deserialize)]
#[typesense(
    collection_name = "mega_products",
    default_sorting_field = "price",
    symbols_to_index = ["+", "-"]
)]
struct MegaProduct {
    id: String,

    #[typesense(infix, stem)]
    title: String,

    #[typesense(facet)]
    brand: String,

    #[typesense(sort)]
    price: f32,

    #[typesense(rename = "product_name", sort = true)]
    #[serde(rename = "product_name")]
    official_name: String,
}

// update a Typesense document using the generated partial struct
let update_payload = MegaProductPartial {
    price: Some(25.99),
    ..Default::default()
};

let result = client
    .collection::<MegaProduct>()
    .document("product-1")
    .update(&update_payload, None)
    .await;
```

### Supported collection parameters

| Key                     | Type            | Description / Notes                                                                                                                         |
| ----------------------- | --------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `collection_name`       | string          | Defaults to the struct name in lowercase                                                                                                    |
| `default_sorting_field` | string          | Must match the field name after `rename` if used                                                                                            |
| `enable_nested_fields`  | bool            | Enables Typesense nested field support                                                                                                      |
| `token_separators`      | list of strings | List of symbols or special characters to be used for splitting the text into individual words in addition to space and new-line characters. |
| `symbols_to_index`      | list of strings | List of symbols or special characters to be indexed.                                                                                        |

### Supported field parameter

| Attribute     | Type    | Description / Notes                                                                                                   |
| ------------- | ------- | --------------------------------------------------------------------------------------------------------------------- |
| `facet`       | bool    | Enable faceting for the field                                                                                         |
| `sort`        | bool    | Marks the field as sortable                                                                                           |
| `index`       | bool    | Whether to index the field in memmory                                                                                 |
| `store`       | bool    | Whether to store the field on disk                                                                                    |
| `infix`       | bool    | Enables infix search                                                                                                  |
| `stem`        | bool    | Values are stemmed before indexing in-memory.                                                                         |
| `range_index` | bool    | Enables an index optimized for range filtering on numerical fields                                                    |
| `optional`    | bool    | Fields with type `Option<T>` are optional in the generated Typesense schema. Setting this attribute will override it. |
| `num_dim`     | integer | Set this to a non-zero value to treat a field of type `float[]` as a vector field.                                    |
| `locale`      | string  | Locale for text processing                                                                                            |
| `vec_dist`    | string  | Distance metric to be used for vector search                                                                          |
| `type`        | string  | Override the field type in Typesense                                                                                  |
| `rename`      | string  | Rename the field in the Typesense schema                                                                              |
| `flatten`     | --      | Generate Typesense field schemas for a nested struct                                                                  |
| `skip`        | --      | Skips this field in the Typesense schema                                                                              |

All boolean attributes can be either set to `true` using shorthand flags or explicitly set a value `=true/false`. Example:

```rust
#[typesense(facet)]
brand: String,

#[typesense(facet = false)]
weight: f32,
```

#### Indexing nested objects

When you have fields that are also structs, you need to mark all structs with `#[derive(Typesense)]`. The generated Typesense schema for those fields will have type of `object` (or `object[]` if the field is a vector).

Applying `#[typesense(flatten)]` on a field will expand the nested field schemas into the parent.

```rust
#[typesense(flatten)]
supplier: SupplierInfo,
```

If the field has a rename:

```rust
#[typesense(flatten, rename = "logistics_data")]
logistics: Logistics,
```

flattened fields become `logistics_data.field_name`.

`#[typesense(flatten, skip)]` produces only the flattened fields and omits the parent object field.

#### Nested objects example:

```rust
#[derive(Typesense, Serialize, Deserialize)]
struct ProductDetails {
    #[typesense(facet)]
    part_number: String,
    #[typesense(skip)]
    description: String,
}

#[derive(Typesense, Serialize, Deserialize)]
#[typesense(
    collection_name = "mega_products",
)]
struct MegaProduct {
    #[typesense(flatten)]
    details: ProductDetails,
}
```

Will generate this schema:

```jsonc
{
    "name": "mega_products"
    "fields": [
        {"name": "details", "type": "object"}, // <-- `#[typesense(flatten, skip)]` will omit this field
        {"name": "details.part_number", "type": "string", "facet": true}
        // `description` is skipped
    ],
}
```

## Development

When updating or adding new parameters and endpoints, make changes directly in the [Typesense API spec repository](https://github.com/typesense/typesense-api-spec).

Once your changes are merged, you can update this project as follows (you can also run tasks individually):

```bash
cargo xtask fetch preprocess code-gen
```

This will:

- Download the latest API spec.
- Write it to our local [`openapi.yml`](./openapi.yml).
- Preprocess it into [`preprocessed_openapi.yml`](./preprocessed_openapi.yml).
- Regenerate the `/typesense_codegen` crate.

The preprocessing step does two things:

- Flatten the URL params defined as objects into individual URL parameters (in [`preprocess_openapi.rs`](xtask/src/preprocess_openapi.rs))
- Inject OpenAPI vendor attributes (e.g., generic parameters, schema builders) into the spec before code generation (in [`add_vendor_attributes.rs`](./xtask/src/add_vendor_attributes.rs))

You can also run `code-gen` directly through Docker:

```
docker run --rm \
    -v $PWD:/local openapitools/openapi-generator-cli generate \
    -i /local/preprocessed_openapi.yml \
    -g rust \
    -o /local/typesense_codegen \
    -t /local/openapi-generator-template \
    --additional-properties library=reqwest \
    --additional-properties supportMiddleware=true \
    --additional-properties useSingleRequestParameter=true
```

### Testing

Make sure you have a Typesense server up and running:

```bash
docker compose up
```

Then run this command in the root folder to run the integration tests:

```bash
cargo test-clean -- --all-features
```

This is an alias command which will run a script to clean up your Typesense server after the tests finish. You can pass any arguments of `cargo test` after the `--`.

To run test for wasm (chrome, headless):

```bash
cargo test-clean --wasm
```

If you'd like to contribute, please join our [Slack Community](https://join.slack.com/t/typesense-community/shared_invite/zt-mx4nbsbn-AuOL89O7iBtvkz136egSJg) and say hello!
