# `datafusion_common::test_util`

Crate `datafusion-common` · 8 public items · structured records in [`model/datafusion_common.test_util.json`](../model/datafusion_common.test_util.json)

## arrow_test_data

`function` · `datafusion_common::test_util::arrow_test_data`

Also reachable as `datafusion::test_util::arrow_test_data`

```rust
fn arrow_test_data() -> String
```

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

---

## batches_to_sort_string

`function` · `datafusion_common::test_util::batches_to_sort_string`

```rust
fn batches_to_sort_string(batches: &[arrow::array::RecordBatch]) -> String
```

---

## batches_to_string

`function` · `datafusion_common::test_util::batches_to_string`

```rust
fn batches_to_string(batches: &[arrow::array::RecordBatch]) -> String
```

---

## datafusion_test_data

`function` · `datafusion_common::test_util::datafusion_test_data`

```rust
fn datafusion_test_data() -> String
```

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

---

## format_batches

`function` · `datafusion_common::test_util::format_batches`

```rust
fn format_batches(results: &[arrow::array::RecordBatch]) -> Result<impl Display, arrow::error::ArrowError>
```

---

## get_data_dir

`function` · `datafusion_common::test_util::get_data_dir`

Also reachable as `datafusion::test_util::get_data_dir`

```rust
fn get_data_dir(udf_env: &str, submodule_data: &str) -> Result<std::path::PathBuf, Box<dyn Error>>
```

Returns a directory path for finding test data.

udf_env: name of an environment variable

submodule_dir: fallback path (relative to CARGO_MANIFEST_DIR)

 Returns either:
The path referred to in `udf_env` if that variable is set and refers to a directory
The submodule_data directory relative to CARGO_MANIFEST_PATH

---

## parquet_test_data

`function` · `datafusion_common::test_util::parquet_test_data`

Also reachable as `datafusion::test_util::parquet_test_data`

```rust
fn parquet_test_data() -> String
```

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

---

## IntoArrayRef

`trait` · `datafusion_common::test_util::IntoArrayRef`

```rust
trait IntoArrayRef
```

**Implementors** (1)

- `alloc::vec::Vec`

**Methods** (1)

```rust
fn into_array_ref(self) -> ArrayRef
```

Converts a vector or array into an ArrayRef.

---
