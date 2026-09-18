# `arrow_array::temporal_conversions::as_duration`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.as_duration.json).

<a id="op-bc3facdd5d805f29f379dc9e"></a>
## as_duration

`function` · `arrow_array::temporal_conversions::as_duration` · arrow-array 59.3.0

```rust
fn as_duration<T: ArrowPrimitiveType>(v: i64) -> Option<chrono::Duration>
```

Source: `src/temporal_conversions.rs:296`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Converts an [`ArrowPrimitiveType`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-ddd581d2aed24174207ba803) to [`Duration`]

Unresolved upstream links (retained, not inferred): ``Duration``.
