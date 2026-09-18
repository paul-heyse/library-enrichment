# `datafusion_common::test_util::get_data_dir`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.test_util.get_data_dir.json).

<a id="op-b921a9071db271aac4dd19be"></a>
## get_data_dir

`function` · `datafusion_common::test_util::get_data_dir` · datafusion-common 55.1.0

```rust
fn get_data_dir(udf_env: &str, submodule_data: &str) -> Result<std::path::PathBuf, Box<dyn Error>>
```

Source: `src/test_util.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a directory path for finding test data.

udf_env: name of an environment variable

submodule_dir: fallback path (relative to CARGO_MANIFEST_DIR)

 Returns either:
The path referred to in `udf_env` if that variable is set and refers to a directory
The submodule_data directory relative to CARGO_MANIFEST_PATH
