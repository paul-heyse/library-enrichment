# `datafusion_physical_expr_common::physical_expr`

Crate `datafusion-physical-expr-common` · 10 public items · structured records in [`model/datafusion_physical_expr_common.physical_expr.json`](../model/datafusion_physical_expr_common.physical_expr.json)

## fmt_sql

`function` · `datafusion_physical_expr_common::physical_expr::fmt_sql`

```rust
fn fmt_sql(expr: &dyn PhysicalExpr) -> impl Display + '_
```

Prints a [`PhysicalExpr`] in a SQL-like format

# Example
```
# // The boilerplate needed to create a `PhysicalExpr` for the example
use std::collections::HashMap;
# use std::fmt::Formatter;
# use std::sync::Arc;
# use arrow::array::RecordBatch;
# use arrow::datatypes::{DataType, Field, FieldRef, Schema};
# use datafusion_common::Result;
# use datafusion_expr_common::columnar_value::ColumnarValue;
# use datafusion_physical_expr_common::physical_expr::{fmt_sql, DynEq, PhysicalExpr};
# #[derive(Debug, PartialEq, Eq, Hash)]
# struct MyExpr {}
# impl PhysicalExpr for MyExpr {
# fn data_type(&self, input_schema: &Schema) -> Result<DataType> { unimplemented!() }
# fn nullable(&self, input_schema: &Schema) -> Result<bool> { unimplemented!() }
# fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue> { unimplemented!() }
# fn return_field(&self, input_schema: &Schema) -> Result<FieldRef> { unimplemented!() }
# fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>{ unimplemented!() }
# fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>> { unimplemented!() }
# fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "CASE a > b THEN 1 ELSE 0 END") }
# }
# impl std::fmt::Display for MyExpr {fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { unimplemented!() } }
# fn make_physical_expr() -> Arc<dyn PhysicalExpr> { Arc::new(MyExpr{}) }
let expr: Arc<dyn PhysicalExpr> = make_physical_expr();
// wrap the expression in `sql_fmt` which can be used with
// `format!`, `to_string()`, etc
let expr_as_sql = fmt_sql(expr.as_ref());
assert_eq!(
  "The SQL: CASE a > b THEN 1 ELSE 0 END",
  format!("The SQL: {expr_as_sql}")
);
```

---

## format_physical_expr_list

`function` · `datafusion_physical_expr_common::physical_expr::format_physical_expr_list`

```rust
fn format_physical_expr_list<T>(exprs: T) -> impl Display where T: IntoIterator, T::Item: Display, T::IntoIter: Clone
```

Returns [`Display`] able a list of [`PhysicalExpr`]

Example output: `[a + 1, b]`

---

## is_dynamic_physical_expr

`function` · `datafusion_physical_expr_common::physical_expr::is_dynamic_physical_expr`

> **Deprecated** — since 55.0.0: Downcast to `DynamicFilterPhysicalExpr`, or use `DynamicFilterTracking::classify(expr).contains_dynamic_filter()` from `datafusion_physical_expr`

```rust
fn is_dynamic_physical_expr(expr: &std::sync::Arc<dyn PhysicalExpr>) -> bool
```

Check if the given `PhysicalExpr` is dynamic.
Internally this calls [`snapshot_generation`] to check if the generation is non-zero,
any dynamic `PhysicalExpr` should have a non-zero generation.

---

## is_volatile

`function` · `datafusion_physical_expr_common::physical_expr::is_volatile`

```rust
fn is_volatile(expr: &std::sync::Arc<dyn PhysicalExpr>) -> bool
```

Returns true if the expression is volatile, i.e. whether it can return different
results when evaluated multiple times with the same input.

For example the function call `RANDOM()` is volatile as each call will
return a different value.

This method recursively checks if any sub-expression is volatile, for example
`1 + RANDOM()` will return `true`.

---

## snapshot_generation

`function` · `datafusion_physical_expr_common::physical_expr::snapshot_generation`

```rust
fn snapshot_generation(expr: &std::sync::Arc<dyn PhysicalExpr>) -> u64
```

Check the generation of this `PhysicalExpr`.
Dynamic `PhysicalExpr`s may have a generation that is incremented
every time the state of the `PhysicalExpr` changes.
If the generation changes that means this `PhysicalExpr` or one of its children
has changed since the last time it was evaluated.

This algorithm will not produce collisions as long as the structure of the
`PhysicalExpr` does not change and no `PhysicalExpr` decrements its own generation.

---

## snapshot_physical_expr

`function` · `datafusion_physical_expr_common::physical_expr::snapshot_physical_expr`

```rust
fn snapshot_physical_expr(expr: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Take a snapshot of the given `PhysicalExpr` if it is dynamic.

Take a snapshot of this `PhysicalExpr` if it is dynamic.
This is used to capture the current state of `PhysicalExpr`s that may contain
dynamic references to other operators in order to serialize it over the wire
or treat it via downcast matching.

See the documentation of [`PhysicalExpr::snapshot`] for more details.

# Returns

Returns a snapshot of the `PhysicalExpr` if it is dynamic, otherwise
returns itself.

---

## snapshot_physical_expr_opt

`function` · `datafusion_physical_expr_common::physical_expr::snapshot_physical_expr_opt`

```rust
fn snapshot_physical_expr_opt(expr: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn PhysicalExpr>>>
```

Take a snapshot of the given `PhysicalExpr` if it is dynamic.

Take a snapshot of this `PhysicalExpr` if it is dynamic.
This is used to capture the current state of `PhysicalExpr`s that may contain
dynamic references to other operators in order to serialize it over the wire
or treat it via downcast matching.

See the documentation of [`PhysicalExpr::snapshot`] for more details.

# Returns

Returns a `[`Transformed`] indicating whether a snapshot was taken,
along with the resulting `PhysicalExpr`.

---

## with_new_children_if_necessary

`function` · `datafusion_physical_expr_common::physical_expr::with_new_children_if_necessary`

```rust
fn with_new_children_if_necessary(expr: std::sync::Arc<dyn PhysicalExpr>, children: Vec<std::sync::Arc<dyn PhysicalExpr>>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Returns a copy of this expr if we change any child according to the pointer comparison.
The size of `children` must be equal to the size of `PhysicalExpr::children()`.

---

## PhysicalExpr

`trait` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

Also reachable as `datafusion::physical_expr::PhysicalExpr`, `datafusion::physical_plan::PhysicalExpr`, `datafusion_physical_expr::PhysicalExpr`, `datafusion_physical_plan::PhysicalExpr`, `datafusion_physical_plan::execution_plan::PhysicalExpr`

```rust
trait PhysicalExpr: Any + Send + Sync + Display + Debug + DynEq + DynHash
```

**Implementors** (25)

- `datafusion_ffi::physical_expr::ForeignPhysicalExpr`
- `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr`
- `datafusion_physical_expr::expressions::binary::BinaryExpr`
- `datafusion_physical_expr::expressions::case::CaseExpr`
- `datafusion_physical_expr::expressions::cast::CastExpr`
- `datafusion_physical_expr::expressions::column::Column`
- `datafusion_physical_expr::expressions::dynamic_filters::DynamicFilterPhysicalExpr`
- `datafusion_physical_expr::expressions::in_list::InListExpr`
- `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr`
- `datafusion_physical_expr::expressions::is_null::IsNullExpr`
- `datafusion_physical_expr::expressions::lambda::LambdaExpr`
- `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable`
- `datafusion_physical_expr::expressions::like::LikeExpr`
- `datafusion_physical_expr::expressions::literal::Literal`
- `datafusion_physical_expr::expressions::negative::NegativeExpr`
- `datafusion_physical_expr::expressions::no_op::NoOp`
- `datafusion_physical_expr::expressions::not::NotExpr`
- `datafusion_physical_expr::expressions::try_cast::TryCastExpr`
- `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn`
- `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr`
- `datafusion_physical_expr::scalar_function::ScalarFunctionExpr`
- `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr`
- `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr`
- `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr`
- `datafusion_physical_plan::repartition::RangeExpr`

**Methods** (19)

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn evaluate_bounds(&self, _children: &[&Interval]) -> Result<Interval>
fn evaluate_selection(&self, batch: &RecordBatch, selection: &BooleanArray) -> Result<ColumnarValue>
fn evaluate_statistics(&self, children: &[&Distribution]) -> Result<Distribution>
fn expression_id(&self) -> Option<u64>
fn fmt_sql(&self, f: &mut Formatter<'_>) -> fmt::Result
fn get_properties(&self, _children: &[ExprProperties]) -> Result<ExprProperties>
fn is_volatile_node(&self) -> bool
fn nullable(&self, input_schema: &Schema) -> Result<bool>
fn placement(&self) -> ExpressionPlacement
fn propagate_constraints(&self, _interval: &Interval, _children: &[&Interval]) -> Result<Option<Vec<Interval>>>
fn propagate_statistics(&self, parent: &Distribution, children: &[&Distribution]) -> Result<Option<Vec<Distribution>>>
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
fn snapshot(&self) -> Result<Option<Arc<dyn PhysicalExpr>>>
fn snapshot_generation(&self) -> u64
fn try_to_proto(&self, _ctx: &proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

[`PhysicalExpr`]s represent expressions such as `A + 1` or `CAST(c1 AS int)`.

`PhysicalExpr` knows its type, nullability and can be evaluated directly on
a [`RecordBatch`] (see [`Self::evaluate`]).

`PhysicalExpr` are the physical counterpart to [`Expr`] used in logical
planning. They are typically created from [`Expr`] by a [`PhysicalPlanner`]
invoked from a higher level API

Some important examples of `PhysicalExpr` are:
* [`Column`]: Represents a column at a given index in a RecordBatch

To create `PhysicalExpr` from  `Expr`, see
* [`SessionContext::create_physical_expr`]: A high level API
* [`create_physical_expr`]: A low level API

# Formatting `PhysicalExpr` as strings
There are three ways to format `PhysicalExpr` as a string:
* [`Debug`]: Standard Rust debugging format (e.g. `Constant { value: ... }`)
* [`Display`]: Detailed SQL-like format that shows expression structure (e.g. (`Utf8 ("foobar")`). This is often used for debugging and tests
* [`Self::fmt_sql`]: SQL-like human readable format (e.g. ('foobar')`), See also [`sql_fmt`]

[`SessionContext::create_physical_expr`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.create_physical_expr
[`PhysicalPlanner`]: https://docs.rs/datafusion/latest/datafusion/physical_planner/trait.PhysicalPlanner.html
[`Expr`]: https://docs.rs/datafusion/latest/datafusion/logical_expr/enum.Expr.html
[`create_physical_expr`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/fn.create_physical_expr.html
[`Column`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/expressions/struct.Column.html

---

## PhysicalExprRef

`type_alias` · `datafusion_physical_expr_common::physical_expr::PhysicalExprRef`

Also reachable as `datafusion::physical_expr::PhysicalExprRef`, `datafusion_physical_expr::PhysicalExprRef`

```rust
type PhysicalExprRef = std::sync::Arc<dyn PhysicalExpr>
```

Shared [`PhysicalExpr`].

---
