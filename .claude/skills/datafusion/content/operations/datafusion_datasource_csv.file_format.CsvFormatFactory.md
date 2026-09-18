# `datafusion_datasource_csv::file_format::CsvFormatFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_csv.file_format.CsvFormatFactory.json).

<a id="op-f7504536b0426564f06b43e4"></a>
## CsvFormatFactory

`struct` · `datafusion_datasource_csv::file_format::CsvFormatFactory` · datafusion-datasource-csv 55.1.0

```rust
struct CsvFormatFactory
```

Source: `src/file_format.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Factory used to create [`CsvFormat`](../operations/datafusion_datasource_csv.file_format.CsvFormat.md#op-be96ebde8567146a8ce2670f)

<a id="op-f0978bf4c9a6cb564901c2cf"></a>
## create

`function` · `datafusion_datasource_csv::file_format::CsvFormatFactory::create` · datafusion-datasource-csv 55.1.0

```rust
fn create(&self, state: &dyn Session, format_options: &HashMap<String, String>) -> Result<Arc<dyn FileFormat>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormatFactory", "path": "CsvFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [124, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormatFactory", "path": "FileFormatFactory"}, "trait_path": "datafusion_datasource::file_format::FileFormatFactory"}`

Source: `src/file_format.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cb88233467b9ac3c198cb57"></a>
## default

`function` · `datafusion_datasource_csv::file_format::CsvFormatFactory::default` · datafusion-datasource-csv 55.1.0

```rust
fn default(&self) -> Arc<dyn FileFormat>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormatFactory", "path": "CsvFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [124, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormatFactory", "path": "FileFormatFactory"}, "trait_path": "datafusion_datasource::file_format::FileFormatFactory"}`

Source: `src/file_format.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d4c0e6738b506bd0b3dd425"></a>
## default

`function` · `datafusion_datasource_csv::file_format::CsvFormatFactory::default` · datafusion-datasource-csv 55.1.0

```rust
fn default() -> CsvFormatFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormatFactory", "path": "CsvFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 17], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d50920e8977ef37a3421d330"></a>
## fmt

`function` · `datafusion_datasource_csv::file_format::CsvFormatFactory::fmt` · datafusion-datasource-csv 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormatFactory", "path": "CsvFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [94, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d694ab0a390ffb128d3b49cb"></a>
## get_ext

`function` · `datafusion_datasource_csv::file_format::CsvFormatFactory::get_ext` · datafusion-datasource-csv 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormatFactory", "path": "CsvFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [131, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_common::file_options::file_type::GetExt", "path": "GetExt"}, "trait_path": "datafusion_common::file_options::file_type::GetExt"}`

Source: `src/file_format.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ab0757b88deedb71f33a4c7"></a>
## new

`function` · `datafusion_datasource_csv::file_format::CsvFormatFactory::new` · datafusion-datasource-csv 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormatFactory", "path": "CsvFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [86, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Creates an instance of [`CsvFormatFactory`](../operations/datafusion_datasource_csv.file_format.CsvFormatFactory.md#op-f7504536b0426564f06b43e4)

<a id="op-86b179b0103fc8fb258ba38b"></a>
## new_with_options

`function` · `datafusion_datasource_csv::file_format::CsvFormatFactory::new_with_options` · datafusion-datasource-csv 55.1.0

```rust
fn new_with_options(options: CsvOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvFormatFactory", "path": "CsvFormatFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [86, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Creates an instance of [`CsvFormatFactory`](../operations/datafusion_datasource_csv.file_format.CsvFormatFactory.md#op-f7504536b0426564f06b43e4) with customized default options

<a id="op-ea423e08363abce8963c35d4"></a>
## options

`struct_field` · `datafusion_datasource_csv::file_format::CsvFormatFactory::options` · datafusion-datasource-csv 55.1.0

```rust
options: Option<datafusion_common::config::CsvOptions>
```

Source: `src/file_format.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

the options for csv file read
