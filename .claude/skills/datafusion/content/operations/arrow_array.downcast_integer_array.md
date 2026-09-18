# `arrow_array::downcast_integer_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.downcast_integer_array.json).

<a id="op-2a063ce5d9feef4b5cb41ff0"></a>
## downcast_integer_array

`macro` · `arrow_array::downcast_integer_array` · arrow-array 59.3.0

```rust
macro_rules! downcast_integer_array
```

Source: `src/cast.rs:140`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Given one or more expressions evaluating to an integer [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) invokes the provided macro
with the corresponding array, along with match statements for any non integer array types

```
# use arrow_array::{Array, downcast_integer_array, cast::as_string_array, cast::as_largestring_array};
# use arrow_schema::DataType;

fn print_integer(array: &dyn Array) {
    downcast_integer_array!(
        array => {
            for v in array {
                println!("{:?}", v);
            }
        }
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
