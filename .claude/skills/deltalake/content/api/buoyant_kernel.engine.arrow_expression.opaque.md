# `buoyant_kernel::engine::arrow_expression::opaque`

Crate `buoyant_kernel` · 4 public items · structured records in [`model/buoyant_kernel.engine.arrow_expression.opaque.json`](../model/buoyant_kernel.engine.arrow_expression.opaque.json)

## ArrowOpaqueExpression

`trait` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpression`

Also reachable as `delta_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpression`

```rust
trait ArrowOpaqueExpression
```

**Implementors** (1)

- `buoyant_kernel::expressions::Expression`

**Methods** (1)

```rust
fn arrow_opaque(op: impl ArrowOpaqueExpressionOp, exprs: impl IntoIterator<Item = Expression>) -> Expression
```

Extension trait for turning [`ArrowOpaqueExpressionOp`] into an [`Expression`].

---

## ArrowOpaqueExpressionOp

`trait` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpressionOp`

Also reachable as `delta_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpressionOp`

```rust
trait ArrowOpaqueExpressionOp: DynPartialEq + std::fmt::Debug
```

**Methods** (3)

```rust
fn eval_expr(&self, args: &[Expression], batch: &RecordBatch, result_type: Option<&DataType>) -> DeltaResult<ArrayRef>
fn eval_expr_scalar(&self, eval_expr: &ScalarExpressionEvaluator<'_>, exprs: &[Expression]) -> DeltaResult<Scalar>
fn name(&self) -> &str
```

An arrow-enhanced opaque expression op that supports full expression evaluation over arrow data.

NOTE: This trait includes all methods of [`OpaqueExpressionOp`], but intentionally does not
implement it. This is to prevent accidentally creating an [`Expression`] directly from an object
that implements this trait. Doing so would "clip" it [`&dyn OpaqueExpressionOp`], and we would
not be able to recover a [`&dyn ArrowOpaqueExpressionOp`] from it later. Instead, we use
[`ArrowOpaqueExpression::arrow_opaque`] to safely convert it to an [`OpaqueExpressionOp`].

---

## ArrowOpaquePredicate

`trait` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicate`

Also reachable as `delta_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicate`

```rust
trait ArrowOpaquePredicate
```

**Implementors** (1)

- `buoyant_kernel::expressions::Predicate`

**Methods** (1)

```rust
fn arrow_opaque<T: ArrowOpaquePredicateOp>(op: T, exprs: impl IntoIterator<Item = Expression>) -> Predicate
```

Extension trait for safely turning [`ArrowOpaquePredicateOp`] into an opaque [`Predicate`].

---

## ArrowOpaquePredicateOp

`trait` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicateOp`

Also reachable as `delta_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicateOp`

```rust
trait ArrowOpaquePredicateOp: DynPartialEq + std::fmt::Debug
```

**Methods** (5)

```rust
fn as_data_skipping_predicate(&self, predicate_evaluator: &IndirectDataSkippingPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> Option<Predicate>
fn eval_as_data_skipping_predicate(&self, predicate_evaluator: &DirectDataSkippingPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> Option<bool>
fn eval_pred(&self, args: &[Expression], batch: &RecordBatch, inverted: bool) -> DeltaResult<BooleanArray>
fn eval_pred_scalar(&self, eval_expr: &ScalarExpressionEvaluator<'_>, eval_pred: &DirectPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> DeltaResult<Option<bool>>
fn name(&self) -> &str
```

An arrow-enhanced opaque predicate op that supports full predicate evaluation over arrow data.

NOTE: This trait includes all methods of [`OpaquePredicateOp`], but intentionally does not
implement it. This is to prevent accidentally creating an [`Predicate`] directly from an object
that implements this trait. Doing so would "clip" it [`&dyn OpaquePredicateOp`], and we would
not be able to recover a [`&dyn ArrowOpaquePredicateOp`] from it later. Instead, we use
[`ArrowOpaquePredicate::arrow_opaque`] to safely convert it to an [`OpaquePredicateOp`].

---
