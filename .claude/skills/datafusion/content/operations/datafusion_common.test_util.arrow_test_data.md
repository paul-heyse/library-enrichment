# `datafusion_common::test_util::arrow_test_data`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.test_util.arrow_test_data.json).

<a id="op-d84f8b7b4c7512dae808c04c"></a>
## arrow_test_data

`function` · `datafusion_common::test_util::arrow_test_data` · datafusion-common 55.1.0

```rust
fn arrow_test_data() -> String
```

Source: `src/test_util.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the arrow test data directory, which is by default stored
in a git submodule rooted at `testing/data`.

The default can be overridden by the optional environment
variable `ARROW_TEST_DATA`

panics when the directory can not be found.

Example:
```
let testdata = datafusion_common::test_util::arrow_test_data();
let csvdata = format!("{}/csv/aggregate_test_100.csv", testdata);
assert!(std::path::PathBuf::from(csvdata).exists());
```
