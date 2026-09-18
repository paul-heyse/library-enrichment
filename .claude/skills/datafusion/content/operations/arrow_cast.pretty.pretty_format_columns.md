# `arrow_cast::pretty::pretty_format_columns`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.pretty.pretty_format_columns.json).

<a id="op-310c9b04416d6d60bfac5b97"></a>
## pretty_format_columns

`function` · `arrow_cast::pretty::pretty_format_columns` · arrow-cast 59.3.0

```rust
fn pretty_format_columns(col_name: &str, results: &[arrow_array::ArrayRef]) -> Result<impl Display + use<>, arrow_schema::ArrowError>
```

Source: `src/pretty.rs:140`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Create a visual representation of [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)

Uses default values for display. See [`pretty_format_columns_with_options`](../operations/arrow_cast.pretty.pretty_format_columns_with_options.md#op-3cc9be9d957135bb411518ed)

See [`pretty_format_batches`](../operations/arrow_cast.pretty.pretty_format_batches.md#op-42add46948080a47d285119f) for an example
