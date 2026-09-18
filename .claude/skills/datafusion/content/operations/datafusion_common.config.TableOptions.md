# `datafusion_common::config::TableOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.TableOptions.json).

<a id="op-0523542c8dd1656aea13ac4e"></a>
## TableOptions

`struct` · `datafusion_common::config::TableOptions` · datafusion-common 55.1.0

```rust
struct TableOptions
```

Source: `src/config.rs:2681`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents the configuration options available for handling different table formats within a data processing application.
This struct encompasses options for various file formats including CSV, Parquet, and JSON, allowing for flexible configuration
of parsing and writing behaviors specific to each format. Additionally, it supports extending functionality through custom extensions.

<a id="op-8be31098b1d07f216e1b7d4f"></a>
## alter_with_string_hash_map

`function` · `datafusion_common::config::TableOptions::alter_with_string_hash_map` · datafusion-common 55.1.0

```rust
fn alter_with_string_hash_map(&mut self, settings: &HashMap<String, String>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2934, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2887`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Modifies the current `TableOptions` instance with settings from a hash map.

# Parameters

* `settings`: A hash map where each key-value pair represents a configuration setting.

# Returns

A result indicating success or failure in applying the settings.

<a id="op-b86c49df501bf5df735535cd"></a>
## clone

`function` · `datafusion_common::config::TableOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2680, 17], "end": [2680, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:2680`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7561df5f84456a9a20598be"></a>
## combine_with_session_config

`function` · `datafusion_common::config::TableOptions::combine_with_session_config` · datafusion-common 55.1.0

```rust
fn combine_with_session_config(&self, config: &ConfigOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2934, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2793`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Updates the current `TableOptions` with settings from a given session config.

# Parameters

* `config`: A reference to the session `ConfigOptions` whose settings are to be applied.

# Returns

A new `TableOptions` instance with updated settings from the session config.

<a id="op-02605ccfb8edd95bb35b9489"></a>
## csv

`struct_field` · `datafusion_common::config::TableOptions::csv` · datafusion-common 55.1.0

```rust
csv: CsvOptions
```

Source: `src/config.rs:2684`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Configuration options for CSV file handling. This includes settings like the delimiter,
quote character, and whether the first row is considered as headers.

<a id="op-08442f7b03297e7cde2330be"></a>
## current_format

`struct_field` · `datafusion_common::config::TableOptions::current_format` · datafusion-common 55.1.0

```rust
current_format: Option<ConfigFileType>
```

Source: `src/config.rs:2695`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The current file format that the table operations should assume. This option allows
for dynamic switching between the supported file types (e.g., CSV, Parquet, JSON).

<a id="op-36cbe317718d56cc0eb18cc7"></a>
## default

`function` · `datafusion_common::config::TableOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2680, 24], "end": [2680, 31], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:2680`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c44adbccd75c5b5f23d96fb1"></a>
## default_from_session_config

`function` · `datafusion_common::config::TableOptions::default_from_session_config` · datafusion-common 55.1.0

```rust
fn default_from_session_config(config: &ConfigOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2934, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2778`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new `TableOptions` instance initialized with settings from a given session config.

# Parameters

* `config`: A reference to the session `ConfigOptions` from which to derive initial settings.

# Returns

A new `TableOptions` instance with settings applied from the session config.

<a id="op-bd678c9234f31f455cb36cd2"></a>
## entries

`function` · `datafusion_common::config::TableOptions::entries` · datafusion-common 55.1.0

```rust
fn entries(&self) -> Vec<ConfigEntry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2934, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2902`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Retrieves all configuration entries from this `TableOptions`.

# Returns

A vector of `ConfigEntry` instances, representing all the configuration options within this `TableOptions`.

<a id="op-f9a8702f43606d89a3085b63"></a>
## extensions

`struct_field` · `datafusion_common::config::TableOptions::extensions` · datafusion-common 55.1.0

```rust
extensions: Extensions
```

Source: `src/config.rs:2700`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Optional extensions that can be used to extend or customize the behavior of the table
options. Extensions can be registered using `Extensions::insert` and might include
custom file handling logic, additional configuration parameters, or other enhancements.

<a id="op-88efe90a172f9879ef46cbf5"></a>
## fmt

`function` · `datafusion_common::config::TableOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2680, 10], "end": [2680, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:2680`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e9730e62e2c5a6550014e77"></a>
## from_string_hash_map

`function` · `datafusion_common::config::TableOptions::from_string_hash_map` · datafusion-common 55.1.0

```rust
fn from_string_hash_map(settings: &HashMap<String, String>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2934, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2869`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Initializes a new `TableOptions` from a hash map of string settings.

# Parameters

* `settings`: A hash map where each key-value pair represents a configuration setting.

# Returns

A result containing the new `TableOptions` instance or an error if any setting could not be applied.

<a id="op-e135690cc9eaae2bec3c9869"></a>
## json

`struct_field` · `datafusion_common::config::TableOptions::json` · datafusion-common 55.1.0

```rust
json: JsonOptions
```

Source: `src/config.rs:2691`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Configuration options for JSON file handling.

<a id="op-e9dca593089751a282213bb0"></a>
## new

`function` · `datafusion_common::config::TableOptions::new` · datafusion-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2934, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2765`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Constructs a new instance of `TableOptions` with default settings.

# Returns

A new `TableOptions` instance with default configuration values.

<a id="op-77480c3c37889cb5020fa9f4"></a>
## parquet

`struct_field` · `datafusion_common::config::TableOptions::parquet` · datafusion-common 55.1.0

```rust
parquet: TableParquetOptions
```

Source: `src/config.rs:2688`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Configuration options for Parquet file handling. This includes settings for compression,
encoding, and other Parquet-specific file characteristics.

<a id="op-19983462ff26fdc54616c517"></a>
## set

`function` · `datafusion_common::config::TableOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2934, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2832`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets a specific configuration option.

# Parameters

* `key`: The configuration key (e.g., "format.delimiter").
* `value`: The value to set for the specified key.

# Returns

A result indicating success or failure in setting the configuration option.

<a id="op-d69433fa8eae87c4bc0f251f"></a>
## set

`function` · `datafusion_common::config::TableOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2703, 1], "end": [2757, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:2739`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets a configuration value for a specific key within `TableOptions`.

This method delegates setting configuration values to the specific file format configurations,
based on the current format selected. If no format is selected, it returns an error.

# Parameters

* `key`: The configuration key specifying which setting to adjust, prefixed with the format (e.g., "format.delimiter")
  for CSV format.
* `value`: The value to set for the specified configuration key.

# Returns

A result indicating success or an error if the key is not recognized, if a format is not specified,
or if setting the configuration value fails for the specific format.

<a id="op-9bab5a07fb51f6ceb5b4cd44"></a>
## set_config_format

`function` · `datafusion_common::config::TableOptions::set_config_format` · datafusion-common 55.1.0

```rust
fn set_config_format(&mut self, format: ConfigFileType)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2934, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2804`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets the file format for the table.

# Parameters

* `format`: The file format to use (e.g., CSV, Parquet).

<a id="op-fc1881c9531d68d77e855185"></a>
## visit

`function` · `datafusion_common::config::TableOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, _key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2703, 1], "end": [2757, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Visits configuration settings for the current file format, or all formats if none is selected.

This method adapts the behavior based on whether a file format is currently selected in `current_format`.
If a format is selected, it visits only the settings relevant to that format. Otherwise,
it visits all available format settings.

<a id="op-2c21f9b3ae2c24514dbe333e"></a>
## with_extensions

`function` · `datafusion_common::config::TableOptions::with_extensions` · datafusion-common 55.1.0

```rust
fn with_extensions(self, extensions: Extensions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2759, 1], "end": [2934, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets the extensions for this `TableOptions` instance.

# Parameters

* `extensions`: The `Extensions` instance to set.

# Returns

A new `TableOptions` instance with the specified extensions applied.
