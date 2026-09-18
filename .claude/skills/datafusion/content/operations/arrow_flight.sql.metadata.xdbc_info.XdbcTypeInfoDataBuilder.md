# `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.metadata.xdbc_info.XdbcTypeInfoDataBuilder.json).

<a id="op-0294db3bf2620399969ccd2c"></a>
## XdbcTypeInfoDataBuilder

`struct` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder` · arrow-flight 59.3.0

```rust
struct XdbcTypeInfoDataBuilder
```

Source: `src/sql/metadata/xdbc_info.rs:150`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A builder for [`XdbcTypeInfoData`](../operations/arrow_flight.sql.metadata.xdbc_info.XdbcTypeInfoData.md#op-d426704bc8f23224b94e60b1) which is used to create [`CommandGetXdbcTypeInfo`](../operations/arrow_flight.sql.gen.CommandGetXdbcTypeInfo.md#op-9d0281cac1bf91cc4944a7dc) responses.

# Example
```
use arrow_flight::sql::{Nullable, Searchable, XdbcDataType};
use arrow_flight::sql::metadata::{XdbcTypeInfo, XdbcTypeInfoDataBuilder};
// Create the list of metadata describing the server. Since this would not change at
// runtime, using once_cell::Lazy or similar patterns to constuct the list is a common approach.
let mut builder = XdbcTypeInfoDataBuilder::new();
builder.append(XdbcTypeInfo {
    type_name: "INTEGER".into(),
    data_type: XdbcDataType::XdbcInteger,
    column_size: Some(32),
    literal_prefix: None,
    literal_suffix: None,
    create_params: None,
    nullable: Nullable::NullabilityNullable,
    case_sensitive: false,
    searchable: Searchable::Full,
    unsigned_attribute: Some(false),
    fixed_prec_scale: false,
    auto_increment: Some(false),
    local_type_name: Some("INTEGER".into()),
    minimum_scale: None,
    maximum_scale: None,
    sql_data_type: XdbcDataType::XdbcInteger,
    datetime_subcode: None,
    num_prec_radix: Some(2),
    interval_precision: None,
});
let info_list = builder.build().unwrap();

// to access the underlying record batch
let batch = info_list.record_batch(None);
```

<a id="op-2798f94e11238722e5564aef"></a>
## append

`function` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder::append` · arrow-flight 59.3.0

```rust
fn append(&mut self, info: XdbcTypeInfo)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder", "path": "XdbcTypeInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [292, 2], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/xdbc_info.rs:167`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Append a new row

<a id="op-22e1c6c68ec264221be33614"></a>
## build

`function` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder::build` · arrow-flight 59.3.0

```rust
fn build(self) -> Result<XdbcTypeInfoData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder", "path": "XdbcTypeInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [292, 2], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/xdbc_info.rs:172`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create helper structure for handling xdbc metadata requests.

<a id="op-c66cd1bab7acbbc7f325d713"></a>
## default

`function` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder", "path": "XdbcTypeInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [158, 2], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/metadata/xdbc_info.rs:155`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cda025a1dc429591880382de"></a>
## new

`function` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder::new` · arrow-flight 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder", "path": "XdbcTypeInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [292, 2], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/xdbc_info.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new instance of [`XdbcTypeInfoDataBuilder`](../operations/arrow_flight.sql.metadata.xdbc_info.XdbcTypeInfoDataBuilder.md#op-0294db3bf2620399969ccd2c).

<a id="op-fab9603aa0b8611bc695f6a2"></a>
## schema

`function` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder::schema` · arrow-flight 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder", "path": "XdbcTypeInfoDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [292, 2], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/xdbc_info.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return the [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) for a GetSchema RPC call with [`CommandGetXdbcTypeInfo`](../operations/arrow_flight.sql.gen.CommandGetXdbcTypeInfo.md#op-9d0281cac1bf91cc4944a7dc)
