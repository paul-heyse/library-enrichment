# `arrow_flight::sql::metadata::xdbc_info`

Crate `arrow-flight` · 4 public items · structured records in [`model/arrow_flight.sql.metadata.xdbc_info.json`](../model/arrow_flight.sql.metadata.xdbc_info.json)

## GetXdbcTypeInfoBuilder

`struct` · `arrow_flight::sql::metadata::xdbc_info::GetXdbcTypeInfoBuilder`

```rust
struct GetXdbcTypeInfoBuilder<'a>
```

A builder for a [`CommandGetXdbcTypeInfo`] response.

---

## XdbcTypeInfo

`struct` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo`

Also reachable as `arrow_flight::sql::metadata::XdbcTypeInfo`

```rust
struct XdbcTypeInfo
```

**Fields**: `type_name`, `data_type`, `column_size`, `literal_prefix`, `literal_suffix`, `create_params`, `nullable`, `case_sensitive`, `searchable`, `unsigned_attribute`, `fixed_prec_scale`, `auto_increment`, `local_type_name`, `minimum_scale`, `maximum_scale`, `sql_data_type`, `datetime_subcode`, `num_prec_radix`, `interval_precision`

**Derives**: Clone, Debug, Default

Data structure representing type information for xdbc types.

---

## XdbcTypeInfoData

`struct` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoData`

Also reachable as `arrow_flight::sql::metadata::XdbcTypeInfoData`

```rust
struct XdbcTypeInfoData
```

**Methods** (2)

```rust
fn record_batch(&self, data_type: impl Into<Option<i32>>) -> Result<RecordBatch>
fn schema(&self) -> SchemaRef
```

Helper to create [`CommandGetXdbcTypeInfo`] responses.

[`CommandGetXdbcTypeInfo`] are metadata requests used by a Flight SQL
server to communicate supported capabilities to Flight SQL clients.

Servers constuct - usually static - [`XdbcTypeInfoData`] via the [`XdbcTypeInfoDataBuilder`],
and build responses using [`CommandGetXdbcTypeInfo::into_builder`].

---

## XdbcTypeInfoDataBuilder

`struct` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfoDataBuilder`

Also reachable as `arrow_flight::sql::metadata::XdbcTypeInfoDataBuilder`

```rust
struct XdbcTypeInfoDataBuilder
```

**Derives**: Default

**Methods** (4)

```rust
fn append(&mut self, info: XdbcTypeInfo)
fn build(self) -> Result<XdbcTypeInfoData>
fn new() -> Self
fn schema(&self) -> SchemaRef
```

A builder for [`XdbcTypeInfoData`] which is used to create [`CommandGetXdbcTypeInfo`] responses.

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

---
