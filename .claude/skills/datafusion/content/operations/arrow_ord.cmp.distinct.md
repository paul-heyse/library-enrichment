# `arrow_ord::cmp::distinct`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.cmp.distinct.json).

<a id="op-cf0f2d119b1f484558b1d05b"></a>
## distinct

`function` · `arrow_ord::cmp::distinct` · arrow-ord 59.3.0

```rust
fn distinct(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

Source: `src/cmp.rs:182`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Perform `left IS DISTINCT FROM right` operation on two [`Datum`](../operations/arrow_array.scalar.Datum.md#op-8fd206fb21d47e60f8e9716f)

[`distinct`](../operations/arrow_ord.cmp.distinct.md#op-cf0f2d119b1f484558b1d05b) is similar to [`neq`](../operations/arrow_ord.cmp.neq.md#op-12e2c5bc948547865761dc22), only differing in null handling. In particular, two
operands are considered DISTINCT if they have a different value or if one of them is NULL
and the other isn't. The result of [`distinct`](../operations/arrow_ord.cmp.distinct.md#op-cf0f2d119b1f484558b1d05b) is never NULL.

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`](../operations/arrow_ord.ord.make_comparator.md#op-81ddd97bea600d32278eaff7)

Unresolved upstream links (retained, not inferred): ``f32::total_cmp``, ``f64::total_cmp``.
