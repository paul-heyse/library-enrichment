# `tracing_subscriber::field::delimited`

Crate `tracing-subscriber` · 2 public items · structured records in [`model/tracing_subscriber.field.delimited.json`](../model/tracing_subscriber.field.delimited.json)

## Delimited

`struct` · `tracing_subscriber::field::delimited::Delimited`

```rust
struct Delimited<D, V>
```

**Implements**: `tracing_subscriber::field::MakeVisitor`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(delimiter: D, inner: V) -> Self
```

**via `tracing_subscriber::field::MakeVisitor`**

```rust
fn make_visitor(&self, target: T) -> Self::Visitor
```

A `MakeVisitor` wrapper that wraps a visitor that writes formatted output so
that a delimiter is inserted between writing formatted field values.

---

## VisitDelimited

`struct` · `tracing_subscriber::field::delimited::VisitDelimited`

```rust
struct VisitDelimited<D, V>
```

**Implements**: `tracing_core::field::Visit`, `tracing_subscriber::field::VisitFmt`, `tracing_subscriber::field::VisitOutput`

**Derives**: Debug

**Methods** (1)

```rust
fn new(delimiter: D, inner: V) -> Self
```

**via `tracing_core::field::Visit`**

```rust
fn record_bool(&mut self, field: &Field, value: bool)
fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug)
fn record_i64(&mut self, field: &Field, value: i64)
fn record_str(&mut self, field: &Field, value: &str)
fn record_u64(&mut self, field: &Field, value: u64)
```

**via `tracing_subscriber::field::VisitFmt`**

```rust
fn writer(&mut self) -> &mut dyn fmt::Write
```

**via `tracing_subscriber::field::VisitOutput`**

```rust
fn finish(self) -> fmt::Result
```

A visitor wrapper that inserts a delimiter after the wrapped visitor formats
a field value.

---
