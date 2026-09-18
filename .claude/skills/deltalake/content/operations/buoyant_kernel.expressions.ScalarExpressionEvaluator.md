# `buoyant_kernel::expressions::ScalarExpressionEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.ScalarExpressionEvaluator.json).

<a id="op-48e1e0e9457531f77d34d9eb"></a>
## ScalarExpressionEvaluator

`type_alias` · `buoyant_kernel::expressions::ScalarExpressionEvaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type ScalarExpressionEvaluator<'a> = dyn Fn(&Expression) -> Option<Scalar> + 'a
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L134).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:134`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A kernel-supplied scalar expression evaluator which in particular can convert column references
(i.e. [`Expression::Column`](../operations/buoyant_kernel.expressions.Expression.md#op-5c6f2251feb782a0ea3b0a8f)) to [`Scalar`](../operations/buoyant_kernel.expressions.scalars.Scalar.md#op-8dd45baeed3da91aa357441a) values. [`OpaqueExpressionOp::eval_expr_scalar`](../operations/buoyant_kernel.expressions.OpaqueExpressionOp.md#op-72e53dd7b620c240b2eb2046) and
[`OpaquePredicateOp::eval_pred_scalar`](../operations/buoyant_kernel.expressions.OpaquePredicateOp.md#op-571941d64fe303783048df47) rely on this evaluator.

If the evaluator produces `None`, it means kernel was unable to evaluate
the input expression. Otherwise, `Some(Scalar)` is the result of that evaluation (possibly
`Scalar::Null` if the output was NULL).
