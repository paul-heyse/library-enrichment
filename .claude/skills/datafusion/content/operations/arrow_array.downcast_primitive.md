# `arrow_array::downcast_primitive`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.downcast_primitive.json).

<a id="op-844ab4ae2906bafc1ae05583"></a>
## downcast_primitive

`macro` · `arrow_array::downcast_primitive` · arrow-array 59.3.0

```rust
macro_rules! downcast_primitive
```

Source: `src/cast.rs:361`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Given one or more expressions evaluating to primitive [`DataType`] invokes the provided macro
`m` with the corresponding [`ArrowPrimitiveType`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-ddd581d2aed24174207ba803), followed by any additional arguments

```
# use arrow_array::{downcast_primitive, ArrowPrimitiveType};
# use arrow_schema::DataType;

macro_rules! primitive_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn primitive_size(t: &DataType) -> u8 {
    downcast_primitive! {
        t => (primitive_size_helper, u8),
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX
    }
}

assert_eq!(primitive_size(&DataType::Int32), 4);
assert_eq!(primitive_size(&DataType::Int64), 8);
assert_eq!(primitive_size(&DataType::Float16), 2);
assert_eq!(primitive_size(&DataType::Decimal128(38, 10)), 16);
assert_eq!(primitive_size(&DataType::Decimal256(76, 20)), 32);
```

[`DataType`]: arrow_schema::DataType
