# `arrow_arith::aggregate::max_fixed_size_binary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.max_fixed_size_binary.json).

<a id="op-fa6e63fa03830499b45e6089"></a>
## max_fixed_size_binary

`function` · `arrow_arith::aggregate::max_fixed_size_binary` · arrow-arith 59.3.0

```rust
fn max_fixed_size_binary(array: &FixedSizeBinaryArray) -> Option<&[u8]>
```

Source: `src/aggregate.rs:531`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the maximum value in the fixed size binary array, according to the natural order.
