# `datafusion_expr::utils::can_hash`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.can_hash.json).

<a id="op-6a0d16a7966a249e0c881d5a"></a>
## can_hash

`function` · `datafusion_expr::utils::can_hash` · datafusion-expr 55.1.0

```rust
fn can_hash(data_type: &arrow::datatypes::DataType) -> bool
```

Source: `src/utils.rs:963`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Can this data type be used in hash join equal conditions??
Data types here come from function 'equal_rows', if more data types are supported
in create_hashes, add those data types here to generate join logical plan.
