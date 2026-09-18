# `datafusion_common::test_util::datafusion_test_data`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.test_util.datafusion_test_data.json).

<a id="op-59d82a53bcca0282f556a72e"></a>
## datafusion_test_data

`function` · `datafusion_common::test_util::datafusion_test_data` · datafusion-common 55.1.0

```rust
fn datafusion_test_data() -> String
```

Source: `src/test_util.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the datafusion test data directory, which is by default rooted at `datafusion/core/tests/data`.

The default can be overridden by the optional environment
variable `DATAFUSION_TEST_DATA`

panics when the directory can not be found.

Example:
```
let testdata = datafusion_common::test_util::datafusion_test_data();
let csvdata = format!("{}/window_1.csv", testdata);
assert!(std::path::PathBuf::from(csvdata).exists());
```
