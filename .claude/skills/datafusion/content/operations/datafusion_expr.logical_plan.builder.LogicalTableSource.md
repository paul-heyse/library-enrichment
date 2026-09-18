# `datafusion_expr::logical_plan::builder::LogicalTableSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.LogicalTableSource.json).

<a id="op-7a4cbfc4eec8fef6084ae52a"></a>
## LogicalTableSource

`struct` · `datafusion_expr::logical_plan::builder::LogicalTableSource` · datafusion-expr 55.1.0

```rust
struct LogicalTableSource
```

Source: `src/logical_plan/builder.rs:2210`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Basic TableSource implementation intended for use in tests and documentation. It is expected
that users will provide their own TableSource implementations or use DataFusion's
DefaultTableSource.

<a id="op-c216205fd945d41c3893dc76"></a>
## constraints

`function` · `datafusion_expr::logical_plan::builder::LogicalTableSource::constraints` · datafusion-expr 55.1.0

```rust
fn constraints(&self) -> Option<&Constraints>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalTableSource", "path": "LogicalTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2230, 1], "end": [2245, 2], "filename": "src/logical_plan/builder.rs"}, "trait": {"args": null, "id": "datafusion_expr::table_source::TableSource", "path": "TableSource"}, "trait_path": "datafusion_expr::table_source::TableSource"}`

Source: `src/logical_plan/builder.rs:2235`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08a32bde19fab4c243f63ac5"></a>
## new

`function` · `datafusion_expr::logical_plan::builder::LogicalTableSource::new` · datafusion-expr 55.1.0

```rust
fn new(table_schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalTableSource", "path": "LogicalTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2215, 1], "end": [2228, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:2217`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new LogicalTableSource

<a id="op-b1ecdd6e1fa0b43b5122c537"></a>
## schema

`function` · `datafusion_expr::logical_plan::builder::LogicalTableSource::schema` · datafusion-expr 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalTableSource", "path": "LogicalTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2230, 1], "end": [2245, 2], "filename": "src/logical_plan/builder.rs"}, "trait": {"args": null, "id": "datafusion_expr::table_source::TableSource", "path": "TableSource"}, "trait_path": "datafusion_expr::table_source::TableSource"}`

Source: `src/logical_plan/builder.rs:2231`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f466c7c107ab345aa5608d4d"></a>
## supports_filters_pushdown

`function` · `datafusion_expr::logical_plan::builder::LogicalTableSource::supports_filters_pushdown` · datafusion-expr 55.1.0

```rust
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalTableSource", "path": "LogicalTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2230, 1], "end": [2245, 2], "filename": "src/logical_plan/builder.rs"}, "trait": {"args": null, "id": "datafusion_expr::table_source::TableSource", "path": "TableSource"}, "trait_path": "datafusion_expr::table_source::TableSource"}`

Source: `src/logical_plan/builder.rs:2239`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d41f90307bb63d155f2bbb17"></a>
## with_constraints

`function` · `datafusion_expr::logical_plan::builder::LogicalTableSource::with_constraints` · datafusion-expr 55.1.0

```rust
fn with_constraints(self, constraints: Constraints) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalTableSource", "path": "LogicalTableSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2215, 1], "end": [2228, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:2224`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
