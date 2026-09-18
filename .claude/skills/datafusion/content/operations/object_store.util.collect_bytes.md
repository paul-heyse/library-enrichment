# `object_store::util::collect_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.util.collect_bytes.json).

<a id="op-551551c5dfd7f7c0988e561d"></a>
## collect_bytes

`function` · `object_store::util::collect_bytes` · object_store 0.13.2

```rust
async fn collect_bytes<S, E>(stream: S, size_hint: Option<u64>) -> super::Result<bytes::Bytes, E> where E: Send, S: Stream<Item = super::Result<bytes::Bytes, E>> + Send + Unpin
```

Source: `src/util.rs:52`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Collect a stream into [`Bytes`] avoiding copying in the event of a single chunk

Unresolved upstream links (retained, not inferred): ``Bytes``.
