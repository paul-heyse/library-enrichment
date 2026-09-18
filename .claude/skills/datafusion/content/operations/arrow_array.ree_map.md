# `arrow_array::ree_map`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.ree_map.json).

<a id="op-2ceeaf1363cc7b8af96b0579"></a>
## ree_map

`macro` · `arrow_array::ree_map` · arrow-array 59.3.0

```rust
macro_rules! ree_map
```

Source: `src/array/run_array.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Recursively applies a function to the values of a RunEndEncoded array, preserving the run structure.

# Example

```ignore
let result = ree_recurse!(array, Int32Type, my_function)?;
```

This macro is useful for implementing functions that should work on the logical values
of a REE array while preserving the run-end encoding structure.
