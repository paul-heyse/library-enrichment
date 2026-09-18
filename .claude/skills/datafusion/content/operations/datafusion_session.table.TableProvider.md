# `datafusion_session::table::TableProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.table.TableProvider.json).

<a id="op-76e5c2e5b081ebf294e9493e"></a>
## TableProvider

`trait` · `datafusion_session::table::TableProvider` · datafusion-session 55.1.0

```rust
trait TableProvider: Any + Debug + Sync + Send
```

Source: `src/table.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

A table which can be queried and modified.

Please see [`CatalogProvider`] for details of implementing a custom catalog.

[`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) represents a source of data which can provide data as
Apache Arrow [`RecordBatch`]es. Implementations of this trait provide
important information for planning such as:

1. [`Self::schema`](../operations/datafusion_session.table.TableProvider.md#op-61d424ca40b676444809c002): The schema (columns and their types) of the table
2. [`Self::supports_filters_pushdown`](../operations/datafusion_session.table.TableProvider.md#op-a929088704b5681a65528134): Should filters be pushed into this scan
2. [`Self::scan`](../operations/datafusion_session.table.TableProvider.md#op-116d00acf0401874b7f0d9f0): An [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) that can read data

[`RecordBatch`]: https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatch.html
[`CatalogProvider`]: super::CatalogProvider

<a id="op-6c55f7a62ce287403cbba30c"></a>
## constraints

`function` · `datafusion_session::table::TableProvider::constraints` · datafusion-session 55.1.0

```rust
fn constraints(&self) -> Option<&Constraints>
```

Source: `src/table.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get a reference to the constraints of the table.
Returns:
- `None` for tables that do not support constraints.
- `Some(&Constraints)` for tables supporting constraints.
Therefore, a `Some(&Constraints::empty())` return value indicates that
this table supports constraints, but there are no constraints.

<a id="op-426bf0e3302c16986dfbd3d2"></a>
## delete_from

`function` · `datafusion_session::table::TableProvider::delete_from` · datafusion-session 55.1.0

```rust
async fn delete_from(&self, _state: &dyn Session, _filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/table.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Delete rows matching the filter predicates.

Returns an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) producing a single row with `count` (UInt64).
Empty `filters` deletes all rows.

<a id="op-adcbe3de0762cb1b8d4854e2"></a>
## get_column_default

`function` · `datafusion_session::table::TableProvider::get_column_default` · datafusion-session 55.1.0

```rust
fn get_column_default(&self, _column: &str) -> Option<&Expr>
```

Source: `src/table.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the default value for a column, if available.

<a id="op-49264030dc5aa7fd163dbbb0"></a>
## get_logical_plan

`function` · `datafusion_session::table::TableProvider::get_logical_plan` · datafusion-session 55.1.0

```rust
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
```

Source: `src/table.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) of this table, if available.

<a id="op-5b228609d3e68c89a98303ea"></a>
## get_table_definition

`function` · `datafusion_session::table::TableProvider::get_table_definition` · datafusion-session 55.1.0

```rust
fn get_table_definition(&self) -> Option<&str>
```

Source: `src/table.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the create statement used to create this table, if available.

<a id="op-f6f850e68c1fe795e5e5fac4"></a>
## insert_into

`function` · `datafusion_session::table::TableProvider::insert_into` · datafusion-session 55.1.0

```rust
async fn insert_into(&self, _state: &dyn Session, _input: Arc<dyn ExecutionPlan>, _insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/table.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) to insert data into this table, if
supported.

The returned plan should return a single row in a UInt64
column called "count" such as the following

```text
+-------+,
| count |,
+-------+,
| 6     |,
+-------+,
```

# See Also

See [`DataSinkExec`] for the common pattern of inserting a
streams of `RecordBatch`es as files to an ObjectStore.

[`DataSinkExec`]: https://docs.rs/datafusion-datasource/latest/datafusion_datasource/sink/struct.DataSinkExec.html

<a id="op-1aca28f9d347f049654ea55b"></a>
## merge_into

`function` · `datafusion_session::table::TableProvider::merge_into` · datafusion-session 55.1.0

```rust
async fn merge_into(&self, _state: &dyn Session, _source: Arc<dyn ExecutionPlan>, _merge_schema: DFSchemaRef, _on: Expr, _clauses: Vec<MergeIntoClause>) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/table.rs:394`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Merge rows from a source into this table.

The `source` is an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) representing the USING clause.
The `merge_schema` contains the target columns followed by the source
columns, preserving their logical qualifiers. Providers can use this
schema to resolve the logical expressions against the combined rows
they construct while executing the merge.
The `on` condition is the join predicate from the ON clause.
The `clauses` describe the WHEN MATCHED / WHEN NOT MATCHED actions.

Returns an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) producing a single row with `count` (UInt64).

<a id="op-116d00acf0401874b7f0d9f0"></a>
## scan

`function` · `datafusion_session::table::TableProvider::scan` · datafusion-session 55.1.0

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/table.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) for scanning the table with optional
`projection`, `filter`, and `limit`, described below.

The returned `ExecutionPlan` is responsible for scanning the datasource's
partitions in a streaming, parallelized fashion.

# Projection

If specified, only a subset of columns should be returned, in the order
specified. The projection is a set of indexes of the fields in
[`Self::schema`](../operations/datafusion_session.table.TableProvider.md#op-61d424ca40b676444809c002).

DataFusion provides the projection so the scan reads only the columns
actually used in the query, an optimization called "Projection
Pushdown". Some datasources, such as Parquet, can use this information
to go significantly faster when only a subset of columns is required.

# Filters

A list of boolean filter [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)s to evaluate *during* the scan, in the
manner specified by [`Self::supports_filters_pushdown`](../operations/datafusion_session.table.TableProvider.md#op-a929088704b5681a65528134). Only rows for
which *all* of the `Expr`s evaluate to `true` must be returned (that is,
the expressions are `AND`ed together).

To enable filter pushdown, override
[`Self::supports_filters_pushdown`](../operations/datafusion_session.table.TableProvider.md#op-a929088704b5681a65528134). The default implementation does not
push down filters, and `filters` will be empty.

DataFusion pushes filters into scans whenever possible ("Filter
Pushdown"). Depending on the data format and implementation, evaluating
predicates during the scan can significantly improve performance.

## Note: Some columns may appear *only* in Filters

In some cases, a query may use a column only in a filter and the
projection will not contain all columns referenced by the filter
expressions.

For example, given the query `SELECT t.a FROM t WHERE t.b > 5`,

```text
┌────────────────────┐
│  Projection(t.a)   │
└────────────────────┘
           ▲
           │
           │
┌────────────────────┐     Filter     ┌────────────────────┐   Projection    ┌────────────────────┐
│  Filter(t.b > 5)   │────Pushdown──▶ │  Projection(t.a)   │ ───Pushdown───▶ │  Projection(t.a)   │
└────────────────────┘                └────────────────────┘                 └────────────────────┘
           ▲                                     ▲                                      ▲
           │                                     │                                      │
           │                                     │                           ┌────────────────────┐
┌────────────────────┐                ┌────────────────────┐                 │        Scan        │
│        Scan        │                │        Scan        │                 │  filter=(t.b > 5)  │
└────────────────────┘                │  filter=(t.b > 5)  │                 │  projection=(t.a)  │
                                      └────────────────────┘                 └────────────────────┘

Initial Plan                  If `TableProviderFilterPushDown`           Projection pushdown notes that
                              returns true, filter pushdown              the scan only needs t.a
                              pushes the filter into the scan
                                                                         BUT internally evaluating the
                                                                         predicate still requires t.b
```

# Limit

If `limit` is specified, the scan must produce *at least* this many
rows, though it may return more. Like Projection Pushdown and Filter
Pushdown, DataFusion pushes `LIMIT`s as far down in the plan as
possible. This is called "Limit Pushdown", and some sources can use the
information to improve performance.

Note: If any pushed-down filters are `Inexact`, the `LIMIT` cannot be
pushed down. Inexact filters do not guarantee that every filtered row is
removed, so applying the limit could leave too few rows to return in the
final result.

# Evaluation Order

The logical evaluation order is `filters`, then `limit`, then
`projection`.

Note that `limit` applies to the filtered result, not to the unfiltered
input, and `projection` affects only which columns are returned, not
which rows qualify.

For example, if a scan receives:

- `projection = [a]`
- `filters = [b > 5]`
- `limit = Some(3)`

It must logically produce results equivalent to:

```text
PROJECTION a (LIMIT 3 (SCAN WHERE b > 5))
```

As noted above, columns referenced only by pushed-down filters may be
absent from `projection`.

<a id="op-274938cecb923835571bdb77"></a>
## scan_with_args

`function` · `datafusion_session::table::TableProvider::scan_with_args` · datafusion-session 55.1.0

```rust
async fn scan_with_args<'a>(&self, state: &dyn Session, args: ScanArgs<'a>) -> Result<ScanResult>
```

Source: `src/table.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) for scanning the table using structured arguments.

This method uses [`ScanArgs`](../operations/datafusion_session.table.ScanArgs.md#op-4b7e85b7401a64fe31c9b2cc) to pass scan parameters in a structured way
and returns a [`ScanResult`](../operations/datafusion_session.table.ScanResult.md#op-c55b4a2c2cef2e5d170d09c4) containing the execution plan.

Table providers can override this method to take advantage of additional
parameters like the upcoming `preferred_ordering` that may not be available through
other scan methods.

# Arguments
* `state` - The session state containing configuration and context
* `args` - Structured scan arguments including projection, filters, limit, and ordering preferences

# Returns
A [`ScanResult`](../operations/datafusion_session.table.ScanResult.md#op-c55b4a2c2cef2e5d170d09c4) containing the [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) for scanning the table

See [`Self::scan`](../operations/datafusion_session.table.TableProvider.md#op-116d00acf0401874b7f0d9f0) for detailed documentation about projection, filters, and limits.

<a id="op-61d424ca40b676444809c002"></a>
## schema

`function` · `datafusion_session::table::TableProvider::schema` · datafusion-session 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Source: `src/table.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get a reference to the schema for this table

<a id="op-baa45f22e70e3ade58373306"></a>
## statistics

`function` · `datafusion_session::table::TableProvider::statistics` · datafusion-session 55.1.0

```rust
fn statistics(&self) -> Option<Statistics>
```

Source: `src/table.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get statistics for this table, if available
Although not presently used in mainline DataFusion, this allows implementation specific
behavior for downstream repositories, in conjunction with specialized optimizer rules to
perform operations such as re-ordering of joins.

<a id="op-a929088704b5681a65528134"></a>
## supports_filters_pushdown

`function` · `datafusion_session::table::TableProvider::supports_filters_pushdown` · datafusion-session 55.1.0

```rust
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
```

Source: `src/table.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Specify if DataFusion should provide filter expressions to the
TableProvider to apply *during* the scan.

Some TableProviders can evaluate filters more efficiently than the
`Filter` operator in DataFusion, for example by using an index.

# Parameters and Return Value

The return `Vec` must have one element for each element of the `filters`
argument. The value of each element indicates if the TableProvider can
apply the corresponding filter during the scan. The position in the return
value corresponds to the expression in the `filters` parameter.

If the length of the resulting `Vec` does not match the `filters` input
an error will be thrown.

Each element in the resulting `Vec` is one of the following:
* [`Exact`] or [`Inexact`]: The TableProvider can apply the filter
during scan
* [`Unsupported`]: The TableProvider cannot apply the filter during scan

By default, this function returns [`Unsupported`] for all filters,
meaning no filters will be provided to [`Self::scan`](../operations/datafusion_session.table.TableProvider.md#op-116d00acf0401874b7f0d9f0).

[`Unsupported`]: TableProviderFilterPushDown::Unsupported
[`Exact`]: TableProviderFilterPushDown::Exact
[`Inexact`]: TableProviderFilterPushDown::Inexact
# Example

```rust
# use std::any::Any;
# use std::sync::Arc;
# use arrow_schema::SchemaRef;
# use async_trait::async_trait;
# use datafusion_session::{TableProvider, Session};
# use datafusion_common::Result;
# use datafusion_expr::{Expr, TableProviderFilterPushDown, TableType};
# use datafusion_physical_plan::ExecutionPlan;
// Define a struct that implements the TableProvider trait
#[derive(Debug)]
struct TestDataSource {}

#[async_trait]
impl TableProvider for TestDataSource {
# fn schema(&self) -> SchemaRef { todo!() }
# fn table_type(&self) -> TableType { todo!() }
# async fn scan(&self, s: &dyn Session, p: Option<&Vec<usize>>, f: &[Expr], l: Option<usize>) -> Result<Arc<dyn ExecutionPlan>> {
        todo!()
# }
    // Override the supports_filters_pushdown to evaluate which expressions
    // to accept as pushdown predicates.
    fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>> {
        // Process each filter
        let support: Vec<_> = filters.iter().map(|expr| {
          match expr {
            // This example only supports a between expr with a single column named "c1".
            Expr::Between(between_expr) => {
                between_expr.expr
                .try_as_col()
                .map(|column| {
                    if column.name == "c1" {
                        TableProviderFilterPushDown::Exact
                    } else {
                        TableProviderFilterPushDown::Unsupported
                    }
                })
                // If there is no column in the expr set the filter to unsupported.
                .unwrap_or(TableProviderFilterPushDown::Unsupported)
            }
            _ => {
                // For all other cases return Unsupported.
                TableProviderFilterPushDown::Unsupported
            }
        }
    }).collect();
    Ok(support)
    }
}
```

<a id="op-9303e6c8500fc22ca01f4955"></a>
## table_type

`function` · `datafusion_session::table::TableProvider::table_type` · datafusion-session 55.1.0

```rust
fn table_type(&self) -> TableType
```

Source: `src/table.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get the type of this table for metadata/catalog purposes.

<a id="op-8967f4e4aed2039e1902ea0b"></a>
## truncate

`function` · `datafusion_session::table::TableProvider::truncate` · datafusion-session 55.1.0

```rust
async fn truncate(&self, _state: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/table.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Remove all rows from the table.

Should return an [ExecutionPlan](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) producing a single row with count (UInt64),
representing the number of rows removed.

<a id="op-830e4960883b924b13957170"></a>
## update

`function` · `datafusion_session::table::TableProvider::update` · datafusion-session 55.1.0

```rust
async fn update(&self, _state: &dyn Session, _assignments: Vec<(String, Expr)>, _filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/table.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Update rows matching the filter predicates.

Returns an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) producing a single row with `count` (UInt64).
Empty `filters` updates all rows.
