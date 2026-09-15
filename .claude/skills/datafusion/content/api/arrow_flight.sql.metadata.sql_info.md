# `arrow_flight::sql::metadata::sql_info`

Crate `arrow-flight` · 5 public items · structured records in [`model/arrow_flight.sql.metadata.sql_info.json`](../model/arrow_flight.sql.metadata.sql_info.json)

## SqlInfoValue

`enum` · `arrow_flight::sql::metadata::sql_info::SqlInfoValue`

```rust
enum SqlInfoValue
```

**Variants**: `String`, `Bool`, `BigInt`, `Bitmask`, `StringList`, `ListMap`

Represents a dynamic value

---

## GetSqlInfoBuilder

`struct` · `arrow_flight::sql::metadata::sql_info::GetSqlInfoBuilder`

```rust
struct GetSqlInfoBuilder<'a>
```

A builder for a [`CommandGetSqlInfo`] response.

---

## SqlInfoData

`struct` · `arrow_flight::sql::metadata::sql_info::SqlInfoData`

Also reachable as `arrow_flight::sql::metadata::SqlInfoData`

```rust
struct SqlInfoData
```

**Methods** (2)

```rust
fn record_batch(&self, info: impl IntoIterator<Item = u32>) -> Result<RecordBatch>
fn schema(&self) -> SchemaRef
```

A builder for [`SqlInfoData`] which is used to create [`CommandGetSqlInfo`] responses.

# Example
```
# use arrow_flight::sql::{metadata::SqlInfoDataBuilder, SqlInfo, SqlSupportedTransaction};
// Create the list of metadata describing the server
let mut builder = SqlInfoDataBuilder::new();
builder.append(SqlInfo::FlightSqlServerName, "server name");
    // ... add other SqlInfo here ..
builder.append(
    SqlInfo::FlightSqlServerTransaction,
    SqlSupportedTransaction::Transaction as i32,
);

// Create the batch to send back to the client
let info_data = builder.build().unwrap();
```

[protos]: https://github.com/apache/arrow/blob/6d3d2fca2c9693231fa1e52c142ceef563fc23f9/format/FlightSql.proto#L71-L820

---

## SqlInfoDataBuilder

`struct` · `arrow_flight::sql::metadata::sql_info::SqlInfoDataBuilder`

Also reachable as `arrow_flight::sql::metadata::SqlInfoDataBuilder`

```rust
struct SqlInfoDataBuilder
```

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn append(&mut self, name: impl SqlInfoName, value: impl Into<SqlInfoValue>)
fn build(self) -> Result<SqlInfoData>
fn new() -> Self
fn schema() -> &'static Schema
```

Helper to create [`CommandGetSqlInfo`] responses.

[`CommandGetSqlInfo`] are metadata requests used by a Flight SQL
server to communicate supported capabilities to Flight SQL clients.

Servers constuct - usually static - [`SqlInfoData`] via the [`SqlInfoDataBuilder`],
and build responses using [`CommandGetSqlInfo::into_builder`]

---

## SqlInfoName

`trait` · `arrow_flight::sql::metadata::sql_info::SqlInfoName`

```rust
trait SqlInfoName
```

**Methods** (1)

```rust
fn as_u32(&self) -> u32
```

Something that can be converted into u32 (the represenation of a [`SqlInfo`] name)

---
