# `arrow_array::array::dictionary_array::Int16DictionaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.dictionary_array.Int16DictionaryArray.json).

<a id="op-744ea40ed17c570222426db9"></a>
## Int16DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::Int16DictionaryArray` · arrow-array 59.3.0

```rust
type Int16DictionaryArray = DictionaryArray<Int16Type>
```

Source: `src/array/dictionary_array.rs:64`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) indexed by `i16`

# Example: Using `collect`
```
# use arrow_array::{Array, Int16DictionaryArray, Int16Array, StringArray};
# use std::sync::Arc;

let array: Int16DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &Int16Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) for more information and examples
