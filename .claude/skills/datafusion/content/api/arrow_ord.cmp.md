# `arrow_ord::cmp`

Crate `arrow-ord` · 9 public items · structured records in [`model/arrow_ord.cmp.json`](../model/arrow_ord.cmp.json)

## compare_byte_view

`function` · `arrow_ord::cmp::compare_byte_view`

Also reachable as `arrow::compute::kernels::cmp::compare_byte_view`

```rust
fn compare_byte_view<T: ByteViewType>(left: &arrow_array::GenericByteViewArray<T>, left_idx: usize, right: &arrow_array::GenericByteViewArray<T>, right_idx: usize) -> std::cmp::Ordering
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.cmp.compare_byte_view.md).


Compares two [`GenericByteViewArray`] at index `left_idx` and `right_idx`

---

## distinct

`function` · `arrow_ord::cmp::distinct`

Also reachable as `arrow::compute::kernels::cmp::distinct`

```rust
fn distinct(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.cmp.distinct.md).


Perform `left IS DISTINCT FROM right` operation on two [`Datum`]

[`distinct`] is similar to [`neq`], only differing in null handling. In particular, two
operands are considered DISTINCT if they have a different value or if one of them is NULL
and the other isn't. The result of [`distinct`] is never NULL.

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`]

---

## eq

`function` · `arrow_ord::cmp::eq`

Also reachable as `arrow::compute::kernels::cmp::eq`

```rust
fn eq(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.cmp.eq.md).


Perform `left == right` operation on two [`Datum`].

Comparing null values on either side will yield a null in the corresponding
slot of the resulting [`BooleanArray`].

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`]

---

## gt

`function` · `arrow_ord::cmp::gt`

Also reachable as `arrow::compute::kernels::cmp::gt`

```rust
fn gt(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.cmp.gt.md).


Perform `left > right` operation on two [`Datum`].

Comparing null values on either side will yield a null in the corresponding
slot of the resulting [`BooleanArray`].

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`]

---

## gt_eq

`function` · `arrow_ord::cmp::gt_eq`

Also reachable as `arrow::compute::kernels::cmp::gt_eq`

```rust
fn gt_eq(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.cmp.gt_eq.md).


Perform `left >= right` operation on two [`Datum`].

Comparing null values on either side will yield a null in the corresponding
slot of the resulting [`BooleanArray`].

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`]

---

## lt

`function` · `arrow_ord::cmp::lt`

Also reachable as `arrow::compute::kernels::cmp::lt`

```rust
fn lt(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.cmp.lt.md).


Perform `left < right` operation on two [`Datum`].

Comparing null values on either side will yield a null in the corresponding
slot of the resulting [`BooleanArray`].

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`]

---

## lt_eq

`function` · `arrow_ord::cmp::lt_eq`

Also reachable as `arrow::compute::kernels::cmp::lt_eq`

```rust
fn lt_eq(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.cmp.lt_eq.md).


Perform `left <= right` operation on two [`Datum`].

Comparing null values on either side will yield a null in the corresponding
slot of the resulting [`BooleanArray`].

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`]

---

## neq

`function` · `arrow_ord::cmp::neq`

Also reachable as `arrow::compute::kernels::cmp::neq`

```rust
fn neq(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.cmp.neq.md).


Perform `left != right` operation on two [`Datum`].

Comparing null values on either side will yield a null in the corresponding
slot of the resulting [`BooleanArray`].

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`]

---

## not_distinct

`function` · `arrow_ord::cmp::not_distinct`

Also reachable as `arrow::compute::kernels::cmp::not_distinct`

```rust
fn not_distinct(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<arrow_array::BooleanArray, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.cmp.not_distinct.md).


Perform `left IS NOT DISTINCT FROM right` operation on two [`Datum`]

[`not_distinct`] is similar to [`eq`], only differing in null handling. In particular, two
operands are considered `NOT DISTINCT` if they have the same value or if both of them
is NULL. The result of [`not_distinct`] is never NULL.

For floating values like f32 and f64, this comparison produces an ordering in accordance to
the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard.
Note that totalOrder treats positive and negative zeros as different. If it is necessary
to treat them as equal, please normalize zeros before calling this kernel. See
[`f32::total_cmp`] and [`f64::total_cmp`].

Nested types, such as lists, are not supported as the null semantics are not well-defined.
For comparisons involving nested types see [`crate::ord::make_comparator`]

---
