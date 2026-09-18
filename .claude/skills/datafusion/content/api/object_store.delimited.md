# `object_store::delimited`

Crate `object_store` · 1 public items · structured records in [`model/object_store.delimited.json`](../model/object_store.delimited.json)

## newline_delimited_stream

`function` · `object_store::delimited::newline_delimited_stream`

```rust
fn newline_delimited_stream<S>(s: S) -> impl Stream<Item = super::Result<bytes::Bytes>> where S: Stream<Item = super::Result<bytes::Bytes>> + Unpin
```

[Full member, field, variant and typed contracts](../operations/object_store.delimited.newline_delimited_stream.md).


Given a [`Stream`] of [`Bytes`] returns a [`Stream`] where each
yielded [`Bytes`] contains a whole number of new line delimited records
accounting for `\` style escapes and `"` quotes

---
