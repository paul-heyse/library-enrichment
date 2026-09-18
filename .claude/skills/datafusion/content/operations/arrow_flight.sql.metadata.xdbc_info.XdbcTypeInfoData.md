# `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoData`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.metadata.xdbc_info.XdbcTypeInfoData.json).

<a id="op-d426704bc8f23224b94e60b1"></a>
## XdbcTypeInfoData

`struct` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoData` · arrow-flight 59.3.0

```rust
struct XdbcTypeInfoData
```

Source: `src/sql/metadata/xdbc_info.rs:91`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Helper to create [`CommandGetXdbcTypeInfo`](../operations/arrow_flight.sql.gen.CommandGetXdbcTypeInfo.md#op-9d0281cac1bf91cc4944a7dc) responses.

[`CommandGetXdbcTypeInfo`](../operations/arrow_flight.sql.gen.CommandGetXdbcTypeInfo.md#op-9d0281cac1bf91cc4944a7dc) are metadata requests used by a Flight SQL
server to communicate supported capabilities to Flight SQL clients.

Servers constuct - usually static - [`XdbcTypeInfoData`](../operations/arrow_flight.sql.metadata.xdbc_info.XdbcTypeInfoData.md#op-d426704bc8f23224b94e60b1) via the [`XdbcTypeInfoDataBuilder`](../operations/arrow_flight.sql.metadata.xdbc_info.XdbcTypeInfoDataBuilder.md#op-0294db3bf2620399969ccd2c),
and build responses using [`CommandGetXdbcTypeInfo::into_builder`](../operations/arrow_flight.sql.gen.CommandGetXdbcTypeInfo.md#op-1e3c5fc281a7a6533b19ed49).

<a id="op-e7930c331994ff3e9eb96719"></a>
## record_batch

`function` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoData::record_batch` · arrow-flight 59.3.0

```rust
fn record_batch(&self, data_type: impl Into<Option<i32>>) -> Result<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoData", "path": "XdbcTypeInfoData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [113, 2], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/xdbc_info.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return the raw (not encoded) RecordBatch that will be returned
from [`CommandGetXdbcTypeInfo`](../operations/arrow_flight.sql.gen.CommandGetXdbcTypeInfo.md#op-9d0281cac1bf91cc4944a7dc)

<a id="op-b94e429233bfa94f18b3894e"></a>
## schema

`function` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoData::schema` · arrow-flight 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoData", "path": "XdbcTypeInfoData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [113, 2], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/xdbc_info.rs:110`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return the schema of the RecordBatch that will be returned
from [`CommandGetXdbcTypeInfo`](../operations/arrow_flight.sql.gen.CommandGetXdbcTypeInfo.md#op-9d0281cac1bf91cc4944a7dc)
