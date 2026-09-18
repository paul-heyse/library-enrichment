# `datafusion_sql::unparser::dialect::SqliteDialect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.SqliteDialect.json).

<a id="op-c8e02e698b5d23cf03cdc966"></a>
## SqliteDialect

`struct` · `datafusion_sql::unparser::dialect::SqliteDialect` · datafusion-sql 55.1.0

```rust
struct SqliteDialect
```

Source: `src/unparser/dialect.rs:627`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-567a95213dc274aa4fd6846a"></a>
## character_length_style

`function` · `datafusion_sql::unparser::dialect::SqliteDialect::character_length_style` · datafusion-sql 55.1.0

```rust
fn character_length_style(&self) -> CharacterLengthStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SqliteDialect", "path": "SqliteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [684, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:646`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79d6299be49172daeb8a1969"></a>
## date32_cast_dtype

`function` · `datafusion_sql::unparser::dialect::SqliteDialect::date32_cast_dtype` · datafusion-sql 55.1.0

```rust
fn date32_cast_dtype(&self) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SqliteDialect", "path": "SqliteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [684, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:642`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cfa3cbee2764756514d48dc"></a>
## date_field_extract_style

`function` · `datafusion_sql::unparser::dialect::SqliteDialect::date_field_extract_style` · datafusion-sql 55.1.0

```rust
fn date_field_extract_style(&self) -> DateFieldExtractStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SqliteDialect", "path": "SqliteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [684, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:638`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19e1c76d62184fe02c1e9ab6"></a>
## distinct_from_style

`function` · `datafusion_sql::unparser::dialect::SqliteDialect::distinct_from_style` · datafusion-sql 55.1.0

```rust
fn distinct_from_style(&self) -> DistinctFromStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SqliteDialect", "path": "SqliteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [684, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:650`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16b6bc6b06989a3438784ddc"></a>
## identifier_quote_style

`function` · `datafusion_sql::unparser::dialect::SqliteDialect::identifier_quote_style` · datafusion-sql 55.1.0

```rust
fn identifier_quote_style(&self, _: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SqliteDialect", "path": "SqliteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [684, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:634`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50c42dd7782076bcb3e5cb09"></a>
## scalar_function_to_sql_overrides

`function` · `datafusion_sql::unparser::dialect::SqliteDialect::scalar_function_to_sql_overrides` · datafusion-sql 55.1.0

```rust
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SqliteDialect", "path": "SqliteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [684, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:666`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-305db7f1ce38eedceaf27ae3"></a>
## supports_column_alias_in_table_alias

`function` · `datafusion_sql::unparser::dialect::SqliteDialect::supports_column_alias_in_table_alias` · datafusion-sql 55.1.0

```rust
fn supports_column_alias_in_table_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SqliteDialect", "path": "SqliteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [684, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:654`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c44917b20cf3d24104f0b2f6"></a>
## supports_qualify

`function` · `datafusion_sql::unparser::dialect::SqliteDialect::supports_qualify` · datafusion-sql 55.1.0

```rust
fn supports_qualify(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SqliteDialect", "path": "SqliteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [684, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:630`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fd649d636b84be210aaed45"></a>
## timestamp_cast_dtype

`function` · `datafusion_sql::unparser::dialect::SqliteDialect::timestamp_cast_dtype` · datafusion-sql 55.1.0

```rust
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, _tz: &Option<Arc<str>>) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::SqliteDialect", "path": "SqliteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [684, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:658`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
