# `buoyant_kernel::table_changes::scan::TableChangesScanBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_changes.scan.TableChangesScanBuilder.json).

<a id="op-4e86551bf6a1e8510c810267"></a>
## TableChangesScanBuilder

`struct` · `buoyant_kernel::table_changes::scan::TableChangesScanBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TableChangesScanBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L68).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This builder constructs a [`TableChangesScan`](../operations/buoyant_kernel.table_changes.scan.TableChangesScan.md#op-298dbadcbcd42cc5a5dc83e9) that can be used to read the [`TableChanges`](../operations/buoyant_kernel.table_changes.TableChanges.md#op-9b64c4444ea3b8dc26e08d7e)
of a table. [`TableChangesScanBuilder`](../operations/buoyant_kernel.table_changes.scan.TableChangesScanBuilder.md#op-4e86551bf6a1e8510c810267) allows you to specify a schema to project the columns
or specify a predicate to filter rows in the Change Data Feed. Predicates referencing the
Change Data Feed columns `_change_type`, `_commit_version`, and `_commit_timestamp` are
accepted but have no filtering effect, because those columns are synthesized during scan
execution rather than read from data files; apply such filters in connector code after the
scan returns. See issue [#525](https://github.com/delta-io/delta-kernel-rs/issues/525).

Note: There is a lot of shared functionality between [`TableChangesScanBuilder`](../operations/buoyant_kernel.table_changes.scan.TableChangesScanBuilder.md#op-4e86551bf6a1e8510c810267) and
[`ScanBuilder`].

[`ScanBuilder`]: crate::scan::ScanBuilder
# Example
Construct a [`TableChangesScan`](../operations/buoyant_kernel.table_changes.scan.TableChangesScan.md#op-298dbadcbcd42cc5a5dc83e9) from `table_changes` with a given schema and predicate
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

<a id="op-4e12b022d391de6058910b59"></a>
## build

`function` · `buoyant_kernel::table_changes::scan::TableChangesScanBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> DeltaResult<TableChangesScan>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L112).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::scan::TableChangesScanBuilder", "path": "TableChangesScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [137, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:112`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Build the [`TableChangesScan`](../operations/buoyant_kernel.table_changes.scan.TableChangesScan.md#op-298dbadcbcd42cc5a5dc83e9).

This does not scan the table at this point, but does do some work to ensure that the
provided schema make sense, and to prepare some metadata that the scan will need.  The
[`TableChangesScan`](../operations/buoyant_kernel.table_changes.scan.TableChangesScan.md#op-298dbadcbcd42cc5a5dc83e9) type itself can be used to fetch the files and associated metadata
required to perform actual data reads.

<a id="op-096c1cd96db2fb1e059f68ab"></a>
## fmt

`function` · `buoyant_kernel::table_changes::scan::TableChangesScanBuilder::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L67).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::scan::TableChangesScanBuilder", "path": "TableChangesScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:67`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3b6073f6d8f384cf7aeec31"></a>
## new

`function` · `buoyant_kernel::table_changes::scan::TableChangesScanBuilder::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(table_changes: impl Into<Arc<TableChanges>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L76).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::scan::TableChangesScanBuilder", "path": "TableChangesScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [137, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new [`TableChangesScanBuilder`](../operations/buoyant_kernel.table_changes.scan.TableChangesScanBuilder.md#op-4e86551bf6a1e8510c810267) instance.

<a id="op-36cc5a4657f429b2baf1790a"></a>
## with_predicate

`function` · `buoyant_kernel::table_changes::scan::TableChangesScanBuilder::with_predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_predicate(self, predicate: impl Into<Option<PredicateRef>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::scan::TableChangesScanBuilder", "path": "TableChangesScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [137, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:101`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Optionally provide an expression to filter rows. For example, using the predicate `x <
4` to return a subset of the rows in the scan which satisfy the filter. If `predicate_opt`
is `None`, this is a no-op.

NOTE: The filtering is best-effort and can produce false positives (rows that should should
have been filtered out but were kept).

<a id="op-d4e78da5961e510d6ef7b94e"></a>
## with_schema

`function` · `buoyant_kernel::table_changes::scan::TableChangesScanBuilder::with_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_schema(self, schema: impl Into<Option<SchemaRef>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L90).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::scan::TableChangesScanBuilder", "path": "TableChangesScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [137, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:90`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Provide [`Schema`] for columns to select from the [`TableChanges`](../operations/buoyant_kernel.table_changes.TableChanges.md#op-9b64c4444ea3b8dc26e08d7e).

A table with columns `[a, b, c]` could have a scan which reads only the first
two columns by using the schema `[a, b]`.

[`Schema`]: crate::schema::Schema

<a id="op-755a1a43fef770e4573f1b71"></a>
## predicate

`struct_field` · `buoyant_kernel::table_changes::scan::TableChangesScanBuilder::predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<PredicateRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L71).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:71`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52440cf0e35192be15b601cf"></a>
## schema

`struct_field` · `buoyant_kernel::table_changes::scan::TableChangesScanBuilder::schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
schema: Option<schema::SchemaRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L70).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2cbce458431c6c55ce96781"></a>
## table_changes

`struct_field` · `buoyant_kernel::table_changes::scan::TableChangesScanBuilder::table_changes` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_changes: std::sync::Arc<super::TableChanges>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L69).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:69`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
