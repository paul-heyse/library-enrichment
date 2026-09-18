# `arrow_array::array::list_array::OffsetSizeTrait`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.list_array.OffsetSizeTrait.json).

<a id="op-12e7920dc4d5863d6b6252b3"></a>
## OffsetSizeTrait

`trait` · `arrow_array::array::list_array::OffsetSizeTrait` · arrow-array 59.3.0

```rust
trait OffsetSizeTrait: ArrowNativeType + std::ops::AddAssign + Integer + num_traits::CheckedAdd + num_traits::CheckedSub
```

Source: `src/array/list_array.rs:40`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A type that can be used within a variable-size array to encode offset information

See [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456), [`LargeListArray`](../operations/arrow_array.array.list_array.LargeListArray.md#op-e34a694617d66e91221407db), [`BinaryArray`], [`LargeBinaryArray`],
[`StringArray`] and [`LargeStringArray`]

[`BinaryArray`]: crate::array::BinaryArray
[`LargeBinaryArray`]: crate::array::LargeBinaryArray
[`StringArray`]: crate::array::StringArray
[`LargeStringArray`]: crate::array::LargeStringArray

<a id="op-e9bc56ef246d6b95289e5f6c"></a>
## IS_LARGE

`assoc_const` · `arrow_array::array::list_array::OffsetSizeTrait::IS_LARGE` · arrow-array 59.3.0

```rust
IS_LARGE
```

Source: `src/array/list_array.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

True for 64 bit offset size and false for 32 bit offset size

<a id="op-5c7c7f4a25036b77eb702d8c"></a>
## MAX_OFFSET

`assoc_const` · `arrow_array::array::list_array::OffsetSizeTrait::MAX_OFFSET` · arrow-array 59.3.0

```rust
MAX_OFFSET
```

Source: `src/array/list_array.rs:48`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The max `usize` offset

<a id="op-6ab94a3fa75397672083c157"></a>
## PREFIX

`assoc_const` · `arrow_array::array::list_array::OffsetSizeTrait::PREFIX` · arrow-array 59.3.0

```rust
PREFIX
```

Source: `src/array/list_array.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Prefix for the offset size
