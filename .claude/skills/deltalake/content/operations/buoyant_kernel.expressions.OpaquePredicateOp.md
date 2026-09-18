# `buoyant_kernel::expressions::OpaquePredicateOp`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.OpaquePredicateOp.json).

<a id="op-0af44540a36e1b39fab1b889"></a>
## OpaquePredicateOp

`trait` · `buoyant_kernel::expressions::OpaquePredicateOp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait OpaquePredicateOp: DynPartialEq + std::fmt::Debug
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L159).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:159`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An opaque predicate operation (ie defined and implemented by the engine).

<a id="op-4bffdb86bb124935618292d9"></a>
## as_data_skipping_predicate

`function` · `buoyant_kernel::expressions::OpaquePredicateOp::as_data_skipping_predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_data_skipping_predicate(&self, evaluator: &IndirectDataSkippingPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> Option<Predicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L211).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:211`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Converts this (possibly inverted) opaque predicate to a data skipping predicate on behalf of
an [`IndirectDataSkippingPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.IndirectDataSkippingPredicateEvaluator.md#op-c35596042fd4d3b57c5f5523), e.g. for stats-based file pruning.

Implementations can transform the predicate and its child expressions however they see fit,
possibly by calling back to the owning [`IndirectDataSkippingPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.IndirectDataSkippingPredicateEvaluator.md#op-c35596042fd4d3b57c5f5523).

An output of `None` indicates that this operation does not support conversion to a data
skipping predicate, or was invoked incorrectly (e.g. wrong number and/or types of arguments,
None input, etc.); the operation is disqualified from participating in file pruning.

<a id="op-e9f4bae529fd87b9c27127ec"></a>
## eval_as_data_skipping_predicate

`function` · `buoyant_kernel::expressions::OpaquePredicateOp::eval_as_data_skipping_predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_as_data_skipping_predicate(&self, evaluator: &DirectDataSkippingPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L192).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:192`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Evaluates this (possibly inverted) opaque predicate for data skipping on behalf of a
[`DirectDataSkippingPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.DirectDataSkippingPredicateEvaluator.md#op-55861932ef47a08b45ec2d61), e.g. for parquet row group skipping.

Implementations can evaluate the child expressions however they see fit, possibly by
calling back to the provided [`DirectDataSkippingPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.DirectDataSkippingPredicateEvaluator.md#op-55861932ef47a08b45ec2d61).

An output of `None` indicates that this operation does not support evaluation as a data
skipping predicate, or was invoked incorrectly (e.g. wrong number and/or types of arguments,
None input, etc.); the operation is disqualified from participating in row group skipping.

<a id="op-571941d64fe303783048df47"></a>
## eval_pred_scalar

`function` · `buoyant_kernel::expressions::OpaquePredicateOp::eval_pred_scalar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_scalar(&self, eval_expr: &ScalarExpressionEvaluator<'_>, eval_pred: &DirectPredicateEvaluator<'_>, exprs: &[Expression], inverted: bool) -> DeltaResult<Option<bool>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L175).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:175`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attempts scalar evaluation of this (possibly inverted) opaque predicate on behalf of a
[`DirectPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.DirectPredicateEvaluator.md#op-ae005f39b2a93dbdf09be0fc), e.g. for partition pruning or to evaluate an opaque data
skipping predicate produced previously by an [`IndirectDataSkippingPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.IndirectDataSkippingPredicateEvaluator.md#op-c35596042fd4d3b57c5f5523).

Implementations can evaluate the child expressions however they see fit, possibly by calling
back to the provided [`ScalarExpressionEvaluator`](../operations/buoyant_kernel.expressions.ScalarExpressionEvaluator.md#op-48e1e0e9457531f77d34d9eb) and/or [`DirectPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.DirectPredicateEvaluator.md#op-ae005f39b2a93dbdf09be0fc).

An output of `Err` indicates that this operation does not support scalar evaluation, or was
invoked incorrectly (e.g. wrong number and/or types of arguments, None input, etc); the
operation is disqualified from participating in partition pruning and/or data skipping.

`Ok(None)` means the operation actually produced a legitimately NULL output.

<a id="op-f21681d40215159ce425176e"></a>
## name

`function` · `buoyant_kernel::expressions::OpaquePredicateOp::name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn name(&self) -> &str
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L161).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:161`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Succinctly identifies this op
