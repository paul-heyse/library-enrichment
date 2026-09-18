# `arrow_select::merge::MergeIndex`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.merge.MergeIndex.json).

<a id="op-a174ae8976a0e08fcee9e6c8"></a>
## MergeIndex

`trait` · `arrow_select::merge::MergeIndex` · arrow-select 59.3.0

```rust
trait MergeIndex: PartialEq + Eq + Copy
```

Source: `src/merge.rs:35`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

An index for the [merge_n](../operations/arrow_select.merge.merge_n.md#op-7175e59dc6ab13f99ed7b34d) function.

This trait allows the indices argument for [merge_n](../operations/arrow_select.merge.merge_n.md#op-7175e59dc6ab13f99ed7b34d) to be stored using a more
compact representation than `usize` when the input arrays are small.
If the number of input arrays is less than 256 for instance, the indices can be stored as `u8`.

Implementation must ensure that all values which return `None` from [MergeIndex::index](../operations/arrow_select.merge.MergeIndex.md#op-eabcbe8fd1c0f3ce1673ed59) are
considered equal by the [PartialEq] and [Eq] implementations.

Unresolved upstream links (retained, not inferred): `Eq`, `PartialEq`.

<a id="op-eabcbe8fd1c0f3ce1673ed59"></a>
## index

`function` · `arrow_select::merge::MergeIndex::index` · arrow-select 59.3.0

```rust
fn index(&self) -> Option<usize>
```

Source: `src/merge.rs:40`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Returns the index value as an `Option<usize>`.

`None` values returned by this function indicate holes in the index array and will result
in null values in the array created by [merge](../operations/arrow_select.merge.merge.md#op-9664f8c4a218e3503f2ba759).
