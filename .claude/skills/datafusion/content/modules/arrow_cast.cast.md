# `arrow_cast::cast`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.json).

<a id="op-cd782e2aa7ab80ed438c6b66"></a>
## cast

`module` · `arrow_cast::cast` · arrow-cast 59.3.0

```rust
mod cast
```

Source: `src/cast/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Cast kernels to convert [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)  between supported datatypes.

See [`cast_with_options`](../operations/arrow_cast.cast.cast_with_options.md#op-3809055ee1c85876012267ab) for more information on specific conversions.

Example:

```
# use arrow_array::*;
# use arrow_cast::cast;
# use arrow_schema::DataType;
# use std::sync::Arc;
# use arrow_array::types::Float64Type;
# use arrow_array::cast::AsArray;
// int32 to float64
let a = Int32Array::from(vec![5, 6, 7]);
let b = cast(&a, &DataType::Float64).unwrap();
let c = b.as_primitive::<Float64Type>();
assert_eq!(5.0, c.value(0));
assert_eq!(6.0, c.value(1));
assert_eq!(7.0, c.value(2));
```
