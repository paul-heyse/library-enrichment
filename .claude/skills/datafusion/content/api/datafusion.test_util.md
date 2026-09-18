# `datafusion::test_util`

Crate `datafusion` · 13 public items · structured records in [`model/datafusion.test_util.json`](../model/datafusion.test_util.json)

## aggr_test_schema

`function` · `datafusion::test_util::aggr_test_schema`

```rust
fn aggr_test_schema() -> arrow::datatypes::SchemaRef
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.aggr_test_schema.md).


Get the schema for the aggregate_test_* csv files

---

## bounded_stream

`function` · `datafusion::test_util::bounded_stream`

```rust
fn bounded_stream(record_batch: arrow::record_batch::RecordBatch, limit: usize) -> execution::SendableRecordBatchStream
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.bounded_stream.md).


Creates a bounded stream that emits the same record batch a specified number of times.
This is useful for testing purposes.

---

## plan_and_collect

`function` · `datafusion::test_util::plan_and_collect`

```rust
async fn plan_and_collect(ctx: &prelude::SessionContext, sql: &str) -> error::Result<Vec<arrow::record_batch::RecordBatch>>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.plan_and_collect.md).


Execute SQL and return results

---

## populate_csv_partitions

`function` · `datafusion::test_util::populate_csv_partitions`

```rust
fn populate_csv_partitions(tmp_dir: &tempfile::TempDir, partition_count: usize, file_extension: &str) -> error::Result<arrow::datatypes::SchemaRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.populate_csv_partitions.md).


Generate CSV partitions within the supplied directory

---

## register_aggregate_csv

`function` · `datafusion::test_util::register_aggregate_csv`

```rust
async fn register_aggregate_csv(ctx: &prelude::SessionContext, table_name: &str) -> error::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.register_aggregate_csv.md).


Register session context for the aggregate_test_100.csv file

---

## register_unbounded_file_with_ordering

`function` · `datafusion::test_util::register_unbounded_file_with_ordering`

```rust
fn register_unbounded_file_with_ordering(ctx: &prelude::SessionContext, schema: arrow::datatypes::SchemaRef, file_path: &std::path::Path, table_name: &str, file_sort_order: Vec<Vec<datafusion_expr::SortExpr>>) -> error::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.register_unbounded_file_with_ordering.md).


This function creates an unbounded sorted file for testing purposes.

---

## scan_empty

`function` · `datafusion::test_util::scan_empty`

```rust
fn scan_empty(name: Option<&str>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>) -> error::Result<logical_expr::LogicalPlanBuilder>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.scan_empty.md).


Scan an empty data source, mainly used in tests

---

## scan_empty_with_partitions

`function` · `datafusion::test_util::scan_empty_with_partitions`

```rust
fn scan_empty_with_partitions(name: Option<&str>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>, partitions: usize) -> error::Result<logical_expr::LogicalPlanBuilder>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.scan_empty_with_partitions.md).


Scan an empty data source with configured partition, mainly used in tests.

---

## test_table

`function` · `datafusion::test_util::test_table`

```rust
async fn test_table() -> error::Result<dataframe::DataFrame>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.test_table.md).


Create a table from the aggregate_test_100.csv file with the name "aggregate_test_100"

---

## test_table_with_cache_factory

`function` · `datafusion::test_util::test_table_with_cache_factory`

```rust
async fn test_table_with_cache_factory() -> error::Result<dataframe::DataFrame>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.test_table_with_cache_factory.md).


Create a test table registered to a session context with an associated cache factory

---

## test_table_with_name

`function` · `datafusion::test_util::test_table_with_name`

```rust
async fn test_table_with_name(name: &str) -> error::Result<dataframe::DataFrame>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.test_table_with_name.md).


Create a table from the aggregate_test_100.csv file with the specified name

---

## TestTableFactory

`struct` · `datafusion::test_util::TestTableFactory`

```rust
struct TestTableFactory
```

**Implements**: `datafusion_session::table::TableProviderFactory`

**Derives**: Debug, Default

**via `datafusion_session::table::TableProviderFactory`**

```rust
async fn create(&self, _: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.TestTableFactory.md).


TableFactory for tests

---

## TestTableProvider

`struct` · `datafusion::test_util::TestTableProvider`

```rust
struct TestTableProvider
```

**Fields**: `url`, `schema`

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Debug

**via `datafusion_session::table::TableProvider`**

```rust
async fn scan(&self, _state: &dyn Session, _projection: Option<&Vec<usize>>, _filters: &[Expr], _limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn table_type(&self) -> TableType
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.TestTableProvider.md).


TableProvider for testing purposes

---
