# `datafusion_physical_expr::scalar_subquery`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.scalar_subquery.json`](../model/datafusion_physical_expr.scalar_subquery.json)

## ScalarSubqueryExpr

`struct` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr`

```rust
struct ScalarSubqueryExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn data_type(&self) -> &DataType
fn index(&self) -> SubqueryIndex
fn new(data_type: DataType, nullable: bool, index: SubqueryIndex, results: ScalarSubqueryResults) -> Self
fn nullable(&self) -> bool
fn results(&self) -> &ScalarSubqueryResults
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, _ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>, results: &ScalarSubqueryResults) -> Result<Arc<dyn PhysicalExpr>>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn evaluate(&self, _batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn get_properties(&self, _children: &[ExprProperties]) -> Result<ExprProperties>
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

A physical expression whose value is provided by a scalar subquery.

Subquery execution is handled by `ScalarSubqueryExec`, which stores the
result in a shared [`ScalarSubqueryResults`] container. This expression
simply reads from that container at the appropriate index.

---
