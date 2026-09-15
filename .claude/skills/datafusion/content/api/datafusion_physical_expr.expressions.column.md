# `datafusion_physical_expr::expressions::column`

Crate `datafusion-physical-expr` · 3 public items · structured records in [`model/datafusion_physical_expr.expressions.column.json`](../model/datafusion_physical_expr.expressions.column.json)

## col

`function` · `datafusion_physical_expr::expressions::column::col`

Also reachable as `datafusion_physical_expr::expressions::col`, `datafusion_physical_plan::execution_plan::expressions::col`, `datafusion_physical_plan::expressions::col`

```rust
fn col(name: &str, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Create a column expression

---

## with_new_schema

`function` · `datafusion_physical_expr::expressions::column::with_new_schema`

Also reachable as `datafusion_physical_expr::expressions::with_new_schema`, `datafusion_physical_plan::execution_plan::expressions::with_new_schema`, `datafusion_physical_plan::expressions::with_new_schema`

```rust
fn with_new_schema(expr: std::sync::Arc<dyn PhysicalExpr>, schema: &arrow::datatypes::SchemaRef) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Rewrites an expression according to new schema; i.e. changes the columns it
refers to with the column at corresponding index in the new schema. Returns
an error if the given schema has fewer columns than the original schema.
Note that the resulting expression may not be valid if data types in the
new schema is incompatible with expression nodes.

---

## Column

`struct` · `datafusion_physical_expr::expressions::column::Column`

Also reachable as `datafusion_physical_expr::expressions::Column`, `datafusion_physical_plan::execution_plan::expressions::Column`, `datafusion_physical_plan::expressions::Column`

```rust
struct Column
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn index(&self) -> usize
fn name(&self) -> &str
fn new(name: &str, index: usize) -> Self
fn new_with_schema(name: &str, schema: &Schema) -> Result<Self>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, _ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

**via `core::convert::From`**

```rust
fn from(c: &datafusion_proto_models::protobuf::PhysicalColumn) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
fn nullable(&self, input_schema: &Schema) -> Result<bool>
fn placement(&self) -> ExpressionPlacement
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Represents the column at a given index in a RecordBatch

This is a physical expression that represents a column at a given index in an
arrow [`Schema`] / [`RecordBatch`].

Unlike the [logical `Expr::Column`], this expression is always resolved by schema index,
even though it does have a name. This is because the physical plan is always
resolved to a specific schema and there is no concept of "relation"

# Example:
 If the schema is `a`, `b`, `c` the `Column` for `b` would be represented by
 index 1, since `b` is the second column in the schema.

```
# use datafusion_physical_expr::expressions::Column;
# use arrow::datatypes::{DataType, Field, Schema};
// Schema with columns a, b, c
let schema = Schema::new(vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Int32, false),
    Field::new("c", DataType::Int32, false),
]);

// reference to column b is index 1
let column_b = Column::new_with_schema("b", &schema).unwrap();
assert_eq!(column_b.index(), 1);

// reference to column c is index 2
let column_c = Column::new_with_schema("c", &schema).unwrap();
assert_eq!(column_c.index(), 2);
```
[logical `Expr::Column`]: https://docs.rs/datafusion/latest/datafusion/logical_expr/enum.Expr.html#variant.Column

---
