# `datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.column_rewriter.PhysicalColumnRewriter.json).

<a id="op-6ebc436eff71d1fa20fc542c"></a>
## PhysicalColumnRewriter

`struct` · `datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter` · datafusion-physical-plan 55.1.0

```rust
struct PhysicalColumnRewriter<'a>
```

Source: `src/column_rewriter.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Rewrite column references in a physical expr according to a mapping.

This rewriter traverses the expression tree and replaces [`Column`](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28) nodes
with the corresponding expression found in the `column_map`.

If a column is found in the map, it is replaced by the mapped expression.
If a column is NOT found in the map, a `DataFusionError::Internal` is
returned.

<a id="op-e2ad743deca2912199771d52"></a>
## Node

`assoc_type` · `datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter::Node` · datafusion-physical-plan 55.1.0

```rust
Node
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter", "path": "PhysicalColumnRewriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [71, 2], "filename": "src/column_rewriter.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRewriter", "path": "TreeNodeRewriter"}, "trait_path": "datafusion_common::tree_node::TreeNodeRewriter"}`

Source: `src/column_rewriter.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3a804ff743af6e404fedd6e"></a>
## column_map

`struct_field` · `datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter::column_map` · datafusion-physical-plan 55.1.0

```rust
column_map: &'a datafusion_common::HashMap<datafusion_physical_expr::expressions::Column, std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/column_rewriter.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Mapping from original column to new column.

<a id="op-b5f706722741bd5597e5c8f7"></a>
## f_down

`function` · `datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter::f_down` · datafusion-physical-plan 55.1.0

```rust
fn f_down(&mut self, node: Self::Node) -> datafusion_common::Result<Transformed<Self::Node>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter", "path": "PhysicalColumnRewriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [71, 2], "filename": "src/column_rewriter.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRewriter", "path": "TreeNodeRewriter"}, "trait_path": "datafusion_common::tree_node::TreeNodeRewriter"}`

Source: `src/column_rewriter.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf88b74ddf25f7c5eedb0852"></a>
## new

`function` · `datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter::new` · datafusion-physical-plan 55.1.0

```rust
fn new(column_map: &'a HashMap<Column, Arc<dyn PhysicalExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter", "path": "PhysicalColumnRewriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [44, 2], "filename": "src/column_rewriter.rs"}, "trait": null, "trait_path": null}`

Source: `src/column_rewriter.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new PhysicalColumnRewriter with the given column mapping.
