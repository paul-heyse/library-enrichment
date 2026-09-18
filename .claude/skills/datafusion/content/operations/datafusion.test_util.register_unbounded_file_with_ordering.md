# `datafusion::test_util::register_unbounded_file_with_ordering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.register_unbounded_file_with_ordering.json).

<a id="op-0b97fb8dcb672c304f572148"></a>
## register_unbounded_file_with_ordering

`function` · `datafusion::test_util::register_unbounded_file_with_ordering` · datafusion 55.1.0

```rust
fn register_unbounded_file_with_ordering(ctx: &prelude::SessionContext, schema: arrow::datatypes::SchemaRef, file_path: &std::path::Path, table_name: &str, file_sort_order: Vec<Vec<datafusion_expr::SortExpr>>) -> error::Result<()>
```

Source: `src/test_util/mod.rs:234`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

This function creates an unbounded sorted file for testing purposes.
