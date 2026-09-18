# `datafusion_physical_expr_adapter::rewrite`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.rewrite.json).

<a id="op-5e7da2e99d8bfb11d221f208"></a>
## rewrite

`module` · `datafusion_physical_expr_adapter::rewrite` · datafusion-physical-expr-adapter 55.1.0

```rust
mod rewrite
```

Source: `src/rewrite.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Rewrite expressions in preparation for files being scanned, such as scan-metadata scalar UDFs.

Functions like [`file_row_index()`] and [`input_file_name()`] are placeholders
whose value is only known during a file scan. The helpers here replace those
UDFs with ordinary physical expressions bound to the current file: a column
reference into a source-provided row-index column, or a per-file literal, etc.

[`file_row_index()`]: datafusion_functions::core::file_row_index::FileRowIndexFunc
[`input_file_name()`]: datafusion_functions::core::input_file_name::InputFileNameFunc
