# `arrow_cast::cast::can_cast_types`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.can_cast_types.json).

<a id="op-13b2ca9fae0da4f7ebf10839"></a>
## can_cast_types

`function` · `arrow_cast::cast::can_cast_types` · arrow-cast 59.3.0

```rust
fn can_cast_types(from_type: &DataType, to_type: &DataType) -> bool
```

Source: `src/cast/mod.rs:115`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Return true if a value of type `from_type` can be cast into a value of `to_type`.

See [`cast_with_options`](../operations/arrow_cast.cast.cast_with_options.md#op-3809055ee1c85876012267ab) for more information
