# `datafusion_common::file_options::parquet_writer::parse_compression_string`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.file_options.parquet_writer.parse_compression_string.json).

<a id="op-615f8c70eba00b83d92cb9d3"></a>
## parse_compression_string

`function` · `datafusion_common::file_options::parquet_writer::parse_compression_string` · datafusion-common 55.1.0

```rust
fn parse_compression_string(str_setting: &str) -> Result<parquet::basic::Compression>
```

Source: `src/file_options/parquet_writer.rs:365`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Parses datafusion.execution.parquet.compression String to a parquet::basic::Compression
