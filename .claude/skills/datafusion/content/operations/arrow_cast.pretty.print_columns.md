# `arrow_cast::pretty::print_columns`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.pretty.print_columns.json).

<a id="op-ecb1aef3d727e0100d69cacf"></a>
## print_columns

`function` · `arrow_cast::pretty::print_columns` · arrow-cast 59.3.0

```rust
fn print_columns(col_name: &str, results: &[arrow_array::ArrayRef]) -> Result<(), arrow_schema::ArrowError>
```

Source: `src/pretty.rs:166`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Prints a visual representation of a list of column to stdout
