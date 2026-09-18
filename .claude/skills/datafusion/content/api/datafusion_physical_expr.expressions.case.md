# `datafusion_physical_expr::expressions::case`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.case.json`](../model/datafusion_physical_expr.expressions.case.json)

## case

`function` · `datafusion_physical_expr::expressions::case::case`

Also reachable as `datafusion_physical_expr::expressions::case`, `datafusion_physical_plan::execution_plan::expressions::case`, `datafusion_physical_plan::expressions::case`

```rust
fn case(expr: Option<std::sync::Arc<dyn PhysicalExpr>>, when_thens: Vec<(std::sync::Arc<dyn PhysicalExpr>, std::sync::Arc<dyn PhysicalExpr>)>, else_expr: Option<std::sync::Arc<dyn PhysicalExpr>>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.case.case.md).


Create a CASE expression

---

## CaseExpr

`struct` · `datafusion_physical_expr::expressions::case::CaseExpr`

Also reachable as `datafusion_physical_expr::expressions::CaseExpr`, `datafusion_physical_plan::execution_plan::expressions::CaseExpr`, `datafusion_physical_plan::expressions::CaseExpr`

```rust
struct CaseExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn else_expr(&self) -> Option<&Arc<dyn PhysicalExpr>>
fn expr(&self) -> Option<&Arc<dyn PhysicalExpr>>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
fn try_new(expr: Option<Arc<dyn PhysicalExpr>>, when_then_expr: Vec<(std::sync::Arc<dyn PhysicalExpr>, std::sync::Arc<dyn PhysicalExpr>)>, else_expr: Option<Arc<dyn PhysicalExpr>>) -> Result<Self>
fn when_then_expr(&self) -> &[(std::sync::Arc<dyn PhysicalExpr>, std::sync::Arc<dyn PhysicalExpr>)]
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result
fn nullable(&self, input_schema: &Schema) -> Result<bool>
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.case.CaseExpr.md).


The CASE expression is similar to a series of nested if/else and there are two forms that
can be used. The first form consists of a series of boolean "when" expressions with
corresponding "then" expressions, and an optional "else" expression.

CASE WHEN condition THEN result
     [WHEN ...]
     [ELSE result]
END

The second form uses a base expression and then a series of "when" clauses that match on a
literal value.

CASE expression
    WHEN value THEN result
    [WHEN ...]
    [ELSE result]
END

---
