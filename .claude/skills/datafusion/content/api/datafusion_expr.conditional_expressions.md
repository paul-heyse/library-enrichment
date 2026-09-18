# `datafusion_expr::conditional_expressions`

Crate `datafusion-expr` · 1 public items · structured records in [`model/datafusion_expr.conditional_expressions.json`](../model/datafusion_expr.conditional_expressions.json)

## CaseBuilder

`struct` · `datafusion_expr::conditional_expressions::CaseBuilder`

```rust
struct CaseBuilder
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn end(&self) -> Result<Expr>
fn new(expr: Option<Box<Expr>>, when_expr: Vec<Expr>, then_expr: Vec<Expr>, else_expr: Option<Box<Expr>>) -> Self
fn otherwise(&mut self, else_expr: Expr) -> Result<Expr>
fn when(&mut self, when: Expr, then: Expr) -> CaseBuilder
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.conditional_expressions.CaseBuilder.md).


Helper struct for building [Expr::Case]

---
