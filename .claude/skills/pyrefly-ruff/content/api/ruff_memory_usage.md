# `ruff_memory_usage`

Crate `ruff_memory_usage` · 3 public items · structured records in [`model/ruff_memory_usage.json`](../model/ruff_memory_usage.json)

## TRACKER

`constant` · `ruff_memory_usage::TRACKER`

```rust
const TRACKER: thread::LocalKey<std::cell::RefCell<Option<get_size2::StandardTracker>>> = _
```

---

## attach_tracker

`function` · `ruff_memory_usage::attach_tracker`

```rust
fn attach_tracker<R>(tracker: get_size2::StandardTracker, f: impl FnOnce() -> R) -> R
```

---

## heap_size

`function` · `ruff_memory_usage::heap_size`

```rust
fn heap_size<T: GetSize>(value: &T) -> usize
```

Returns the memory usage of the provided object, using a global tracker to avoid
double-counting shared objects.

---
