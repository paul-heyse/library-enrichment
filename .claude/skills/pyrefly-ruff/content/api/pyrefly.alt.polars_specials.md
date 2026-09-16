# `pyrefly::alt::polars_specials`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.alt.polars_specials.json`](../model/pyrefly.alt.polars_specials.json)

## is_polars_series

`function` · `pyrefly::alt::polars_specials::is_polars_series`

```rust
fn is_polars_series(cls: &types::class::Class) -> bool
```

---

## polars_degrade_for_mutation

`function` · `pyrefly::alt::polars_specials::polars_degrade_for_mutation`

```rust
fn polars_degrade_for_mutation(ty: &pyrefly_types::types::Type, kind: &binding::polars::PolarsMutationKind, is_polars_series: impl Fn(&ruff_python_ast::Expr) -> bool) -> pyrefly_types::types::Type
```

Apply a binding-time mutation to a tracked frame schema.

---
