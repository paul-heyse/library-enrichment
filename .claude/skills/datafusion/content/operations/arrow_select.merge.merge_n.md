# `arrow_select::merge::merge_n`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.merge.merge_n.json).

<a id="op-7175e59dc6ab13f99ed7b34d"></a>
## merge_n

`function` · `arrow_select::merge::merge_n` · arrow-select 59.3.0

```rust
fn merge_n(values: &[&dyn Array], indices: &[impl MergeIndex]) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Source: `src/merge.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Merges elements by index from a list of [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), creating a new [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) from
those values.

Each element in `indices` is the index of an array in `values`. The `indices` array is processed
sequentially. The first occurrence of index value `n` will be mapped to the first
value of the array at index `n`. The second occurrence to the second value, and so on.
An index value where `MergeIndex::index` returns `None` is interpreted as a null value.

# Implementation notes

This algorithm is similar in nature to both [zip](../operations/arrow_select.zip.zip.md#op-c40a0b0a379f198055f80c56) and
[interleave](crate::interleave::interleave), but there are some important differences.

In contrast to [zip](../operations/arrow_select.zip.zip.md#op-c40a0b0a379f198055f80c56), this function supports multiple input arrays. Instead of
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
