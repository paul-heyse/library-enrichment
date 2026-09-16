# `pyrefly_types::globals`

Crate `pyrefly_types` · 1 public items · structured records in [`model/pyrefly_types.globals.json`](../model/pyrefly_types.globals.json)

## ImplicitGlobal

`struct` · `pyrefly_types::globals::ImplicitGlobal`

```rust
struct ImplicitGlobal
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn as_type(&self, stdlib: &Stdlib, heap: &TypeHeap) -> Type
fn doc(has_docstring: bool) -> Self
fn implicit_globals(has_docstring: bool) -> impl Iterator<Item = ImplicitGlobal>
fn name(&self) -> &Name
```

---
