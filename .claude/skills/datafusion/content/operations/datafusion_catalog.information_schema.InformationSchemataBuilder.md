# `datafusion_catalog::information_schema::InformationSchemataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.information_schema.InformationSchemataBuilder.json).

<a id="op-396b7174acc6ef795c9624db"></a>
## InformationSchemataBuilder

`struct` · `datafusion_catalog::information_schema::InformationSchemataBuilder` · datafusion-catalog 55.1.0

```rust
struct InformationSchemataBuilder
```

Source: `src/information_schema.rs:1066`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Builder that produces [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) values matching the schema of
`information_schema.schemata` (see [`schemata_schema`](../operations/datafusion_catalog.information_schema.schemata_schema.md#op-313993bf073277021adf5e64)).

Intended for downstream catalog implementations that need to emit
`schemata` rows from their own metadata source rather than going
through DataFusion's `InformationSchemaProvider`, which enumerates
schemas synchronously via `CatalogProviderList` and so is unsuitable
for catalog backends that resolve asynchronously.

<a id="op-eb5476fa1c9b6d7f3a6dbadb"></a>
## add_schemata

`function` · `datafusion_catalog::information_schema::InformationSchemataBuilder::add_schemata` · datafusion-catalog 55.1.0

```rust
fn add_schemata(&mut self, catalog_name: &str, schema_name: &str, schema_owner: Option<&str>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemataBuilder", "path": "InformationSchemataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1083, 1], "end": [1141, 2], "filename": "src/information_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/information_schema.rs:1102`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Append one row to the builder. `schema_owner` is the optional SQL
schema owner; the three `default_character_set_*` columns and
`sql_path` are written as null (DataFusion does not model those
concepts; see the PostgreSQL docs link on [`schemata_schema`](../operations/datafusion_catalog.information_schema.schemata_schema.md#op-313993bf073277021adf5e64)).

<a id="op-17d8466a71df170ad1628cf4"></a>
## default

`function` · `datafusion_catalog::information_schema::InformationSchemataBuilder::default` · datafusion-catalog 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemataBuilder", "path": "InformationSchemataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1077, 1], "end": [1081, 2], "filename": "src/information_schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/information_schema.rs:1078`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73278313aa3f032dadd80f9c"></a>
## finish

`function` · `datafusion_catalog::information_schema::InformationSchemataBuilder::finish` · datafusion-catalog 55.1.0

```rust
fn finish(&mut self) -> Result<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemataBuilder", "path": "InformationSchemataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1083, 1], "end": [1141, 2], "filename": "src/information_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/information_schema.rs:1126`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Finalize the builder into a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

Returns an error only if Arrow buffer construction fails, which
the builder's column-count and type invariants make unreachable
under normal use. The `Result` return type preserves room to add
validation in the future without a breaking API change.

<a id="op-f904b728fcc0fd96be4070f4"></a>
## fmt

`function` · `datafusion_catalog::information_schema::InformationSchemataBuilder::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemataBuilder", "path": "InformationSchemataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1065, 10], "end": [1065, 15], "filename": "src/information_schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/information_schema.rs:1065`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c29d1ca932d9051b232b3170"></a>
## new

`function` · `datafusion_catalog::information_schema::InformationSchemataBuilder::new` · datafusion-catalog 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemataBuilder", "path": "InformationSchemataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1083, 1], "end": [1141, 2], "filename": "src/information_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/information_schema.rs:1085`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Construct an empty builder.
