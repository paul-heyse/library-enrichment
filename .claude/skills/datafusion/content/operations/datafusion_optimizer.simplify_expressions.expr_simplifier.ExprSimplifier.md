# `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.simplify_expressions.expr_simplifier.ExprSimplifier.json).

<a id="op-670776ff9168d6afaa2dbf2b"></a>
## ExprSimplifier

`struct` · `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier` · datafusion-optimizer 55.1.0

```rust
struct ExprSimplifier
```

Source: `src/simplify_expressions/expr_simplifier.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

This structure handles API for expression simplification

Provides simplification information based on DFSchema and
[`ExecutionProps`](../operations/datafusion_expr.execution_props.ExecutionProps.md#op-52e4deb424494aeff92bf188). This is the default implementation used by DataFusion

For example:
```
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_common::{DataFusionError, ToDFSchema};
use datafusion_expr::simplify::SimplifyContext;
use datafusion_expr::{col, lit};
use datafusion_optimizer::simplify_expressions::ExprSimplifier;

// Create the schema
let schema = Schema::new(vec![Field::new("i", DataType::Int64, false)])
    .to_dfschema_ref()
    .unwrap();

// Create the simplifier
let context = SimplifyContext::builder().with_schema(schema).build();
let simplifier = ExprSimplifier::new(context);

// Use the simplifier

// b < 2 or (1 > 3)
let expr = col("b").lt(lit(2)).or(lit(1).gt(lit(3)));

// b < 2
let simplified = simplifier.simplify(expr).unwrap();
assert_eq!(simplified, col("b").lt(lit(2)));
```

<a id="op-ab6282f359cd840ffaae3af5"></a>
## coerce

`function` · `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier::coerce` · datafusion-optimizer 55.1.0

```rust
fn coerce(&self, expr: Expr, schema: &DFSchema) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier", "path": "ExprSimplifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [428, 2], "filename": "src/simplify_expressions/expr_simplifier.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify_expressions/expr_simplifier.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Apply type coercion to an [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) so that it can be
evaluated as a [`PhysicalExpr`](datafusion_physical_expr::PhysicalExpr).

See the [type coercion module](datafusion_expr::type_coercion)
documentation for more details on type coercion

<a id="op-58233c3d5f6489bf78e430bd"></a>
## new

`function` · `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier::new` · datafusion-optimizer 55.1.0

```rust
fn new(info: SimplifyContext) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier", "path": "ExprSimplifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [428, 2], "filename": "src/simplify_expressions/expr_simplifier.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify_expressions/expr_simplifier.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create a new `ExprSimplifier` with the given [`SimplifyContext`].
See [`simplify`](Self::simplify) for an example.

[`SimplifyContext`]: datafusion_expr::simplify::SimplifyContext

<a id="op-24b1e713502751d6ff36f0a4"></a>
## simplify

`function` · `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier::simplify` · datafusion-optimizer 55.1.0

```rust
fn simplify(&self, expr: Expr) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier", "path": "ExprSimplifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [428, 2], "filename": "src/simplify_expressions/expr_simplifier.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify_expressions/expr_simplifier.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Simplifies this [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) as much as possible, evaluating
constants and applying algebraic simplifications.

The types of the expression must match what operators expect,
or else an error may occur trying to evaluate. See
[`coerce`](Self::coerce) for a function to help.

# Example:

`b > 2 AND b > 2`

can be written to

`b > 2`

```
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_common::{DFSchema, ToDFSchema};
use datafusion_common::Result;
use datafusion_expr::simplify::SimplifyContext;
use datafusion_expr::{col, lit, Expr};
use datafusion_optimizer::simplify_expressions::ExprSimplifier;
use std::sync::Arc;

// Create a schema and SimplifyContext
let schema = Schema::new(vec![Field::new("b", DataType::Int32, true)])
    .to_dfschema_ref()
    .unwrap();
// Create the simplifier
let context = SimplifyContext::builder().with_schema(schema).build();
let simplifier = ExprSimplifier::new(context);

// b < 2
let b_lt_2 = col("b").gt(lit(2));

// (b < 2) OR (b < 2)
let expr = b_lt_2.clone().or(b_lt_2.clone());

// (b < 2) OR (b < 2) --> (b < 2)
let expr = simplifier.simplify(expr).unwrap();
assert_eq!(expr, b_lt_2);
```

<a id="op-bd6a2839af3bc69c7667430a"></a>
## simplify_with_cycle_count

`function` · `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier::simplify_with_cycle_count` · datafusion-optimizer 55.1.0

```rust
fn simplify_with_cycle_count(&self, expr: Expr) -> Result<(Expr, u32)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier", "path": "ExprSimplifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [428, 2], "filename": "src/simplify_expressions/expr_simplifier.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify_expressions/expr_simplifier.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Like [Self::simplify](../operations/datafusion_optimizer.simplify_expressions.expr_simplifier.ExprSimplifier.md#op-24b1e713502751d6ff36f0a4), simplifies this [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) as much as possible, evaluating
constants and applying algebraic simplifications. Additionally returns a `u32`
representing the number of simplification cycles performed, which can be useful for testing
optimizations.

See [Self::simplify](../operations/datafusion_optimizer.simplify_expressions.expr_simplifier.ExprSimplifier.md#op-24b1e713502751d6ff36f0a4) for details and usage examples.

<a id="op-043e18f52db5b5fa829d8a61"></a>
## simplify_with_cycle_count_transformed

`function` · `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier::simplify_with_cycle_count_transformed` · datafusion-optimizer 55.1.0

```rust
fn simplify_with_cycle_count_transformed(&self, expr: Expr) -> Result<(Transformed<Expr>, u32)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier", "path": "ExprSimplifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [428, 2], "filename": "src/simplify_expressions/expr_simplifier.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify_expressions/expr_simplifier.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Like [Self::simplify](../operations/datafusion_optimizer.simplify_expressions.expr_simplifier.ExprSimplifier.md#op-24b1e713502751d6ff36f0a4), simplifies this [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) as much as possible, evaluating
constants and applying algebraic simplifications. Additionally returns a `u32`
representing the number of simplification cycles performed, which can be useful for testing
optimizations.

# Returns

A tuple containing:
- The simplified expression wrapped in a `Transformed<Expr>` indicating if changes were made
- The number of simplification cycles that were performed

See [Self::simplify](../operations/datafusion_optimizer.simplify_expressions.expr_simplifier.ExprSimplifier.md#op-24b1e713502751d6ff36f0a4) for details and usage examples.

<a id="op-455496995dfd4c662d4710d6"></a>
## with_canonicalize

`function` · `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier::with_canonicalize` · datafusion-optimizer 55.1.0

```rust
fn with_canonicalize(self, canonicalize: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier", "path": "ExprSimplifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [428, 2], "filename": "src/simplify_expressions/expr_simplifier.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify_expressions/expr_simplifier.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Should `Canonicalizer` be applied before simplification?

If true (the default), the expression will be rewritten to canonical
form before simplification. This is useful to ensure that the simplifier
can apply all possible simplifications.

Some expressions, such as those in some Joins, can not be canonicalized
without changing their meaning. In these cases, canonicalization should
be disabled.

```rust
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_common::{Result, ScalarValue, ToDFSchema};
use datafusion_expr::interval_arithmetic::{Interval, NullableInterval};
use datafusion_expr::simplify::SimplifyContext;
use datafusion_expr::{col, lit, Expr};
use datafusion_optimizer::simplify_expressions::ExprSimplifier;

let schema = Schema::new(vec![
    Field::new("a", DataType::Int64, false),
    Field::new("b", DataType::Int64, false),
    Field::new("c", DataType::Int64, false),
])
.to_dfschema_ref()
.unwrap();

// Create the simplifier
let context = SimplifyContext::builder().with_schema(schema).build();
let simplifier = ExprSimplifier::new(context);

// Expression: a = c AND 1 = b
let expr = col("a").eq(col("c")).and(lit(1).eq(col("b")));

// With canonicalization, the expression is rewritten to canonical form
// (though it is no simpler in this case):
let canonical = simplifier.simplify(expr.clone()).unwrap();
// Expression has been rewritten to: (c = a AND b = 1)
assert_eq!(canonical, col("c").eq(col("a")).and(col("b").eq(lit(1))));

// If canonicalization is disabled, the expression is not changed
let non_canonicalized = simplifier
    .with_canonicalize(false)
    .simplify(expr.clone())
    .unwrap();

assert_eq!(non_canonicalized, expr);
```

<a id="op-06509272ae4ef694a1886eab"></a>
## with_guarantees

`function` · `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier::with_guarantees` · datafusion-optimizer 55.1.0

```rust
fn with_guarantees(self, guarantees: Vec<(Expr, NullableInterval)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier", "path": "ExprSimplifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [428, 2], "filename": "src/simplify_expressions/expr_simplifier.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify_expressions/expr_simplifier.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Input guarantees about the values of columns.

The guarantees can simplify expressions. For example, if a column `x` is
guaranteed to be `3`, then the expression `x > 1` can be replaced by the
literal `true`.

The guarantees are provided as a `Vec<(Expr, NullableInterval)>`,
where the [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) is a column reference and the [NullableInterval](../operations/datafusion_expr_common.interval_arithmetic.NullableInterval.md#op-09810b7c2ae237278cd95e21)
is an interval representing the known possible values of that column.

```rust
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_common::{Result, ScalarValue, ToDFSchema};
use datafusion_expr::interval_arithmetic::{Interval, NullableInterval};
use datafusion_expr::simplify::SimplifyContext;
use datafusion_expr::{col, lit, Expr};
use datafusion_optimizer::simplify_expressions::ExprSimplifier;

let schema = Schema::new(vec![
    Field::new("x", DataType::Int64, false),
    Field::new("y", DataType::UInt32, false),
    Field::new("z", DataType::Int64, false),
])
.to_dfschema_ref()
.unwrap();

// Create the simplifier
let context = SimplifyContext::builder().with_schema(schema).build();

// Expression: (x >= 3) AND (y + 2 < 10) AND (z > 5)
let expr_x = col("x").gt_eq(lit(3_i64));
let expr_y = (col("y") + lit(2_u32)).lt(lit(10_u32));
let expr_z = col("z").gt(lit(5_i64));
let expr = expr_x.and(expr_y).and(expr_z.clone());

let guarantees = vec![
    // x ∈ [3, 5]
    (
        col("x"),
        NullableInterval::NotNull {
            values: Interval::make(Some(3_i64), Some(5_i64)).unwrap(),
        },
    ),
    // y = 3
    (
        col("y"),
        NullableInterval::from(ScalarValue::UInt32(Some(3))),
    ),
];
let simplifier = ExprSimplifier::new(context).with_guarantees(guarantees);
let output = simplifier.simplify(expr).unwrap();
// Expression becomes: true AND true AND (z > 5), which simplifies to
// z > 5.
assert_eq!(output, expr_z);
```

<a id="op-d361401d7cb602289b7ee800"></a>
## with_max_cycles

`function` · `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier::with_max_cycles` · datafusion-optimizer 55.1.0

```rust
fn with_max_cycles(self, max_simplifier_cycles: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier", "path": "ExprSimplifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [428, 2], "filename": "src/simplify_expressions/expr_simplifier.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify_expressions/expr_simplifier.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Specifies the maximum number of simplification cycles to run.

The simplifier can perform multiple passes of simplification. This is
because the output of one simplification step can allow more optimizations
in another simplification step. For example, constant evaluation can allow more
expression simplifications, and expression simplifications can allow more constant
evaluations.

This method specifies the maximum number of allowed iteration cycles before the simplifier
returns an [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) output. However, it does not always perform the maximum number of cycles.
The simplifier will attempt to detect when an [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) is unchanged by all the simplification
passes, and return early. This avoids wasting time on unnecessary [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) tree traversals.

If no maximum is specified, the value of [DEFAULT_MAX_SIMPLIFIER_CYCLES](../operations/datafusion_optimizer.simplify_expressions.expr_simplifier.DEFAULT_MAX_SIMPLIFIER_CYCLES.md#op-ced3386e858f433c2f3aac0e) is used
instead.

```rust
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_expr::{col, lit, Expr};
use datafusion_common::{Result, ScalarValue, ToDFSchema};
use datafusion_expr::simplify::SimplifyContext;
use datafusion_optimizer::simplify_expressions::ExprSimplifier;

let schema = Schema::new(vec![
  Field::new("a", DataType::Int64, false),
  ])
  .to_dfschema_ref().unwrap();

// Create the simplifier
let context = SimplifyContext::builder().with_schema(schema).build();
let simplifier = ExprSimplifier::new(context);

// Expression: a IS NOT NULL
let expr = col("a").is_not_null();

// When using default maximum cycles, 2 cycles will be performed.
let (simplified_expr, count) = simplifier.simplify_with_cycle_count_transformed(expr.clone()).unwrap();
assert_eq!(simplified_expr.data, lit(true));
// 2 cycles were executed, but only 1 was needed
assert_eq!(count, 2);

// Only 1 simplification pass is necessary here, so we can set the maximum cycles to 1.
let (simplified_expr, count) = simplifier.with_max_cycles(1).simplify_with_cycle_count_transformed(expr.clone()).unwrap();
// Expression has been rewritten to: (c = a AND b = 1)
assert_eq!(simplified_expr.data, lit(true));
// Only 1 cycle was executed
assert_eq!(count, 1);
```
