# `arrow_select::merge`

Crate `arrow-select` · 3 public items · structured records in [`model/arrow_select.merge.json`](../model/arrow_select.merge.json)

## merge

`function` · `arrow_select::merge::merge`

Also reachable as `arrow::compute::kernels::merge::merge`

```rust
fn merge(mask: &arrow_array::BooleanArray, truthy: &dyn Datum, falsy: &dyn Datum) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Merges two arrays in the order specified by a boolean mask.

This algorithm is a variant of [zip] that does not require the truthy and
falsy arrays to have the same length.

When truthy of falsy are [Scalar](arrow_array::Scalar), the single
scalar value is repeated whenever the mask array contains true or false respectively.

# Example

```text
 truthy
┌─────────┐  mask
│    A    │  ┌─────────┐                             ┌─────────┐
├─────────┤  │  true   │                             │    A    │
│    C    │  ├─────────┤                             ├─────────┤
├─────────┤  │  true   │                             │    C    │
│   NULL  │  ├─────────┤                             ├─────────┤
├─────────┤  │  false  │  merge(mask, truthy, falsy) │    B    │
│    D    │  ├─────────┤  ─────────────────────────▶ ├─────────┤
└─────────┘  │  true   │                             │   NULL  │
 falsy       ├─────────┤                             ├─────────┤
┌─────────┐  │  false  │                             │    E    │
│    B    │  ├─────────┤                             ├─────────┤
├─────────┤  │  true   │                             │    D    │
│    E    │  └─────────┘                             └─────────┘
└─────────┘
```

---

## merge_n

`function` · `arrow_select::merge::merge_n`

Also reachable as `arrow::compute::kernels::merge::merge_n`

```rust
fn merge_n(values: &[&dyn Array], indices: &[impl MergeIndex]) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Merges elements by index from a list of [`Array`], creating a new [`Array`] from
those values.

Each element in `indices` is the index of an array in `values`. The `indices` array is processed
sequentially. The first occurrence of index value `n` will be mapped to the first
value of the array at index `n`. The second occurrence to the second value, and so on.
An index value where `MergeIndex::index` returns `None` is interpreted as a null value.

# Implementation notes

This algorithm is similar in nature to both [zip] and
[interleave](crate::interleave::interleave), but there are some important differences.

In contrast to [zip], this function supports multiple input arrays. Instead of
a boolean selection vector, an index array is to take values from the input arrays, and a special
marker values can be used to indicate null values.

In contrast to [interleave](crate::interleave::interleave), this function does not use pairs of
indices. The values in `indices` serve the same purpose as the first value in the pairs passed
to `interleave`.
The index in the array is implicit and is derived from the number of times a particular array
index occurs.
The more constrained indexing mechanism used by this algorithm makes it easier to copy values
in contiguous slices. In the example below, the two subsequent elements from array `2` can be
copied in a single operation from the source array instead of copying them one by one.
Long spans of null values are also especially cheap because they do not need to be represented
in an input array.

# Panics

This function does not check that the number of occurrences of any particular array index matches
the length of the corresponding input array. If an array contains more values than required, the
spurious values will be ignored. If an array contains fewer values than necessary, this function
will panic.

# Example

```text
┌───────────┐  ┌─────────┐                             ┌─────────┐
│┌─────────┐│  │   None  │                             │   NULL  │
││    A    ││  ├─────────┤                             ├─────────┤
│└─────────┘│  │    1    │                             │    B    │
│┌─────────┐│  ├─────────┤                             ├─────────┤
││    B    ││  │    0    │    merge(values, indices)   │    A    │
│└─────────┘│  ├─────────┤  ─────────────────────────▶ ├─────────┤
│┌─────────┐│  │   None  │                             │   NULL  │
││    C    ││  ├─────────┤                             ├─────────┤
│├─────────┤│  │    2    │                             │    C    │
││    D    ││  ├─────────┤                             ├─────────┤
│└─────────┘│  │    2    │                             │    D    │
└───────────┘  └─────────┘                             └─────────┘
   values        indices                                  result

```

---

## MergeIndex

`trait` · `arrow_select::merge::MergeIndex`

Also reachable as `arrow::compute::kernels::merge::MergeIndex`

```rust
trait MergeIndex: PartialEq + Eq + Copy
```

**Implementors** (1)

- `core::option::Option`

**Methods** (1)

```rust
fn index(&self) -> Option<usize>
```

An index for the [merge_n] function.

This trait allows the indices argument for [merge_n] to be stored using a more
compact representation than `usize` when the input arrays are small.
If the number of input arrays is less than 256 for instance, the indices can be stored as `u8`.

Implementation must ensure that all values which return `None` from [MergeIndex::index] are
considered equal by the [PartialEq] and [Eq] implementations.

---
