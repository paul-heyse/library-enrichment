# `arrow_ord::cmp::not_distinct`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.cmp.not_distinct.json).

<a id="op-65f70dcd6ed5f7b6cb12d015"></a>
## not_distinct

`function` · `arrow_ord::cmp::not_distinct` · arrow-ord 59.3.0

```rust
fn not_distinct(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

Source: `src/cmp.rs:200`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Perform `left IS NOT DISTINCT FROM right` operation on two [`Datum`](../operations/arrow_array.scalar.Datum.md#op-8fd206fb21d47e60f8e9716f)

[`not_distinct`](../operations/arrow_ord.cmp.not_distinct.md#op-65f70dcd6ed5f7b6cb12d015) is similar to [`eq`](../operations/arrow_ord.cmp.eq.md#op-bb67c631d08cd3a9244030d0), only differing in null handling. In particular, two
operands are considered `NOT DISTINCT` if they have the same value or if both of them
is NULL. The result of [`not_distinct`](../operations/arrow_ord.cmp.not_distinct.md#op-65f70dcd6ed5f7b6cb12d015) is never NULL.

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`](../operations/arrow_ord.ord.make_comparator.md#op-81ddd97bea600d32278eaff7)

Unresolved upstream links (retained, not inferred): ``f64::total_cmp``, ``f32::total_cmp``.
