# `arrow_array::downcast_dictionary_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.downcast_dictionary_array.json).

<a id="op-1344eb67c551c0d14914695c"></a>
## downcast_dictionary_array

`macro` · `arrow_array::downcast_dictionary_array` · arrow-array 59.3.0

```rust
macro_rules! downcast_dictionary_array
```

Source: `src/cast.rs:557`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) to a [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) based on its [`DataType`], accepts
a number of subsequent patterns to match the data type

```
# use arrow_array::{Array, StringArray, downcast_dictionary_array, cast::as_string_array, cast::as_largestring_array};
# use arrow_schema::DataType;

fn print_strings(array: &dyn Array) {
    downcast_dictionary_array!(
        array => match array.values().data_type() {
            DataType::Utf8 => {
                for v in array.downcast_dict::<StringArray>().unwrap() {
                    println!("{:?}", v);
                }
            }
            t => println!("Unsupported dictionary value type {}", t),
        },
        DataType::Utf8 => {
            for v in as_string_array(array) {
                println!("{:?}", v);
            }
        }
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => {
            for v in as_largestring_array(array) {
                println!("{:?}", v);
            }
        }
        t => println!("Unsupported datatype {}", t)
    )
}
```

[`DataType`]: arrow_schema::DataType
