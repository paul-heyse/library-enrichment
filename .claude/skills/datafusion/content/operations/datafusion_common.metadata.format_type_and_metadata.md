# `datafusion_common::metadata::format_type_and_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.metadata.format_type_and_metadata.json).

<a id="op-5a386a5c8d3369c9c4aa66c4"></a>
## format_type_and_metadata

`function` · `datafusion_common::metadata::format_type_and_metadata` · datafusion-common 55.1.0

```rust
fn format_type_and_metadata(data_type: &arrow::datatypes::DataType, metadata: Option<&std::collections::HashMap<String, String>>) -> String
```

Source: `src/metadata.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Given a data type represented by storage and optional metadata, generate
a user-facing string

This function exists to reduce the number of Field debug strings that are
used to communicate type information in error messages and plan explain
renderings.
