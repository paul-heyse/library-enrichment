# `arrow_array::downcast_run_end_index`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.downcast_run_end_index.json).

<a id="op-3c88c69ba71a2433e215404e"></a>
## downcast_run_end_index

`macro` · `arrow_array::downcast_run_end_index` · arrow-array 59.3.0

```rust
macro_rules! downcast_run_end_index
```

Source: `src/cast.rs:194`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Given one or more expressions evaluating to an integer [`DataType`] invokes the provided macro
`m` with the corresponding integer [`RunEndIndexType`](../operations/arrow_array.types.RunEndIndexType.md#op-b20dc51c212ffcdcff0708b7), followed by any additional arguments

```
# use std::sync::Arc;
# use arrow_array::{downcast_primitive, ArrowPrimitiveType, downcast_run_end_index};
# use arrow_schema::{DataType, Field};

macro_rules! run_end_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn run_end_index_size(t: &DataType) -> u8 {
    match t {
        DataType::RunEndEncoded(k, _) => downcast_run_end_index! {
            k.data_type() => (run_end_size_helper, u8),
            _ => unreachable!(),
        },
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX,
    }
}

assert_eq!(run_end_index_size(&DataType::RunEndEncoded(Arc::new(Field::new("a", DataType::Int32, false)), Arc::new(Field::new("b", DataType::Utf8, true)))), 4);
assert_eq!(run_end_index_size(&DataType::RunEndEncoded(Arc::new(Field::new("a", DataType::Int64, false)), Arc::new(Field::new("b", DataType::Utf8, true)))), 8);
assert_eq!(run_end_index_size(&DataType::RunEndEncoded(Arc::new(Field::new("a", DataType::Int16, false)), Arc::new(Field::new("b", DataType::Utf8, true)))), 2);
```

[`DataType`]: arrow_schema::DataType
