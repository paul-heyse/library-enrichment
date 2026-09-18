# `datafusion_sql::unparser::dialect::MySqlDialect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.MySqlDialect.json).

<a id="op-2a6f9e4b9d8a8af39f3f6832"></a>
## MySqlDialect

`struct` · `datafusion_sql::unparser::dialect::MySqlDialect` · datafusion-sql 55.1.0

```rust
struct MySqlDialect
```

Source: `src/unparser/dialect.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-198cdd780622ecd607dd662c"></a>
## date_field_extract_style

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::date_field_extract_style` · datafusion-sql 55.1.0

```rust
fn date_field_extract_style(&self) -> DateFieldExtractStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89620c22f45c363a7c815db4"></a>
## distinct_from_style

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::distinct_from_style` · datafusion-sql 55.1.0

```rust
fn distinct_from_style(&self) -> DistinctFromStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:589`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d5a25bf615d8dd7d35ce316"></a>
## identifier_quote_style

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::identifier_quote_style` · datafusion-sql 55.1.0

```rust
fn identifier_quote_style(&self, _: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13f67af45b8c76020b4257e2"></a>
## int32_cast_dtype

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::int32_cast_dtype` · datafusion-sql 55.1.0

```rust
fn int32_cast_dtype(&self) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:597`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e775958b8e2b19877adb5a8"></a>
## int64_cast_dtype

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::int64_cast_dtype` · datafusion-sql 55.1.0

```rust
fn int64_cast_dtype(&self) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:593`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79890030ed0e1b6aa0d1df23"></a>
## interval_style

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::interval_style` · datafusion-sql 55.1.0

```rust
fn interval_style(&self) -> IntervalStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:573`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06b3615446a7c0d8c54574f8"></a>
## large_utf8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::large_utf8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn large_utf8_cast_dtype(&self) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:581`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dc2d95018f13be3f3c3cd2a"></a>
## requires_derived_table_alias

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::requires_derived_table_alias` · datafusion-sql 55.1.0

```rust
fn requires_derived_table_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:609`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1067b465bc386a27edb83978"></a>
## scalar_function_to_sql_overrides

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::scalar_function_to_sql_overrides` · datafusion-sql 55.1.0

```rust
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:613`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4c61265403f23aead2600a1"></a>
## supports_nulls_first_in_sort

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::supports_nulls_first_in_sort` · datafusion-sql 55.1.0

```rust
fn supports_nulls_first_in_sort(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:569`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d5500a9f6b40f764f562b75"></a>
## supports_qualify

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::supports_qualify` · datafusion-sql 55.1.0

```rust
fn supports_qualify(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:561`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ebfc29b220c1dfa31359642"></a>
## timestamp_cast_dtype

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::timestamp_cast_dtype` · datafusion-sql 55.1.0

```rust
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, _tz: &Option<Arc<str>>) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:601`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5c0ff3eb42ff01944056455"></a>
## utf8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::MySqlDialect::utf8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn utf8_cast_dtype(&self) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [625, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:577`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
