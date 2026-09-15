# `parquet::parquet_thrift`

Crate `parquet` · 1 public items · structured records in [`model/parquet.parquet_thrift.json`](../model/parquet.parquet_thrift.json)

## OrderedF64

`struct` · `parquet::parquet_thrift::OrderedF64`

```rust
struct OrderedF64
```

Wrapper for thrift `double` fields. This is used to provide
an implementation of `Eq` for floats. This implementation
uses IEEE 754 total order.

---
