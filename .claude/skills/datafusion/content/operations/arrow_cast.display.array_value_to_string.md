# `arrow_cast::display::array_value_to_string`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.display.array_value_to_string.json).

<a id="op-06d3429777f6f2a12a86ca20"></a>
## array_value_to_string

`function` · `arrow_cast::display::array_value_to_string` · arrow-cast 59.3.0

```rust
fn array_value_to_string(column: &dyn Array, row: usize) -> Result<String, ArrowError>
```

Source: `src/display.rs:1385`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Get the value at the given row in an array as a String.

Note this function is quite inefficient and is unlikely to be
suitable for converting large arrays or record batches.

Please see [`ArrayFormatter`](../operations/arrow_cast.display.ArrayFormatter.md#op-c2a994be3b7cca3db9a55992) for a more performant interface
