# `arrow::util::test_util::arrow_test_data`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.test_util.arrow_test_data.json).

<a id="op-bb872ced1d6acb20151980dc"></a>
## arrow_test_data

`function` · `arrow::util::test_util::arrow_test_data` · arrow 59.3.0

```rust
fn arrow_test_data() -> String
```

Source: `src/util/test_util.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Returns the arrow test data directory, which is by default stored
in a git submodule rooted at `arrow/testing/data`.

The default can be overridden by the optional environment
variable `ARROW_TEST_DATA`

panics when the directory can not be found.

Example:
```
let testdata = arrow::util::test_util::arrow_test_data();
let csvdata = format!("{}/csv/aggregate_test_100.csv", testdata);
assert!(std::path::PathBuf::from(csvdata).exists());
```
