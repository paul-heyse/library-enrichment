# `arrow_array::array::primitive_array::Decimal256Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Decimal256Array.json).

<a id="op-1a3e2bb3c3e8ce5062b9f560"></a>
## Decimal256Array

`type_alias` · `arrow_array::array::primitive_array::Decimal256Array` · arrow-array 59.3.0

```rust
type Decimal256Array = PrimitiveArray<Decimal256Type>
```

Source: `src/array/primitive_array.rs:488`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of 256-bit fixed point decimals

# Examples

Construction

```
# use arrow_array::Decimal256Array;
use arrow_buffer::i256;
// Create from Vec<Option<i256>>
let arr = Decimal256Array::from(vec![Some(i256::from(1)), None, Some(i256::from(2))]);
// Create from Vec<i256>
let arr = Decimal256Array::from(vec![i256::from(1), i256::from(2), i256::from(3)]);
// Create iter/collect
let arr: Decimal256Array = std::iter::repeat(i256::from(42)).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
