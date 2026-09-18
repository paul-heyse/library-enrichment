# `datafusion_common::config::FormatOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.FormatOptions.json).

<a id="op-51fb9ef129fc808bbffee22a"></a>
## FormatOptions

`struct` · `datafusion_common::config::FormatOptions` · datafusion-common 55.1.0

```rust
struct FormatOptions
```

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options controlling the format of output when printing record batches
Copies [`arrow::util::display::FormatOptions`](../operations/arrow_cast.display.FormatOptions.md#op-739ef37988037ce736e7faff)

<a id="op-98e5be404478a239b804eb8c"></a>
## clone

`function` · `datafusion_common::config::FormatOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> FormatOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1828, 1], "end": [1852, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d325fb406893cee7adca857"></a>
## date_format

`struct_field` · `datafusion_common::config::FormatOptions::date_format` · datafusion-common 55.1.0

```rust
date_format: Option<String>
```

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Date format for date arrays

<a id="op-15c919ec3f17b16454afe986"></a>
## datetime_format

`struct_field` · `datafusion_common::config::FormatOptions::datetime_format` · datafusion-common 55.1.0

```rust
datetime_format: Option<String>
```

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Format for DateTime arrays

<a id="op-ae340a08b457d95fcce5a4c8"></a>
## default

`function` · `datafusion_common::config::FormatOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1828, 1], "end": [1852, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fd3f925800aacf8738fe715"></a>
## duration_format

`struct_field` · `datafusion_common::config::FormatOptions::duration_format` · datafusion-common 55.1.0

```rust
duration_format: String
```

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Duration format. Can be either `"pretty"` or `"ISO8601"`

<a id="op-6793c127aee3560909494b27"></a>
## eq

`function` · `datafusion_common::config::FormatOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &FormatOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1828, 1], "end": [1852, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c91c0c056be2943783823a49"></a>
## fmt

`function` · `datafusion_common::config::FormatOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1828, 1], "end": [1852, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de5cb9454220d485e28c6645"></a>
## null

`struct_field` · `datafusion_common::config::FormatOptions::null` · datafusion-common 55.1.0

```rust
null: String
```

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Format string for nulls

<a id="op-f07bc50987955149bcaf64fb"></a>
## reset

`function` · `datafusion_common::config::FormatOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1828, 1], "end": [1852, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a4e5037cd3407c7f243ed4a"></a>
## safe

`struct_field` · `datafusion_common::config::FormatOptions::safe` · datafusion-common 55.1.0

```rust
safe: bool
```

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If set to `true` any formatting errors will be written to the output
instead of being converted into a [`std::fmt::Error`]

Unresolved upstream links (retained, not inferred): ``std::fmt::Error``.

<a id="op-4ae540e92b62a85ed0a33bec"></a>
## set

`function` · `datafusion_common::config::FormatOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1828, 1], "end": [1852, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6c5bc04d910e2daf8976dfb"></a>
## time_format

`struct_field` · `datafusion_common::config::FormatOptions::time_format` · datafusion-common 55.1.0

```rust
time_format: Option<String>
```

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Time format for time arrays

<a id="op-810a2849bf1d82cac236f741"></a>
## timestamp_format

`struct_field` · `datafusion_common::config::FormatOptions::timestamp_format` · datafusion-common 55.1.0

```rust
timestamp_format: Option<String>
```

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Timestamp format for timestamp arrays

<a id="op-e35ec3f8a96ded43be990c79"></a>
## timestamp_tz_format

`struct_field` · `datafusion_common::config::FormatOptions::timestamp_tz_format` · datafusion-common 55.1.0

```rust
timestamp_tz_format: Option<String>
```

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Timestamp format for timestamp with timezone arrays. When `None`, ISO 8601 format is used.

<a id="op-0d92c75ba39d61edab114943"></a>
## types_info

`struct_field` · `datafusion_common::config::FormatOptions::types_info` · datafusion-common 55.1.0

```rust
types_info: bool
```

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Show types in visual representation batches

<a id="op-73de1ecdf2cd0acd9f23d3f9"></a>
## visit

`function` · `datafusion_common::config::FormatOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::FormatOptions", "path": "FormatOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1828, 1], "end": [1852, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1828`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
