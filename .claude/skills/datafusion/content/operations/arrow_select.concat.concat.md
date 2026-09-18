# `arrow_select::concat::concat`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.concat.concat.json).

<a id="op-c51f8d209df4c3e48cba7bce"></a>
## concat

`function` · `arrow_select::concat::concat` · arrow-select 59.3.0

```rust
fn concat(arrays: &[&dyn Array]) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/concat.rs:501`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Concatenate multiple [Array](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) of the same type into a single [ArrayRef](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1).
