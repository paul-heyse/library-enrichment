# `datafusion_common::metadata::check_metadata_with_storage_equal`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.metadata.check_metadata_with_storage_equal.json).

<a id="op-2e533ed72e60567a59b7742f"></a>
## check_metadata_with_storage_equal

`function` · `datafusion_common::metadata::check_metadata_with_storage_equal` · datafusion-common 55.1.0

```rust
fn check_metadata_with_storage_equal(actual: (&arrow::datatypes::DataType, Option<&std::collections::HashMap<String, String>>), expected: (&arrow::datatypes::DataType, Option<&std::collections::HashMap<String, String>>), what: &str, context: &str) -> Result<(), DataFusionError>
```

Source: `src/metadata.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Assert equality of data types where one or both sides may have field metadata

This currently compares absent metadata (e.g., one side was a DataType) and
empty metadata (e.g., one side was a field where the field had no metadata)
as equal and uses byte-for-byte comparison for the keys and values of the
fields, even though this is potentially too strict for some cases (e.g.,
extension types where extension metadata is represented by JSON, or cases
where field metadata is orthogonal to the interpretation of the data type).

Returns a planning error with suitably formatted type representations if
actual and expected do not compare to equal.
