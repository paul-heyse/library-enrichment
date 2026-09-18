# `datafusion_expr::utils::check_all_columns_from_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.check_all_columns_from_schema.json).

<a id="op-979306e566127da49251a939"></a>
## check_all_columns_from_schema

`function` · `datafusion_expr::utils::check_all_columns_from_schema` · datafusion-expr 55.1.0

```rust
fn check_all_columns_from_schema(columns: &std::collections::HashSet<&datafusion_common::Column>, schema: &datafusion_common::DFSchema) -> datafusion_common::Result<bool>
```

Source: `src/utils.rs:1013`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Check whether all columns are from the schema.
