# `parquet::record::triplet::TypedTripletIter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.triplet.TypedTripletIter.json).

<a id="op-903d861658d3adb83833cfce"></a>
## TypedTripletIter

`struct` · `parquet::record::triplet::TypedTripletIter` · parquet 59.3.0

```rust
struct TypedTripletIter<T: DataType>
```

Source: `src/record/triplet.rs:174`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Internal typed triplet iterator as a wrapper for column reader
(primitive leaf column), provides per-element access.
