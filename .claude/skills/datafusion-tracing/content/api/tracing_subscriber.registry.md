# `tracing_subscriber::registry`

Crate `tracing-subscriber` · 5 public items · structured records in [`model/tracing_subscriber.registry.json`](../model/tracing_subscriber.registry.json)

## Scope

`struct` · `tracing_subscriber::registry::Scope`

```rust
struct Scope<'a, R>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn from_root(self) -> ScopeFromRoot<'a, R>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

An iterator over the parents of a span, ordered from leaf to root.

This is returned by the [`SpanRef::scope`] method.

---

## ScopeFromRoot

`struct` · `tracing_subscriber::registry::ScopeFromRoot`

```rust
struct ScopeFromRoot<'a, R> where R: LookupSpan<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over the parents of a span, ordered from root to leaf.

This is returned by the [`Scope::from_root`] method.

---

## SpanRef

`struct` · `tracing_subscriber::registry::SpanRef`

```rust
struct SpanRef<'a, R: LookupSpan<'a>>
```

**Derives**: Debug

**Methods** (8)

```rust
fn extensions(&self) -> Extensions<'_>
fn extensions_mut(&self) -> ExtensionsMut<'_>
fn fields(&self) -> &FieldSet
fn id(&self) -> Id
fn metadata(&self) -> &'static Metadata<'static>
fn name(&self) -> &'static str
fn parent(&self) -> Option<Self>
fn scope(&self) -> Scope<'a, R>
```

A reference to [span data] and the associated [registry].

This type implements all the same methods as [`SpanData`], and provides
additional methods for querying the registry based on values from the span.

[registry]: LookupSpan

---

## LookupSpan

`trait` · `tracing_subscriber::registry::LookupSpan`

```rust
trait LookupSpan<'a>
```

**Implementors** (3)

- `tracing_subscriber::fmt::Subscriber`
- `tracing_subscriber::layer::layered::Layered`
- `tracing_subscriber::registry::sharded::Registry`

**Methods** (3)

```rust
fn register_filter(&mut self) -> FilterId
fn span(&'a self, id: &Id) -> Option<SpanRef<'a, Self>> where Self: Sized
fn span_data(&'a self, id: &Id) -> Option<Self::Data>
```

Provides access to stored span data.

Subscribers which store span data and associate it with span IDs should
implement this trait; if they do, any [`Layer`]s wrapping them can look up
metadata via the [`Context`] type's [`span()`] method.

[`Layer`]: super::layer::Layer
[`Context`]: super::layer::Context
[`span()`]: super::layer::Context::span

---

## SpanData

`trait` · `tracing_subscriber::registry::SpanData`

```rust
trait SpanData<'a>
```

**Implementors** (1)

- `tracing_subscriber::registry::sharded::Data`

**Methods** (6)

```rust
fn extensions(&self) -> Extensions<'_>
fn extensions_mut(&self) -> ExtensionsMut<'_>
fn id(&self) -> Id
fn is_enabled_for(&self, filter: FilterId) -> bool
fn metadata(&self) -> &'static Metadata<'static>
fn parent(&self) -> Option<&Id>
```

A stored representation of data associated with a span.

---
