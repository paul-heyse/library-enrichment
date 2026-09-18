# `arrow_array::array::dictionary_array::UInt64DictionaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.dictionary_array.UInt64DictionaryArray.json).

<a id="op-b14839e4a33e0677f0d69fca"></a>
## UInt64DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::UInt64DictionaryArray` · arrow-array 59.3.0

```rust
type UInt64DictionaryArray = DictionaryArray<UInt64Type>
```

Source: `src/array/dictionary_array.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) indexed by `u64`

# Example: Using `collect`
```
# use arrow_array::{Array, UInt64DictionaryArray, UInt64Array, StringArray};
# use std::sync::Arc;

let array: UInt64DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &UInt64Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) for more information and examples
