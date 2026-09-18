# `arrow_array::downcast_primitive_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.downcast_primitive_array.json).

<a id="op-708abd9f6da5079cc119217c"></a>
## downcast_primitive_array

`macro` · `arrow_array::downcast_primitive_array` · arrow-array 59.3.0

```rust
macro_rules! downcast_primitive_array
```

Source: `src/cast.rs:458`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcast an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) to a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) based on its [`DataType`]
accepts a number of subsequent patterns to match the data type

```
# use arrow_array::{Array, downcast_primitive_array, cast::as_string_array, cast::as_largestring_array};
# use arrow_schema::DataType;

fn print_primitive(array: &dyn Array) {
    downcast_primitive_array!(
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
