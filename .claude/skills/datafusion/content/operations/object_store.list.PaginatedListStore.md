# `object_store::list::PaginatedListStore`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.list.PaginatedListStore.json).

<a id="op-d20f95ad769f16324ef55fa5"></a>
## PaginatedListStore

`trait` · `object_store::list::PaginatedListStore` · object_store 0.13.2

```rust
trait PaginatedListStore: Send + Sync + 'static
```

Source: `src/list.rs:75`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A low-level interface for interacting with paginated listing APIs

Most use-cases should prefer [`ObjectStore::list`] as this is supported by more
backends, including [`LocalFileSystem`], however, [`PaginatedListStore`](../operations/object_store.list.PaginatedListStore.md#op-d20f95ad769f16324ef55fa5) can be
used where stateless pagination or non-path segment based listing is required

[`ObjectStore::list`]: crate::ObjectStore::list
[`LocalFileSystem`]: crate::local::LocalFileSystem

<a id="op-3430e88ed385f8468e105340"></a>
## list_paginated

`function` · `object_store::list::PaginatedListStore::list_paginated` · object_store 0.13.2

```rust
async fn list_paginated(&self, prefix: Option<&str>, opts: PaginatedListOptions) -> Result<PaginatedListResult>
```

Source: `src/list.rs:83`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform a paginated list request

Note: the order of returned objects is not guaranteed and
unlike [`ObjectStore::list`] a trailing delimiter is not
automatically added to `prefix`

[`ObjectStore::list`]: crate::ObjectStore::list
