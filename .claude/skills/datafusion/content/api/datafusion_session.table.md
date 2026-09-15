# `datafusion_session::table`

Crate `datafusion-session` · 7 public items · structured records in [`model/datafusion_session.table.json`](../model/datafusion_session.table.json)

## ScanArgs

`struct` · `datafusion_session::table::ScanArgs`

Also reachable as `datafusion_session::ScanArgs`

```rust
struct ScanArgs<'a>
```

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn filters(&self) -> Option<&'a [Expr]>
fn limit(&self) -> Option<usize>
fn projection(&self) -> Option<&'a [usize]>
fn statistics_requests(&self) -> &'a [StatisticsRequest]
fn with_filters(self, filters: Option<&'a [Expr]>) -> Self
fn with_limit(self, limit: Option<usize>) -> Self
fn with_projection(self, projection: Option<&'a [usize]>) -> Self
fn with_statistics_requests(self, statistics_requests: &'a [StatisticsRequest]) -> Self
```

Arguments for scanning a table with [`TableProvider::scan_with_args`].

---

## ScanResult

`struct` · `datafusion_session::table::ScanResult`

Also reachable as `datafusion_session::ScanResult`

```rust
struct ScanResult
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn into_inner(self) -> Arc<dyn ExecutionPlan>
fn new(plan: Arc<dyn ExecutionPlan>) -> Self
fn plan(&self) -> &Arc<dyn ExecutionPlan>
```

**via `core::convert::From`**

```rust
fn from(plan: Arc<dyn ExecutionPlan>) -> Self
```

Result of a table scan operation from [`TableProvider::scan_with_args`].

---

## TableFunction

`struct` · `datafusion_session::table::TableFunction`

Also reachable as `datafusion_session::TableFunction`

```rust
struct TableFunction
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn create_table_provider(&self, args: &[Expr]) -> Result<Arc<dyn TableProvider>>
fn create_table_provider_with_args(&self, args: TableFunctionArgs<'_, '_>) -> Result<Arc<dyn TableProvider>>
fn function(&self) -> &Arc<dyn TableFunctionImpl>
fn name(&self) -> &str
fn new(name: String, fun: Arc<dyn TableFunctionImpl>) -> Self
```

A table that uses a function to generate data

---

## TableFunctionArgs

`struct` · `datafusion_session::table::TableFunctionArgs`

Also reachable as `datafusion_session::TableFunctionArgs`

```rust
struct TableFunctionArgs<'e, 's>
```

**Methods** (3)

```rust
fn exprs(&self) -> &'e [Expr]
fn new(exprs: &'e [Expr], session: &'s dyn Session) -> Self
fn session(&self) -> &'s dyn Session
```

Describes arguments provided to the table function call.

---

## TableFunctionImpl

`trait` · `datafusion_session::table::TableFunctionImpl`

Also reachable as `datafusion_session::TableFunctionImpl`

```rust
trait TableFunctionImpl: Debug + Sync + Send + Any
```

**Implementors** (3)

- `datafusion_ffi::udtf::ForeignTableFunction`
- `datafusion_functions_table::generate_series::GenerateSeriesFunc`
- `datafusion_functions_table::generate_series::RangeFunc`

**Methods** (2)

```rust
fn call(&self, _exprs: &[Expr]) -> Result<Arc<dyn TableProvider>>
fn call_with_args(&self, args: TableFunctionArgs<'_, '_>) -> Result<Arc<dyn TableProvider>>
```

A trait for table function implementations

---

## TableProvider

`trait` · `datafusion_session::table::TableProvider`

Also reachable as `datafusion::datasource::TableProvider`, `datafusion_session::TableProvider`

```rust
trait TableProvider: Any + Debug + Sync + Send
```

**Implementors** (10)

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

**Methods** (15)

```rust
fn constraints(&self) -> Option<&Constraints>
async fn delete_from(&self, _state: &dyn Session, _filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
fn get_column_default(&self, _column: &str) -> Option<&Expr>
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
fn get_table_definition(&self) -> Option<&str>
async fn insert_into(&self, _state: &dyn Session, _input: Arc<dyn ExecutionPlan>, _insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
async fn merge_into(&self, _state: &dyn Session, _source: Arc<dyn ExecutionPlan>, _merge_schema: DFSchemaRef, _on: Expr, _clauses: Vec<MergeIntoClause>) -> Result<Arc<dyn ExecutionPlan>>
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
async fn scan_with_args<'a>(&self, state: &dyn Session, args: ScanArgs<'a>) -> Result<ScanResult>
fn schema(&self) -> SchemaRef
fn statistics(&self) -> Option<Statistics>
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
fn table_type(&self) -> TableType
async fn truncate(&self, _state: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
async fn update(&self, _state: &dyn Session, _assignments: Vec<(String, Expr)>, _filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
```

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

---

## TableProviderFactory

`trait` · `datafusion_session::table::TableProviderFactory`

Also reachable as `datafusion_session::TableProviderFactory`

```rust
trait TableProviderFactory: Debug + Sync + Send
```

**Implementors** (5)

- `datafusion::datasource::listing_table_factory::ListingTableFactory`
- `datafusion::datasource::provider::DefaultTableFactory`
- `datafusion::test_util::TestTableFactory`
- `datafusion_catalog::stream::StreamTableFactory`
- `datafusion_ffi::table_provider_factory::ForeignTableProviderFactory`

**Methods** (1)

```rust
async fn create(&self, state: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

A factory which creates [`TableProvider`]s at runtime given a URL.

For example, this can be used to create a table "on the fly"
from a directory of files only when that name is referenced.

---
