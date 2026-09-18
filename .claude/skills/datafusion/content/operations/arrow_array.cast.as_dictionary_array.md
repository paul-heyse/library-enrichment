# `arrow_array::cast::as_dictionary_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_dictionary_array.json).

<a id="op-e88bf129ae40a85ac7b20bd8"></a>
## as_dictionary_array

`function` · `arrow_array::cast::as_dictionary_array` · arrow-array 59.3.0

```rust
fn as_dictionary_array<T>(arr: &dyn Array) -> &DictionaryArray<T> where T: ArrowDictionaryKeyType
```

Source: `src/cast.rs:589`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to
[`DictionaryArray<T>`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47), panic'ing on failure.

# Example

```
# use arrow_array::{ArrayRef, DictionaryArray};
# use arrow_array::cast::as_dictionary_array;
# use arrow_array::types::Int32Type;

let arr: DictionaryArray<Int32Type> = vec![Some("foo")].into_iter().collect();
let arr: ArrayRef = std::sync::Arc::new(arr);
let dict_array: &DictionaryArray<Int32Type> = as_dictionary_array::<Int32Type>(&arr);
```
