# `arrow_arith::aggregate::max_string_view`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.max_string_view.json).

<a id="op-25843f3cf965c671deabdef2"></a>
## max_string_view

`function` · `arrow_arith::aggregate::max_string_view` · arrow-arith 59.3.0

```rust
fn max_string_view(array: &StringViewArray) -> Option<&str>
```

Source: `src/aggregate.rs:556`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the maximum value in the string view array, according to the natural order.
