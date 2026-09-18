# `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicateOp`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.opaque.ArrowOpaquePredicateOp.json).

<a id="op-cc579bb2abb02a2a2faf79a6"></a>
## ArrowOpaquePredicateOp

`trait` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicateOp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ArrowOpaquePredicateOp: DynPartialEq + std::fmt::Debug
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L47).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:47`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An arrow-enhanced opaque predicate op that supports full predicate evaluation over arrow data.

NOTE: This trait includes all methods of [`OpaquePredicateOp`](../operations/buoyant_kernel.expressions.OpaquePredicateOp.md#op-0af44540a36e1b39fab1b889), but intentionally does not
implement it. This is to prevent accidentally creating an [`Predicate`](../operations/buoyant_kernel.expressions.Predicate.md#op-0c0ee21b4ebfcd851e64ceb4) directly from an object
that implements this trait. Doing so would "clip" it [`&dyn OpaquePredicateOp`], and we would
not be able to recover a [`&dyn ArrowOpaquePredicateOp`] from it later. Instead, we use
[`ArrowOpaquePredicate::arrow_opaque`](../operations/buoyant_kernel.engine.arrow_expression.opaque.ArrowOpaquePredicate.md#op-2904c4f777d6369bc6490ceb) to safely convert it to an [`OpaquePredicateOp`](../operations/buoyant_kernel.expressions.OpaquePredicateOp.md#op-0af44540a36e1b39fab1b889).

<a id="op-b6b0883fb5cb8550170fa74c"></a>
## as_data_skipping_predicate

`function` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicateOp::as_data_skipping_predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_data_skipping_predicate(&self, predicate_evaluator: &IndirectDataSkippingPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> Option<Predicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L79).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:79`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`OpaquePredicateOp::as_data_skipping_predicate`](../operations/buoyant_kernel.expressions.OpaquePredicateOp.md#op-4bffdb86bb124935618292d9).

<a id="op-1726aa8c327e889761660d19"></a>
## eval_as_data_skipping_predicate

`function` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicateOp::eval_as_data_skipping_predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_as_data_skipping_predicate(&self, predicate_evaluator: &DirectDataSkippingPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L71).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:71`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`OpaquePredicateOp::eval_as_data_skipping_predicate`](../operations/buoyant_kernel.expressions.OpaquePredicateOp.md#op-e9f4bae529fd87b9c27127ec).

<a id="op-5ef37b43fbcc4a29ade9900a"></a>
## eval_pred

`function` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicateOp::eval_pred` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred(&self, args: &[Expression], batch: &RecordBatch, inverted: bool) -> DeltaResult<BooleanArray>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L51).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Evaluates this (possibly inverted) predicate over the provided input args, returning `Err`
in case of an incorrect or unsupported invocation (e.g. wrong number and/or types of
arguments.

<a id="op-5c04aad3943c6b4b692b2d1f"></a>
## eval_pred_scalar

`function` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicateOp::eval_pred_scalar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_scalar(&self, eval_expr: &ScalarExpressionEvaluator<'_>, eval_pred: &DirectPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> DeltaResult<Option<bool>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L62).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:62`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`OpaquePredicateOp::eval_pred_scalar`](../operations/buoyant_kernel.expressions.OpaquePredicateOp.md#op-571941d64fe303783048df47).

<a id="op-6faf5f13eb5080ec5283d5fb"></a>
## name

`function` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicateOp::name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn name(&self) -> &str
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L59).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:59`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`OpaquePredicateOp::name`](../operations/buoyant_kernel.expressions.OpaquePredicateOp.md#op-f21681d40215159ce425176e).
