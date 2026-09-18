# `object_store::delimited::newline_delimited_stream`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.delimited.newline_delimited_stream.json).

<a id="op-d42cc34e0e3dc1ef0d92784f"></a>
## newline_delimited_stream

`function` · `object_store::delimited::newline_delimited_stream` · object_store 0.13.2

```rust
fn newline_delimited_stream<S>(s: S) -> impl Stream<Item = super::Result<bytes::Bytes>> where S: Stream<Item = super::Result<bytes::Bytes>> + Unpin
```

Source: `src/delimited.rs:152`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Given a [`Stream`] of [`Bytes`] returns a [`Stream`] where each
yielded [`Bytes`] contains a whole number of new line delimited records
accounting for `\` style escapes and `"` quotes

Unresolved upstream links (retained, not inferred): ``Bytes``, ``Stream``.
