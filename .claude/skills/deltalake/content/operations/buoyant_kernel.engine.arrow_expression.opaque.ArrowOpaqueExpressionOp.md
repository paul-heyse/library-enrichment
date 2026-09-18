# `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpressionOp`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.opaque.ArrowOpaqueExpressionOp.json).

<a id="op-80f92f1fe42bcae22548fb12"></a>
## ArrowOpaqueExpressionOp

`trait` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpressionOp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ArrowOpaqueExpressionOp: DynPartialEq + std::fmt::Debug
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L19).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An arrow-enhanced opaque expression op that supports full expression evaluation over arrow data.

NOTE: This trait includes all methods of [`OpaqueExpressionOp`](../operations/buoyant_kernel.expressions.OpaqueExpressionOp.md#op-353b5fa46cb006e99cf79ad2), but intentionally does not
implement it. This is to prevent accidentally creating an [`Expression`](../operations/buoyant_kernel.expressions.Expression.md#op-b6e8a7405239f2ae57fc5e51) directly from an object
that implements this trait. Doing so would "clip" it [`&dyn OpaqueExpressionOp`], and we would
not be able to recover a [`&dyn ArrowOpaqueExpressionOp`] from it later. Instead, we use
[`ArrowOpaqueExpression::arrow_opaque`](../operations/buoyant_kernel.engine.arrow_expression.opaque.ArrowOpaqueExpression.md#op-9162d663f3d217de7bfae426) to safely convert it to an [`OpaqueExpressionOp`](../operations/buoyant_kernel.expressions.OpaqueExpressionOp.md#op-353b5fa46cb006e99cf79ad2).

<a id="op-1a2317cc2e1109ea91500caa"></a>
## eval_expr

`function` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpressionOp::eval_expr` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_expr(&self, args: &[Expression], batch: &RecordBatch, result_type: Option<&DataType>) -> DeltaResult<ArrayRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L22).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:22`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Evaluates this expression over the provided input expressions, returning `Err` in case of an
incorrect or unsupported invocation (e.g. wrong number and/or types of arguments.

<a id="op-fd988b967d1684d35340b1a8"></a>
## eval_expr_scalar

`function` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpressionOp::eval_expr_scalar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_expr_scalar(&self, eval_expr: &ScalarExpressionEvaluator<'_>, exprs: &[Expression]) -> DeltaResult<Scalar>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L33).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:33`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`OpaqueExpressionOp::eval_expr_scalar`](../operations/buoyant_kernel.expressions.OpaqueExpressionOp.md#op-72e53dd7b620c240b2eb2046).

<a id="op-25ba614f849d53f55a330554"></a>
## name

`function` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpressionOp::name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn name(&self) -> &str
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L30).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:30`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`OpaqueExpressionOp::name`](../operations/buoyant_kernel.expressions.OpaqueExpressionOp.md#op-7ea5181e7923a8f2e1ebb861).
