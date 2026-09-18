# `arrow_array::array::list_array::ListArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.list_array.ListArray.json).

<a id="op-fbfbbaae30c81192ec1e4456"></a>
## ListArray

`type_alias` · `arrow_array::array::list_array::ListArray` · arrow-array 59.3.0

```rust
type ListArray = GenericListArray<i32>
```

Source: `src/array/list_array.rs:697`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) of variable size lists, storing offsets as `i32`.

See [`ListBuilder`](crate::builder::ListBuilder) for how to construct a [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456)

<a id="op-b91aecfe05df6163407ef970"></a>
## from

`function` · `arrow_array::array::list_array::ListArray::from` · arrow-array 59.3.0

```rust
fn from(value: MapArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::list_array::ListArray", "path": "crate::ListArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 1], "end": [603, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/map_array.rs:592`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
