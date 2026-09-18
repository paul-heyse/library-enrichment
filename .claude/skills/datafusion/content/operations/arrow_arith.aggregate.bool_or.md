# `arrow_arith::aggregate::bool_or`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.bool_or.json).

<a id="op-11ab131a90e918fcd006dc63"></a>
## bool_or

`function` · `arrow_arith::aggregate::bool_or` · arrow-arith 59.3.0

```rust
fn bool_or(array: &BooleanArray) -> Option<bool>
```

Source: `src/aggregate.rs:887`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns true if any non-null input value is true, otherwise false.

Returns `None` if the array is empty or only contains null values.
