# `datafusion_expr::function::WindowFunctionSimplification`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.function.WindowFunctionSimplification.json).

<a id="op-4e3825e9eb717b05a23f2e0d"></a>
## WindowFunctionSimplification

`type_alias` · `datafusion_expr::function::WindowFunctionSimplification` · datafusion-expr 55.1.0

```rust
type WindowFunctionSimplification = Box<dyn Fn(expr::WindowFunction, &simplify::SimplifyContext) -> datafusion_common::Result<Expr>>
```

Source: `src/function.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Type alias for [crate::udwf::WindowUDFImpl::simplify](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-c1739b18874ddfb371d89fe5).

This closure is invoked with:
* `window_function`: [WindowFunction](../operations/datafusion_expr.expr.WindowFunction.md#op-8f5e8e8a659430c3e7448f3a) with already simplified arguments
* `info`: [SimplifyContext](../operations/datafusion_expr.simplify.SimplifyContext.md#op-7b5998cee70553815ef2a10a)

It returns a simplified [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) or an error.
