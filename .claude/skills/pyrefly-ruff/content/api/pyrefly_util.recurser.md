# `pyrefly_util::recurser`

Crate `pyrefly_util` · 2 public items · structured records in [`model/pyrefly_util.recurser.json`](../model/pyrefly_util.recurser.json)

## Guard

`struct` · `pyrefly_util::recurser::Guard`

```rust
struct Guard<'a, T> where T: Hash + Eq
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

---

## Recurser

`struct` · `pyrefly_util::recurser::Recurser`

```rust
struct Recurser<T>
```

**Derives**: Debug, Default

**Methods** (2)

```rust
fn new() -> Self
fn recurse(&self, x: T) -> Option<Guard<'_, T>>
```

---
