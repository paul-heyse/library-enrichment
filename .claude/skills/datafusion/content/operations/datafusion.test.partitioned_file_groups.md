# `datafusion::test::partitioned_file_groups`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test.partitioned_file_groups.json).

<a id="op-308bc3a43cf5049fe1c3c9b8"></a>
## partitioned_file_groups

`function` · `datafusion::test::partitioned_file_groups` · datafusion 55.1.0

```rust
fn partitioned_file_groups(path: &str, filename: &str, partitions: usize, file_format: &std::sync::Arc<dyn FileFormat>, file_compression_type: datasource::file_format::file_compression_type::FileCompressionType, work_dir: &std::path::Path) -> error::Result<Vec<datafusion_datasource::file_groups::FileGroup>>
```

Source: `src/test/mod.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns file groups [`Vec<FileGroup>`] for scanning `partitions` of `filename`

Unresolved upstream links (retained, not inferred): ``Vec<FileGroup>``.
