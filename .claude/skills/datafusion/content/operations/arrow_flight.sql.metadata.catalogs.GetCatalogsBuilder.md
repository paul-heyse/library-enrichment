# `arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.metadata.catalogs.GetCatalogsBuilder.json).

<a id="op-c8f333fc351087b2c1f0ffd0"></a>
## GetCatalogsBuilder

`struct` · `arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder` · arrow-flight 59.3.0

```rust
struct GetCatalogsBuilder
```

Source: `src/sql/metadata/catalogs.rs:32`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A builder for a [`CommandGetCatalogs`](../operations/arrow_flight.sql.gen.CommandGetCatalogs.md#op-79bcbfce6a9ae0588357547a) response.

Builds rows like this:

* catalog_name: utf8,

<a id="op-758b39dd49dda977c18f7c2b"></a>
## append

`function` · `arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder::append` · arrow-flight 59.3.0

```rust
fn append(&mut self, catalog_name: impl Into<String>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder", "path": "GetCatalogsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [88, 2], "filename": "src/sql/metadata/catalogs.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/catalogs.rs:64`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Append a row

<a id="op-fe39f1cad456da8cac947698"></a>
## build

`function` · `arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder::build` · arrow-flight 59.3.0

```rust
fn build(self) -> Result<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder", "path": "GetCatalogsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [88, 2], "filename": "src/sql/metadata/catalogs.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/catalogs.rs:70`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

builds a `RecordBatch` with the correct schema for a
[`CommandGetCatalogs`](../operations/arrow_flight.sql.gen.CommandGetCatalogs.md#op-79bcbfce6a9ae0588357547a) response

<a id="op-b97283ec714e37077f476bf4"></a>
## default

`function` · `arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder", "path": "GetCatalogsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [53, 2], "filename": "src/sql/metadata/catalogs.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/metadata/catalogs.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8555973a14bb62f7040ec6d"></a>
## from

`function` · `arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder::from` · arrow-flight 59.3.0

```rust
fn from(_: CommandGetCatalogs) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder", "path": "GetCatalogsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [47, 2], "filename": "src/sql/metadata/catalogs.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "CommandGetCatalogs"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sql/metadata/catalogs.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d87775d63fe385408ac543be"></a>
## new

`function` · `arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder::new` · arrow-flight 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder", "path": "GetCatalogsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [88, 2], "filename": "src/sql/metadata/catalogs.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/catalogs.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new instance of [`GetCatalogsBuilder`](../operations/arrow_flight.sql.metadata.catalogs.GetCatalogsBuilder.md#op-c8f333fc351087b2c1f0ffd0)

<a id="op-b48b5e62f8e92052cf8e4a3c"></a>
## schema

`function` · `arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder::schema` · arrow-flight 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder", "path": "GetCatalogsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [88, 2], "filename": "src/sql/metadata/catalogs.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/catalogs.rs:85`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the schema that will result from [`CommandGetCatalogs`]

[`CommandGetCatalogs`]: crate::sql::CommandGetCatalogs
