# `pyrefly_util::no_hash`

Crate `pyrefly_util` · 2 public items · structured records in [`model/pyrefly_util.no_hash.json`](../model/pyrefly_util.no_hash.json)

## NoHash

`struct` · `pyrefly_util::no_hash::NoHash`

```rust
struct NoHash
```

**Implements**: `core::hash::Hasher`

**Derives**: Debug, Default

**via `core::hash::Hasher`**

```rust
fn finish(&self) -> u64
fn write(&mut self, _bytes: &[u8])
fn write_u64(&mut self, n: u64)
```

---

## BuildNoHash

`type_alias` · `pyrefly_util::no_hash::BuildNoHash`

```rust
type BuildNoHash = std::hash::BuildHasherDefault<NoHash>
```

---
