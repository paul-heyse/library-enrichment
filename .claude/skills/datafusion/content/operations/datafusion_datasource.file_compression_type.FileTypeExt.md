# `datafusion_datasource::file_compression_type::FileTypeExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_compression_type.FileTypeExt.json).

<a id="op-2187b474ee08bbaa1645443c"></a>
## FileTypeExt

`trait` · `datafusion_datasource::file_compression_type::FileTypeExt` · datafusion-datasource 55.1.0

```rust
trait FileTypeExt
```

Source: `src/file_compression_type.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Trait for extending the functionality of the `FileType` enum.

<a id="op-986da3bf99b68ab29b7b73b3"></a>
## get_ext_with_compression

`function` · `datafusion_datasource::file_compression_type::FileTypeExt::get_ext_with_compression` · datafusion-datasource 55.1.0

```rust
fn get_ext_with_compression(&self, c: FileCompressionType) -> Result<String>
```

Source: `src/file_compression_type.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Given a `FileCompressionType`, return the `FileType`'s extension with compression suffix
