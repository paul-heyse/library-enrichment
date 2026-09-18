# `deltalake_core::logstore::storage`

Crate `deltalake-core` · 6 public items · structured records in [`model/deltalake_core.logstore.storage.json`](../model/deltalake_core.logstore.storage.json)

## client_options_from_certificate

`function` · `deltalake_core::logstore::storage::client_options_from_certificate`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.storage.client_options_from_certificate.md)

Also reachable as `deltalake::logstore::client_options_from_certificate`, `deltalake_core::logstore::client_options_from_certificate`

```rust
fn client_options_from_certificate(path: &str) -> DeltaResult<object_store::ClientOptions>
```

Read a PEM certificate file and build [`object_store::ClientOptions`] with it.

---

## CertificateConfig

`struct` · `deltalake_core::logstore::storage::CertificateConfig`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.storage.CertificateConfig.md)

```rust
struct CertificateConfig
```

**Fields**: `certificate_path`

**Implements**: `core::iter::traits::collect::FromIterator`, `deltalake_core::logstore::config::TryUpdateKey`

**Derives**: Clone, Debug, Default

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

**via `deltalake_core::logstore::config::TryUpdateKey`**

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
fn try_update_key(&mut self, key: &str, v: &str) -> DeltaResult<Option<()>>
```

---

## DefaultObjectStoreRegistry

`struct` · `deltalake_core::logstore::storage::DefaultObjectStoreRegistry`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.storage.DefaultObjectStoreRegistry.md)

Also reachable as `deltalake::logstore::DefaultObjectStoreRegistry`, `deltalake_core::logstore::DefaultObjectStoreRegistry`

```rust
struct DefaultObjectStoreRegistry
```

**Implements**: `deltalake_core::logstore::storage::ObjectStoreRegistry`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `deltalake_core::logstore::storage::ObjectStoreRegistry`**

```rust
fn get_store(&self, url: &Url) -> DeltaResult<Arc<dyn ObjectStore>>
fn register_store(&self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

The default [`ObjectStoreRegistry`]

---

## LimitConfig

`struct` · `deltalake_core::logstore::storage::LimitConfig`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.storage.LimitConfig.md)

```rust
struct LimitConfig
```

**Fields**: `max_concurrency`

**Implements**: `core::iter::traits::collect::FromIterator`, `deltalake_core::logstore::config::TryUpdateKey`

**Derives**: Clone, Debug, Default

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

**via `deltalake_core::logstore::config::TryUpdateKey`**

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
fn try_update_key(&mut self, key: &str, v: &str) -> DeltaResult<Option<()>>
```

---

## ObjectStoreRegistry

`trait` · `deltalake_core::logstore::storage::ObjectStoreRegistry`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.storage.ObjectStoreRegistry.md)

Also reachable as `deltalake::logstore::ObjectStoreRegistry`, `deltalake_core::logstore::ObjectStoreRegistry`

```rust
trait ObjectStoreRegistry: Send + Sync + std::fmt::Debug + 'static
```

**Implementors** (1)

- `deltalake_core::logstore::storage::DefaultObjectStoreRegistry`

**Methods** (2)

```rust
fn get_store(&self, url: &Url) -> DeltaResult<Arc<dyn ObjectStore>>
fn register_store(&self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

A registry mapping URLs to [`ObjectStore`] instances, supporting registration and lookup
(with optional lazy, ad-hoc discovery on miss).

---

## ObjectStoreRef

`type_alias` · `deltalake_core::logstore::storage::ObjectStoreRef`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.storage.ObjectStoreRef.md)

Also reachable as `deltalake::logstore::ObjectStoreRef`, `deltalake_core::logstore::ObjectStoreRef`

```rust
type ObjectStoreRef = std::sync::Arc<object_store::DynObjectStore>
```

Sharable reference to [`ObjectStore`]

---
