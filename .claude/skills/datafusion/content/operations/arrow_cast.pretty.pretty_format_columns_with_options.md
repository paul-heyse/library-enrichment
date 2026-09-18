# `arrow_cast::pretty::pretty_format_columns_with_options`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.pretty.pretty_format_columns_with_options.json).

<a id="op-3cc9be9d957135bb411518ed"></a>
## pretty_format_columns_with_options

`function` · `arrow_cast::pretty::pretty_format_columns_with_options` · arrow-cast 59.3.0

```rust
fn pretty_format_columns_with_options(col_name: &str, results: &[arrow_array::ArrayRef], options: &display::FormatOptions<'_>) -> Result<impl Display + use<>, arrow_schema::ArrowError>
```

Source: `src/pretty.rs:151`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Create a visual representation of [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) with formatting options.

See [`pretty_format_batches_with_options`](../operations/arrow_cast.pretty.pretty_format_batches_with_options.md#op-85221173646a195629d41185) for an example
