# `arrow_array::cast::AsArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.AsArray.json).

<a id="op-687c2dc0675984bae2049af6"></a>
## AsArray

`trait` · `arrow_array::cast::AsArray` · arrow-array 59.3.0

```rust
trait AsArray: private::Sealed
```

Source: `src/cast.rs:835`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An extension trait for `dyn Array` that provides ergonomic downcasting

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array};
# use arrow_array::cast::AsArray;
# use arrow_array::types::Int32Type;
let col = Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef;
assert_eq!(col.as_primitive::<Int32Type>().values(), &[1, 2, 3]);
```

<a id="op-936d9f41d40b7be15147c47e"></a>
## as_any_dictionary

`function` · `arrow_array::cast::AsArray::as_any_dictionary` · arrow-array 59.3.0

```rust
fn as_any_dictionary(&self) -> &dyn AnyDictionaryArray
```

Source: `src/cast.rs:986`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcasts this to a [`AnyDictionaryArray`](../operations/arrow_array.array.dictionary_array.AnyDictionaryArray.md#op-5955577f57f524b623a2af12) panicking if not possible

<a id="op-ea0c096d64b7dccb3a293969"></a>
## as_any_dictionary_opt

`function` · `arrow_array::cast::AsArray::as_any_dictionary_opt` · arrow-array 59.3.0

```rust
fn as_any_dictionary_opt(&self) -> Option<&dyn AnyDictionaryArray>
```

Source: `src/cast.rs:983`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcasts this to a [`AnyDictionaryArray`](../operations/arrow_array.array.dictionary_array.AnyDictionaryArray.md#op-5955577f57f524b623a2af12) returning `None` if not possible

<a id="op-04498726139c64f351e0369a"></a>
## as_any_ree

`function` · `arrow_array::cast::AsArray::as_any_ree` · arrow-array 59.3.0

```rust
fn as_any_ree(&self) -> &dyn AnyRunEndArray
```

Source: `src/cast.rs:994`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcasts this to a [`AnyRunEndArray`](../operations/arrow_array.array.run_array.AnyRunEndArray.md#op-66f5298fc7c34c1bec0ad508) panicking if not possible

<a id="op-da04e2bbe98d63613d33abf6"></a>
## as_any_ree_opt

`function` · `arrow_array::cast::AsArray::as_any_ree_opt` · arrow-array 59.3.0

```rust
fn as_any_ree_opt(&self) -> Option<&dyn AnyRunEndArray>
```

Source: `src/cast.rs:991`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcasts this to a [`AnyRunEndArray`](../operations/arrow_array.array.run_array.AnyRunEndArray.md#op-66f5298fc7c34c1bec0ad508) returning `None` if not possible

<a id="op-e76489f1749f06d65b1178cb"></a>
## as_binary

`function` · `arrow_array::cast::AsArray::as_binary` · arrow-array 59.3.0

```rust
fn as_binary<O: OffsetSizeTrait>(&self) -> &GenericBinaryArray<O>
```

Source: `src/cast.rs:876`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericBinaryArray`](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6) panicking if not possible

<a id="op-844a9460a80c8bf9199ac1b9"></a>
## as_binary_opt

`function` · `arrow_array::cast::AsArray::as_binary_opt` · arrow-array 59.3.0

```rust
fn as_binary_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericBinaryArray<O>>
```

Source: `src/cast.rs:871`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericBinaryArray`](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6) returning `None` if not possible

<a id="op-fccf943193ccef5330aa9c02"></a>
## as_binary_view

`function` · `arrow_array::cast::AsArray::as_binary_view` · arrow-array 59.3.0

```rust
fn as_binary_view(&self) -> &BinaryViewArray
```

Source: `src/cast.rs:896`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`BinaryViewArray`](../operations/arrow_array.array.byte_view_array.BinaryViewArray.md#op-3ee392743cc781de5833990c) panicking if not possible

<a id="op-e0775c8f509b151c0bd00246"></a>
## as_binary_view_opt

`function` · `arrow_array::cast::AsArray::as_binary_view_opt` · arrow-array 59.3.0

```rust
fn as_binary_view_opt(&self) -> Option<&BinaryViewArray>
```

Source: `src/cast.rs:891`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`BinaryViewArray`](../operations/arrow_array.array.byte_view_array.BinaryViewArray.md#op-3ee392743cc781de5833990c) returning `None` if not possible

<a id="op-3a7a18cf18f2f344c0cdebcd"></a>
## as_boolean

`function` · `arrow_array::cast::AsArray::as_boolean` · arrow-array 59.3.0

```rust
fn as_boolean(&self) -> &BooleanArray
```

Source: `src/cast.rs:840`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) panicking if not possible

<a id="op-b50d99aad738e51fe5eb5aaf"></a>
## as_boolean_opt

`function` · `arrow_array::cast::AsArray::as_boolean_opt` · arrow-array 59.3.0

```rust
fn as_boolean_opt(&self) -> Option<&BooleanArray>
```

Source: `src/cast.rs:837`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) returning `None` if not possible

<a id="op-84a75948546eb6d79e9b337e"></a>
## as_byte_view

`function` · `arrow_array::cast::AsArray::as_byte_view` · arrow-array 59.3.0

```rust
fn as_byte_view<T: ByteViewType>(&self) -> &GenericByteViewArray<T>
```

Source: `src/cast.rs:904`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) panicking if not possible

<a id="op-e380db35c142f0e06730dcb4"></a>
## as_byte_view_opt

`function` · `arrow_array::cast::AsArray::as_byte_view_opt` · arrow-array 59.3.0

```rust
fn as_byte_view_opt<T: ByteViewType>(&self) -> Option<&GenericByteViewArray<T>>
```

Source: `src/cast.rs:901`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) returning `None` if not possible

<a id="op-381b0eb537312cf409e5135e"></a>
## as_bytes

`function` · `arrow_array::cast::AsArray::as_bytes` · arrow-array 59.3.0

```rust
fn as_bytes<T: ByteArrayType>(&self) -> &GenericByteArray<T>
```

Source: `src/cast.rs:856`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) panicking if not possible

<a id="op-cca1c4dbca854445dc334952"></a>
## as_bytes_opt

`function` · `arrow_array::cast::AsArray::as_bytes_opt` · arrow-array 59.3.0

```rust
fn as_bytes_opt<T: ByteArrayType>(&self) -> Option<&GenericByteArray<T>>
```

Source: `src/cast.rs:853`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) returning `None` if not possible

<a id="op-fc589a0f27a8f074f2e415c2"></a>
## as_dictionary

`function` · `arrow_array::cast::AsArray::as_dictionary` · arrow-array 59.3.0

```rust
fn as_dictionary<K: ArrowDictionaryKeyType>(&self) -> &DictionaryArray<K>
```

Source: `src/cast.rs:970`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) panicking if not possible

<a id="op-24f37dbc5a5cd8bfc35f650c"></a>
## as_dictionary_opt

`function` · `arrow_array::cast::AsArray::as_dictionary_opt` · arrow-array 59.3.0

```rust
fn as_dictionary_opt<K: ArrowDictionaryKeyType>(&self) -> Option<&DictionaryArray<K>>
```

Source: `src/cast.rs:967`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) returning `None` if not possible

<a id="op-e37c77db77ac3a5b6c9181ac"></a>
## as_fixed_size_binary

`function` · `arrow_array::cast::AsArray::as_fixed_size_binary` · arrow-array 59.3.0

```rust
fn as_fixed_size_binary(&self) -> &FixedSizeBinaryArray
```

Source: `src/cast.rs:944`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) panicking if not possible

<a id="op-7fbb4e767549ec0c68a2aa2e"></a>
## as_fixed_size_binary_opt

`function` · `arrow_array::cast::AsArray::as_fixed_size_binary_opt` · arrow-array 59.3.0

```rust
fn as_fixed_size_binary_opt(&self) -> Option<&FixedSizeBinaryArray>
```

Source: `src/cast.rs:941`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) returning `None` if not possible

<a id="op-b5786c1d70b05db5f9b96eb3"></a>
## as_fixed_size_list

`function` · `arrow_array::cast::AsArray::as_fixed_size_list` · arrow-array 59.3.0

```rust
fn as_fixed_size_list(&self) -> &FixedSizeListArray
```

Source: `src/cast.rs:953`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) panicking if not possible

<a id="op-f21bed4c03c66c35b228924b"></a>
## as_fixed_size_list_opt

`function` · `arrow_array::cast::AsArray::as_fixed_size_list_opt` · arrow-array 59.3.0

```rust
fn as_fixed_size_list_opt(&self) -> Option<&FixedSizeListArray>
```

Source: `src/cast.rs:950`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) returning `None` if not possible

<a id="op-25e9cc02317b80f882be0746"></a>
## as_list

`function` · `arrow_array::cast::AsArray::as_list` · arrow-array 59.3.0

```rust
fn as_list<O: OffsetSizeTrait>(&self) -> &GenericListArray<O>
```

Source: `src/cast.rs:928`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) panicking if not possible

<a id="op-4dd84b65a6d7173b9c3794ce"></a>
## as_list_opt

`function` · `arrow_array::cast::AsArray::as_list_opt` · arrow-array 59.3.0

```rust
fn as_list_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericListArray<O>>
```

Source: `src/cast.rs:925`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) returning `None` if not possible

<a id="op-3ac27d6ae7b86ec1d2ed54ea"></a>
## as_list_view

`function` · `arrow_array::cast::AsArray::as_list_view` · arrow-array 59.3.0

```rust
fn as_list_view<O: OffsetSizeTrait>(&self) -> &GenericListViewArray<O>
```

Source: `src/cast.rs:936`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) panicking if not possible

<a id="op-4dfe8c614109227a726f0faf"></a>
## as_list_view_opt

`function` · `arrow_array::cast::AsArray::as_list_view_opt` · arrow-array 59.3.0

```rust
fn as_list_view_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericListViewArray<O>>
```

Source: `src/cast.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) returning `None` if not possible

<a id="op-bf8641cd169c53260ec6f7be"></a>
## as_map

`function` · `arrow_array::cast::AsArray::as_map` · arrow-array 59.3.0

```rust
fn as_map(&self) -> &MapArray
```

Source: `src/cast.rs:962`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f) panicking if not possible

<a id="op-b76791132d5dff4a37ab82ab"></a>
## as_map_opt

`function` · `arrow_array::cast::AsArray::as_map_opt` · arrow-array 59.3.0

```rust
fn as_map_opt(&self) -> Option<&MapArray>
```

Source: `src/cast.rs:959`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f) returning `None` if not possible

<a id="op-928141978f71c213d31665a3"></a>
## as_primitive

`function` · `arrow_array::cast::AsArray::as_primitive` · arrow-array 59.3.0

```rust
fn as_primitive<T: ArrowPrimitiveType>(&self) -> &PrimitiveArray<T>
```

Source: `src/cast.rs:848`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) panicking if not possible

<a id="op-f1aed375c67d2e5d70b1fc52"></a>
## as_primitive_opt

`function` · `arrow_array::cast::AsArray::as_primitive_opt` · arrow-array 59.3.0

```rust
fn as_primitive_opt<T: ArrowPrimitiveType>(&self) -> Option<&PrimitiveArray<T>>
```

Source: `src/cast.rs:845`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) returning `None` if not possible

<a id="op-34bd73ad85acd7b010882e55"></a>
## as_run

`function` · `arrow_array::cast::AsArray::as_run` · arrow-array 59.3.0

```rust
fn as_run<K: RunEndIndexType>(&self) -> &RunArray<K>
```

Source: `src/cast.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) panicking if not possible

<a id="op-5300ae43f073ba9fae7f0791"></a>
## as_run_opt

`function` · `arrow_array::cast::AsArray::as_run_opt` · arrow-array 59.3.0

```rust
fn as_run_opt<K: RunEndIndexType>(&self) -> Option<&RunArray<K>>
```

Source: `src/cast.rs:975`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) returning `None` if not possible

<a id="op-3299ca186268fd6adf7c1cd3"></a>
## as_string

`function` · `arrow_array::cast::AsArray::as_string` · arrow-array 59.3.0

```rust
fn as_string<O: OffsetSizeTrait>(&self) -> &GenericStringArray<O>
```

Source: `src/cast.rs:866`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) panicking if not possible

<a id="op-518416fccb36c875eba7fc4d"></a>
## as_string_opt

`function` · `arrow_array::cast::AsArray::as_string_opt` · arrow-array 59.3.0

```rust
fn as_string_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericStringArray<O>>
```

Source: `src/cast.rs:861`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) returning `None` if not possible

<a id="op-fa0ec4fbe39a360349e4c44a"></a>
## as_string_view

`function` · `arrow_array::cast::AsArray::as_string_view` · arrow-array 59.3.0

```rust
fn as_string_view(&self) -> &StringViewArray
```

Source: `src/cast.rs:886`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0) panicking if not possible

<a id="op-1562a59cf46dc8b724cfe4a8"></a>
## as_string_view_opt

`function` · `arrow_array::cast::AsArray::as_string_view_opt` · arrow-array 59.3.0

```rust
fn as_string_view_opt(&self) -> Option<&StringViewArray>
```

Source: `src/cast.rs:881`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0) returning `None` if not possible

<a id="op-1a071787f37703a26bafad28"></a>
## as_struct

`function` · `arrow_array::cast::AsArray::as_struct` · arrow-array 59.3.0

```rust
fn as_struct(&self) -> &StructArray
```

Source: `src/cast.rs:912`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) panicking if not possible

<a id="op-d9aba84d88a8a8768d1e5cbd"></a>
## as_struct_opt

`function` · `arrow_array::cast::AsArray::as_struct_opt` · arrow-array 59.3.0

```rust
fn as_struct_opt(&self) -> Option<&StructArray>
```

Source: `src/cast.rs:909`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) returning `None` if not possible

<a id="op-7d925dbb5a756ae047f3b297"></a>
## as_union

`function` · `arrow_array::cast::AsArray::as_union` · arrow-array 59.3.0

```rust
fn as_union(&self) -> &UnionArray
```

Source: `src/cast.rs:920`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`UnionArray`](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac) panicking if not possible

<a id="op-482176222b9641c0452c59b2"></a>
## as_union_opt

`function` · `arrow_array::cast::AsArray::as_union_opt` · arrow-array 59.3.0

```rust
fn as_union_opt(&self) -> Option<&UnionArray>
```

Source: `src/cast.rs:917`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast this to a [`UnionArray`](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac) returning `None` if not possible
