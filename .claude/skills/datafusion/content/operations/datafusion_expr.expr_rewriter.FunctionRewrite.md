# `datafusion_expr::expr_rewriter::FunctionRewrite`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.FunctionRewrite.json).

<a id="op-d16387f6b2d1bd76e4ec29b7"></a>
## FunctionRewrite

`trait` · `datafusion_expr::expr_rewriter::FunctionRewrite` · datafusion-expr 55.1.0

```rust
trait FunctionRewrite: Debug
```

Source: `src/expr_rewriter/mod.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Trait for rewriting [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)s into function calls.

This trait is used with `FunctionRegistry::register_function_rewrite` to
to evaluating `Expr`s using functions that may not be built in to DataFusion

For example, concatenating arrays `a || b` is represented as
`Operator::ArrowAt`, but can be implemented by calling a function
`array_concat` from the `functions-nested` crate.

<a id="op-352fa6244e8b7b2f77872ab7"></a>
## name

`function` · `datafusion_expr::expr_rewriter::FunctionRewrite::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/expr_rewriter/mod.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a human readable name for this rewrite

<a id="op-05c0562531c3a8fed6299326"></a>
## rewrite

`function` · `datafusion_expr::expr_rewriter::FunctionRewrite::rewrite` · datafusion-expr 55.1.0

```rust
fn rewrite(&self, expr: Expr, schema: &DFSchema, config: &ConfigOptions) -> Result<Transformed<Expr>>
```

Source: `src/expr_rewriter/mod.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Potentially rewrite `expr` to some other expression

Note that recursion is handled by the caller -- this method should only
handle `expr`, not recurse to its children.
