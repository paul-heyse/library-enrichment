# `arrow_cast::pretty::print_batches`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.pretty.print_batches.json).

<a id="op-edcf36a6ae46a1633b8cbeba"></a>
## print_batches

`function` · `arrow_cast::pretty::print_batches` · arrow-cast 59.3.0

```rust
fn print_batches(results: &[arrow_array::RecordBatch]) -> Result<(), arrow_schema::ArrowError>
```

Source: `src/pretty.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Prints a visual representation of record batches to stdout
