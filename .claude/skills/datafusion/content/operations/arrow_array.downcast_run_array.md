# `arrow_array::downcast_run_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.downcast_run_array.json).

<a id="op-10ed1ea12aa88c78f0997859"></a>
## downcast_run_array

`macro` · `arrow_array::downcast_run_array` · arrow-array 59.3.0

```rust
macro_rules! downcast_run_array
```

Source: `src/cast.rs:665`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) to a [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) based on its [`DataType`], accepts
a number of subsequent patterns to match the data type

```
# use arrow_array::{Array, StringArray, downcast_run_array, cast::as_string_array, cast::as_largestring_array};
# use arrow_schema::DataType;

fn print_strings(array: &dyn Array) {
    downcast_run_array!(
        array => match array.values().data_type() {
            DataType::Utf8 => {
                for v in array.downcast::<StringArray>().unwrap() {
                    println!("{:?}", v);
                }
            }
            t => println!("Unsupported run array value type {}", t),
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
