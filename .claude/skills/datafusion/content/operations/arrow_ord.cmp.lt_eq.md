# `arrow_ord::cmp::lt_eq`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.cmp.lt_eq.json).

<a id="op-7ba1d6a9f0799fe6883f8511"></a>
## lt_eq

`function` · `arrow_ord::cmp::lt_eq` · arrow-ord 59.3.0

```rust
fn lt_eq(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

Source: `src/cmp.rs:130`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Perform `left <= right` operation on two [`Datum`](../operations/arrow_array.scalar.Datum.md#op-8fd206fb21d47e60f8e9716f).

Comparing null values on either side will yield a null in the corresponding
slot of the resulting [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505).

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`](../operations/arrow_ord.ord.make_comparator.md#op-81ddd97bea600d32278eaff7)

Unresolved upstream links (retained, not inferred): ``f64::total_cmp``, ``f32::total_cmp``.
