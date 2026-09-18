# `arrow_array::cast::as_union_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_union_array.json).

<a id="op-c6fceaabf01ff7ad74c2bfba"></a>
## as_union_array

`function` · `arrow_array::cast::as_union_array` · arrow-array 59.3.0

```rust
fn as_union_array(arr: &dyn Array) -> &UnionArray
```

Source: `src/cast.rs:786`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to 
[`UnionArray`](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac), panicking on failure.
