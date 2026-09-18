# `deltalake_opendal::adapter`

Crate `deltalake-opendal` · 3 public items · structured records in [`model/deltalake_opendal.adapter.json`](../model/deltalake_opendal.adapter.json)

## GenericAdapter

`struct` · `deltalake_opendal::adapter::GenericAdapter`
[Full member contracts, output types and access classification](../operations/deltalake_opendal.adapter.GenericAdapter.md)

Also reachable as `deltalake::opendal::GenericAdapter`, `deltalake_opendal::GenericAdapter`

```rust
struct GenericAdapter
```

**Fields**: `service`, `option_prefix`

**Implements**: `deltalake_opendal::adapter::OpendalAdapter`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(service: impl Into<String>) -> Self
```

**via `deltalake_opendal::adapter::OpendalAdapter`**

```rust
fn resolve(&self, url: &Url, config: &StorageConfig) -> DeltaResult<OperatorSpec>
```

Adapter for "simple" OpenDAL services whose operator is scoped at the bucket
root: the URL host is the bucket and the URL path is the table prefix.

---

## OperatorSpec

`struct` · `deltalake_opendal::adapter::OperatorSpec`
[Full member contracts, output types and access classification](../operations/deltalake_opendal.adapter.OperatorSpec.md)

Also reachable as `deltalake::opendal::OperatorSpec`, `deltalake_opendal::OperatorSpec`

```rust
struct OperatorSpec
```

**Fields**: `scheme`, `config`, `table_prefix`

**Derives**: Clone, Debug

Everything the generic factories need to build an OpenDAL-backed store for a
delta table: which OpenDAL service to use, its config, and where the table
lives within the resulting operator.

---

## OpendalAdapter

`trait` · `deltalake_opendal::adapter::OpendalAdapter`
[Full member contracts, output types and access classification](../operations/deltalake_opendal.adapter.OpendalAdapter.md)

Also reachable as `deltalake::opendal::OpendalAdapter`, `deltalake_opendal::OpendalAdapter`

```rust
trait OpendalAdapter: Send + Sync + std::fmt::Debug
```

**Implementors** (1)

- `deltalake_opendal::adapter::GenericAdapter`

**Methods** (3)

```rust
fn logstore_prefix(&self, spec: &OperatorSpec) -> Path
fn resolve(&self, url: &Url, config: &StorageConfig) -> DeltaResult<OperatorSpec>
fn wrap_store(&self, store: ObjectStoreRef, _spec: &OperatorSpec) -> ObjectStoreRef
```

Per-service specialization for the generic OpenDAL factories.

Only three things vary between OpenDAL services: how a delta URL plus storage
options map onto an [`OperatorSpec`], whether the resulting store needs
wrapping, and what prefix the log store should use. Everything else is shared
by [`crate::factory`].

---
