# `datafusion_catalog::default_table_source::DefaultTableSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.default_table_source.DefaultTableSource.json).

<a id="op-85fc77e4b8d486bbf7ed0f5d"></a>
## DefaultTableSource

`struct` · `datafusion_catalog::default_table_source::DefaultTableSource` · datafusion-catalog 55.1.0

```rust
struct DefaultTableSource
```

Source: `src/default_table_source.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Implements [`TableSource`](../operations/datafusion_expr.table_source.TableSource.md#op-d697a775a03008dbce355bf7) for a [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e)

This structure adapts a [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) (a physical plan trait) to the
[`TableSource`](../operations/datafusion_expr.table_source.TableSource.md#op-d697a775a03008dbce355bf7) (logical plan trait).

It is used so logical plans in the `datafusion_expr` crate do not have a
direct dependency on physical plans, such as [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e)s.

<a id="op-f9666c1133498ef465b5d472"></a>
## constraints

`function` · `datafusion_catalog::default_table_source::DefaultTableSource::constraints` · datafusion-catalog 55.1.0

```rust
fn constraints(&self) -> Option<&Constraints>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::default_table_source::DefaultTableSource", "path": "DefaultTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [80, 2], "filename": "src/default_table_source.rs"}, "trait": {"args": null, "id": "datafusion_expr::table_source::TableSource", "path": "TableSource"}, "trait_path": "datafusion_expr::table_source::TableSource"}`

Source: `src/default_table_source.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Get a reference to applicable constraints, if any exists.

<a id="op-c701af65abbe9a1367b91134"></a>
## get_column_default

`function` · `datafusion_catalog::default_table_source::DefaultTableSource::get_column_default` · datafusion-catalog 55.1.0

```rust
fn get_column_default(&self, column: &str) -> Option<&Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::default_table_source::DefaultTableSource", "path": "DefaultTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [80, 2], "filename": "src/default_table_source.rs"}, "trait": {"args": null, "id": "datafusion_expr::table_source::TableSource", "path": "TableSource"}, "trait_path": "datafusion_expr::table_source::TableSource"}`

Source: `src/default_table_source.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79b70757eefff14b5902064c"></a>
## get_logical_plan

`function` · `datafusion_catalog::default_table_source::DefaultTableSource::get_logical_plan` · datafusion-catalog 55.1.0

```rust
fn get_logical_plan(&self) -> Option<Cow<'_, datafusion_expr::LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::default_table_source::DefaultTableSource", "path": "DefaultTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [80, 2], "filename": "src/default_table_source.rs"}, "trait": {"args": null, "id": "datafusion_expr::table_source::TableSource", "path": "TableSource"}, "trait_path": "datafusion_expr::table_source::TableSource"}`

Source: `src/default_table_source.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ab55aaf7a6b6b4c5ca8260c"></a>
## new

`function` · `datafusion_catalog::default_table_source::DefaultTableSource::new` · datafusion-catalog 55.1.0

```rust
fn new(table_provider: Arc<dyn TableProvider>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::default_table_source::DefaultTableSource", "path": "DefaultTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [46, 2], "filename": "src/default_table_source.rs"}, "trait": null, "trait_path": null}`

Source: `src/default_table_source.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Create a new DefaultTableSource to wrap a TableProvider

<a id="op-55998013e33ef81b2ba42e24"></a>
## schema

`function` · `datafusion_catalog::default_table_source::DefaultTableSource::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::default_table_source::DefaultTableSource", "path": "DefaultTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [80, 2], "filename": "src/default_table_source.rs"}, "trait": {"args": null, "id": "datafusion_expr::table_source::TableSource", "path": "TableSource"}, "trait_path": "datafusion_expr::table_source::TableSource"}`

Source: `src/default_table_source.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Get a reference to the schema for this table

<a id="op-6e4c6b459ad265f8ebef36e0"></a>
## supports_filters_pushdown

`function` · `datafusion_catalog::default_table_source::DefaultTableSource::supports_filters_pushdown` · datafusion-catalog 55.1.0

```rust
fn supports_filters_pushdown(&self, filter: &[&Expr]) -> datafusion_common::Result<Vec<TableProviderFilterPushDown>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::default_table_source::DefaultTableSource", "path": "DefaultTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [80, 2], "filename": "src/default_table_source.rs"}, "trait": {"args": null, "id": "datafusion_expr::table_source::TableSource", "path": "TableSource"}, "trait_path": "datafusion_expr::table_source::TableSource"}`

Source: `src/default_table_source.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Tests whether the table provider can make use of any or all filter expressions
to optimize data retrieval.

<a id="op-91d64d303b36491c6c37922e"></a>
## table_provider

`struct_field` · `datafusion_catalog::default_table_source::DefaultTableSource::table_provider` · datafusion-catalog 55.1.0

```rust
table_provider: std::sync::Arc<dyn TableProvider>
```

Source: `src/default_table_source.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

table provider

<a id="op-4ed1082729ee25bea88e4314"></a>
## table_type

`function` · `datafusion_catalog::default_table_source::DefaultTableSource::table_type` · datafusion-catalog 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::default_table_source::DefaultTableSource", "path": "DefaultTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [80, 2], "filename": "src/default_table_source.rs"}, "trait": {"args": null, "id": "datafusion_expr::table_source::TableSource", "path": "TableSource"}, "trait_path": "datafusion_expr::table_source::TableSource"}`

Source: `src/default_table_source.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Get the type of this table for metadata/catalog purposes.
