# `datafusion_sql::unparser::dialect::SnowflakeDialect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.SnowflakeDialect.json).

<a id="op-ee715b18ac9dfe751696eec7"></a>
## SnowflakeDialect

`struct` · `datafusion_sql::unparser::dialect::SnowflakeDialect` · datafusion-sql 55.1.0

```rust
struct SnowflakeDialect
```

Source: `src/unparser/dialect.rs:783`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Dialect for Snowflake SQL.

Key differences from the default dialect:
- Uses double-quote identifier quoting
- Supports `NULLS FIRST`/`NULLS LAST` in `ORDER BY`
- Does not support empty select lists (`SELECT FROM t`)
- Does not support column aliases in table alias definitions
  (Snowflake accepts the syntax but silently ignores the renames in join contexts)
- Unparses `UNNEST` plans as `LATERAL FLATTEN(INPUT => expr, ...)`

<a id="op-309e6aa8b7762d7332eed959"></a>
## identifier_quote_style

`function` · `datafusion_sql::unparser::dialect::SnowflakeDialect::identifier_quote_style` · datafusion-sql 55.1.0

```rust
fn identifier_quote_style(&self, _: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [793, 1], "end": [825, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:794`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7a045eee0ce344edcdcc39e"></a>
## new

`function` · `datafusion_sql::unparser::dialect::SnowflakeDialect::new` · datafusion-sql 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [786, 1], "end": [791, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:788`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0cefa13b8a0ed367d53af14"></a>
## supports_column_alias_in_table_alias

`function` · `datafusion_sql::unparser::dialect::SnowflakeDialect::supports_column_alias_in_table_alias` · datafusion-sql 55.1.0

```rust
fn supports_column_alias_in_table_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [793, 1], "end": [825, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:806`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-646f2cbd1031a5b38f904bc8"></a>
## supports_empty_select_list

`function` · `datafusion_sql::unparser::dialect::SnowflakeDialect::supports_empty_select_list` · datafusion-sql 55.1.0

```rust
fn supports_empty_select_list(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [793, 1], "end": [825, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aec414784e58554810445e71"></a>
## supports_nulls_first_in_sort

`function` · `datafusion_sql::unparser::dialect::SnowflakeDialect::supports_nulls_first_in_sort` · datafusion-sql 55.1.0

```rust
fn supports_nulls_first_in_sort(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [793, 1], "end": [825, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:798`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2791f88bfb310de2c08abf6"></a>
## timestamp_cast_dtype

`function` · `datafusion_sql::unparser::dialect::SnowflakeDialect::timestamp_cast_dtype` · datafusion-sql 55.1.0

```rust
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, tz: &Option<Arc<str>>) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [793, 1], "end": [825, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:810`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1c5174750a1b446e9b5debc"></a>
## unnest_as_lateral_flatten

`function` · `datafusion_sql::unparser::dialect::SnowflakeDialect::unnest_as_lateral_flatten` · datafusion-sql 55.1.0

```rust
fn unnest_as_lateral_flatten(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [793, 1], "end": [825, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:822`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
