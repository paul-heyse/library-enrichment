# `datafusion_expr::expr_fn`

Crate `datafusion-expr` · 54 public items · structured records in [`model/datafusion_expr.expr_fn.json`](../model/datafusion_expr.expr_fn.json)

## ExprFuncKind

`enum` · `datafusion_expr::expr_fn::ExprFuncKind`

Also reachable as `datafusion::logical_expr::ExprFuncKind`, `datafusion::prelude::ExprFuncKind`, `datafusion_expr::ExprFuncKind`

```rust
enum ExprFuncKind
```

**Variants**: `Aggregate`, `Window`

**Derives**: Clone, Debug

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.ExprFuncKind.md).


---

## and

`function` · `datafusion_expr::expr_fn::and`

Also reachable as `datafusion::logical_expr::and`, `datafusion::prelude::and`, `datafusion_expr::and`

```rust
fn and(left: Expr, right: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.and.md).


Return a new expression with a logical AND

---

## binary_expr

`function` · `datafusion_expr::expr_fn::binary_expr`

Also reachable as `datafusion::logical_expr::binary_expr`, `datafusion::prelude::binary_expr`, `datafusion_expr::binary_expr`

```rust
fn binary_expr(left: Expr, op: Operator, right: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.binary_expr.md).


Return a new expression `left <op> right`

---

## bitwise_and

`function` · `datafusion_expr::expr_fn::bitwise_and`

Also reachable as `datafusion::logical_expr::bitwise_and`, `datafusion::prelude::bitwise_and`, `datafusion_expr::bitwise_and`

```rust
fn bitwise_and(left: Expr, right: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.bitwise_and.md).


Return a new expression with bitwise AND

---

## bitwise_or

`function` · `datafusion_expr::expr_fn::bitwise_or`

Also reachable as `datafusion::logical_expr::bitwise_or`, `datafusion::prelude::bitwise_or`, `datafusion_expr::bitwise_or`

```rust
fn bitwise_or(left: Expr, right: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.bitwise_or.md).


Return a new expression with bitwise OR

---

## bitwise_shift_left

`function` · `datafusion_expr::expr_fn::bitwise_shift_left`

Also reachable as `datafusion::logical_expr::bitwise_shift_left`, `datafusion::prelude::bitwise_shift_left`, `datafusion_expr::bitwise_shift_left`

```rust
fn bitwise_shift_left(left: Expr, right: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.bitwise_shift_left.md).


Return a new expression with bitwise SHIFT LEFT

---

## bitwise_shift_right

`function` · `datafusion_expr::expr_fn::bitwise_shift_right`

Also reachable as `datafusion::logical_expr::bitwise_shift_right`, `datafusion::prelude::bitwise_shift_right`, `datafusion_expr::bitwise_shift_right`

```rust
fn bitwise_shift_right(left: Expr, right: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.bitwise_shift_right.md).


Return a new expression with bitwise SHIFT RIGHT

---

## bitwise_xor

`function` · `datafusion_expr::expr_fn::bitwise_xor`

Also reachable as `datafusion::logical_expr::bitwise_xor`, `datafusion::prelude::bitwise_xor`, `datafusion_expr::bitwise_xor`

```rust
fn bitwise_xor(left: Expr, right: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.bitwise_xor.md).


Return a new expression with bitwise XOR

---

## case

`function` · `datafusion_expr::expr_fn::case`

Also reachable as `datafusion::logical_expr::case`, `datafusion::prelude::case`, `datafusion_expr::case`

```rust
fn case(expr: Expr) -> conditional_expressions::CaseBuilder
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.case.md).


Create a CASE WHEN statement with literal WHEN expressions for comparison to the base expression.

---

## cast

`function` · `datafusion_expr::expr_fn::cast`

Also reachable as `datafusion::logical_expr::cast`, `datafusion::prelude::cast`, `datafusion_expr::cast`

```rust
fn cast(expr: Expr, data_type: arrow::datatypes::DataType) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.cast.md).


Create a cast expression

---

## col

`function` · `datafusion_expr::expr_fn::col`

Also reachable as `datafusion::logical_expr::col`, `datafusion::prelude::col`, `datafusion_expr::col`

```rust
fn col(ident: impl Into<datafusion_common::Column>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.col.md).


Create a column expression based on a qualified or unqualified column name. Will
normalize unquoted identifiers according to SQL rules (identifiers will become lowercase).

For example:

```rust
# use datafusion_expr::col;
let c1 = col("a");
let c2 = col("A");
assert_eq!(c1, c2);

// note how quoting with double quotes preserves the case
let c3 = col(r#""A""#);
assert_ne!(c1, c3);
```

---

## create_udaf

`function` · `datafusion_expr::expr_fn::create_udaf`

Also reachable as `datafusion::logical_expr::create_udaf`, `datafusion::prelude::create_udaf`, `datafusion_expr::create_udaf`

```rust
fn create_udaf(name: &str, input_type: Vec<arrow::datatypes::DataType>, return_type: std::sync::Arc<arrow::datatypes::DataType>, volatility: Volatility, accumulator: function::AccumulatorFactoryFunction, state_type: std::sync::Arc<Vec<arrow::datatypes::DataType>>) -> AggregateUDF
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.create_udaf.md).


Creates a new UDAF with a specific signature, state type and return type.
The signature and state type must match the `Accumulator's implementation`.

---

## create_udf

`function` · `datafusion_expr::expr_fn::create_udf`

Also reachable as `datafusion::logical_expr::create_udf`, `datafusion::prelude::create_udf`, `datafusion_expr::create_udf`

```rust
fn create_udf(name: &str, input_types: Vec<arrow::datatypes::DataType>, return_type: arrow::datatypes::DataType, volatility: Volatility, fun: ScalarFunctionImplementation) -> ScalarUDF
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.create_udf.md).


Convenience method to create a new user defined scalar function (UDF) with a
specific signature and specific return type.

Note this function does not expose all available features of [`ScalarUDF`],
such as

* computing return types based on input types
* multiple [`Signature`]s
* aliases

See [`ScalarUDF`] for details and examples on how to use the full
functionality.

---

## create_udwf

`function` · `datafusion_expr::expr_fn::create_udwf`

Also reachable as `datafusion::logical_expr::create_udwf`, `datafusion::prelude::create_udwf`, `datafusion_expr::create_udwf`

```rust
fn create_udwf(name: &str, input_type: arrow::datatypes::DataType, return_type: std::sync::Arc<arrow::datatypes::DataType>, volatility: Volatility, partition_evaluator_factory: function::PartitionEvaluatorFactory) -> WindowUDF
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.create_udwf.md).


Creates a new UDWF with a specific signature, state type and return type.

The signature and state type must match the [`PartitionEvaluator`]'s implementation`.

[`PartitionEvaluator`]: crate::PartitionEvaluator

---

## cube

`function` · `datafusion_expr::expr_fn::cube`

Also reachable as `datafusion::logical_expr::cube`, `datafusion::prelude::cube`, `datafusion_expr::cube`

```rust
fn cube(exprs: Vec<Expr>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.cube.md).


Create a grouping set for all combination of `exprs`

---

## exists

`function` · `datafusion_expr::expr_fn::exists`

Also reachable as `datafusion::logical_expr::exists`, `datafusion::prelude::exists`, `datafusion_expr::exists`

```rust
fn exists(subquery: std::sync::Arc<LogicalPlan>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.exists.md).


Create an EXISTS subquery expression

---

## grouping_set

`function` · `datafusion_expr::expr_fn::grouping_set`

Also reachable as `datafusion::logical_expr::grouping_set`, `datafusion::prelude::grouping_set`, `datafusion_expr::grouping_set`

```rust
fn grouping_set(exprs: Vec<Vec<Expr>>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.grouping_set.md).


Create a grouping set

---

## ident

`function` · `datafusion_expr::expr_fn::ident`

Also reachable as `datafusion::logical_expr::ident`, `datafusion::prelude::ident`, `datafusion_expr::ident`

```rust
fn ident(name: impl Into<String>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.ident.md).


Create an unqualified column expression from the provided name, without normalizing
the column.

For example:

```rust
# use datafusion_expr::{col, ident};
let c1 = ident("A"); // not normalized staying as column 'A'
let c2 = col("A"); // normalized via SQL rules becoming column 'a'
assert_ne!(c1, c2);

let c3 = col(r#""A""#);
assert_eq!(c1, c3);

let c4 = col("t1.a"); // parses as relation 't1' column 'a'
let c5 = ident("t1.a"); // parses as column 't1.a'
assert_ne!(c4, c5);
```

---

## in_list

`function` · `datafusion_expr::expr_fn::in_list`

Also reachable as `datafusion::logical_expr::in_list`, `datafusion::prelude::in_list`, `datafusion_expr::in_list`

```rust
fn in_list(expr: Expr, list: Vec<Expr>, negated: bool) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.in_list.md).


Create an in_list expression

---

## in_subquery

`function` · `datafusion_expr::expr_fn::in_subquery`

Also reachable as `datafusion::logical_expr::in_subquery`, `datafusion::prelude::in_subquery`, `datafusion_expr::in_subquery`

```rust
fn in_subquery(expr: Expr, subquery: std::sync::Arc<LogicalPlan>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.in_subquery.md).


Create an IN subquery expression

---

## interval_datetime_lit

`function` · `datafusion_expr::expr_fn::interval_datetime_lit`

Also reachable as `datafusion::logical_expr::interval_datetime_lit`, `datafusion::prelude::interval_datetime_lit`, `datafusion_expr::interval_datetime_lit`

```rust
fn interval_datetime_lit(value: &str) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.interval_datetime_lit.md).


---

## interval_month_day_nano_lit

`function` · `datafusion_expr::expr_fn::interval_month_day_nano_lit`

Also reachable as `datafusion::logical_expr::interval_month_day_nano_lit`, `datafusion::prelude::interval_month_day_nano_lit`, `datafusion_expr::interval_month_day_nano_lit`

```rust
fn interval_month_day_nano_lit(value: &str) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.interval_month_day_nano_lit.md).


---

## interval_year_month_lit

`function` · `datafusion_expr::expr_fn::interval_year_month_lit`

Also reachable as `datafusion::logical_expr::interval_year_month_lit`, `datafusion::prelude::interval_year_month_lit`, `datafusion_expr::interval_year_month_lit`

```rust
fn interval_year_month_lit(value: &str) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.interval_year_month_lit.md).


---

## is_false

`function` · `datafusion_expr::expr_fn::is_false`

Also reachable as `datafusion::logical_expr::is_false`, `datafusion::prelude::is_false`, `datafusion_expr::is_false`

```rust
fn is_false(expr: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.is_false.md).


Create is false expression

---

## is_not_false

`function` · `datafusion_expr::expr_fn::is_not_false`

Also reachable as `datafusion::logical_expr::is_not_false`, `datafusion::prelude::is_not_false`, `datafusion_expr::is_not_false`

```rust
fn is_not_false(expr: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.is_not_false.md).


Create is not false expression

---

## is_not_null

`function` · `datafusion_expr::expr_fn::is_not_null`

Also reachable as `datafusion::logical_expr::is_not_null`, `datafusion::prelude::is_not_null`, `datafusion_expr::is_not_null`

```rust
fn is_not_null(expr: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.is_not_null.md).


Create is not null expression

---

## is_not_true

`function` · `datafusion_expr::expr_fn::is_not_true`

Also reachable as `datafusion::logical_expr::is_not_true`, `datafusion::prelude::is_not_true`, `datafusion_expr::is_not_true`

```rust
fn is_not_true(expr: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.is_not_true.md).


Create is not true expression

---

## is_not_unknown

`function` · `datafusion_expr::expr_fn::is_not_unknown`

Also reachable as `datafusion::logical_expr::is_not_unknown`, `datafusion::prelude::is_not_unknown`, `datafusion_expr::is_not_unknown`

```rust
fn is_not_unknown(expr: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.is_not_unknown.md).


Create is not unknown expression

---

## is_null

`function` · `datafusion_expr::expr_fn::is_null`

Also reachable as `datafusion::logical_expr::is_null`, `datafusion::prelude::is_null`, `datafusion_expr::is_null`

```rust
fn is_null(expr: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.is_null.md).


Create is null expression

---

## is_true

`function` · `datafusion_expr::expr_fn::is_true`

Also reachable as `datafusion::logical_expr::is_true`, `datafusion::prelude::is_true`, `datafusion_expr::is_true`

```rust
fn is_true(expr: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.is_true.md).


Create is true expression

---

## is_unknown

`function` · `datafusion_expr::expr_fn::is_unknown`

Also reachable as `datafusion::logical_expr::is_unknown`, `datafusion::prelude::is_unknown`, `datafusion_expr::is_unknown`

```rust
fn is_unknown(expr: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.is_unknown.md).


Create is unknown expression

---

## lambda

`function` · `datafusion_expr::expr_fn::lambda`

Also reachable as `datafusion::logical_expr::lambda`, `datafusion::prelude::lambda`, `datafusion_expr::lambda`

```rust
fn lambda(params: impl IntoIterator<Item = impl Into<String>>, body: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.lambda.md).


Create a lambda expression

---

## lambda_var

`function` · `datafusion_expr::expr_fn::lambda_var`

Also reachable as `datafusion::logical_expr::lambda_var`, `datafusion::prelude::lambda_var`, `datafusion_expr::lambda_var`

```rust
fn lambda_var(name: impl Into<String>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.lambda_var.md).


Create an unresolved lambda variable expression

The expression tree or [`LogicalPlan`] which
owns this variable must be resolved before usage with either
[`Expr::resolve_lambda_variables`] or [`LogicalPlan::resolve_lambda_variables`].

[LogicalPlan::resolve_lambda_variables]: crate::LogicalPlan::resolve_lambda_variables

---

## not

`function` · `datafusion_expr::expr_fn::not`

Also reachable as `datafusion::logical_expr::not`, `datafusion::prelude::not`, `datafusion_expr::not`

```rust
fn not(expr: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.not.md).


Return a new expression with a logical NOT

---

## not_exists

`function` · `datafusion_expr::expr_fn::not_exists`

Also reachable as `datafusion::logical_expr::not_exists`, `datafusion::prelude::not_exists`, `datafusion_expr::not_exists`

```rust
fn not_exists(subquery: std::sync::Arc<LogicalPlan>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.not_exists.md).


Create a NOT EXISTS subquery expression

---

## not_in_subquery

`function` · `datafusion_expr::expr_fn::not_in_subquery`

Also reachable as `datafusion::logical_expr::not_in_subquery`, `datafusion::prelude::not_in_subquery`, `datafusion_expr::not_in_subquery`

```rust
fn not_in_subquery(expr: Expr, subquery: std::sync::Arc<LogicalPlan>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.not_in_subquery.md).


Create a NOT IN subquery expression

---

## or

`function` · `datafusion_expr::expr_fn::or`

Also reachable as `datafusion::logical_expr::or`, `datafusion::prelude::or`, `datafusion_expr::or`

```rust
fn or(left: Expr, right: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.or.md).


Return a new expression with a logical OR

---

## out_ref_col

`function` · `datafusion_expr::expr_fn::out_ref_col`

Also reachable as `datafusion::logical_expr::out_ref_col`, `datafusion::prelude::out_ref_col`, `datafusion_expr::out_ref_col`

```rust
fn out_ref_col(dt: arrow::datatypes::DataType, ident: impl Into<datafusion_common::Column>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.out_ref_col.md).


Create an out reference column which hold a reference that has been resolved to a field
outside of the current plan.
The expression created by this function does not preserve the metadata of the outer column.
Please use `out_ref_col_with_metadata` if you want to preserve the metadata.

---

## out_ref_col_with_metadata

`function` · `datafusion_expr::expr_fn::out_ref_col_with_metadata`

Also reachable as `datafusion::logical_expr::out_ref_col_with_metadata`, `datafusion::prelude::out_ref_col_with_metadata`, `datafusion_expr::out_ref_col_with_metadata`

```rust
fn out_ref_col_with_metadata(dt: arrow::datatypes::DataType, metadata: std::collections::HashMap<String, String>, ident: impl Into<datafusion_common::Column>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.out_ref_col_with_metadata.md).


Create an out reference column from an existing field (preserving metadata)

---

## placeholder

`function` · `datafusion_expr::expr_fn::placeholder`

Also reachable as `datafusion::logical_expr::placeholder`, `datafusion::prelude::placeholder`, `datafusion_expr::placeholder`

```rust
fn placeholder(id: impl Into<String>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.placeholder.md).


Create placeholder value that will be filled in (such as `$1`)

Note the parameter type can be inferred using [`Expr::infer_placeholder_types`]

# Example

```rust
# use datafusion_expr::{placeholder};
let p = placeholder("$1"); // $1, refers to parameter 1
assert_eq!(p.to_string(), "$1")
```

---

## qualified_wildcard

`function` · `datafusion_expr::expr_fn::qualified_wildcard`

Also reachable as `datafusion::logical_expr::qualified_wildcard`, `datafusion::prelude::qualified_wildcard`, `datafusion_expr::qualified_wildcard`

```rust
fn qualified_wildcard(qualifier: impl Into<datafusion_common::TableReference>) -> select_expr::SelectExpr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.qualified_wildcard.md).


Create an 't.*' [`Expr::Wildcard`] expression that matches all columns from a specific table

# Example

```rust
# use datafusion_common::TableReference;
# use datafusion_expr::{qualified_wildcard};
let p = qualified_wildcard(TableReference::bare("t"));
assert_eq!(p.to_string(), "t.*")
```

---

## qualified_wildcard_with_options

`function` · `datafusion_expr::expr_fn::qualified_wildcard_with_options`

Also reachable as `datafusion::logical_expr::qualified_wildcard_with_options`, `datafusion::prelude::qualified_wildcard_with_options`, `datafusion_expr::qualified_wildcard_with_options`

```rust
fn qualified_wildcard_with_options(qualifier: impl Into<datafusion_common::TableReference>, options: expr::WildcardOptions) -> select_expr::SelectExpr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.qualified_wildcard_with_options.md).


Create an 't.*' [`Expr::Wildcard`] expression with the wildcard options

---

## rollup

`function` · `datafusion_expr::expr_fn::rollup`

Also reachable as `datafusion::logical_expr::rollup`, `datafusion::prelude::rollup`, `datafusion_expr::rollup`

```rust
fn rollup(exprs: Vec<Expr>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.rollup.md).


Create a grouping set for rollup

---

## scalar_subquery

`function` · `datafusion_expr::expr_fn::scalar_subquery`

Also reachable as `datafusion::logical_expr::scalar_subquery`, `datafusion::prelude::scalar_subquery`, `datafusion_expr::scalar_subquery`

```rust
fn scalar_subquery(subquery: std::sync::Arc<LogicalPlan>) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.scalar_subquery.md).


Create a scalar subquery expression

---

## try_cast

`function` · `datafusion_expr::expr_fn::try_cast`

Also reachable as `datafusion::logical_expr::try_cast`, `datafusion::prelude::try_cast`, `datafusion_expr::try_cast`

```rust
fn try_cast(expr: Expr, data_type: arrow::datatypes::DataType) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.try_cast.md).


Create a try cast expression

---

## unnest

`function` · `datafusion_expr::expr_fn::unnest`

Also reachable as `datafusion::logical_expr::unnest`, `datafusion::prelude::unnest`, `datafusion_expr::unnest`

```rust
fn unnest(expr: Expr) -> Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.unnest.md).


Create a Unnest expression with default (non-outer) semantics.

---

## when

`function` · `datafusion_expr::expr_fn::when`

Also reachable as `datafusion::logical_expr::when`, `datafusion::prelude::when`, `datafusion_expr::when`

```rust
fn when(when: Expr, then: Expr) -> conditional_expressions::CaseBuilder
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.when.md).


Create a CASE WHEN statement with boolean WHEN expressions and no base expression.

---

## wildcard

`function` · `datafusion_expr::expr_fn::wildcard`

Also reachable as `datafusion::logical_expr::wildcard`, `datafusion::prelude::wildcard`, `datafusion_expr::wildcard`

```rust
fn wildcard() -> select_expr::SelectExpr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.wildcard.md).


Create an '*' [`Expr::Wildcard`] expression that matches all columns

# Example

```rust
# use datafusion_expr::{wildcard};
let p = wildcard();
assert_eq!(p.to_string(), "*")
```

---

## wildcard_with_options

`function` · `datafusion_expr::expr_fn::wildcard_with_options`

Also reachable as `datafusion::logical_expr::wildcard_with_options`, `datafusion::prelude::wildcard_with_options`, `datafusion_expr::wildcard_with_options`

```rust
fn wildcard_with_options(options: expr::WildcardOptions) -> select_expr::SelectExpr
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.wildcard_with_options.md).


Create an '*' [`Expr::Wildcard`] expression with the wildcard options

---

## ExprFuncBuilder

`struct` · `datafusion_expr::expr_fn::ExprFuncBuilder`

Also reachable as `datafusion::logical_expr::ExprFuncBuilder`, `datafusion::prelude::ExprFuncBuilder`, `datafusion_expr::ExprFuncBuilder`

```rust
struct ExprFuncBuilder
```

**Implements**: `datafusion_expr::expr_fn::ExprFunctionExt`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn build(self) -> Result<Expr>
```

**via `datafusion_expr::expr_fn::ExprFunctionExt`**

```rust
fn distinct(self) -> ExprFuncBuilder
fn filter(self, filter: Expr) -> ExprFuncBuilder
fn null_treatment(self, null_treatment: impl Into<Option<NullTreatment>>) -> ExprFuncBuilder
fn order_by(self, order_by: Vec<Sort>) -> ExprFuncBuilder
fn partition_by(self, partition_by: Vec<Expr>) -> ExprFuncBuilder
fn window_frame(self, window_frame: WindowFrame) -> ExprFuncBuilder
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.ExprFuncBuilder.md).


Implementation of [`ExprFunctionExt`].

See [`ExprFunctionExt`] for usage and examples

---

## SimpleAggregateUDF

`struct` · `datafusion_expr::expr_fn::SimpleAggregateUDF`

Also reachable as `datafusion::logical_expr::SimpleAggregateUDF`, `datafusion::prelude::SimpleAggregateUDF`, `datafusion_expr::SimpleAggregateUDF`

```rust
struct SimpleAggregateUDF
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new(name: impl Into<String>, input_type: Vec<DataType>, return_type: DataType, volatility: Volatility, accumulator: AccumulatorFactoryFunction, state_fields: Vec<FieldRef>) -> Self
fn new_with_signature(name: impl Into<String>, signature: Signature, return_type: DataType, accumulator: AccumulatorFactoryFunction, state_fields: Vec<FieldRef>) -> Self
```

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.SimpleAggregateUDF.md).


Implements [`AggregateUDFImpl`] for functions that have a single signature and
return type.

---

## SimpleScalarUDF

`struct` · `datafusion_expr::expr_fn::SimpleScalarUDF`

Also reachable as `datafusion::logical_expr::SimpleScalarUDF`, `datafusion::prelude::SimpleScalarUDF`, `datafusion_expr::SimpleScalarUDF`

```rust
struct SimpleScalarUDF
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new(name: impl Into<String>, input_types: Vec<DataType>, return_type: DataType, volatility: Volatility, fun: ScalarFunctionImplementation) -> Self
fn new_with_signature(name: impl Into<String>, signature: Signature, return_type: DataType, fun: ScalarFunctionImplementation) -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.SimpleScalarUDF.md).


Implements [`ScalarUDFImpl`] for functions that have a single signature and
return type.

---

## SimpleWindowUDF

`struct` · `datafusion_expr::expr_fn::SimpleWindowUDF`

Also reachable as `datafusion::logical_expr::SimpleWindowUDF`, `datafusion::prelude::SimpleWindowUDF`, `datafusion_expr::SimpleWindowUDF`

```rust
struct SimpleWindowUDF
```

**Implements**: `datafusion_expr::udwf::WindowUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: impl Into<String>, input_type: DataType, return_type: DataType, volatility: Volatility, partition_evaluator_factory: PartitionEvaluatorFactory) -> Self
```

**via `datafusion_expr::udwf::WindowUDFImpl`**

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
fn name(&self) -> &str
fn partition_evaluator(&self, _partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.SimpleWindowUDF.md).


Implements [`WindowUDFImpl`] for functions that have a single signature and
return type.

---

## ExprFunctionExt

`trait` · `datafusion_expr::expr_fn::ExprFunctionExt`

Also reachable as `datafusion::logical_expr::ExprFunctionExt`, `datafusion::prelude::ExprFunctionExt`, `datafusion_expr::ExprFunctionExt`

```rust
trait ExprFunctionExt
```

**Implementors** (2)

- `datafusion_expr::expr::Expr`
- `datafusion_expr::expr_fn::ExprFuncBuilder`

**Methods** (6)

```rust
fn distinct(self) -> ExprFuncBuilder
fn filter(self, filter: Expr) -> ExprFuncBuilder
fn null_treatment(self, null_treatment: impl Into<Option<NullTreatment>>) -> ExprFuncBuilder
fn order_by(self, order_by: Vec<Sort>) -> ExprFuncBuilder
fn partition_by(self, partition_by: Vec<Expr>) -> ExprFuncBuilder
fn window_frame(self, window_frame: WindowFrame) -> ExprFuncBuilder
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_fn.ExprFunctionExt.md).


Extensions for configuring [`Expr::AggregateFunction`] or [`Expr::WindowFunction`]

Adds methods to [`Expr`] that make it easy to set optional options
such as `ORDER BY`, `FILTER` and `DISTINCT`

# Example
```no_run
# use datafusion_common::Result;
# use datafusion_expr::expr::NullTreatment;
# use datafusion_expr::test::function_stub::count;
# use datafusion_expr::{ExprFunctionExt, lit, Expr, col};
# // first_value is an aggregate function in another crate
# fn first_value(_arg: Expr) -> Expr {
unimplemented!() }
# fn main() -> Result<()> {
// Create an aggregate count, filtering on column y > 5
let agg = count(col("x")).filter(col("y").gt(lit(5))).build()?;

// Find the first value in an aggregate sorted by column y
// equivalent to:
// `FIRST_VALUE(x ORDER BY y ASC IGNORE NULLS)`
let sort_expr = col("y").sort(true, true);
let agg = first_value(col("x"))
    .order_by(vec![sort_expr])
    .null_treatment(NullTreatment::IgnoreNulls)
    .build()?;

// Create a window expression for percent rank partitioned on column a
// equivalent to:
// `PERCENT_RANK() OVER (PARTITION BY a ORDER BY b ASC NULLS LAST IGNORE NULLS)`
// percent_rank is an udwf function in another crate
# fn percent_rank() -> Expr {
unimplemented!() }
let window = percent_rank()
    .partition_by(vec![col("a")])
    .order_by(vec![col("b").sort(true, true)])
    .null_treatment(NullTreatment::IgnoreNulls)
    .build()?;
#     Ok(())
# }
```

---
