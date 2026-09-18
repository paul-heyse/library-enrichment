# `datafusion_common::cast::as_dictionary_array`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cast.as_dictionary_array.json).

<a id="op-1d64db2fe28ed3b558f95481"></a>
## as_dictionary_array

`function` · `datafusion_common::cast::as_dictionary_array` · datafusion-common 55.1.0

```rust
fn as_dictionary_array<T: ArrowDictionaryKeyType>(array: &dyn Array) -> Result<&arrow::array::DictionaryArray<T>>
```

Source: `src/cast.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
