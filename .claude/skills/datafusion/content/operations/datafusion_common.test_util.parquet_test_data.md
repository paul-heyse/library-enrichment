# `datafusion_common::test_util::parquet_test_data`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.test_util.parquet_test_data.json).

<a id="op-e5c4d0b4b68dba18f57821c6"></a>
## parquet_test_data

`function` · `datafusion_common::test_util::parquet_test_data` · datafusion-common 55.1.0

```rust
fn parquet_test_data() -> String
```

Source: `src/test_util.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the parquet test data directory, which is by default
stored in a git submodule rooted at
`parquet-testing/data`.

The default can be overridden by the optional environment variable
`PARQUET_TEST_DATA`

panics when the directory can not be found.

Example:
```
let testdata = datafusion_common::test_util::parquet_test_data();
let filename = format!("{}/binary.parquet", testdata);
assert!(std::path::PathBuf::from(filename).exists());
```
