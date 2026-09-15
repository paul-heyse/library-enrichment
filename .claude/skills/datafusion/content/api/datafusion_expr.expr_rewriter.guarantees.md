# `datafusion_expr::expr_rewriter::guarantees`

Crate `datafusion-expr` · 3 public items · structured records in [`model/datafusion_expr.expr_rewriter.guarantees.json`](../model/datafusion_expr.expr_rewriter.guarantees.json)

## rewrite_with_guarantees

`function` · `datafusion_expr::expr_rewriter::guarantees::rewrite_with_guarantees`

Also reachable as `datafusion_expr::expr_rewriter::rewrite_with_guarantees`

```rust
fn rewrite_with_guarantees<'a>(expr: Expr, guarantees: impl IntoIterator<Item = &'a (Expr, datafusion_expr_common::interval_arithmetic::NullableInterval)>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<Expr>>
```

Rewrite expressions to incorporate guarantees.

Guarantees are a mapping from an expression (which currently is always a
column reference) to a [NullableInterval] that represents the known possible
values of the expression.

Rewriting expressions using this type of guarantee can make the work of other expression
simplifications, like const evaluation, easier.

For example, if we know that a column is not null and has values in the
range [1, 10), we can rewrite `x IS NULL` to `false` or `x < 10` to `true`.

If the set of guarantees will be used to rewrite more than one expression, consider using
[rewrite_with_guarantees_map] instead.

A full example of using this rewrite rule can be found in
[`ExprSimplifier::with_guarantees()`](https://docs.rs/datafusion/latest/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.with_guarantees).

---

## rewrite_with_guarantees_map

`function` · `datafusion_expr::expr_rewriter::guarantees::rewrite_with_guarantees_map`

Also reachable as `datafusion_expr::expr_rewriter::rewrite_with_guarantees_map`

```rust
fn rewrite_with_guarantees_map<'a>(expr: Expr, guarantees: &'a datafusion_common::HashMap<&'a Expr, &'a datafusion_expr_common::interval_arithmetic::NullableInterval>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<Expr>>
```

Rewrite expressions to incorporate guarantees.

Guarantees are a mapping from an expression (which currently is always a
column reference) to a [NullableInterval]. The interval represents the known
possible values of the column.

For example, if we know that a column is not null and has values in the
range [1, 10), we can rewrite `x IS NULL` to `false` or `x < 10` to `true`.

---

## GuaranteeRewriter

`struct` · `datafusion_expr::expr_rewriter::guarantees::GuaranteeRewriter`

Also reachable as `datafusion_expr::expr_rewriter::GuaranteeRewriter`, `datafusion_optimizer::simplify_expressions::GuaranteeRewriter`

```rust
struct GuaranteeRewriter<'a>
```

**Implements**: `datafusion_common::tree_node::TreeNodeRewriter`

**Methods** (1)

```rust
fn new(guarantees: impl IntoIterator<Item = &'a (Expr, NullableInterval)>) -> Self
```

**via `datafusion_common::tree_node::TreeNodeRewriter`**

```rust
fn f_up(&mut self, expr: Expr) -> Result<Transformed<Expr>>
```

Rewrite expressions to incorporate guarantees.

See [`rewrite_with_guarantees`] for more information

---
