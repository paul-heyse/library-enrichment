# `pyrefly_graph::calculation`

Crate `pyrefly_graph` · 2 public items · structured records in [`model/pyrefly_graph.calculation.json`](../model/pyrefly_graph.calculation.json)

## ProposalResult

`enum` · `pyrefly_graph::calculation::ProposalResult`

```rust
enum ProposalResult<T>
```

**Variants**: `Calculatable`, `Calculated`

**Derives**: Clone, Debug

The result of proposing a calculation in the current thread. See
`propose_calculation` for more details on how it is used.

---

## Calculation

`struct` · `pyrefly_graph::calculation::Calculation`

```rust
struct Calculation<T>
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug, Default, Sync

**Methods** (8)

```rust
fn calculate(&self, calculate: impl FnOnce() -> T) -> Option<T>
fn get(&self) -> Option<T>
fn new() -> Self
unsafe fn propose_calculation(&self) -> ProposalResult<T>
fn record_value(&self, value: T) -> (T, bool)
fn write_lock(&self) -> bool
fn write_unlock(&self, value: T) -> (T, bool)
fn write_unlock_empty(&self)
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A cached calculation where recursive calculation returns None.

---
