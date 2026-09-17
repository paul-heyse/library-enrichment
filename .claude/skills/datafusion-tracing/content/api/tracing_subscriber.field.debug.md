# `tracing_subscriber::field::debug`

Crate `tracing-subscriber` · 1 public items · structured records in [`model/tracing_subscriber.field.debug.json`](../model/tracing_subscriber.field.debug.json)

## Alt

`struct` · `tracing_subscriber::field::debug::Alt`

```rust
struct Alt<V>
```

**Implements**: `tracing_core::field::Visit`, `tracing_subscriber::field::MakeVisitor`, `tracing_subscriber::field::VisitFmt`, `tracing_subscriber::field::VisitOutput`, `tracing_subscriber::field::VisitWrite`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(inner: V) -> Self
```

**via `tracing_core::field::Visit`**

```rust
fn record_bool(&mut self, field: &Field, value: bool)
fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug)
fn record_f64(&mut self, field: &Field, value: f64)
fn record_i64(&mut self, field: &Field, value: i64)
fn record_str(&mut self, field: &Field, value: &str)
fn record_u64(&mut self, field: &Field, value: u64)
```

**via `tracing_subscriber::field::MakeVisitor`**

```rust
fn make_visitor(&self, target: T) -> Self::Visitor
```

**via `tracing_subscriber::field::VisitFmt`**

```rust
fn writer(&mut self) -> &mut dyn fmt::Write
```

**via `tracing_subscriber::field::VisitOutput`**

```rust
fn finish(self) -> O
```

**via `tracing_subscriber::field::VisitWrite`**

```rust
fn writer(&mut self) -> &mut dyn io::Write
```

A visitor wrapper that ensures any `fmt::Debug` fields are formatted using
the alternate (`:#`) formatter.

---
