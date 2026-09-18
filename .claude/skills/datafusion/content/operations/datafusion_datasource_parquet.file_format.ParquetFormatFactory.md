# `datafusion_datasource_parquet::file_format::ParquetFormatFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.file_format.ParquetFormatFactory.json).

<a id="op-df54def7a8ec224f4d63cc4b"></a>
## ParquetFormatFactory

`struct` · `datafusion_datasource_parquet::file_format::ParquetFormatFactory` · datafusion-datasource-parquet 55.1.0

```rust
struct ParquetFormatFactory
```

Source: `src/file_format.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Factory struct used to create [ParquetFormat](../operations/datafusion_datasource_parquet.file_format.ParquetFormat.md#op-730d8a0235d3096826b9a46a)

<a id="op-383921f04cba7b9226524246"></a>
## create

`function` · `datafusion_datasource_parquet::file_format::ParquetFormatFactory::create` · datafusion-datasource-parquet 55.1.0

```rust
fn create(&self, state: &dyn Session, format_options: &std::collections::HashMap<String, String>) -> Result<Arc<dyn FileFormat>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormatFactory", "path": "ParquetFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [127, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormatFactory", "path": "FileFormatFactory"}, "trait_path": "datafusion_datasource::file_format::FileFormatFactory"}`

Source: `src/file_format.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39e3dcffab3c524322f80829"></a>
## default

`function` · `datafusion_datasource_parquet::file_format::ParquetFormatFactory::default` · datafusion-datasource-parquet 55.1.0

```rust
fn default(&self) -> Arc<dyn FileFormat>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormatFactory", "path": "ParquetFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [127, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormatFactory", "path": "FileFormatFactory"}, "trait_path": "datafusion_datasource::file_format::FileFormatFactory"}`

Source: `src/file_format.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fcfc2def7c07725ab0cd063"></a>
## default

`function` · `datafusion_datasource_parquet::file_format::ParquetFormatFactory::default` · datafusion-datasource-parquet 55.1.0

```rust
fn default() -> ParquetFormatFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormatFactory", "path": "ParquetFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 10], "end": [76, 17], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbce8145de698a904ac4010c"></a>
## fmt

`function` · `datafusion_datasource_parquet::file_format::ParquetFormatFactory::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormatFactory", "path": "ParquetFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [142, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-460fb2fda66643d4e5ccceea"></a>
## get_ext

`function` · `datafusion_datasource_parquet::file_format::ParquetFormatFactory::get_ext` · datafusion-datasource-parquet 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormatFactory", "path": "ParquetFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [134, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_common::file_options::file_type::GetExt", "path": "GetExt"}, "trait_path": "datafusion_common::file_options::file_type::GetExt"}`

Source: `src/file_format.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d37eef6afea3b1f349cdf16c"></a>
## new

`function` · `datafusion_datasource_parquet::file_format::ParquetFormatFactory::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormatFactory", "path": "ParquetFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [95, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Creates an instance of [ParquetFormatFactory](../operations/datafusion_datasource_parquet.file_format.ParquetFormatFactory.md#op-df54def7a8ec224f4d63cc4b)

<a id="op-4c44693abfb506f64280251d"></a>
## new_with_options

`function` · `datafusion_datasource_parquet::file_format::ParquetFormatFactory::new_with_options` · datafusion-datasource-parquet 55.1.0

```rust
fn new_with_options(options: TableParquetOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormatFactory", "path": "ParquetFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [95, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Creates an instance of [ParquetFormatFactory](../operations/datafusion_datasource_parquet.file_format.ParquetFormatFactory.md#op-df54def7a8ec224f4d63cc4b) with customized default options

<a id="op-34663134e259f9a8233a0ab6"></a>
## options

`struct_field` · `datafusion_datasource_parquet::file_format::ParquetFormatFactory::options` · datafusion-datasource-parquet 55.1.0

```rust
options: Option<datafusion_common::config::TableParquetOptions>
```

Source: `src/file_format.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

inner options for parquet
