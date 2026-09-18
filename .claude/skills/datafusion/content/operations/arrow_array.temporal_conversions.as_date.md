# `arrow_array::temporal_conversions::as_date`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.as_date.json).

<a id="op-7522cf255f845f801e2349bb"></a>
## as_date

`function` · `arrow_array::temporal_conversions::as_date` · arrow-array 59.3.0

```rust
fn as_date<T: ArrowPrimitiveType>(v: i64) -> Option<chrono::NaiveDate>
```

Source: `src/temporal_conversions.rs:267`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Converts an [`ArrowPrimitiveType`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-ddd581d2aed24174207ba803) to [`NaiveDate`]

Unresolved upstream links (retained, not inferred): ``NaiveDate``.
