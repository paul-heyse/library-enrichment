# `pyrefly_util::memory`

Crate `pyrefly_util` · 4 public items · structured records in [`model/pyrefly_util.memory.json`](../model/pyrefly_util.memory.json)

## Bytes

`struct` · `pyrefly_util::memory::Bytes`

```rust
struct Bytes
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## JemallocStats

`struct` · `pyrefly_util::memory::JemallocStats`

```rust
struct JemallocStats
```

**Fields**: `active`, `allocated`

---

## MemoryUsage

`struct` · `pyrefly_util::memory::MemoryUsage`

```rust
struct MemoryUsage
```

**Fields**: `physical`, `allocated`, `active`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn max_by_field(x: &Self, y: &Self) -> Self
fn now() -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## MemoryUsageTrace

`struct` · `pyrefly_util::memory::MemoryUsageTrace`

```rust
struct MemoryUsageTrace
```

**Methods** (3)

```rust
fn peak(&self) -> MemoryUsage
fn start(frequency: Duration) -> Self
fn stop(&mut self)
```

---
