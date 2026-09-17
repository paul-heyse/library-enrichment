# `tracing_subscriber::registry::extensions`

Crate `tracing-subscriber` · 2 public items · structured records in [`model/tracing_subscriber.registry.extensions.json`](../model/tracing_subscriber.registry.extensions.json)

## Extensions

`struct` · `tracing_subscriber::registry::extensions::Extensions`

Also reachable as `tracing_subscriber::registry::Extensions`

```rust
struct Extensions<'a>
```

**Derives**: Debug

**Methods** (1)

```rust
fn get<T: 'static>(&self) -> Option<&T>
```

An immutable, read-only reference to a Span's extensions.

---

## ExtensionsMut

`struct` · `tracing_subscriber::registry::extensions::ExtensionsMut`

Also reachable as `tracing_subscriber::registry::ExtensionsMut`

```rust
struct ExtensionsMut<'a>
```

**Derives**: Debug

**Methods** (4)

```rust
fn get_mut<T: 'static>(&mut self) -> Option<&mut T>
fn insert<T: Send + Sync + 'static>(&mut self, val: T)
fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T>
fn replace<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T>
```

An mutable reference to a Span's extensions.

---
