# `datafusion_sql::unparser::dialect::BigQueryDialect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.BigQueryDialect.json).

<a id="op-426144daa854ec1978083b56"></a>
## BigQueryDialect

`struct` · `datafusion_sql::unparser::dialect::BigQueryDialect` · datafusion-sql 55.1.0

```rust
struct BigQueryDialect
```

Source: `src/unparser/dialect.rs:687`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-994b585ca2dc9d2fcfed8814"></a>
## col_alias_overrides

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::col_alias_overrides` · datafusion-sql 55.1.0

```rust
fn col_alias_overrides(&self, alias: &str) -> Result<Option<String>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:694`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80ecd94b683f231ddefb9718"></a>
## date_field_extract_style

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::date_field_extract_style` · datafusion-sql 55.1.0

```rust
fn date_field_extract_style(&self) -> DateFieldExtractStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:745`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cefca74f3a29059732dfb63e"></a>
## default

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::default` · datafusion-sql 55.1.0

```rust
fn default() -> BigQueryDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [686, 10], "end": [686, 17], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/dialect.rs:686`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25f5a49579f34cad8c8cee6b"></a>
## float64_ast_dtype

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::float64_ast_dtype` · datafusion-sql 55.1.0

```rust
fn float64_ast_dtype(&self) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:725`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88fd946447e87732eac1ec00"></a>
## identifier_quote_style

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::identifier_quote_style` · datafusion-sql 55.1.0

```rust
fn identifier_quote_style(&self, _: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:690`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66ab7f1f3d44f0bbebcb5dac"></a>
## interval_style

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::interval_style` · datafusion-sql 55.1.0

```rust
fn interval_style(&self) -> IntervalStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:749`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7778566286f8800e2eafbb1e"></a>
## large_utf8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::large_utf8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn large_utf8_cast_dtype(&self) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:733`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb7ab36886f57ab9261d1a3b"></a>
## new

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::new` · datafusion-sql 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 1], "end": [772, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:769`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b0a3beb5f27b16e7596e51b"></a>
## scalar_function_to_sql_overrides

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::scalar_function_to_sql_overrides` · datafusion-sql 55.1.0

```rust
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:753`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe3c9f95b1b01de56475ffa9"></a>
## supports_column_alias_in_table_alias

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::supports_column_alias_in_table_alias` · datafusion-sql 55.1.0

```rust
fn supports_column_alias_in_table_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:721`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f45a57cb77712971b40da85"></a>
## timestamp_cast_dtype

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::timestamp_cast_dtype` · datafusion-sql 55.1.0

```rust
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, _tz: &Option<Arc<str>>) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:737`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fa1335014dabdda7d5ce00c"></a>
## unnest_as_table_factor

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::unnest_as_table_factor` · datafusion-sql 55.1.0

```rust
fn unnest_as_table_factor(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:717`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7e5d1bc394b5607ee9c640e"></a>
## utf8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::BigQueryDialect::utf8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn utf8_cast_dtype(&self) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 1], "end": [765, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:729`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
