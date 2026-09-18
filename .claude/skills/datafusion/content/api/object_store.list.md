# `object_store::list`

Crate `object_store` · 3 public items · structured records in [`model/object_store.list.json`](../model/object_store.list.json)

## PaginatedListOptions

`struct` · `object_store::list::PaginatedListOptions`

```rust
struct PaginatedListOptions
```

**Fields**: `offset`, `delimiter`, `max_keys`, `page_token`, `extensions`

**Derives**: Clone, Debug, Default

[Full member, field, variant and typed contracts](../operations/object_store.list.PaginatedListOptions.md).


Options for a paginated list request

---

## PaginatedListResult

`struct` · `object_store::list::PaginatedListResult`

```rust
struct PaginatedListResult
```

**Fields**: `result`, `page_token`

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/object_store.list.PaginatedListResult.md).


A [`ListResult`] with optional pagination token

---

## PaginatedListStore

`trait` · `object_store::list::PaginatedListStore`

```rust
trait PaginatedListStore: Send + Sync + 'static
```

**Implementors** (3)

- `object_store::aws::AmazonS3`
- `object_store::azure::MicrosoftAzure`
- `object_store::gcp::GoogleCloudStorage`

**Methods** (1)

```rust
async fn list_paginated(&self, prefix: Option<&str>, opts: PaginatedListOptions) -> Result<PaginatedListResult>
```

[Full member, field, variant and typed contracts](../operations/object_store.list.PaginatedListStore.md).


A low-level interface for interacting with paginated listing APIs

Most use-cases should prefer [`ObjectStore::list`] as this is supported by more
backends, including [`LocalFileSystem`], however, [`PaginatedListStore`] can be
used where stateless pagination or non-path segment based listing is required

[`ObjectStore::list`]: crate::ObjectStore::list
[`LocalFileSystem`]: crate::local::LocalFileSystem

---
