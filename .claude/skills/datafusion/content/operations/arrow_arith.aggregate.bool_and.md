# `arrow_arith::aggregate::bool_and`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.bool_and.json).

<a id="op-4789500c89f99956b87c6878"></a>
## bool_and

`function` · `arrow_arith::aggregate::bool_and` · arrow-arith 59.3.0

```rust
fn bool_and(array: &BooleanArray) -> Option<bool>
```

Source: `src/aggregate.rs:880`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns true if all non-null input values are true, otherwise false.

Returns `None` if the array is empty or only contains null values.
