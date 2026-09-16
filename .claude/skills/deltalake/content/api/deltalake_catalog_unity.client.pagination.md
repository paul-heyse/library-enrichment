# `deltalake_catalog_unity::client::pagination`

Crate `deltalake-catalog-unity` · 1 public items · structured records in [`model/deltalake_catalog_unity.client.pagination.json`](../model/deltalake_catalog_unity.client.pagination.json)

## stream_paginated

`function` · `deltalake_catalog_unity::client::pagination::stream_paginated`

```rust
fn stream_paginated<F, Fut, S, T>(state: S, op: F) -> impl Stream<Item = deltalake_core::data_catalog::DataCatalogResult<T>> where F: Fn(S, Option<String>) -> Fut + Copy, Fut: Future<Output = deltalake_core::data_catalog::DataCatalogResult<(T, S, Option<String>)>>
```

Takes a paginated operation `op` that when called with:

- A state `S`
- An optional next token `Option<String>`

Returns

- A response value `T`
- The next state `S`
- The next continuation token `Option<String>`

And converts it into a `Stream<Result<T>>` which will first call `op(state, None)`, and yield
the returned response `T`. If the returned continuation token was `None` the stream will then
finish, otherwise it will continue to call `op(state, token)` with the values returned by the
previous call to `op`, until a continuation token of `None` is returned

---
