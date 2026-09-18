# `arrow_array::array::primitive_array::Float16Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Float16Array.json).

<a id="op-0e3c389d1a55c67f964933fc"></a>
## Float16Array

`type_alias` · `arrow_array::array::primitive_array::Float16Array` · arrow-array 59.3.0

```rust
type Float16Array = PrimitiveArray<Float16Type>
```

Source: `src/array/primitive_array.rs:214`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `f16`

# Examples

Construction

```
# use arrow_array::Float16Array;
use half::f16;
// Create from Vec<Option<f16>>
let arr = Float16Array::from(vec![Some(f16::from_f64(1.0)), Some(f16::from_f64(2.0))]);
// Create from Vec<i8>
let arr = Float16Array::from(vec![f16::from_f64(1.0), f16::from_f64(2.0), f16::from_f64(3.0)]);
// Create iter/collect
let arr: Float16Array = std::iter::repeat(f16::from_f64(1.0)).take(10).collect();
```

# Example: Using `collect`
```
# use arrow_array::Float16Array;
use half::f16;
let arr : Float16Array = [Some(f16::from_f64(1.0)), Some(f16::from_f64(2.0))].into_iter().collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
