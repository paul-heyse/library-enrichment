# `arrow_array::array::dictionary_array::Int32DictionaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.dictionary_array.Int32DictionaryArray.json).

<a id="op-4e48e87e565db2ceff83c1bc"></a>
## Int32DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::Int32DictionaryArray` · arrow-array 59.3.0

```rust
type Int32DictionaryArray = DictionaryArray<Int32Type>
```

Source: `src/array/dictionary_array.rs:80`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) indexed by `i32`

# Example: Using `collect`
```
# use arrow_array::{Array, Int32DictionaryArray, Int32Array, StringArray};
# use std::sync::Arc;

let array: Int32DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &Int32Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) for more information and examples
