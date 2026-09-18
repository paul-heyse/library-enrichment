# `datafusion_common::config::SqlParserOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.SqlParserOptions.json).

<a id="op-9136dabd1b661c04a50818cc"></a>
## SqlParserOptions

`struct` · `datafusion_common::config::SqlParserOptions` · datafusion-common 55.1.0

```rust
struct SqlParserOptions
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options related to SQL parser

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

<a id="op-06990833ec7b200b526e9afb"></a>
## clone

`function` · `datafusion_common::config::SqlParserOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> SqlParserOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SqlParserOptions", "path": "SqlParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [325, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0e4bfb187614a830992406d"></a>
## collect_spans

`struct_field` · `datafusion_common::config::SqlParserOptions::collect_spans` · datafusion-common 55.1.0

```rust
collect_spans: bool
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the source locations relative to the original SQL
query (i.e. [`Span`](https://docs.rs/sqlparser/latest/sqlparser/tokenizer/struct.Span.html)) will be collected
and recorded in the logical plan nodes.

<a id="op-69b644529eec171f53154c8b"></a>
## default

`function` · `datafusion_common::config::SqlParserOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SqlParserOptions", "path": "SqlParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [325, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a49d05a40a053762552c53e8"></a>
## default_null_ordering

`struct_field` · `datafusion_common::config::SqlParserOptions::default_null_ordering` · datafusion-common 55.1.0

```rust
default_null_ordering: String
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Specifies the default null ordering for query results. There are 4 options:
- `nulls_max`: Nulls appear last in ascending order.
- `nulls_min`: Nulls appear first in ascending order.
- `nulls_first`: Nulls always be first in any order.
- `nulls_last`: Nulls always be last in any order.

By default, `nulls_max` is used to follow Postgres's behavior.
postgres rule: <https://www.postgresql.org/docs/current/queries-order.html>

<a id="op-61de312ad9738b9c515acef5"></a>
## dialect

`struct_field` · `datafusion_common::config::SqlParserOptions::dialect` · datafusion-common 55.1.0

```rust
dialect: Dialect
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Configure the SQL dialect used by DataFusion's parser.
The configuration reference lists the supported values from [`Dialect::available`](../operations/datafusion_common.config.Dialect.md#op-7a339ad8e54ce60c9d25761b).

<a id="op-ace7b15c0bb788df90c9a048"></a>
## enable_ident_normalization

`struct_field` · `datafusion_common::config::SqlParserOptions::enable_ident_normalization` · datafusion-common 55.1.0

```rust
enable_ident_normalization: bool
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, SQL parser will normalize ident (convert ident to lowercase when not quoted)

<a id="op-7d128df75f2c318fdee669e1"></a>
## enable_options_value_normalization

`struct_field` · `datafusion_common::config::SqlParserOptions::enable_options_value_normalization` · datafusion-common 55.1.0

```rust
enable_options_value_normalization: bool
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, SQL parser will normalize options value (convert value to lowercase).
Note that this option is ignored and will be removed in the future. All case-insensitive values
are normalized automatically.

<a id="op-756bd25f4329c263b56a752c"></a>
## enable_subquery_sort_elimination

`struct_field` · `datafusion_common::config::SqlParserOptions::enable_subquery_sort_elimination` · datafusion-common 55.1.0

```rust
enable_subquery_sort_elimination: bool
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, DataFusion may remove `ORDER BY` clauses from
subqueries or CTEs during SQL planning when their ordering cannot
affect the result, such as when no `LIMIT` or other
order-sensitive operator depends on them.

Disable this option to preserve explicit subquery ordering in the
planned query.

<a id="op-91b7b9f313d57efaa0ff15f8"></a>
## eq

`function` · `datafusion_common::config::SqlParserOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &SqlParserOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SqlParserOptions", "path": "SqlParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [325, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efc8b6b54847575a2ee6639a"></a>
## fmt

`function` · `datafusion_common::config::SqlParserOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SqlParserOptions", "path": "SqlParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [325, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fe4505decf42b3d51a34aac"></a>
## map_string_types_to_utf8view

`struct_field` · `datafusion_common::config::SqlParserOptions::map_string_types_to_utf8view` · datafusion-common 55.1.0

```rust
map_string_types_to_utf8view: bool
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If true, string types (VARCHAR, CHAR, Text, and String) are mapped to `Utf8View` during SQL planning.
If false, they are mapped to `Utf8`.
Default is true.

<a id="op-98741b5f828624f723f88c3a"></a>
## parse_float_as_decimal

`struct_field` · `datafusion_common::config::SqlParserOptions::parse_float_as_decimal` · datafusion-common 55.1.0

```rust
parse_float_as_decimal: bool
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, SQL parser will parse float as decimal type

<a id="op-92811a3519e57bcc0feb58f9"></a>
## recursion_limit

`struct_field` · `datafusion_common::config::SqlParserOptions::recursion_limit` · datafusion-common 55.1.0

```rust
recursion_limit: ConfigNonZeroUsize
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Specifies the recursion depth limit when parsing complex SQL Queries

<a id="op-ae7acd1b58fd79f712f6af18"></a>
## reset

`function` · `datafusion_common::config::SqlParserOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SqlParserOptions", "path": "SqlParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [325, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f562be2839e413aaf7786e19"></a>
## set

`function` · `datafusion_common::config::SqlParserOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SqlParserOptions", "path": "SqlParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [325, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-626f10b6274c3c17fbad4144"></a>
## support_varchar_with_length

`struct_field` · `datafusion_common::config::SqlParserOptions::support_varchar_with_length` · datafusion-common 55.1.0

```rust
support_varchar_with_length: bool
```

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If true, permit lengths for `VARCHAR` such as `VARCHAR(20)`, but
ignore the length. If false, error if a `VARCHAR` with a length is
specified. The Arrow type system does not have a notion of maximum
string length and thus DataFusion can not enforce such limits.

<a id="op-ee1848691976b13afab7177d"></a>
## visit

`function` · `datafusion_common::config::SqlParserOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SqlParserOptions", "path": "SqlParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [325, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
