# `datafusion_common::config::CatalogOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.CatalogOptions.json).

<a id="op-cc8848630c802be1a4995eea"></a>
## CatalogOptions

`struct` · `datafusion_common::config::CatalogOptions` · datafusion-common 55.1.0

```rust
struct CatalogOptions
```

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options related to catalog and directory scanning

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

<a id="op-fca7c5da303de2f245f0f7f7"></a>
## clone

`function` · `datafusion_common::config::CatalogOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> CatalogOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CatalogOptions", "path": "CatalogOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [262, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-849b141d5d9d012ec3c58a58"></a>
## create_default_catalog_and_schema

`struct_field` · `datafusion_common::config::CatalogOptions::create_default_catalog_and_schema` · datafusion-common 55.1.0

```rust
create_default_catalog_and_schema: bool
```

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Whether the default catalog and schema should be created automatically.

<a id="op-3b71edb0110c635a825e8a86"></a>
## default

`function` · `datafusion_common::config::CatalogOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CatalogOptions", "path": "CatalogOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [262, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05f1a56c9246d6f65e813bcd"></a>
## default_catalog

`struct_field` · `datafusion_common::config::CatalogOptions::default_catalog` · datafusion-common 55.1.0

```rust
default_catalog: String
```

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The default catalog name - this impacts what SQL queries use if not specified

<a id="op-2984cf59ca1fe7621fc03ca3"></a>
## default_schema

`struct_field` · `datafusion_common::config::CatalogOptions::default_schema` · datafusion-common 55.1.0

```rust
default_schema: String
```

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The default schema name - this impacts what SQL queries use if not specified

<a id="op-12bd55c592ef54c2b8feb313"></a>
## eq

`function` · `datafusion_common::config::CatalogOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &CatalogOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CatalogOptions", "path": "CatalogOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [262, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d23fd8ba28abb8c040f75e41"></a>
## fmt

`function` · `datafusion_common::config::CatalogOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CatalogOptions", "path": "CatalogOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [262, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a85040635912ec034b88377"></a>
## format

`struct_field` · `datafusion_common::config::CatalogOptions::format` · datafusion-common 55.1.0

```rust
format: Option<String>
```

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Type of `TableProvider` to use when loading `default` schema

<a id="op-cbfa0a1f37c46eb4e8cd5a1b"></a>
## has_header

`struct_field` · `datafusion_common::config::CatalogOptions::has_header` · datafusion-common 55.1.0

```rust
has_header: bool
```

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Default value for `format.has_header` for `CREATE EXTERNAL TABLE`
if not specified explicitly in the statement.

<a id="op-367fbc09079d8db9bc29ef5c"></a>
## information_schema

`struct_field` · `datafusion_common::config::CatalogOptions::information_schema` · datafusion-common 55.1.0

```rust
information_schema: bool
```

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion provide access to `information_schema`
virtual tables for displaying schema information

<a id="op-8d304a1a0a5c5e53bb352943"></a>
## location

`struct_field` · `datafusion_common::config::CatalogOptions::location` · datafusion-common 55.1.0

```rust
location: Option<String>
```

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Location scanned to load tables for `default` schema

<a id="op-cad16cbd95501757728b5311"></a>
## newlines_in_values

`struct_field` · `datafusion_common::config::CatalogOptions::newlines_in_values` · datafusion-common 55.1.0

```rust
newlines_in_values: bool
```

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Specifies whether newlines in (quoted) CSV values are supported.

This is the default value for `format.newlines_in_values` for `CREATE EXTERNAL TABLE`
if not specified explicitly in the statement.

Parsing newlines in quoted values may be affected by execution behaviour such as
parallel file scanning. Setting this to `true` ensures that newlines in values are
parsed successfully, which may reduce performance.

<a id="op-ab54f469ecd06868cc4ae8f7"></a>
## reset

`function` · `datafusion_common::config::CatalogOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CatalogOptions", "path": "CatalogOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [262, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6d4178520614aa96fc641dc"></a>
## set

`function` · `datafusion_common::config::CatalogOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CatalogOptions", "path": "CatalogOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [262, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88ae4db27cf9779969da9fed"></a>
## visit

`function` · `datafusion_common::config::CatalogOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::CatalogOptions", "path": "CatalogOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [262, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
