# `pyrefly_util::forgetter`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.forgetter.json`](../model/pyrefly_util.forgetter.json)

## Forgetter

`struct` · `pyrefly_util::forgetter::Forgetter`

```rust
struct Forgetter<T>
```

**Implements**: `core::convert::AsMut`, `core::convert::AsRef`, `core::ops::drop::Drop`

**Methods** (1)

```rust
fn new(value: T, forget: bool) -> Self
```

**via `core::convert::AsMut`**

```rust
fn as_mut(&mut self) -> &mut T
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &T
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Type that might be forgotten when it goes out of scope.

---
