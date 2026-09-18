# `arrow_cast::cast::cast`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.cast.json).

<a id="op-b083b1d68dd0a1995d60c584"></a>
## cast

`function` · `arrow_cast::cast::cast` · arrow-cast 59.3.0

```rust
fn cast(array: &dyn Array, to_type: &DataType) -> Result<ArrayRef, ArrowError>
```

Source: `src/cast/mod.rs:354`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Cast `array` to the provided data type and return a new Array with type `to_type`, if possible.

See [`cast_with_options`](../operations/arrow_cast.cast.cast_with_options.md#op-3809055ee1c85876012267ab) for more information
