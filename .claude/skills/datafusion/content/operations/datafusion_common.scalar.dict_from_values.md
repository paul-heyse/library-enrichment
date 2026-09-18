# `datafusion_common::scalar::dict_from_values`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.scalar.dict_from_values.json).

<a id="op-820808386a36e6f61c820fb5"></a>
## dict_from_values

`function` · `datafusion_common::scalar::dict_from_values` · datafusion-common 55.1.0

```rust
fn dict_from_values<K: ArrowDictionaryKeyType>(values_array: arrow::array::ArrayRef) -> error::Result<arrow::array::ArrayRef>
```

Source: `src/scalar/mod.rs:1126`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a `DictionaryArray` from the provided values array.

Each element gets a unique key (`0..N-1`), without deduplication.
Useful for wrapping arrays in dictionary form.

# Input
["alice", "bob", "alice", null, "carol"]

# Output
`DictionaryArray<Int32>`
{
  keys:   [0, 1, 2, 3, 4],
  values: ["alice", "bob", "alice", null, "carol"]
}
