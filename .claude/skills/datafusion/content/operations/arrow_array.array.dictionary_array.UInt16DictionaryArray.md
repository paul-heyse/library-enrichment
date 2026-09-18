# `arrow_array::array::dictionary_array::UInt16DictionaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.dictionary_array.UInt16DictionaryArray.json).

<a id="op-15dc5f62266ef02e5366ec6d"></a>
## UInt16DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::UInt16DictionaryArray` · arrow-array 59.3.0

```rust
type UInt16DictionaryArray = DictionaryArray<UInt16Type>
```

Source: `src/array/dictionary_array.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) indexed by `u16`

# Example: Using `collect`
```
# use arrow_array::{Array, UInt16DictionaryArray, UInt16Array, StringArray};
# use std::sync::Arc;

let array: UInt16DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &UInt16Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) for more information and examples
