# Typesense (rust library)

[![Crate](https://img.shields.io/crates/v/typesense.svg)](https://crates.io/crates/typesense)
[![API](https://docs.rs/typesense/badge.svg)](https://docs.rs/typesense)

Rust client library for Typesense | Work In Progress &amp; Help Wanted!

### Development

When updating or adding new parameters and endpoints, make changes directly in the [Typesense API spec repository](https://github.com/typesense/typesense-api-spec).

Once your changes are merged, you can update this project as follows:

```bash
cargo xtask fetch code-gen
```

This will:

- Download the latest API spec.
- Write it to our local [`openapi.yml`](./openapi.yml).
- Preprocess it into [`preprocessed_openapi.yml`](./preprocessed_openapi.yml).
- Regenerate the `/typesense_codegen` crate.

The preprocessing step does two things:

- Unwrap/flatten the URL params defined as objects into individual URL parameters (in [`preprocess_openapi.rs`](xtask/src/preprocess_openapi.rs))
- Inject OpenAPI vendor attributes (e.g., generic parameters, schema builders) into the spec before code generation (in [`add_vendor_attributes.rs`](./xtask/src/add_vendor_attributes.rs))

If you'd like to contribute, please join our [Slack Community](https://join.slack.com/t/typesense-community/shared_invite/zt-mx4nbsbn-AuOL89O7iBtvkz136egSJg) and say hello!
