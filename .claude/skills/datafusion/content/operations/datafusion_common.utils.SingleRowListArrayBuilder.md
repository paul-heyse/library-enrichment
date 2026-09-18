# `datafusion_common::utils::SingleRowListArrayBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.SingleRowListArrayBuilder.json).

<a id="op-f58900a15dc7a9ba9a0558e1"></a>
## SingleRowListArrayBuilder

`struct` · `datafusion_common::utils::SingleRowListArrayBuilder` · datafusion-common 55.1.0

```rust
struct SingleRowListArrayBuilder
```

Source: `src/utils/mod.rs:552`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates single element [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456), [`LargeListArray`](../operations/arrow_array.array.list_array.LargeListArray.md#op-e34a694617d66e91221407db) and
[`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) from other arrays

For example this builder can convert `[1, 2, 3]` into `[[1, 2, 3]]`

# Example
```
# use std::sync::Arc;
# use arrow::array::{Array, ListArray};
# use arrow::array::types::Int64Type;
# use datafusion_common::utils::SingleRowListArrayBuilder;
// Array is [1, 2, 3]
let arr = ListArray::from_iter_primitive::<Int64Type, _, _>(vec![Some(vec![
    Some(1),
    Some(2),
    Some(3),
])]);
// Wrap as a list array: [[1, 2, 3]]
let list_arr = SingleRowListArrayBuilder::new(Arc::new(arr)).build_list_array();
assert_eq!(list_arr.len(), 1);
```

<a id="op-70a3ce707bd5656c9adffc58"></a>
## build_fixed_size_list_array

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::build_fixed_size_list_array` · datafusion-common 55.1.0

```rust
fn build_fixed_size_list_array(self, list_size: usize) -> FixedSizeListArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:615`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a single element [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee)

<a id="op-a0dea93aa41e856d9243e209"></a>
## build_fixed_size_list_scalar

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::build_fixed_size_list_scalar` · datafusion-common 55.1.0

```rust
fn build_fixed_size_list_scalar(self, list_size: usize) -> ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:622`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a single element [`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee) and wrap as [`ScalarValue::FixedSizeList`](../operations/datafusion_common.scalar.ScalarValue.md#op-7a219ddf418a98c406f24201)

<a id="op-f55f05357a90f38787f05220"></a>
## build_large_list_array

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::build_large_list_array` · datafusion-common 55.1.0

```rust
fn build_large_list_array(self) -> LargeListArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:603`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a single element [`LargeListArray`](../operations/arrow_array.array.list_array.LargeListArray.md#op-e34a694617d66e91221407db)

<a id="op-8486284246f367694ab23a38"></a>
## build_large_list_scalar

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::build_large_list_scalar` · datafusion-common 55.1.0

```rust
fn build_large_list_scalar(self) -> ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:610`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a single element [`LargeListArray`](../operations/arrow_array.array.list_array.LargeListArray.md#op-e34a694617d66e91221407db) and wrap as [`ScalarValue::LargeList`](../operations/datafusion_common.scalar.ScalarValue.md#op-0f35f90f8f14580301c7b657)

<a id="op-a246ad24095224edc4d7f482"></a>
## build_large_list_view_array

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::build_large_list_view_array` · datafusion-common 55.1.0

```rust
fn build_large_list_view_array(self) -> LargeListViewArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:642`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a single element [`LargeListViewArray`](../operations/arrow_array.array.list_view_array.LargeListViewArray.md#op-184699cca31c6f96df389081)

<a id="op-bc3940e2e9107181a408aaa0"></a>
## build_large_list_view_scalar

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::build_large_list_view_scalar` · datafusion-common 55.1.0

```rust
fn build_large_list_view_scalar(self) -> ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:650`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a single element [`LargeListViewArray`](../operations/arrow_array.array.list_view_array.LargeListViewArray.md#op-184699cca31c6f96df389081) and wrap as [`ScalarValue::LargeListView`](../operations/datafusion_common.scalar.ScalarValue.md#op-c5759bf9933e5866c9665d4d)

<a id="op-06f7fbace2c17c9101051599"></a>
## build_list_array

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::build_list_array` · datafusion-common 55.1.0

```rust
fn build_list_array(self) -> ListArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:591`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a single element [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456)

<a id="op-bac9245b87e60832577628e5"></a>
## build_list_scalar

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::build_list_scalar` · datafusion-common 55.1.0

```rust
fn build_list_scalar(self) -> ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:598`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a single element [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456) and wrap as [`ScalarValue::List`](../operations/datafusion_common.scalar.ScalarValue.md#op-e02ff167c86d13c10b64e274)

<a id="op-3cac848fbb55555f3550c3db"></a>
## build_list_view_array

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::build_list_view_array` · datafusion-common 55.1.0

```rust
fn build_list_view_array(self) -> ListViewArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:627`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a single element [`ListViewArray`](../operations/arrow_array.array.list_view_array.ListViewArray.md#op-f804ab17e2d4b64e9d2e4b55)

<a id="op-cc8a4f3da5d609bdda3efab5"></a>
## build_list_view_scalar

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::build_list_view_scalar` · datafusion-common 55.1.0

```rust
fn build_list_view_scalar(self) -> ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:637`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a single element [`ListViewArray`](../operations/arrow_array.array.list_view_array.ListViewArray.md#op-f804ab17e2d4b64e9d2e4b55) and wrap as [`ScalarValue::ListView`](../operations/datafusion_common.scalar.ScalarValue.md#op-62f0e173b4fc141a6ff47741)

<a id="op-2565875898a4bbb22a263eaa"></a>
## clone

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> SingleRowListArrayBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 17], "end": [551, 22], "filename": "src/utils/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/utils/mod.rs:551`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fb4f5c87419ce52371cfeed"></a>
## fmt

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 10], "end": [551, 15], "filename": "src/utils/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/utils/mod.rs:551`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82c80fab9457dca860418a2c"></a>
## new

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::new` · datafusion-common 55.1.0

```rust
fn new(arr: ArrayRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:564`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new instance of [`SingleRowListArrayBuilder`](../operations/datafusion_common.utils.SingleRowListArrayBuilder.md#op-f58900a15dc7a9ba9a0558e1)

<a id="op-b9e0de6a99e954aa9b779a36"></a>
## with_field

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::with_field` · datafusion-common 55.1.0

```rust
fn with_field(self, field: &Field) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Copies field name and nullable from the specified field

<a id="op-4a8171683b99991d97c5a335"></a>
## with_field_name

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::with_field_name` · datafusion-common 55.1.0

```rust
fn with_field_name(self, field_name: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:579`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

sets the field name for the resulting array

<a id="op-344c98ddeab976d95839049f"></a>
## with_nullable

`function` · `datafusion_common::utils::SingleRowListArrayBuilder::with_nullable` · datafusion-common 55.1.0

```rust
fn with_nullable(self, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::SingleRowListArrayBuilder", "path": "SingleRowListArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [668, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:573`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the nullable flag
