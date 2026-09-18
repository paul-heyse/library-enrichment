# `datafusion_common::utils::proxy`

Crate `datafusion-common` · 2 public items · structured records in [`model/datafusion_common.utils.proxy.json`](../model/datafusion_common.utils.proxy.json)

## HashTableAllocExt

`trait` · `datafusion_common::utils::proxy::HashTableAllocExt`

Also reachable as `datafusion_execution::memory_pool::proxy::HashTableAllocExt`

```rust
trait HashTableAllocExt
```

**Implementors** (1)

- `hashbrown::table::HashTable`

**Methods** (1)

```rust
fn insert_accounted(&mut self, x: Self::T, hasher: impl Fn(&Self::T) -> u64, accounting: &mut usize)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.proxy.HashTableAllocExt.md).


Extension trait for hash browns [`HashTable`] to account for allocations.

---

## VecAllocExt

`trait` · `datafusion_common::utils::proxy::VecAllocExt`

Also reachable as `datafusion_execution::memory_pool::proxy::VecAllocExt`

```rust
trait VecAllocExt
```

**Implementors** (1)

- `alloc::vec::Vec`

**Methods** (2)

```rust
fn allocated_size(&self) -> usize
fn push_accounted(&mut self, x: Self::T, accounting: &mut usize)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.proxy.VecAllocExt.md).


Extension trait for [`Vec`] to account for allocations.

---
