# `deltalake_catalog_unity::client::pagination::stream_paginated`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.client.pagination.stream_paginated.json).

<a id="op-653333a621693328e3bd8ae8"></a>
## stream_paginated

`function` · `deltalake_catalog_unity::client::pagination::stream_paginated` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn stream_paginated<F, Fut, S, T>(state: S, op: F) -> impl Stream<Item = deltalake_core::data_catalog::DataCatalogResult<T>> where F: Fn(S, Option<String>) -> Fut + Copy, Fut: Future<Output = deltalake_core::data_catalog::DataCatalogResult<(T, S, Option<String>)>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/pagination.rs#L24).

Source: `crates/catalog-unity/src/client/pagination.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

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

