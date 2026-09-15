# `object_store::registry`

Crate `object_store` · 2 public items · structured records in [`model/object_store.registry.json`](../model/object_store.registry.json)

## DefaultObjectStoreRegistry

`struct` · `object_store::registry::DefaultObjectStoreRegistry`

```rust
struct DefaultObjectStoreRegistry
```

**Implements**: `object_store::registry::ObjectStoreRegistry`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `object_store::registry::ObjectStoreRegistry`**

```rust
fn register(&self, url: Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
fn resolve(&self, to_resolve: &Url) -> Result<(Arc<dyn ObjectStore>, Path)>
```

An [`ObjectStoreRegistry`] that uses [`parse_url_opts`] to create stores based on the environment

---

## ObjectStoreRegistry

`trait` · `object_store::registry::ObjectStoreRegistry`

```rust
trait ObjectStoreRegistry: Send + Sync + std::fmt::Debug + 'static
```

**Implementors** (1)

- `object_store::registry::DefaultObjectStoreRegistry`

**Methods** (2)

```rust
fn register(&self, url: Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
fn resolve(&self, url: &Url) -> Result<(Arc<dyn ObjectStore>, Path)>
```

[`ObjectStoreRegistry`] maps a URL to an [`ObjectStore`] instance

---
