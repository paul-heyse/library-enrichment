# `arrow::util::test_util::parquet_test_data`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.test_util.parquet_test_data.json).

<a id="op-0e96dc3f8abab2b89777dc76"></a>
## parquet_test_data

`function` · `arrow::util::test_util::parquet_test_data` · arrow 59.3.0

```rust
fn parquet_test_data() -> String
```

Source: `src/util/test_util.rs:100`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Returns the parquest test data directory, which is by default
stored in a git submodule rooted at
`arrow/parquest-testing/data`.

The default can be overridden by the optional environment variable
`PARQUET_TEST_DATA`

panics when the directory can not be found.

Example:
```
let testdata = arrow::util::test_util::parquet_test_data();
let filename = format!("{}/binary.parquet", testdata);
assert!(std::path::PathBuf::from(filename).exists());
```
