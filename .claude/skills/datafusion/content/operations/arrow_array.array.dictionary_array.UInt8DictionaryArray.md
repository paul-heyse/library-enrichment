# `arrow_array::array::dictionary_array::UInt8DictionaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.dictionary_array.UInt8DictionaryArray.json).

<a id="op-9bfdfd3b324524a8884bd96a"></a>
## UInt8DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::UInt8DictionaryArray` · arrow-array 59.3.0

```rust
type UInt8DictionaryArray = DictionaryArray<UInt8Type>
```

Source: `src/array/dictionary_array.rs:112`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) indexed by `u8`

# Example: Using `collect`
```
# use arrow_array::{Array, UInt8DictionaryArray, UInt8Array, StringArray};
# use std::sync::Arc;

let array: UInt8DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &UInt8Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) for more information and examples
