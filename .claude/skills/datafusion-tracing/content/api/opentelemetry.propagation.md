# `opentelemetry::propagation`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.propagation.json`](../model/opentelemetry.propagation.json)

## Extractor

`trait` · `opentelemetry::propagation::Extractor`

```rust
trait Extractor
```

**Implementors** (1)

- `std::collections::hash::map::HashMap`

**Methods** (3)

```rust
fn get(&self, key: &str) -> Option<&str>
fn get_all(&self, key: &str) -> Option<Vec<&str>>
fn keys(&self) -> Vec<&str>
```

Extractor provides an interface for removing fields from an underlying struct like `HashMap`

---

## Injector

`trait` · `opentelemetry::propagation::Injector`

```rust
trait Injector
```

**Implementors** (1)

- `std::collections::hash::map::HashMap`

**Methods** (1)

```rust
fn set(&mut self, key: &str, value: String)
```

Injector provides an interface for adding fields from an underlying struct like `HashMap`

---
