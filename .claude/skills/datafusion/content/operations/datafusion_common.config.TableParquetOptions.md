# `datafusion_common::config::TableParquetOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.TableParquetOptions.json).

<a id="op-87f53f5b4720a7a3f1341024"></a>
## TableParquetOptions

`struct` · `datafusion_common::config::TableParquetOptions` · datafusion-common 55.1.0

```rust
struct TableParquetOptions
```

Source: `src/config.rs:2943`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options that control how Parquet files are read, including global options
that apply to all columns and optional column-specific overrides

Closely tied to `ParquetWriterOptions` (see `crate::file_options::parquet_writer::ParquetWriterOptions` when the "parquet" feature is enabled).
Properties not included in [`TableParquetOptions`](../operations/datafusion_common.config.TableParquetOptions.md#op-87f53f5b4720a7a3f1341024) may not be configurable at the external API
(e.g. sorting_columns).

<a id="op-6bd0659dd10dc2bfa315e02c"></a>
## arrow_schema

`function` · `datafusion_common::config::TableParquetOptions::arrow_schema` · datafusion-common 55.1.0

```rust
fn arrow_schema(&mut self, schema: &Arc<Schema>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "crate::config::TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [70, 2], "filename": "src/file_options/parquet_writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_options/parquet_writer.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Add the arrow schema to the parquet kv_metadata.
If already exists, then overwrites.

<a id="op-48520ffbca2da66a0c302086"></a>
## clone

`function` · `datafusion_common::config::TableParquetOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> TableParquetOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2942, 10], "end": [2942, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:2942`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-442549e008e22a9b275c9531"></a>
## column_specific_options

`struct_field` · `datafusion_common::config::TableParquetOptions::column_specific_options` · datafusion-common 55.1.0

```rust
column_specific_options: std::collections::HashMap<String, ParquetColumnOptions>
```

Source: `src/config.rs:2947`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Column specific options. Default usage is parquet.XX::column.

<a id="op-04ee2817efd8dc516173ad28"></a>
## crypto

`struct_field` · `datafusion_common::config::TableParquetOptions::crypto` · datafusion-common 55.1.0

```rust
crypto: ParquetEncryptionOptions
```

Source: `src/config.rs:2982`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options for configuring Parquet modular encryption

To use Parquet encryption, you must enable the `parquet_encryption` feature flag, as it is not activated by default.
See ConfigFileEncryptionProperties and ConfigFileDecryptionProperties in datafusion/common/src/config.rs
These can be set via 'format.crypto', for example:
```sql
OPTIONS (
   'format.crypto.file_encryption.encrypt_footer' 'true',
   'format.crypto.file_encryption.footer_key_as_hex' '30313233343536373839303132333435',  -- b"0123456789012345" */
   'format.crypto.file_encryption.column_key_as_hex::double_field' '31323334353637383930313233343530', -- b"1234567890123450"
   'format.crypto.file_encryption.column_key_as_hex::float_field' '31323334353637383930313233343531', -- b"1234567890123451"
    -- Same for decryption
   'format.crypto.file_decryption.footer_key_as_hex' '30313233343536373839303132333435', -- b"0123456789012345"
   'format.crypto.file_decryption.column_key_as_hex::double_field' '31323334353637383930313233343530', -- b"1234567890123450"
   'format.crypto.file_decryption.column_key_as_hex::float_field' '31323334353637383930313233343531', -- b"1234567890123451"
)
```
See datafusion-cli/tests/sql/encrypted_parquet.sql for a more complete example.
Note that keys must be provided as in hex format since these are binary strings.

<a id="op-75034e220ca6fddc4449211d"></a>
## default

`function` · `datafusion_common::config::TableParquetOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> TableParquetOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2942, 17], "end": [2942, 24], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:2942`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f85765821cf9514fd4f3a23"></a>
## entries

`function` · `datafusion_common::config::TableParquetOptions::entries` · datafusion-common 55.1.0

```rust
fn entries(&TableParquetOptions) -> Vec<ConfigEntry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2985, 1], "end": [3041, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3010`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Retrieves all configuration entries from this `TableParquetOptions`.

# Returns

A vector of `ConfigEntry` instances, representing all the configuration options within this

<a id="op-64c4c4c8890fd80dd4f584ae"></a>
## eq

`function` · `datafusion_common::config::TableParquetOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &TableParquetOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2942, 33], "end": [2942, 42], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:2942`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a594ff083a8ad2cf6c2c8053"></a>
## fmt

`function` · `datafusion_common::config::TableParquetOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2942, 26], "end": [2942, 31], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:2942`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-487d24df616f96aad183a916"></a>
## global

`struct_field` · `datafusion_common::config::TableParquetOptions::global` · datafusion-common 55.1.0

```rust
global: ParquetOptions
```

Source: `src/config.rs:2945`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Global Parquet options that propagates to all columns.

<a id="op-e81de69372a57b871269aa13"></a>
## key_value_metadata

`struct_field` · `datafusion_common::config::TableParquetOptions::key_value_metadata` · datafusion-common 55.1.0

```rust
key_value_metadata: std::collections::HashMap<String, Option<String>>
```

Source: `src/config.rs:2962`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Additional file-level metadata to include. Inserted into the key_value_metadata
for the written [`FileMetaData`](https://docs.rs/parquet/latest/parquet/file/metadata/struct.FileMetaData.html).

Multiple entries are permitted
```sql
OPTIONS (
   'format.metadata::key1' '',
   'format.metadata::key2' 'value',
   'format.metadata::key3' 'value has spaces',
   'format.metadata::key4' 'value has special chars :: :',
   'format.metadata::key_dupe' 'original will be overwritten',
   'format.metadata::key_dupe' 'final'
)
```

<a id="op-2ab075a604794c1c8a8514dd"></a>
## new

`function` · `datafusion_common::config::TableParquetOptions::new` · datafusion-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2985, 1], "end": [3041, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2987`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return new default TableParquetOptions

<a id="op-b430a15c1b510ec388556ee9"></a>
## set

`function` · `datafusion_common::config::TableParquetOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3043, 1], "end": [3078, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3052`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ac79914e3427659b82c3e7f"></a>
## visit

`function` · `datafusion_common::config::TableParquetOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3043, 1], "end": [3078, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3044`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f6c45c30d187abca8cffb03"></a>
## with_skip_arrow_metadata

`function` · `datafusion_common::config::TableParquetOptions::with_skip_arrow_metadata` · datafusion-common 55.1.0

```rust
fn with_skip_arrow_metadata(self, skip: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableParquetOptions", "path": "TableParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2985, 1], "end": [3041, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2995`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set whether the encoding of the arrow metadata should occur
during the writing of parquet.

Default is to encode the arrow schema in the file kv_metadata.
