# `arrow_array::downcast_integer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.downcast_integer.json).

<a id="op-c0659878d97e47d99bdd0f38"></a>
## downcast_integer

`macro` · `arrow_array::downcast_integer` · arrow-array 59.3.0

```rust
macro_rules! downcast_integer
```

Source: `src/cast.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Given one or more expressions evaluating to an integer [`DataType`] invokes the provided macro
`m` with the corresponding integer [`ArrowPrimitiveType`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-ddd581d2aed24174207ba803), followed by any additional arguments

```
# use arrow_array::{downcast_primitive, ArrowPrimitiveType, downcast_integer};
# use arrow_schema::DataType;

macro_rules! dictionary_key_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn dictionary_key_size(t: &DataType) -> u8 {
    match t {
        DataType::Dictionary(k, _) => downcast_integer! {
            k.as_ref() => (dictionary_key_size_helper, u8),
            _ => unreachable!(),
        },
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX,
    }
}

assert_eq!(dictionary_key_size(&DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8))), 4);
assert_eq!(dictionary_key_size(&DataType::Dictionary(Box::new(DataType::Int64), Box::new(DataType::Utf8))), 8);
assert_eq!(dictionary_key_size(&DataType::Dictionary(Box::new(DataType::UInt16), Box::new(DataType::Utf8))), 2);
```

[`DataType`]: arrow_schema::DataType
