# `arrow_arith::aggregate::min_fixed_size_binary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.min_fixed_size_binary.json).

<a id="op-775335aec4aff08b7e793052"></a>
## min_fixed_size_binary

`function` · `arrow_arith::aggregate::min_fixed_size_binary` · arrow-arith 59.3.0

```rust
fn min_fixed_size_binary(array: &FixedSizeBinaryArray) -> Option<&[u8]>
```

Source: `src/aggregate.rs:546`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the minimum value in the fixed size binary array, according to the natural order.
