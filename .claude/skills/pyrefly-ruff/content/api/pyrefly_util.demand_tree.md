# `pyrefly_util::demand_tree`

Crate `pyrefly_util` · 5 public items · structured records in [`model/pyrefly_util.demand_tree.json`](../model/pyrefly_util.demand_tree.json)

## DemandKind

`enum` · `pyrefly_util::demand_tree::DemandKind`

```rust
enum DemandKind
```

**Variants**: `Load`, `Exports`, `Answer`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

What kind of cross-module demand an edge represents.

---

## report_json

`function` · `pyrefly_util::demand_tree::report_json`

```rust
fn report_json(roots: &[DemandEdge], module_steps: &[(String, &'static str)]) -> String
```

Serialize a demand tree alongside per-module step info as a pretty-
printed JSON document for `pyrefly check --report-demand-tree`.
`module_steps` should pair each module's name with the highest step
it reached during the run.

---

## DemandCollector

`struct` · `pyrefly_util::demand_tree::DemandCollector`

```rust
struct DemandCollector
```

**Derives**: Clone, Default

**Methods** (5)

```rust
fn enter(&self, from: impl fmt::Display, target: impl fmt::Display, key: impl fmt::Debug) -> DemandSpan<'_>
fn exports_event(&self, from: impl fmt::Display, target: impl fmt::Display, reason: &str)
fn load_event(&self, from: impl fmt::Display, target: impl fmt::Display, reason: &str)
fn new() -> Self
fn take_roots(&self) -> Vec<DemandEdge>
```

A demand-tree collection session. Cloning produces another handle to the
same underlying roots (the collector is reference-counted internally).

---

## DemandEdge

`struct` · `pyrefly_util::demand_tree::DemandEdge`

```rust
struct DemandEdge
```

**Fields**: `from`, `target`, `kind`, `children`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

One observed cross-module demand: an edge from `from` to `target`
carrying what was demanded (`kind`). Edges form a tree because
Answer demands open spans that nest other demands made while
computing them.

---

## DemandSpan

`struct` · `pyrefly_util::demand_tree::DemandSpan`

```rust
struct DemandSpan<'a>
```

**Implements**: `core::ops::drop::Drop`

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

RAII guard for an in-flight Answer demand span. Dropping the guard —
whether via normal control flow or unwind — pops the span off the
per-thread stack and attaches it to its parent (or to the collector's
roots if no parent is in flight).

`enter()` pushes onto the calling thread's `STACK` and `Drop::drop`
pops from the dropping thread's `STACK`, so a guard moved to a
different thread before drop would pop an unrelated entry. The
`PhantomData<*const ()>` makes `DemandSpan` `!Send` to rule that out
at compile time.

---
