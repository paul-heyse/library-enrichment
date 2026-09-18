# `buoyant_kernel::table_changes::scan`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.table_changes.scan.json`](../model/buoyant_kernel.table_changes.scan.json)

## TableChangesScan

`struct` · `buoyant_kernel::table_changes::scan::TableChangesScan`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.table_changes.scan.TableChangesScan.md)

Also reachable as `delta_kernel::table_changes::scan::TableChangesScan`

```rust
struct TableChangesScan
```

**Derives**: Debug

**Methods** (4)

```rust
fn execute(&self, engine: Arc<dyn Engine>) -> DeltaResult<impl Iterator<Item = DeltaResult<Box<dyn EngineData>>>>
fn logical_schema(&self) -> &SchemaRef
fn physical_schema(&self) -> &SchemaRef
fn table_root(&self) -> &Url
```

The result of building a [`TableChanges`] scan over a table. This can be used to get the change
data feed from the table.

---

## TableChangesScanBuilder

`struct` · `buoyant_kernel::table_changes::scan::TableChangesScanBuilder`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.table_changes.scan.TableChangesScanBuilder.md)

Also reachable as `delta_kernel::table_changes::scan::TableChangesScanBuilder`

```rust
struct TableChangesScanBuilder
```

**Derives**: Debug

**Methods** (4)

```rust
fn build(self) -> DeltaResult<TableChangesScan>
fn new(table_changes: impl Into<Arc<TableChanges>>) -> Self
fn with_predicate(self, predicate: impl Into<Option<PredicateRef>>) -> Self
fn with_schema(self, schema: impl Into<Option<SchemaRef>>) -> Self
```

This builder constructs a [`TableChangesScan`] that can be used to read the [`TableChanges`]
of a table. [`TableChangesScanBuilder`] allows you to specify a schema to project the columns
or specify a predicate to filter rows in the Change Data Feed. Predicates referencing the
Change Data Feed columns `_change_type`, `_commit_version`, and `_commit_timestamp` are
accepted but have no filtering effect, because those columns are synthesized during scan
execution rather than read from data files; apply such filters in connector code after the
scan returns. See issue [#525](https://github.com/delta-io/delta-kernel-rs/issues/525).

Note: There is a lot of shared functionality between [`TableChangesScanBuilder`] and
[`ScanBuilder`].

[`ScanBuilder`]: crate::scan::ScanBuilder
# Example
Construct a [`TableChangesScan`] from `table_changes` with a given schema and predicate
```rust
# use buoyant_kernel as delta_kernel;
# use std::sync::Arc;
# use delta_kernel::expressions::{column_expr, Scalar};
# use delta_kernel::Predicate;
# use delta_kernel::table_changes::TableChanges;
# let path = "./tests/data/table-with-cdf";
# let url = delta_kernel::try_parse_uri(path).unwrap();
# use test_utils::delta_kernel_default_engine::{storage::store_from_url, DefaultEngineBuilder};
# let engine = DefaultEngineBuilder::new(store_from_url(&url).unwrap()).build();
# let table_changes = TableChanges::try_new(url, &engine, 0, Some(1)).unwrap();
let schema = table_changes
    .schema()
    .project(&["id", "_commit_version"])
    .unwrap();
let predicate = Arc::new(Predicate::gt(column_expr!("id"), Scalar::from(10)));
let scan = table_changes
    .into_scan_builder()
    .with_schema(schema)
    .with_predicate(predicate.clone())
    .build();
```

---
