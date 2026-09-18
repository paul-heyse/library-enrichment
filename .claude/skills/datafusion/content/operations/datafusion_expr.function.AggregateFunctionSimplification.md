# `datafusion_expr::function::AggregateFunctionSimplification`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.function.AggregateFunctionSimplification.json).

<a id="op-32ef850832a6ca8b0f6da43d"></a>
## AggregateFunctionSimplification

`type_alias` · `datafusion_expr::function::AggregateFunctionSimplification` · datafusion-expr 55.1.0

```rust
type AggregateFunctionSimplification = Box<dyn Fn(expr::AggregateFunction, &simplify::SimplifyContext) -> datafusion_common::Result<Expr>>
```

Source: `src/function.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Type alias for [crate::udaf::AggregateUDFImpl::simplify](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-97048a636c94a209ef572cb4).

This closure is invoked with:
* `aggregate_function`: [AggregateFunction](../operations/datafusion_expr.expr.AggregateFunction.md#op-2c30152da50e6f4ca7d2566e) with already simplified arguments
* `info`: [SimplifyContext](../operations/datafusion_expr.simplify.SimplifyContext.md#op-7b5998cee70553815ef2a10a)

It returns a simplified [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) or an error.
