# `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.json).

<a id="op-1dc6ecdedb87f64e3b8474da"></a>
## KernelPredicateEvaluator

`trait` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait KernelPredicateEvaluator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L87).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:87`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Uses kernel (not engine) logic to evaluate a predicate tree against column names that resolve as
scalars. Useful for testing/debugging but also serves as a reference implementation that
documents the expression semantics that kernel relies on for data skipping.

# Inverted expression semantics

Because inversion (`NOT` operator) has special semantics and can often be optimized away by
pushing it down, most methods take an `inverted` flag. That allows operations like [`Pred::Not`](../operations/buoyant_kernel.expressions.Predicate.md#op-1e98d189910f98d1378713b1)
to simply evaluate their operand with a flipped `inverted` flag, and greatly simplifies the
implementations of most operators (other than those which have to directly implement NOT
semantics, which are unavoidably complex in that regard).

# Parameterized output type

The types involved in predicate evaluation are parameterized and implementation-specific. For
example, a [`DirectDataSkippingPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.DirectDataSkippingPredicateEvaluator.md#op-55861932ef47a08b45ec2d61) directly evaluates the predicate (e.g. using
parquet footer stats) and returns boolean results, while
[`IndirectDataSkippingPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.IndirectDataSkippingPredicateEvaluator.md#op-c35596042fd4d3b57c5f5523) instead transforms the input predicate to a data
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

<a id="op-0f59ec25ff69223532cf2f47"></a>
## Output

`assoc_type` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L88).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:88`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57f59849aae4361e9e9749e6"></a>
## eval

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval(&self, pred: &Pred) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L494).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:494`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A convenient non-inverted wrapper for [`Self::eval_pred`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-cadd45808fb9b9ca2fdc225a)

<a id="op-cadd45808fb9b9ca2fdc225a"></a>
## eval_pred

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred(&self, pred: &Pred, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L314).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:314`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Dispatches a predicate to the specific implementation for each predicate variant.

<a id="op-10182bed19635a02e0457f48"></a>
## eval_pred_binary

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_binary(&self, op: BinaryPredicateOp, left: &Expr, right: &Expr, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L266).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:266`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Dispatches a (possibly inverted) binary expression to each operator's specific
implementation.

NOTE: Only binary operators that produce boolean outputs are supported.

<a id="op-7d78a3236edae7cbeae05b01"></a>
## eval_pred_binary_columns

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_binary_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_binary_columns(&self, op: BinaryPredicateOp, a: &ColumnName, b: &ColumnName, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L121).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) comparison between two columns, e.g. `<colA> != <colB>`.

<a id="op-1da9dd1156dc513bb7d27b4a"></a>
## eval_pred_binary_scalars

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_binary_scalars` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_binary_scalars(&self, op: BinaryPredicateOp, left: &Scalar, right: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L112).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:112`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) comparison between two scalars, e.g. `<valueA> != <valueB>`.

<a id="op-41f26763e60b4f1f9d9c676c"></a>
## eval_pred_column

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_column(&self, col: &ColumnName, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L160).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:160`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) boolean column access, e.g. `[NOT] <col>`.

<a id="op-56498eec1fdcce2498db5a3c"></a>
## eval_pred_distinct

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_distinct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_distinct(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L232).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:232`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) DISTINCT test, e.g. `[NOT] DISTINCT(<col>, false)`. DISTINCT can be
seen as one of two operations, depending on the input:

1. `DISTINCT(<col>, NULL)` is equivalent to `<col> IS NOT NULL`
2. `DISTINCT(<col>, <value>)` is equivalent to `OR(<col> IS NULL, <col> != <value>)`

<a id="op-289effecc0f85f6dfd5fb993"></a>
## eval_pred_eq

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_eq(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L109).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:109`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) equality comparison, e.g. `<col> = <value>` or `<col> != <value>`.

NOTE: Caller is responsible to commute the operation if needed, e.g. `<value> != <col>`
becomes `<col> != <value>`.

<a id="op-a1ee7eee5bee23ddc1eee90b"></a>
## eval_pred_expr

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_expr` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_expr(&self, expr: &Expr, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L172).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:172`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Dispatches a (possibly inverted) boolean expression used as a predicate

<a id="op-a5e2246749588dca4b33a433"></a>
## eval_pred_expr_opaque

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_expr_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_expr_opaque(&self, op: &OpaqueExpressionOpRef, exprs: &[Expr], inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L138).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:138`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Dispatches an opaque expression used as a predicate

<a id="op-68551f2464605fe81c3257e5"></a>
## eval_pred_gt

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_gt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_gt(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L103).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) greater-than comparison, e.g. `<col> > <value>`

<a id="op-c7c87f133b2c52c096ca27f5"></a>
## eval_pred_in

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_in` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_in(&self, _col: &ColumnName, _val: &Scalar, _inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L253).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:253`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) IN-list check, e.g. `<col> [NOT] IN <array-value>`.

Unsupported by default, but implementations can override it if they wish.

<a id="op-9343709f6c2f30a8f1984bc2"></a>
## eval_pred_is_null

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_is_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_is_null(&self, col: &ColumnName, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L97).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) NULL check, e.g. `<expr> IS [NOT] NULL`.

<a id="op-61c5cf8194238e89aec3971c"></a>
## eval_pred_junction

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_junction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_junction(&self, op: JunctionPredicateOp, preds: &[Pred], inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L303).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:303`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Dispatches a predicate junction operation (AND or OR), leveraging each implementation's
[`Self::finish_eval_pred_junction`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-0ee991f832c056b15d36ce28).

<a id="op-0651dedc7ebf7c50809f49c4"></a>
## eval_pred_lt

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_lt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_lt(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L100).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:100`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) less-than comparison, e.g. `<col> < <value>`.

<a id="op-a3044e463a3efacf51a559c4"></a>
## eval_pred_not

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_not` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_not(&self, pred: &Pred, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L167).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:167`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Dispatches a (possibly inverted) NOT predicate

<a id="op-7049b694c2404d487d9852d7"></a>
## eval_pred_opaque

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_opaque(&self, op: &OpaquePredicateOpRef, exprs: &[Expr], inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L130).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:130`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Dispatches an opaque predicate.

<a id="op-43819d3f9c8df9f0fc23a842"></a>
## eval_pred_scalar

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_scalar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_scalar(&self, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L91).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:91`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) boolean scalar value, e.g. `[NOT] <value>`.

<a id="op-279e486aa8a990294c509fe9"></a>
## eval_pred_scalar_is_null

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_scalar_is_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_scalar_is_null(&self, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L94).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:94`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) scalar NULL test, e.g. `<value> IS [NOT] NULL`.

<a id="op-f28210f863ed9143e1821ff4"></a>
## eval_pred_sql_where

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_sql_where` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_sql_where(&self, pred: &Pred, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L437).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:437`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Evaluates a (possibly inverted) predicate with SQL WHERE semantics.

NOTE: A NULL literal in a boolean position is treated as unknown (not false), because
callers like `build_actions_meta_predicate` use NULL as a sentinel for unsupported arms.
Treating it as false would let `AND(supported, NULL)` incorrectly prune files.

By default, [`Self::eval_pred`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-cadd45808fb9b9ca2fdc225a) behaves badly for comparisons involving NULL columns
(e.g. `a < 10` when `a` is NULL), because the comparison correctly evaluates to NULL, but
NULL values are interpreted as "stats missing" (= cannot skip). This ambiguity can "poison"
the entire predicate, causing it to return NULL instead of FALSE that would allow skipping:

```text
WHERE a < 10 -- NULL (can't skip file)
WHERE a < 10 AND TRUE -- NULL (can't skip file)
WHERE a < 10 OR FALSE -- NULL (can't skip file)
```

Meanwhile, SQL WHERE semantics only keeps rows for which the filter evaluates to
TRUE (discarding rows that evaluate to FALSE or NULL):

```text
WHERE a < 10 -- NULL (discard row)
WHERE a < 10 AND TRUE -- NULL (discard row)
WHERE a < 10 OR FALSE -- NULL (discard row)
```

Conceptually, the behavior difference between data skipping and SQL WHERE semantics can be
addressed by evaluating with null-safe semantics, as if by `<expr> IS NOT NULL AND <expr>`:

```text
WHERE (a < 10) IS NOT NULL AND (a < 10) -- FALSE (skip file)
WHERE (a < 10 AND TRUE) IS NOT NULL AND (a < 10 AND TRUE) -- FALSE (skip file)
WHERE (a < 10 OR FALSE) IS NOT NULL AND (a < 10 OR FALSE) -- FALSE (skip file)
```

HOWEVER, we cannot safely NULL-check the result of an arbitrary data skipping predicate
because a predicate will also produce NULL if the value is just plain missing (e.g. data
skipping over a column that lacks stats), and if that NULL should propagate all the way to
top-level, it would be wrongly interpreted as FALSE (= skippable).

To prevent wrong data skipping, the predicate evaluator always returns NULL for a NULL check
over anything except for literals and columns with known values. So we must push the NULL
check down through supported operations (AND as well as null-intolerant comparisons like
`<`, `!=`, etc) until it reaches columns and literals where it can do some good, e.g.:

```text
WHERE a < 10 AND (b < 20 OR c < 30)
```

would conceptually be interpreted as

```text
WHERE
  (a < 10 AND (b < 20 OR c < 30)) IS NOT NULL AND
  (a < 10 AND (b < 20 OR c < 30))
```

We then push the NULL check down through the top-level AND:

```text
WHERE
  (a < 10 IS NOT NULL AND a < 10) AND
  ((b < 20 OR c < 30) IS NOT NULL AND (b < 20 OR c < 30))
```

and attempt to push it further into the `a < 10` and `OR` clauses:

```text
WHERE
  (a IS NOT NULL AND 10 IS NOT NULL AND a < 10) AND
  (b < 20 OR c < 30)
```

Any time the push-down reaches an operator that does not support push-down (such as OR), we
simply drop the NULL check. This way, the top-level NULL check only applies to
sub-predicates that can safely implement it, while ignoring other sub-predicates. The
unsupported sub-predicates could produce nulls at runtime that prevent skipping, but false
positives are OK -- the query will still correctly filter out the unwanted rows that result.

At predicate evaluation time, a NULL value of `a` (from our example) would evaluate as:

```text
AND(..., AND(a IS NOT NULL, 10 IS NOT NULL, a < 10), ...)
AND(..., AND(FALSE, TRUE, NULL), ...)
AND(..., FALSE, ...)
FALSE
```

While a non-NULL value of `a` would instead evaluate as:

```text
AND(..., AND(a IS NOT NULL, 10 IS NOT NULL, a < 10), ...)
AND(..., AND(TRUE, TRUE, <result>), ...)
AND(..., <result>, ...)
```

And a missing value for `a` would safely disable the clause:

```text
AND(..., AND(a IS NOT NULL, 10 IS NOT NULL, a < 10), ...)
AND(..., AND(NULL, TRUE, NULL), ...)
AND(..., NULL, ...)
```

WARNING: Not an idempotent transform. If data skipping eval produces a sql predicate,
evaluating the result with sql semantics has undefined behavior.

<a id="op-ba435f74e66c13bc7e534406"></a>
## eval_pred_unary

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_pred_unary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_unary(&self, op: UnaryPredicateOp, expr: &Expr, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L195).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:195`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Dispatches a (possibly inverted) unary expression to each operator's specific
implementation.

<a id="op-d5cb83328275180b4540b092"></a>
## eval_sql_where

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::eval_sql_where` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_sql_where(&self, pred: &Pred) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L499).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:499`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A convenient non-inverted wrapper for [`Self::eval_pred_sql_where`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-f28210f863ed9143e1821ff4).

<a id="op-0ee991f832c056b15d36ce28"></a>
## finish_eval_pred_junction

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluator::finish_eval_pred_junction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn finish_eval_pred_junction(&self, op: JunctionPredicateOp, preds: &mut dyn Iterator<Item = Option<Self::Output>>, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L150).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:150`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Completes evaluation of a (possibly inverted) junction predicate.

AND and OR are implemented by first evaluating its (possibly inverted) inputs. This part is
always the same, provided by [`Self::eval_pred_junction`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-61c5cf8194238e89aec3971c)). The results are then combined to
become the predicate's output in some implementation-defined way (this method).
