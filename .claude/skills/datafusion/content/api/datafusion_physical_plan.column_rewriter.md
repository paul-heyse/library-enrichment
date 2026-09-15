# `datafusion_physical_plan::column_rewriter`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.column_rewriter.json`](../model/datafusion_physical_plan.column_rewriter.json)

## PhysicalColumnRewriter

`struct` · `datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter`

```rust
struct PhysicalColumnRewriter<'a>
```

**Fields**: `column_map`

**Implements**: `datafusion_common::tree_node::TreeNodeRewriter`

**Methods** (1)

```rust
fn new(column_map: &'a HashMap<Column, Arc<dyn PhysicalExpr>>) -> Self
```

**via `datafusion_common::tree_node::TreeNodeRewriter`**

```rust
fn f_down(&mut self, node: Self::Node) -> datafusion_common::Result<Transformed<Self::Node>>
```

Rewrite column references in a physical expr according to a mapping.

This rewriter traverses the expression tree and replaces [`Column`] nodes
with the corresponding expression found in the `column_map`.

If a column is found in the map, it is replaced by the mapped expression.
If a column is NOT found in the map, a `DataFusionError::Internal` is
returned.

---
