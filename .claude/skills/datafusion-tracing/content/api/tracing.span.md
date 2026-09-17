# `tracing::span`

Crate `tracing` · 4 public items · structured records in [`model/tracing.span.json`](../model/tracing.span.json)

## Entered

`struct` · `tracing::span::Entered`

```rust
struct Entered<'a>
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A guard representing a span which has been entered and is currently
executing.

When the guard is dropped, the span will be exited.

This is returned by the [`Span::enter`] function.

[`Span::enter`]: super::Span::enter

---

## EnteredSpan

`struct` · `tracing::span::EnteredSpan`

```rust
struct EnteredSpan
```

**Implements**: `core::ops::deref::Deref`, `core::ops::drop::Drop`

**Derives**: Debug

**Methods** (2)

```rust
fn exit(self) -> Span
fn id(&self) -> Option<Id>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Span
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

An owned version of [`Entered`], a guard representing a span which has been
entered and is currently executing.

When the guard is dropped, the span will be exited.

This is returned by the [`Span::entered`] function.

[`Span::entered`]: super::Span::entered()

---

## Span

`struct` · `tracing::span::Span`

Also reachable as `tracing::Span`

```rust
struct Span
```

**Implements**: `core::ops::drop::Drop`, `tracing_opentelemetry::span_ext::OpenTelemetrySpanExt`

**Derives**: Clone, Debug, Hash, PartialEq

**Methods** (19)

```rust
fn child_of(parent: impl Into<Option<Id>>, meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span
fn current() -> Span
fn enter(&self) -> Entered<'_>
fn entered(self) -> EnteredSpan
fn field<Q: field::AsField + ?Sized>(&self, field: &Q) -> Option<field::Field>
fn follows_from(&self, from: impl Into<Option<Id>>) -> &Self
fn has_field<Q: field::AsField + ?Sized>(&self, field: &Q) -> bool
fn id(&self) -> Option<Id>
fn in_scope<F: FnOnce() -> T, T>(&self, f: F) -> T
fn is_disabled(&self) -> bool
fn is_none(&self) -> bool
fn metadata(&self) -> Option<&'static Metadata<'static>>
fn new(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span
fn new_disabled(meta: &'static Metadata<'static>) -> Span
fn new_root(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span
const fn none() -> Span
fn or_current(self) -> Self
fn record<Q: field::AsField + ?Sized, V: field::Value>(&self, field: &Q, value: V) -> &Self
fn with_subscriber<T>(&self, f: impl FnOnce((&Id, &Dispatch)) -> T) -> Option<T>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A handle representing a span, with the capability to enter the span if it
exists.

If the span was rejected by the current `Subscriber`'s filter, entering the
span will silently do nothing. Thus, the handle can be used in the same
manner regardless of whether or not the trace is currently being collected.

---

## AsId

`trait` · `tracing::span::AsId`

```rust
trait AsId: sealed::Sealed
```

**Methods** (1)

```rust
fn as_id(&self) -> Option<&Id>
```

Trait implemented by types which have a span `Id`.

---
