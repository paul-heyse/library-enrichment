# `datafusion_common::config::CsvOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.CsvOptions.json).

<a id="op-4b9f7ea8e707b151bfcc6b63"></a>
## CsvOptions

`struct` · `datafusion_common::config::CsvOptions` · datafusion-common 55.1.0

```rust
struct CsvOptions
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options controlling CSV format

<a id="op-25969fe97b600c96c84f1bc1"></a>
## clone

`function` · `datafusion_common::config::CsvOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> CsvOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3604, 1], "end": [3664, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ea8067a6f9dd5783f4bc260"></a>
## comment

`struct_field` · `datafusion_common::config::CsvOptions::comment` · datafusion-common 55.1.0

```rust
comment: Option<u8>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f31916efad79a21504f1c81"></a>
## compression

`struct_field` · `datafusion_common::config::CsvOptions::compression` · datafusion-common 55.1.0

```rust
compression: parsers::CompressionTypeVariant
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e219acd756819f2023e80e0"></a>
## compression_level

`struct_field` · `datafusion_common::config::CsvOptions::compression_level` · datafusion-common 55.1.0

```rust
compression_level: Option<u32>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compression level for the output file. The valid range depends on the
compression algorithm:
- ZSTD: 1 to 22 (default: 3)
- GZIP: 0 to 9 (default: 6)
- BZIP2: 0 to 9 (default: 6)
- XZ: 0 to 9 (default: 6)
If not specified, the default level for the compression algorithm is used.

<a id="op-3140ac0f7e44841172e89f63"></a>
## date_format

`struct_field` · `datafusion_common::config::CsvOptions::date_format` · datafusion-common 55.1.0

```rust
date_format: Option<String>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa8cca2dc272c8b7be6f58f2"></a>
## datetime_format

`struct_field` · `datafusion_common::config::CsvOptions::datetime_format` · datafusion-common 55.1.0

```rust
datetime_format: Option<String>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0483ee1fd8d14b950596c2b"></a>
## default

`function` · `datafusion_common::config::CsvOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3604, 1], "end": [3664, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b80156b0fd851d093bce3f4"></a>
## delimiter

`function` · `datafusion_common::config::CsvOptions::delimiter` · datafusion-common 55.1.0

```rust
fn delimiter(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3797`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The delimiter character.

<a id="op-a7b201ed3eda706d515e4713"></a>
## delimiter

`struct_field` · `datafusion_common::config::CsvOptions::delimiter` · datafusion-common 55.1.0

```rust
delimiter: u8
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99c8ebc3f906888f825d467c"></a>
## double_quote

`struct_field` · `datafusion_common::config::CsvOptions::double_quote` · datafusion-common 55.1.0

```rust
double_quote: Option<bool>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b45130bb49b28addfe25c48"></a>
## eq

`function` · `datafusion_common::config::CsvOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &CsvOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3604, 1], "end": [3664, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38e60131f2e0f741326988c1"></a>
## escape

`function` · `datafusion_common::config::CsvOptions::escape` · datafusion-common 55.1.0

```rust
fn escape(&self) -> Option<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3812`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The escape character.

<a id="op-cee57cdb2b0fba2f677d832f"></a>
## escape

`struct_field` · `datafusion_common::config::CsvOptions::escape` · datafusion-common 55.1.0

```rust
escape: Option<u8>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-034457ed033b2fdc44c47f6a"></a>
## fmt

`function` · `datafusion_common::config::CsvOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3604, 1], "end": [3664, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b8981d672ffa8ee5f164f8b"></a>
## has_header

`struct_field` · `datafusion_common::config::CsvOptions::has_header` · datafusion-common 55.1.0

```rust
has_header: Option<bool>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Specifies whether there is a CSV header (i.e. the first line
consists of is column names). The value `None` indicates that
the configuration should be consulted.

<a id="op-7e04b43d88413b72ca293436"></a>
## has_header

`function` · `datafusion_common::config::CsvOptions::has_header` · datafusion-common 55.1.0

```rust
fn has_header(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3694`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if the first line is a header. If format options does not
specify whether there is a header, returns `None` (indicating that the
configuration should be consulted).

<a id="op-72f269e21ea3b851d85d93b7"></a>
## ignore_leading_whitespace

`struct_field` · `datafusion_common::config::CsvOptions::ignore_leading_whitespace` · datafusion-common 55.1.0

```rust
ignore_leading_whitespace: Option<bool>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Whether to ignore leading whitespace in string values when writing CSV.
Defaults to `false` when `None`.

<a id="op-785e941bd696cbf8b925c6b5"></a>
## ignore_trailing_whitespace

`struct_field` · `datafusion_common::config::CsvOptions::ignore_trailing_whitespace` · datafusion-common 55.1.0

```rust
ignore_trailing_whitespace: Option<bool>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Whether to ignore trailing whitespace in string values when writing CSV.
Defaults to `false` when `None`.

<a id="op-a070e9455dd4e5684bea3c1f"></a>
## newlines_in_values

`struct_field` · `datafusion_common::config::CsvOptions::newlines_in_values` · datafusion-common 55.1.0

```rust
newlines_in_values: Option<bool>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Specifies whether newlines in (quoted) values are supported.

Parsing newlines in quoted values may be affected by execution behaviour such as
parallel file scanning. Setting this to `true` ensures that newlines in values are
parsed successfully, which may reduce performance.

The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting.

<a id="op-41b1348799f3531fac626119"></a>
## null_regex

`struct_field` · `datafusion_common::config::CsvOptions::null_regex` · datafusion-common 55.1.0

```rust
null_regex: Option<String>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59fc5c2f05aea1657a9b84fd"></a>
## null_value

`struct_field` · `datafusion_common::config::CsvOptions::null_value` · datafusion-common 55.1.0

```rust
null_value: Option<String>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d774672276aafb6bd5bfc97a"></a>
## quote

`function` · `datafusion_common::config::CsvOptions::quote` · datafusion-common 55.1.0

```rust
fn quote(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3802`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The quote character.

<a id="op-d95cb49277d38646bf176908"></a>
## quote

`struct_field` · `datafusion_common::config::CsvOptions::quote` · datafusion-common 55.1.0

```rust
quote: u8
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e381cffeb3f4c0a09c564283"></a>
## quote_style

`struct_field` · `datafusion_common::config::CsvOptions::quote_style` · datafusion-common 55.1.0

```rust
quote_style: parsers::CsvQuoteStyle
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Quote style for CSV writing.
One of: "Always", "Necessary", "NonNumeric", "Never"

<a id="op-5f704bc0757de4c4453299ad"></a>
## reset

`function` · `datafusion_common::config::CsvOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3604, 1], "end": [3664, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46252666143921feb3e6352a"></a>
## schema_infer_max_rec

`struct_field` · `datafusion_common::config::CsvOptions::schema_infer_max_rec` · datafusion-common 55.1.0

```rust
schema_infer_max_rec: Option<usize>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cbb7d5b782e0898c53f54fb"></a>
## set

`function` · `datafusion_common::config::CsvOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3604, 1], "end": [3664, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-061e3a52205ece46c65fbd43"></a>
## terminator

`function` · `datafusion_common::config::CsvOptions::terminator` · datafusion-common 55.1.0

```rust
fn terminator(&self) -> Option<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3807`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The terminator character.

<a id="op-6fb89c0f5a6f6d8e2dda06b6"></a>
## terminator

`struct_field` · `datafusion_common::config::CsvOptions::terminator` · datafusion-common 55.1.0

```rust
terminator: Option<u8>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c80b8f1c719026dad542d95"></a>
## time_format

`struct_field` · `datafusion_common::config::CsvOptions::time_format` · datafusion-common 55.1.0

```rust
time_format: Option<String>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0c5d3fdaaaab0891bfc4c1a"></a>
## timestamp_format

`struct_field` · `datafusion_common::config::CsvOptions::timestamp_format` · datafusion-common 55.1.0

```rust
timestamp_format: Option<String>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5278eecd299bab2abef169d"></a>
## timestamp_tz_format

`struct_field` · `datafusion_common::config::CsvOptions::timestamp_tz_format` · datafusion-common 55.1.0

```rust
timestamp_tz_format: Option<String>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e002eb84e8733ad7bbb0ecd7"></a>
## truncated_rows

`struct_field` · `datafusion_common::config::CsvOptions::truncated_rows` · datafusion-common 55.1.0

```rust
truncated_rows: Option<bool>
```

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Whether to allow truncated rows when parsing, both within a single file and across files.

When set to false (default), reading a single CSV file which has rows of different lengths will
error; if reading multiple CSV files with different number of columns, it will also fail.

When set to true, reading a single CSV file with rows of different lengths will pad the truncated
rows with null values for the missing columns; if reading multiple CSV files with different number
of columns, it creates a union schema containing all columns found across the files, and will
pad any files missing columns with null values for their rows.

<a id="op-a55c55b1fc27883a388f4323"></a>
## visit

`function` · `datafusion_common::config::CsvOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3604, 1], "end": [3664, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3604`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f94c5ca5cee3e7e9e26da281"></a>
## with_compression

`function` · `datafusion_common::config::CsvOptions::with_compression` · datafusion-common 55.1.0

```rust
fn with_compression(self, compression_type_variant: CompressionTypeVariant) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3669`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set a limit in terms of records to scan to infer the schema
- default to `DEFAULT_SCHEMA_INFER_MAX_RECORD`

<a id="op-d045da98d0c59db32c6afe97"></a>
## with_compression_level

`function` · `datafusion_common::config::CsvOptions::with_compression_level` · datafusion-common 55.1.0

```rust
fn with_compression_level(self, level: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3791`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the compression level for the output file.
The valid range depends on the compression algorithm.
If not specified, the default level for the algorithm is used.

<a id="op-ffd19e05c58453293f2b22f4"></a>
## with_delimiter

`function` · `datafusion_common::config::CsvOptions::with_delimiter` · datafusion-common 55.1.0

```rust
fn with_delimiter(self, delimiter: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3700`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The character separating values within a row.
- default to ','

<a id="op-cd2b6f11799768c7b612f86a"></a>
## with_double_quote

`function` · `datafusion_common::config::CsvOptions::with_double_quote` · datafusion-common 55.1.0

```rust
fn with_double_quote(self, double_quote: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3728`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set true to indicate that the CSV quotes should be doubled.
- default to true

<a id="op-1b3dd875f20abcc3dc7f1e58"></a>
## with_escape

`function` · `datafusion_common::config::CsvOptions::with_escape` · datafusion-common 55.1.0

```rust
fn with_escape(self, escape: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3721`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The escape character in a row.
- default is None

<a id="op-5a573e3c1aac38c2c82ee479"></a>
## with_file_compression_type

`function` · `datafusion_common::config::CsvOptions::with_file_compression_type` · datafusion-common 55.1.0

```rust
fn with_file_compression_type(self, compression: CompressionTypeVariant) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set a `CompressionTypeVariant` of CSV
- defaults to `CompressionTypeVariant::UNCOMPRESSED`

<a id="op-1c9c8bfef0e830dce56403ee"></a>
## with_has_header

`function` · `datafusion_common::config::CsvOptions::with_has_header` · datafusion-common 55.1.0

```rust
fn with_has_header(self, has_header: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3686`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set true to indicate that the first line is a header.
- default to true

<a id="op-53794d158a5938c47c5e39c0"></a>
## with_ignore_leading_whitespace

`function` · `datafusion_common::config::CsvOptions::with_ignore_leading_whitespace` · datafusion-common 55.1.0

```rust
fn with_ignore_leading_whitespace(self, ignore_leading_whitespace: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3740`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set whether to ignore leading whitespace in string values when writing CSV.

<a id="op-10a7478c3c6f7749a6d12ec3"></a>
## with_ignore_trailing_whitespace

`function` · `datafusion_common::config::CsvOptions::with_ignore_trailing_whitespace` · datafusion-common 55.1.0

```rust
fn with_ignore_trailing_whitespace(self, ignore_trailing_whitespace: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3749`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set whether to ignore trailing whitespace in string values when writing CSV.

<a id="op-ec1d25cbc2ee43e579a0ca33"></a>
## with_newlines_in_values

`function` · `datafusion_common::config::CsvOptions::with_newlines_in_values` · datafusion-common 55.1.0

```rust
fn with_newlines_in_values(self, newlines_in_values: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3764`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Specifies whether newlines in (quoted) values are supported.

Parsing newlines in quoted values may be affected by execution behaviour such as
parallel file scanning. Setting this to `true` ensures that newlines in values are
parsed successfully, which may reduce performance.

The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting.

<a id="op-8ea7461825e003240c0856a2"></a>
## with_quote

`function` · `datafusion_common::config::CsvOptions::with_quote` · datafusion-common 55.1.0

```rust
fn with_quote(self, quote: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3707`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The quote character in a row.
- default to '"'

<a id="op-611979808ebd47cee7729571"></a>
## with_quote_style

`function` · `datafusion_common::config::CsvOptions::with_quote_style` · datafusion-common 55.1.0

```rust
fn with_quote_style(self, quote_style: CsvQuoteStyle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3734`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the quote style for CSV writing.

<a id="op-c932bb5539ba3aee687051a1"></a>
## with_schema_infer_max_rec

`function` · `datafusion_common::config::CsvOptions::with_schema_infer_max_rec` · datafusion-common 55.1.0

```rust
fn with_schema_infer_max_rec(self, max_rec: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3679`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set a limit in terms of records to scan to infer the schema
- default to `DEFAULT_SCHEMA_INFER_MAX_RECORD`

<a id="op-10d750cf0f0f23cba56d33d3"></a>
## with_terminator

`function` · `datafusion_common::config::CsvOptions::with_terminator` · datafusion-common 55.1.0

```rust
fn with_terminator(self, terminator: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3714`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The character that terminates a row.
- default to None (CRLF)

<a id="op-537b505c1f1dde71c4456e14"></a>
## with_truncated_rows

`function` · `datafusion_common::config::CsvOptions::with_truncated_rows` · datafusion-common 55.1.0

```rust
fn with_truncated_rows(self, allow: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3666, 1], "end": [3815, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3783`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Whether to allow truncated rows when parsing.
By default this is set to false and will error if the CSV rows have different lengths.
When set to true then it will allow records with less than the expected number of columns and fill the missing columns with nulls.
If the record’s schema is not nullable, then it will still return an error.
