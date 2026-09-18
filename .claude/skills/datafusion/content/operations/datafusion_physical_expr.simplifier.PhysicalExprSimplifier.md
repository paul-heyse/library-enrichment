# `datafusion_physical_expr::simplifier::PhysicalExprSimplifier`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.simplifier.PhysicalExprSimplifier.json).

<a id="op-7c319dda665b42503117f47d"></a>
## PhysicalExprSimplifier

`struct` · `datafusion_physical_expr::simplifier::PhysicalExprSimplifier` · datafusion-physical-expr 55.1.0

```rust
struct PhysicalExprSimplifier<'a>
```

Source: `src/simplifier/mod.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Simplifies physical expressions by applying various optimizations

This can be useful after adapting expressions from a table schema
to a file schema. For example, casts added to match the types may
potentially be unwrapped.

<a id="op-e555ea53581ff26606dc95c5"></a>
## new

`function` · `datafusion_physical_expr::simplifier::PhysicalExprSimplifier::new` · datafusion-physical-expr 55.1.0

```rust
fn new(schema: &'a Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr::simplifier::PhysicalExprSimplifier", "path": "PhysicalExprSimplifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [92, 2], "filename": "src/simplifier/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplifier/mod.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new physical expression simplifier

<a id="op-85f72d192a6003557eef643b"></a>
## simplify

`function` · `datafusion_physical_expr::simplifier::PhysicalExprSimplifier::simplify` · datafusion-physical-expr 55.1.0

```rust
fn simplify(&self, expr: Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr::simplifier::PhysicalExprSimplifier", "path": "PhysicalExprSimplifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [92, 2], "filename": "src/simplifier/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplifier/mod.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Simplify a physical expression
