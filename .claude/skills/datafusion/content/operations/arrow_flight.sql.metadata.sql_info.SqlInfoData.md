# `arrow_flight::sql::metadata::sql_info::SqlInfoData`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.metadata.sql_info.SqlInfoData.json).

<a id="op-2622035108d77db5eead8631"></a>
## SqlInfoData

`struct` · `arrow_flight::sql::metadata::sql_info::SqlInfoData` · arrow-flight 59.3.0

```rust
struct SqlInfoData
```

Source: `src/sql/metadata/sql_info.rs:402`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A builder for [`SqlInfoData`](../operations/arrow_flight.sql.metadata.sql_info.SqlInfoData.md#op-2622035108d77db5eead8631) which is used to create [`CommandGetSqlInfo`](../operations/arrow_flight.sql.gen.CommandGetSqlInfo.md#op-5df287408ae3ccad4071070f) responses.

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

<a id="op-cd528608d2cbd1d360a7045c"></a>
## record_batch

`function` · `arrow_flight::sql::metadata::sql_info::SqlInfoData::record_batch` · arrow-flight 59.3.0

```rust
fn record_batch(&self, info: impl IntoIterator<Item = u32>) -> Result<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::sql_info::SqlInfoData", "path": "SqlInfoData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [433, 2], "filename": "src/sql/metadata/sql_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/sql_info.rs:409`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return a  [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) containing only the requested `u32`, if any
from [`CommandGetSqlInfo`](../operations/arrow_flight.sql.gen.CommandGetSqlInfo.md#op-5df287408ae3ccad4071070f)

<a id="op-6d06b5e871a797ae1e403999"></a>
## schema

`function` · `arrow_flight::sql::metadata::sql_info::SqlInfoData::schema` · arrow-flight 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::sql_info::SqlInfoData", "path": "SqlInfoData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [433, 2], "filename": "src/sql/metadata/sql_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/sql_info.rs:430`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return the schema of the RecordBatch that will be returned
from [`CommandGetSqlInfo`](../operations/arrow_flight.sql.gen.CommandGetSqlInfo.md#op-5df287408ae3ccad4071070f)
