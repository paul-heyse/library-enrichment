# `arrow_array::array::dictionary_array::Int8DictionaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.dictionary_array.Int8DictionaryArray.json).

<a id="op-fa5f80416e2470a5737eb6eb"></a>
## Int8DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::Int8DictionaryArray` · arrow-array 59.3.0

```rust
type Int8DictionaryArray = DictionaryArray<Int8Type>
```

Source: `src/array/dictionary_array.rs:48`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) indexed by `i8`

# Example: Using `collect`
```
# use arrow_array::{Array, Int8DictionaryArray, Int8Array, StringArray};
# use std::sync::Arc;

let array: Int8DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &Int8Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) for more information and examples
