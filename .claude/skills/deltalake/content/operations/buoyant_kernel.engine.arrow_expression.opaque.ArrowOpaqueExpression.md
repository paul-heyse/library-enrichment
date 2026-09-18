# `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpression`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.opaque.ArrowOpaqueExpression.json).

<a id="op-0dfc33527356571a2c4b569f"></a>
## ArrowOpaqueExpression

`trait` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpression` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ArrowOpaqueExpression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L88).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:88`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extension trait for turning [`ArrowOpaqueExpressionOp`](../operations/buoyant_kernel.engine.arrow_expression.opaque.ArrowOpaqueExpressionOp.md#op-80f92f1fe42bcae22548fb12) into an [`Expression`](../operations/buoyant_kernel.expressions.Expression.md#op-b6e8a7405239f2ae57fc5e51).

<a id="op-9162d663f3d217de7bfae426"></a>
## arrow_opaque

`function` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaqueExpression::arrow_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn arrow_opaque(op: impl ArrowOpaqueExpressionOp, exprs: impl IntoIterator<Item = Expression>) -> Expression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L90).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:90`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new opaque expression. See also [`Expression::opaque`](../operations/buoyant_kernel.expressions.Expression.md#op-fafc306d0bf13887c9564f57).
