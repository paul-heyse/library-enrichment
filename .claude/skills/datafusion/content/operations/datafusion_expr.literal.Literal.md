# `datafusion_expr::literal::Literal`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.literal.Literal.json).

<a id="op-f6cbc5980ed7f3addde2c519"></a>
## Literal

`trait` · `datafusion_expr::literal::Literal` · datafusion-expr 55.1.0

```rust
trait Literal
```

Source: `src/literal.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Trait for converting a type to a [`Literal`](../operations/datafusion_expr.literal.Literal.md#op-f6cbc5980ed7f3addde2c519) literal expression.

<a id="op-217892fbf47fac65a39ac58c"></a>
## lit

`function` · `datafusion_expr::literal::Literal::lit` · datafusion-expr 55.1.0

```rust
fn lit(&self) -> Expr
```

Source: `src/literal.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

convert the value to a Literal expression
