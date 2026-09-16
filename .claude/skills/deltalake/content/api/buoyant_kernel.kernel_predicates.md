# `buoyant_kernel::kernel_predicates`

Crate `buoyant_kernel` · 6 public items · structured records in [`model/buoyant_kernel.kernel_predicates.json`](../model/buoyant_kernel.kernel_predicates.json)

## KernelPredicateEvaluatorDefaults

`struct` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults`

Also reachable as `delta_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults`

```rust
struct KernelPredicateEvaluatorDefaults
```

**Methods** (5)

```rust
fn eval_pred_binary_scalars(op: BinaryPredicateOp, left: &Scalar, right: &Scalar, inverted: bool) -> Option<bool>
fn eval_pred_scalar(val: &Scalar, inverted: bool) -> Option<bool>
fn eval_pred_scalar_is_null(val: &Scalar, inverted: bool) -> Option<bool>
fn finish_eval_pred_junction(op: JunctionPredicateOp, preds: &mut dyn Iterator<Item = Option<bool>>, inverted: bool) -> Option<bool>
fn partial_cmp_scalars(ord: Ordering, a: &Scalar, b: &Scalar, inverted: bool) -> Option<bool>
```

A collection of provided methods from the [`KernelPredicateEvaluator`] trait, factored out to
allow reuse by multiple bool-output predicate evaluator implementations.

---

## DataSkippingPredicateEvaluator

`trait` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator`

Also reachable as `delta_kernel::kernel_predicates::DataSkippingPredicateEvaluator`

```rust
trait DataSkippingPredicateEvaluator
```

**Implementors** (2)

- `buoyant_kernel::scan::data_skipping::DataSkippingPredicateCreator`
- `buoyant_kernel::scan::data_skipping::NullGuardedDataSkippingPredicateCreator`

**Methods** (16)

```rust
fn eval_partial_cmp(&self, ord: Ordering, col: Self::ColumnStat, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_binary_scalars(&self, op: BinaryPredicateOp, left: &Scalar, right: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_eq(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_gt(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_is_null(&self, col: &ColumnName, inverted: bool) -> Option<Self::Output>
fn eval_pred_lt(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_opaque(&self, op: &OpaquePredicateOpRef, exprs: &[Expr], inverted: bool) -> Option<Self::Output>
fn eval_pred_scalar(&self, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_scalar_is_null(&self, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn finish_eval_pred_junction(&self, op: JunctionPredicateOp, preds: &mut dyn Iterator<Item = Option<Self::Output>>, inverted: bool) -> Option<Self::Output>
fn get_max_stat(&self, col: &ColumnName, data_type: &DataType) -> Option<Self::ColumnStat>
fn get_min_stat(&self, col: &ColumnName, data_type: &DataType) -> Option<Self::ColumnStat>
fn get_nullcount_stat(&self, col: &ColumnName) -> Option<Self::ColumnStat>
fn get_rowcount_stat(&self) -> Option<Self::ColumnStat>
fn partial_cmp_max_stat(&self, col: &ColumnName, val: &Scalar, ord: Ordering, inverted: bool) -> Option<Self::Output>
fn partial_cmp_min_stat(&self, col: &ColumnName, val: &Scalar, ord: Ordering, inverted: bool) -> Option<Self::Output>
```

A predicate evaluator that implements data skipping semantics over various column stats. For
example, comparisons involving a column are converted into comparisons over that column's
min/max stats, and NULL checks are converted into comparisons involving the column's nullcount
and rowcount stats.

---

## KernelPredicateEvaluator

`trait` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator`

Also reachable as `delta_kernel::kernel_predicates::KernelPredicateEvaluator`

```rust
trait KernelPredicateEvaluator
```

**Implementors** (1)

- `buoyant_kernel::kernel_predicates::DefaultKernelPredicateEvaluator`

**Methods** (23)

```rust
fn eval(&self, pred: &Pred) -> Option<Self::Output>
fn eval_pred(&self, pred: &Pred, inverted: bool) -> Option<Self::Output>
fn eval_pred_binary(&self, op: BinaryPredicateOp, left: &Expr, right: &Expr, inverted: bool) -> Option<Self::Output>
fn eval_pred_binary_columns(&self, op: BinaryPredicateOp, a: &ColumnName, b: &ColumnName, inverted: bool) -> Option<Self::Output>
fn eval_pred_binary_scalars(&self, op: BinaryPredicateOp, left: &Scalar, right: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_column(&self, col: &ColumnName, inverted: bool) -> Option<Self::Output>
fn eval_pred_distinct(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_eq(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_expr(&self, expr: &Expr, inverted: bool) -> Option<Self::Output>
fn eval_pred_expr_opaque(&self, op: &OpaqueExpressionOpRef, exprs: &[Expr], inverted: bool) -> Option<Self::Output>
fn eval_pred_gt(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_in(&self, _col: &ColumnName, _val: &Scalar, _inverted: bool) -> Option<Self::Output>
fn eval_pred_is_null(&self, col: &ColumnName, inverted: bool) -> Option<Self::Output>
fn eval_pred_junction(&self, op: JunctionPredicateOp, preds: &[Pred], inverted: bool) -> Option<Self::Output>
fn eval_pred_lt(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_not(&self, pred: &Pred, inverted: bool) -> Option<Self::Output>
fn eval_pred_opaque(&self, op: &OpaquePredicateOpRef, exprs: &[Expr], inverted: bool) -> Option<Self::Output>
fn eval_pred_scalar(&self, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_scalar_is_null(&self, val: &Scalar, inverted: bool) -> Option<Self::Output>
fn eval_pred_sql_where(&self, pred: &Pred, inverted: bool) -> Option<Self::Output>
fn eval_pred_unary(&self, op: UnaryPredicateOp, expr: &Expr, inverted: bool) -> Option<Self::Output>
fn eval_sql_where(&self, pred: &Pred) -> Option<Self::Output>
fn finish_eval_pred_junction(&self, op: JunctionPredicateOp, preds: &mut dyn Iterator<Item = Option<Self::Output>>, inverted: bool) -> Option<Self::Output>
```

Uses kernel (not engine) logic to evaluate a predicate tree against column names that resolve as
scalars. Useful for testing/debugging but also serves as a reference implementation that
documents the expression semantics that kernel relies on for data skipping.

# Inverted expression semantics

Because inversion (`NOT` operator) has special semantics and can often be optimized away by
pushing it down, most methods take an `inverted` flag. That allows operations like [`Pred::Not`]
to simply evaluate their operand with a flipped `inverted` flag, and greatly simplifies the
implementations of most operators (other than those which have to directly implement NOT
semantics, which are unavoidably complex in that regard).

# Parameterized output type

The types involved in predicate evaluation are parameterized and implementation-specific. For
example, a [`DirectDataSkippingPredicateEvaluator`] directly evaluates the predicate (e.g. using
parquet footer stats) and returns boolean results, while
[`IndirectDataSkippingPredicateEvaluator`] instead transforms the input predicate to a data
skipping predicate that the engine can evaluated directly against Delta data skipping stats
during log replay. Although this approach is harder to read and reason about at first, the
majority of predicates can be implemented generically, which greatly reduces redundancy and
ensures that all flavors of predicate evaluation have the same semantics.

# NULL and error semantics

Literal NULL values almost always produce cascading changes in the predicate's structure, so we
represent them by `Option::None` rather than `Scalar::Null`. This allows e.g. `A < NULL` to be
rewritten as `NULL`, or `AND(NULL, FALSE)` to be rewritten as `FALSE`.

Almost all operations produce NULL output if any input is `NULL`. Any resolution failures also
produce NULL (such as missing columns or type mismatch between a column and the scalar it is
compared against). NULL-checking operations like `IS [NOT] NULL` and `DISTINCT` are special, and
rely on nullcount stats for their work (NULL/missing nullcount stats makes them output NULL).

For safety reasons, NULL-checking operations only accept literal and column inputs where
stats-based skipping is well-defined. If an arbitrary data skipping predicate evaluates to
NULL, there is no way to tell whether the original predicate really evaluated to NULL (safe to
use), or the data skipping version evaluated to NULL due to missing stats (very unsafe to use).

NOTE: The error-handling semantics of this trait's scalar-based predicate evaluation may differ
from those of the engine's predicate evaluation, because kernel predicates don't include the
necessary type information to reliably detect all type errors.

---

## DirectDataSkippingPredicateEvaluator

`type_alias` · `buoyant_kernel::kernel_predicates::DirectDataSkippingPredicateEvaluator`

Also reachable as `delta_kernel::kernel_predicates::DirectDataSkippingPredicateEvaluator`

```rust
type DirectDataSkippingPredicateEvaluator<'a> = dyn DataSkippingPredicateEvaluator<Output = bool, ColumnStat = expressions::Scalar> + 'a
```

A data skipping predicate evaluator that directly applies data skipping, resolving column
references to scalar stats values such as those provided by parquet footer stats.

---

## DirectPredicateEvaluator

`type_alias` · `buoyant_kernel::kernel_predicates::DirectPredicateEvaluator`

Also reachable as `delta_kernel::kernel_predicates::DirectPredicateEvaluator`

```rust
type DirectPredicateEvaluator<'a> = dyn KernelPredicateEvaluator<Output = bool> + 'a
```

A predicate evaluator that directly evaluates predicates, resolving column references to scalar
values.

---

## IndirectDataSkippingPredicateEvaluator

`type_alias` · `buoyant_kernel::kernel_predicates::IndirectDataSkippingPredicateEvaluator`

Also reachable as `delta_kernel::kernel_predicates::IndirectDataSkippingPredicateEvaluator`

```rust
type IndirectDataSkippingPredicateEvaluator<'a> = dyn DataSkippingPredicateEvaluator<Output = expressions::Predicate, ColumnStat = expressions::Expression> + 'a
```

A data skipping predicate evaluator that rewrites the input to a predicate that performs data
skipping over column stats for all referenced columns. The resulting predicate can be evaluated
against batches of column stats at some future point.

---
