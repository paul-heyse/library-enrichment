# `pyrefly::binding::polars`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.binding.polars.json`](../model/pyrefly.binding.polars.json)

## PolarsMutationKind

`enum` · `pyrefly::binding::polars::PolarsMutationKind`

```rust
enum PolarsMutationKind
```

**Variants**: `Add`, `Replace`, `Insert`

**Derives**: Clone, Debug

How an in-place mutation changes a Polars frame's tracked column set.

---

## polars_column_mutation

`function` · `pyrefly::binding::polars::polars_column_mutation`

```rust
fn polars_column_mutation(method: &str, args: &ruff_python_ast::Arguments) -> Option<PolarsMutationKind>
```

Classify mutations that may change a bound frame's columns.

---
