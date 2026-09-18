# `datafusion_sql::planner::ParserOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.planner.ParserOptions.json).

<a id="op-334a25f33d603162ddd6667c"></a>
## ParserOptions

`struct` · `datafusion_sql::planner::ParserOptions` · datafusion-sql 55.1.0

```rust
struct ParserOptions
```

Source: `src/planner.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

SQL parser options

<a id="op-45a60a77fc2d0270a604e029"></a>
## clone

`function` · `datafusion_sql::planner::ParserOptions::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> ParserOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 17], "end": [44, 22], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/planner.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf548df76ac8493c3fd53383"></a>
## collect_spans

`struct_field` · `datafusion_sql::planner::ParserOptions::collect_spans` · datafusion-sql 55.1.0

```rust
collect_spans: bool
```

Source: `src/planner.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether to collect spans

<a id="op-41ef800ea7d495d6c10ba78f"></a>
## default

`function` · `datafusion_sql::planner::ParserOptions::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [150, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/planner.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d2481db3fb3aa501b1692f6"></a>
## default_null_ordering

`struct_field` · `datafusion_sql::planner::ParserOptions::default_null_ordering` · datafusion-sql 55.1.0

```rust
default_null_ordering: NullOrdering
```

Source: `src/planner.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Default null ordering for sorting expressions.

<a id="op-f85fd1ddee661f8c383f4d7c"></a>
## enable_ident_normalization

`struct_field` · `datafusion_sql::planner::ParserOptions::enable_ident_normalization` · datafusion-sql 55.1.0

```rust
enable_ident_normalization: bool
```

Source: `src/planner.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether to normalize identifiers.

<a id="op-6a7880c82eab4ad8679111af"></a>
## enable_options_value_normalization

`struct_field` · `datafusion_sql::planner::ParserOptions::enable_options_value_normalization` · datafusion-sql 55.1.0

```rust
enable_options_value_normalization: bool
```

Source: `src/planner.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether to normalize options value.

<a id="op-1b34c598f7f619206c30c7f1"></a>
## fmt

`function` · `datafusion_sql::planner::ParserOptions::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3a6625c29de2c053facbfb9"></a>
## from

`function` · `datafusion_sql::planner::ParserOptions::from` · datafusion-sql 55.1.0

```rust
fn from(options: &SqlParserOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [165, 2], "filename": "src/planner.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::SqlParserOptions", "path": "SqlParserOptions"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/planner.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97f06f641a40769724798172"></a>
## map_string_types_to_utf8view

`struct_field` · `datafusion_sql::planner::ParserOptions::map_string_types_to_utf8view` · datafusion-sql 55.1.0

```rust
map_string_types_to_utf8view: bool
```

Source: `src/planner.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether string types (VARCHAR, CHAR, Text, and String) are mapped to `Utf8View` during SQL planning.

<a id="op-16625c063fa25375e017dce8"></a>
## new

`function` · `datafusion_sql::planner::ParserOptions::new` · datafusion-sql 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [144, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Creates a new `ParserOptions` instance with default values.

# Examples

```
use datafusion_sql::planner::ParserOptions;
let opts = ParserOptions::new();
assert_eq!(opts.parse_float_as_decimal, false);
assert_eq!(opts.enable_ident_normalization, true);
```

<a id="op-e4a112307eb76c7b4380f490"></a>
## parse_float_as_decimal

`struct_field` · `datafusion_sql::planner::ParserOptions::parse_float_as_decimal` · datafusion-sql 55.1.0

```rust
parse_float_as_decimal: bool
```

Source: `src/planner.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether to parse float as decimal.

<a id="op-5d2a4bbc273452ea9c7f34aa"></a>
## support_varchar_with_length

`struct_field` · `datafusion_sql::planner::ParserOptions::support_varchar_with_length` · datafusion-sql 55.1.0

```rust
support_varchar_with_length: bool
```

Source: `src/planner.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether to support varchar with length.

<a id="op-cacd0f0e9f5626e9f4151191"></a>
## with_collect_spans

`function` · `datafusion_sql::planner::ParserOptions::with_collect_spans` · datafusion-sql 55.1.0

```rust
fn with_collect_spans(self, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [144, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Sets the `collect_spans` option.

<a id="op-2c1e421de2bee280543877b9"></a>
## with_default_null_ordering

`function` · `datafusion_sql::planner::ParserOptions::with_default_null_ordering` · datafusion-sql 55.1.0

```rust
fn with_default_null_ordering(self, value: NullOrdering) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [144, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Sets the `default_null_ordering` option.

<a id="op-b4511b0e36ad5a22ba7840f7"></a>
## with_enable_ident_normalization

`function` · `datafusion_sql::planner::ParserOptions::with_enable_ident_normalization` · datafusion-sql 55.1.0

```rust
fn with_enable_ident_normalization(self, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [144, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Sets the `enable_ident_normalization` option.

# Examples

```
use datafusion_sql::planner::ParserOptions;
let opts = ParserOptions::new().with_enable_ident_normalization(false);
assert_eq!(opts.enable_ident_normalization, false);
```

<a id="op-884b3aedb64fb88a7dda6427"></a>
## with_enable_options_value_normalization

`function` · `datafusion_sql::planner::ParserOptions::with_enable_options_value_normalization` · datafusion-sql 55.1.0

```rust
fn with_enable_options_value_normalization(self, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [144, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Sets the `enable_options_value_normalization` option.

<a id="op-bb6cb9770266cdf047c4a327"></a>
## with_map_string_types_to_utf8view

`function` · `datafusion_sql::planner::ParserOptions::with_map_string_types_to_utf8view` · datafusion-sql 55.1.0

```rust
fn with_map_string_types_to_utf8view(self, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [144, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Sets the `map_string_types_to_utf8view` option.

<a id="op-933016f5340cb7cd44068bf8"></a>
## with_parse_float_as_decimal

`function` · `datafusion_sql::planner::ParserOptions::with_parse_float_as_decimal` · datafusion-sql 55.1.0

```rust
fn with_parse_float_as_decimal(self, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [144, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Sets the `parse_float_as_decimal` option.

# Examples

```
use datafusion_sql::planner::ParserOptions;
let opts = ParserOptions::new().with_parse_float_as_decimal(true);
assert_eq!(opts.parse_float_as_decimal, true);
```

<a id="op-1e018877ddeeac031f0abec4"></a>
## with_support_varchar_with_length

`function` · `datafusion_sql::planner::ParserOptions::with_support_varchar_with_length` · datafusion-sql 55.1.0

```rust
fn with_support_varchar_with_length(self, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [144, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Sets the `support_varchar_with_length` option.
