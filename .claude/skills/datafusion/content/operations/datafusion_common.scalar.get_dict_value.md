# `datafusion_common::scalar::get_dict_value`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.scalar.get_dict_value.json).

<a id="op-da93a92c2e312a0b62322024"></a>
## get_dict_value

`function` · `datafusion_common::scalar::get_dict_value` · datafusion-common 55.1.0

```rust
fn get_dict_value<K: ArrowDictionaryKeyType>(array: &dyn Array, index: usize) -> error::Result<(&arrow::array::ArrayRef, Option<usize>)>
```

Source: `src/scalar/mod.rs:1080`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return a reference to the values array and the index into it for a
dictionary array

# Errors

Errors if the array cannot be downcasted to DictionaryArray
