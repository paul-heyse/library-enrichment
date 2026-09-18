# `arrow_array::array::ListLikeArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.ListLikeArray.json).

<a id="op-40161f2b0a7feb69b4747c85"></a>
## ListLikeArray

`trait` · `arrow_array::array::ListLikeArray` · arrow-array 59.3.0

```rust
trait ListLikeArray: Array
```

Source: `src/array/mod.rs:755`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A trait for Arrow list-like arrays, abstracting over
[`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b), [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a), and [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee).

This trait provides a uniform interface for accessing the child values and
computing the element range for a given index, regardless of the underlying
list layout (offsets, offsets+sizes, or fixed-size).

<a id="op-d6081ead239cbe6a9ddbd701"></a>
## element_range

`function` · `arrow_array::array::ListLikeArray::element_range` · arrow-array 59.3.0

```rust
fn element_range(&self, index: usize) -> std::ops::Range<usize>
```

Source: `src/array/mod.rs:761`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the start and end indices into the values array for the list
element at `index`.

<a id="op-171bae71e037c8b663eafad7"></a>
## values

`function` · `arrow_array::array::ListLikeArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Source: `src/array/mod.rs:757`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the child values array.
