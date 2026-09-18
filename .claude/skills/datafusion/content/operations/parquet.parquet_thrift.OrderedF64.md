# `parquet::parquet_thrift::OrderedF64`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.parquet_thrift.OrderedF64.json).

<a id="op-b05c7310c31417d6c4f74255"></a>
## OrderedF64

`struct` · `parquet::parquet_thrift::OrderedF64` · parquet 59.3.0

```rust
struct OrderedF64
```

Source: `src/parquet_thrift.rs:116`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Wrapper for thrift `double` fields. This is used to provide
an implementation of `Eq` for floats. This implementation
uses IEEE 754 total order.
