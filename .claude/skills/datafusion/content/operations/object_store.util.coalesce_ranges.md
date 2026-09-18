# `object_store::util::coalesce_ranges`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.util.coalesce_ranges.json).

<a id="op-c80863c21948960e5fcda945"></a>
## coalesce_ranges

`function` · `object_store::util::coalesce_ranges` · object_store 0.13.2

```rust
async fn coalesce_ranges<F, E, Fut>(ranges: &[std::ops::Range<u64>], fetch: F, coalesce: u64) -> super::Result<Vec<bytes::Bytes>, E> where F: Send + FnMut(std::ops::Range<u64>) -> Fut, E: Send, Fut: std::future::Future<Output = super::Result<bytes::Bytes, E>> + Send
```

Source: `src/util.rs:105`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Takes a function `fetch` that can fetch a range of bytes and uses this to
fetch the provided byte `ranges`

To improve performance it will:

* Combine ranges less than `coalesce` bytes apart into a single call to `fetch`
* Make multiple `fetch` requests in parallel (up to maximum of 10)

