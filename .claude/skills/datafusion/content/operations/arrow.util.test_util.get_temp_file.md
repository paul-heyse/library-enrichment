# `arrow::util::test_util::get_temp_file`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.test_util.get_temp_file.json).

<a id="op-8b92de68cccbf26f57b6bc5c"></a>
## get_temp_file

`function` · `arrow::util::test_util::get_temp_file` · arrow 59.3.0

```rust
fn get_temp_file(file_name: &str, content: &[u8]) -> fs::File
```

Source: `src/util/test_util.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Returns file handle for a temp file in 'target' directory with a provided content

TODO: Originates from `parquet` utils, can be merged in [ARROW-4064]
