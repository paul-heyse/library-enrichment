# `buoyant_kernel::expressions::OpaqueExpressionOp`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.OpaqueExpressionOp.json).

<a id="op-353b5fa46cb006e99cf79ad2"></a>
## OpaqueExpressionOp

`trait` · `buoyant_kernel::expressions::OpaqueExpressionOp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait OpaqueExpressionOp: DynPartialEq + std::fmt::Debug
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L137).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:137`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An opaque expression operation (ie defined and implemented by the engine).

<a id="op-72e53dd7b620c240b2eb2046"></a>
## eval_expr_scalar

`function` · `buoyant_kernel::expressions::OpaqueExpressionOp::eval_expr_scalar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_expr_scalar(&self, eval_expr: &ScalarExpressionEvaluator<'_>, exprs: &[Expression]) -> DeltaResult<Scalar>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L151).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:151`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attempts scalar evaluation of this opaque expression, e.g. for partition pruning.

Implementations can evaluate the child expressions however they see fit, possibly by
calling back to the provided [`ScalarExpressionEvaluator`](../operations/buoyant_kernel.expressions.ScalarExpressionEvaluator.md#op-48e1e0e9457531f77d34d9eb),

An output of `Err` indicates that this operation does not support scalar evaluation, or was
invoked incorrectly (e.g. with the wrong number and/or types of arguments, None input,
etc); the operation is disqualified from participating in partition pruning.

`Ok(Scalar::Null)` means the operation actually produced a legitimately NULL result.

<a id="op-7ea5181e7923a8f2e1ebb861"></a>
## name

`function` · `buoyant_kernel::expressions::OpaqueExpressionOp::name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn name(&self) -> &str
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L139).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:139`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Succinctly identifies this op
