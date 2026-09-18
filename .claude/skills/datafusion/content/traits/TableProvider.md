# TableProvider

`datafusion_session::table::TableProvider`

```rust
trait TableProvider: Any + Debug + Sync + Send
```

Also reachable as `datafusion::datasource::TableProvider`, `datafusion_session::TableProvider`

Prose: [`api/datafusion_session.table.md`](../api/datafusion_session.table.md#tableprovider) · records: [`model/datafusion_session.table.json`](../model/datafusion_session.table.json)

## Required

Every implementation must supply these.

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn table_type(&self) -> TableType
```

## Provided

These methods have defaults. Read each full contract before overriding: some defaults reject unsupported operations, while others provide suitable general behavior. Required methods alone do not prove correctness or performance.

```rust
fn constraints(&self) -> Option<&Constraints>
async fn delete_from(&self, _state: &dyn Session, _filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
fn get_column_default(&self, _column: &str) -> Option<&Expr>
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
fn get_table_definition(&self) -> Option<&str>
async fn insert_into(&self, _state: &dyn Session, _input: Arc<dyn ExecutionPlan>, _insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
async fn merge_into(&self, _state: &dyn Session, _source: Arc<dyn ExecutionPlan>, _merge_schema: DFSchemaRef, _on: Expr, _clauses: Vec<MergeIntoClause>) -> Result<Arc<dyn ExecutionPlan>>
async fn scan_with_args<'a>(&self, state: &dyn Session, args: ScanArgs<'a>) -> Result<ScanResult>
fn statistics(&self) -> Option<Statistics>
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
async fn truncate(&self, _state: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
async fn update(&self, _state: &dyn Session, _assignments: Vec<(String, Expr)>, _filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
```

## Implementors (10)

Read one before writing your own.

- `datafusion::test_util::TestTableProvider`
- `datafusion_catalog::cte_worktable::CteWorkTable`
- `datafusion_catalog::empty::EmptyTable`
- `datafusion_catalog::memory::table::MemTable`
- `datafusion_catalog::stream::StreamTable`
- `datafusion_catalog::streaming::StreamingTable`
- `datafusion_catalog::view::ViewTable`
- `datafusion_catalog_listing::table::ListingTable`
- `datafusion_ffi::table_provider::ForeignTableProvider`
- `datafusion_functions_table::generate_series::GenerateSeriesTable`

## Demonstrated by 7 upstream example(s)

- [`corpus/examples/custom_data_source/custom_datasource.rs`](../corpus/examples/custom_data_source/custom_datasource.rs)
- [`corpus/examples/custom_data_source/default_column_values.rs`](../corpus/examples/custom_data_source/default_column_values.rs)
- [`corpus/examples/data_io/parquet_advanced_index.rs`](../corpus/examples/data_io/parquet_advanced_index.rs)
- [`corpus/examples/data_io/parquet_embedded_index.rs`](../corpus/examples/data_io/parquet_embedded_index.rs)
- [`corpus/examples/data_io/parquet_index.rs`](../corpus/examples/data_io/parquet_index.rs)
- [`corpus/examples/data_io/remote_catalog.rs`](../corpus/examples/data_io/remote_catalog.rs)
- [`corpus/examples/udf/simple_udtf.rs`](../corpus/examples/udf/simple_udtf.rs)

## Documentation

A table which can be queried and modified.

Please see [`CatalogProvider`] for details of implementing a custom catalog.

[`TableProvider`] represents a source of data which can provide data as
Apache Arrow [`RecordBatch`]es. Implementations of this trait provide
important information for planning such as:

1. [`Self::schema`]: The schema (columns and their types) of the table
2. [`Self::supports_filters_pushdown`]: Should filters be pushed into this scan
2. [`Self::scan`]: An [`ExecutionPlan`] that can read data

[`RecordBatch`]: https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatch.html
[`CatalogProvider`]: super::CatalogProvider
