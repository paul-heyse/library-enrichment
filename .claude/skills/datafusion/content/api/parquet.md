# `parquet`

Crate `parquet` · 1 public items · structured records in [`model/parquet.json`](../model/parquet.json)

## DecodeResult

`enum` · `parquet::DecodeResult`

Also reachable as `datafusion::parquet::DecodeResult`

```rust
enum DecodeResult<T: Debug>
```

**Variants**: `NeedsData`, `Data`, `Finished`

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/parquet.DecodeResult.md).


What data is needed to read the next item from a decoder.

This is used to communicate between the decoder and the caller
to indicate what data is needed next, or what the result of decoding is.

---
