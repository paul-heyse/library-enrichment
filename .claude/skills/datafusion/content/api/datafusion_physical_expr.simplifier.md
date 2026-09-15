# `datafusion_physical_expr::simplifier`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.simplifier.json`](../model/datafusion_physical_expr.simplifier.json)

## PhysicalExprSimplifier

`struct` · `datafusion_physical_expr::simplifier::PhysicalExprSimplifier`

Also reachable as `datafusion::physical_expr::PhysicalExprSimplifier`, `datafusion_physical_expr::PhysicalExprSimplifier`

```rust
struct PhysicalExprSimplifier<'a>
```

**Methods** (2)

```rust
fn new(schema: &'a Schema) -> Self
fn simplify(&self, expr: Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>
```

Simplifies physical expressions by applying various optimizations

This can be useful after adapting expressions from a table schema
to a file schema. For example, casts added to match the types may
potentially be unwrapped.

---
