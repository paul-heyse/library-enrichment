# `pyrefly_util::small_set1`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.small_set1.json`](../model/pyrefly_util.small_set1.json)

## SmallSet1

`struct` · `pyrefly_util::small_set1::SmallSet1`

```rust
struct SmallSet1<T>
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn contains(&self, x: &T) -> bool where T: Hash + Eq
fn first(&self) -> &T
fn insert(&mut self, x: T) -> bool where T: Hash + Eq
fn new(x: T) -> Self
```

SmallSet but with at least one element.
If only one element is inserted, we won't ever hash it.

---
