# `arrow_array::downcast_temporal`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.downcast_temporal.json).

<a id="op-dbcbc3d8f2696ff0713348ac"></a>
## downcast_temporal

`macro` · `arrow_array::downcast_temporal` · arrow-array 59.3.0

```rust
macro_rules! downcast_temporal
```

Source: `src/cast.rs:239`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Given one or more expressions evaluating to primitive [`DataType`] invokes the provided macro
`m` with the corresponding [`ArrowPrimitiveType`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-ddd581d2aed24174207ba803), followed by any additional arguments

```
# use arrow_array::{downcast_temporal, ArrowPrimitiveType};
# use arrow_schema::DataType;

macro_rules! temporal_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn temporal_size(t: &DataType) -> u8 {
    downcast_temporal! {
        t => (temporal_size_helper, u8),
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX
    }
}

assert_eq!(temporal_size(&DataType::Date32), 4);
assert_eq!(temporal_size(&DataType::Date64), 8);
```

[`DataType`]: arrow_schema::DataType
