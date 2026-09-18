# `datafusion_common::config::ConfigOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ConfigOptions.json).

<a id="op-0fede8afa640e38e337e0cc4"></a>
## ConfigOptions

`struct` · `datafusion_common::config::ConfigOptions` · datafusion-common 55.1.0

```rust
struct ConfigOptions
```

Source: `src/config.rs:1915`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Configuration options struct, able to store both built-in configuration and custom options

<a id="op-591155b026adfd2cdf1b38a5"></a>
## catalog

`struct_field` · `datafusion_common::config::ConfigOptions::catalog` · datafusion-common 55.1.0

```rust
catalog: CatalogOptions
```

Source: `src/config.rs:1917`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Catalog options

<a id="op-53f136bab070e30c749e7bd3"></a>
## clone

`function` · `datafusion_common::config::ConfigOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ConfigOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1913, 17], "end": [1913, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:1913`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2070b4ef16fe1015cd9b3d1"></a>
## default

`function` · `datafusion_common::config::ConfigOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> ConfigOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1913, 24], "end": [1913, 31], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:1913`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f8a87ea16d6a4691215a166"></a>
## entries

`function` · `datafusion_common::config::ConfigOptions::entries` · datafusion-common 55.1.0

```rust
fn entries(&self) -> Vec<ConfigEntry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 1], "end": [2200, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2141`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the [`ConfigEntry`](../operations/datafusion_common.config.ConfigEntry.md#op-e610e9614a810518fe36454b) stored within this [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4)

<a id="op-1a65b541b76cc5e25160a8a5"></a>
## execution

`struct_field` · `datafusion_common::config::ConfigOptions::execution` · datafusion-common 55.1.0

```rust
execution: ExecutionOptions
```

Source: `src/config.rs:1919`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Execution options

<a id="op-f81424a1848d9e790483186d"></a>
## explain

`struct_field` · `datafusion_common::config::ConfigOptions::explain` · datafusion-common 55.1.0

```rust
explain: ExplainOptions
```

Source: `src/config.rs:1925`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Explain options

<a id="op-8604a8d7ebf81341bf6b0e28"></a>
## extensions

`struct_field` · `datafusion_common::config::ConfigOptions::extensions` · datafusion-common 55.1.0

```rust
extensions: Extensions
```

Source: `src/config.rs:1927`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Optional extensions registered using [`Extensions::insert`](../operations/datafusion_common.config.Extensions.md#op-7b82f2b1c0fb03c21d2ca5a2)

<a id="op-48e859fff50e4bfb9e64ab86"></a>
## fmt

`function` · `datafusion_common::config::ConfigOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1913, 10], "end": [1913, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:1913`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-414c20398f421dc7a07149fb"></a>
## format

`struct_field` · `datafusion_common::config::ConfigOptions::format` · datafusion-common 55.1.0

```rust
format: FormatOptions
```

Source: `src/config.rs:1929`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Formatting options when printing batches

<a id="op-58f04b4f42fdef7b384ed3b1"></a>
## from_env

`function` · `datafusion_common::config::ConfigOptions::from_env` · datafusion-common 55.1.0

```rust
fn from_env() -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 1], "end": [2200, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2076`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create new [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4), taking values from environment variables
where possible.

For example, to configure `datafusion.execution.batch_size`
([`ExecutionOptions::batch_size`](../operations/datafusion_common.config.ExecutionOptions.md#op-c25412ecbe58e58a4d83f3dd)) you would set the
`DATAFUSION_EXECUTION_BATCH_SIZE` environment variable.

The name of the environment variable is the option's key, transformed to
uppercase and with periods replaced with underscores.

Values are parsed according to the [same rules used in casts from
Utf8](https://docs.rs/arrow/latest/arrow/compute/kernels/cast/fn.cast.html).

If the value in the environment variable cannot be cast to the type of
the configuration option, the default value will be used instead and a
warning emitted. Environment variables are read when this method is
called, and are not re-read later.

<a id="op-c2ced56a561d6617092b694d"></a>
## from_string_hash_map

`function` · `datafusion_common::config::ConfigOptions::from_string_hash_map` · datafusion-common 55.1.0

```rust
fn from_string_hash_map(settings: &HashMap<String, String>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 1], "end": [2200, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2114`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create new ConfigOptions struct, taking values from a string hash map.

Only the built-in configurations will be extracted from the hash map
and other key value pairs will be ignored.

<a id="op-b9fa5b2f31f01c42a84ab3ce"></a>
## generate_config_markdown

`function` · `datafusion_common::config::ConfigOptions::generate_config_markdown` · datafusion-common 55.1.0

```rust
fn generate_config_markdown() -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 1], "end": [2200, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2175`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Generate documentation that can be included in the user guide

<a id="op-91990134d9c8856d695c5159"></a>
## new

`function` · `datafusion_common::config::ConfigOptions::new` · datafusion-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 1], "end": [2200, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2007`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4) with default values

<a id="op-98c1789a6a1553e6c2b2d9b3"></a>
## optimizer

`struct_field` · `datafusion_common::config::ConfigOptions::optimizer` · datafusion-common 55.1.0

```rust
optimizer: OptimizerOptions
```

Source: `src/config.rs:1921`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Optimizer options

<a id="op-87009370a0477bf2c2e4abca"></a>
## reset

`function` · `datafusion_common::config::ConfigOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1934, 1], "end": [1999, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1961`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Reset a configuration option back to its default value

<a id="op-9494afef6fda732622fc65c0"></a>
## set

`function` · `datafusion_common::config::ConfigOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1934, 1], "end": [1999, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1945`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad8e2c5b532326906bc1dfe7"></a>
## set

`function` · `datafusion_common::config::ConfigOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 1], "end": [2200, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2018`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set a configuration option

<a id="op-e0d5b3ed1bde5cf75fe53871"></a>
## spark

`struct_field` · `datafusion_common::config::ConfigOptions::spark` · datafusion-common 55.1.0

```rust
spark: SparkOptions
```

Source: `src/config.rs:1931`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Spark-compatibility options (functions under `datafusion/spark`)

<a id="op-14693c8e985f2b7d44e4246a"></a>
## sql_parser

`struct_field` · `datafusion_common::config::ConfigOptions::sql_parser` · datafusion-common 55.1.0

```rust
sql_parser: SqlParserOptions
```

Source: `src/config.rs:1923`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

SQL parser options

<a id="op-60e23d348a887de85c6338c0"></a>
## visit

`function` · `datafusion_common::config::ConfigOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, _key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1934, 1], "end": [1999, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1935`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a52d63d11f86cacc7e80e156"></a>
## with_extensions

`function` · `datafusion_common::config::ConfigOptions::with_extensions` · datafusion-common 55.1.0

```rust
fn with_extensions(self, extensions: Extensions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 1], "end": [2200, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2012`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set extensions to provided value
