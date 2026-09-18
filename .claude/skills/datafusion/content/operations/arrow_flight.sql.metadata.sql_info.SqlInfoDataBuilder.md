# `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.metadata.sql_info.SqlInfoDataBuilder.json).

<a id="op-70fe7b015f1a84c1c663a96c"></a>
## SqlInfoDataBuilder

`struct` · `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder` · arrow-flight 59.3.0

```rust
struct SqlInfoDataBuilder
```

Source: `src/sql/metadata/sql_info.rs:332`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Helper to create [`CommandGetSqlInfo`](../operations/arrow_flight.sql.gen.CommandGetSqlInfo.md#op-5df287408ae3ccad4071070f) responses.

[`CommandGetSqlInfo`](../operations/arrow_flight.sql.gen.CommandGetSqlInfo.md#op-5df287408ae3ccad4071070f) are metadata requests used by a Flight SQL
server to communicate supported capabilities to Flight SQL clients.

Servers constuct - usually static - [`SqlInfoData`](../operations/arrow_flight.sql.metadata.sql_info.SqlInfoData.md#op-2622035108d77db5eead8631) via the [`SqlInfoDataBuilder`](../operations/arrow_flight.sql.metadata.sql_info.SqlInfoDataBuilder.md#op-70fe7b015f1a84c1c663a96c),
and build responses using [`CommandGetSqlInfo::into_builder`](../operations/arrow_flight.sql.gen.CommandGetSqlInfo.md#op-362488b54e8bde5cc190613a)

<a id="op-896c6f9088562e5cd3e92ca9"></a>
## append

`function` · `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder::append` · arrow-flight 59.3.0

```rust
fn append(&mut self, name: impl SqlInfoName, value: impl Into<SqlInfoValue>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder", "path": "SqlInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [341, 1], "end": [381, 2], "filename": "src/sql/metadata/sql_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/sql_info.rs:348`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

register the specific sql metadata item

<a id="op-27c28c37b8d7d23ce008df6c"></a>
## build

`function` · `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder::build` · arrow-flight 59.3.0

```rust
fn build(self) -> Result<SqlInfoData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder", "path": "SqlInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [341, 1], "end": [381, 2], "filename": "src/sql/metadata/sql_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/sql_info.rs:355`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Encode the contents of this list according to the [FlightSQL spec]

[FlightSQL spec]: https://github.com/apache/arrow/blob/f9324b79bf4fc1ec7e97b32e3cce16e75ef0f5e3/format/FlightSql.proto#L32-L43

<a id="op-75917f63fdfcc781b55032cb"></a>
## clone

`function` · `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlInfoDataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder", "path": "SqlInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [331, 17], "end": [331, 22], "filename": "src/sql/metadata/sql_info.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/metadata/sql_info.rs:331`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e2771fe0d18d304e2a7e001"></a>
## default

`function` · `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlInfoDataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder", "path": "SqlInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [331, 35], "end": [331, 42], "filename": "src/sql/metadata/sql_info.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/metadata/sql_info.rs:331`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6365e6c67213ec62f47f5dff"></a>
## eq

`function` · `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlInfoDataBuilder) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder", "path": "SqlInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [331, 24], "end": [331, 33], "filename": "src/sql/metadata/sql_info.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/metadata/sql_info.rs:331`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b82bec1a96f7b2e432e0953c"></a>
## fmt

`function` · `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder", "path": "SqlInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [331, 10], "end": [331, 15], "filename": "src/sql/metadata/sql_info.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/metadata/sql_info.rs:331`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-454b990095bdf8a24fa7ddfc"></a>
## new

`function` · `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder::new` · arrow-flight 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder", "path": "SqlInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [341, 1], "end": [381, 2], "filename": "src/sql/metadata/sql_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/sql_info.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new SQL info builder

<a id="op-f5e2e7717bc0855389b1fafe"></a>
## schema

`function` · `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder::schema` · arrow-flight 59.3.0

```rust
fn schema() -> &'static Schema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder", "path": "SqlInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [341, 1], "end": [381, 2], "filename": "src/sql/metadata/sql_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/sql_info.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return the [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) for a GetSchema RPC call with [`crate::sql::CommandGetSqlInfo`](../operations/arrow_flight.sql.gen.CommandGetSqlInfo.md#op-5df287408ae3ccad4071070f)
