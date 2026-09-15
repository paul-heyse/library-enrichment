# `parquet::record::triplet`

Crate `parquet` · 2 public items · structured records in [`model/parquet.record.triplet.json`](../model/parquet.record.triplet.json)

## TripletIter

`enum` · `parquet::record::triplet::TripletIter`

```rust
enum TripletIter
```

**Variants**: `BoolTripletIter`, `Int32TripletIter`, `Int64TripletIter`, `Int96TripletIter`, `FloatTripletIter`, `DoubleTripletIter`, `ByteArrayTripletIter`, `FixedLenByteArrayTripletIter`

High level API wrapper on column reader.
Provides per-element access for each primitive column.

---

## TypedTripletIter

`struct` · `parquet::record::triplet::TypedTripletIter`

```rust
struct TypedTripletIter<T: DataType>
```

Internal typed triplet iterator as a wrapper for column reader
(primitive leaf column), provides per-element access.

---
