# `datafusion_datasource::file_format::FileFormatFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_format.FileFormatFactory.json).

<a id="op-f967009d6b2c0c52e0070973"></a>
## FileFormatFactory

`trait` · `datafusion_datasource::file_format::FileFormatFactory` · datafusion-datasource 55.1.0

```rust
trait FileFormatFactory: Any + Sync + Send + GetExt + fmt::Debug
```

Source: `src/file_format.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Factory for creating [`FileFormat`](../operations/datafusion_datasource.file_format.FileFormat.md#op-81da8109cfb5e6919dc4a7ee) instances based on session and command level options

Users can provide their own `FileFormatFactory` to support arbitrary file formats

<a id="op-a1bf5de4506a169227409e15"></a>
## create

`function` · `datafusion_datasource::file_format::FileFormatFactory::create` · datafusion-datasource 55.1.0

```rust
fn create(&self, state: &dyn Session, format_options: &HashMap<String, String>) -> Result<Arc<dyn FileFormat>>
```

Source: `src/file_format.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Initialize a [FileFormat](../operations/datafusion_datasource.file_format.FileFormat.md#op-81da8109cfb5e6919dc4a7ee) and configure based on session and command level options

<a id="op-3b150459f9fbfc3d072b6a3c"></a>
## default

`function` · `datafusion_datasource::file_format::FileFormatFactory::default` · datafusion-datasource 55.1.0

```rust
fn default(&self) -> Arc<dyn FileFormat>
```

Source: `src/file_format.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Initialize a [FileFormat](../operations/datafusion_datasource.file_format.FileFormat.md#op-81da8109cfb5e6919dc4a7ee) with all options set to default values
