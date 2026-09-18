# `datafusion_sql::unparser::dialect::PostgreSqlDialect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.PostgreSqlDialect.json).

<a id="op-53af3a9fbb20c4e6a044b7ab"></a>
## PostgreSqlDialect

`struct` · `datafusion_sql::unparser::dialect::PostgreSqlDialect` · datafusion-sql 55.1.0

```rust
struct PostgreSqlDialect
```

Source: `src/unparser/dialect.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71ddf8e363576a352b3fca89"></a>
## distinct_from_style

`function` · `datafusion_sql::unparser::dialect::PostgreSqlDialect::distinct_from_style` · datafusion-sql 55.1.0

```rust
fn distinct_from_style(&self) -> DistinctFromStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [424, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:402`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5156148a0235897b5626e245"></a>
## float64_ast_dtype

`function` · `datafusion_sql::unparser::dialect::PostgreSqlDialect::float64_ast_dtype` · datafusion-sql 55.1.0

```rust
fn float64_ast_dtype(&self) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [424, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:394`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e1e784a1a3dbc8d0e2e71c0"></a>
## identifier_quote_style

`function` · `datafusion_sql::unparser::dialect::PostgreSqlDialect::identifier_quote_style` · datafusion-sql 55.1.0

```rust
fn identifier_quote_style(&self, _: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [424, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bbb7034ef6e7b78aee3c33e"></a>
## int8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::PostgreSqlDialect::int8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn int8_cast_dtype(&self) -> ast::DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [424, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:398`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abcd34ed3ed5e152f4c62bb9"></a>
## interval_style

`function` · `datafusion_sql::unparser::dialect::PostgreSqlDialect::interval_style` · datafusion-sql 55.1.0

```rust
fn interval_style(&self) -> IntervalStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [424, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51ebfb0a450d1d074c8a7c60"></a>
## requires_derived_table_alias

`function` · `datafusion_sql::unparser::dialect::PostgreSqlDialect::requires_derived_table_alias` · datafusion-sql 55.1.0

```rust
fn requires_derived_table_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [424, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:378`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-630d3b2ed7086861b4d9a740"></a>
## scalar_function_to_sql_overrides

`function` · `datafusion_sql::unparser::dialect::PostgreSqlDialect::scalar_function_to_sql_overrides` · datafusion-sql 55.1.0

```rust
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [424, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cc487ecccca2271c987f737"></a>
## supports_empty_select_list

`function` · `datafusion_sql::unparser::dialect::PostgreSqlDialect::supports_empty_select_list` · datafusion-sql 55.1.0

```rust
fn supports_empty_select_list(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [424, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b61f923dfc69a796e5ff7c59"></a>
## supports_qualify

`function` · `datafusion_sql::unparser::dialect::PostgreSqlDialect::supports_qualify` · datafusion-sql 55.1.0

```rust
fn supports_qualify(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [424, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9312938063597e9d3cfc03b7"></a>
## use_array_keyword_for_array_literals

`function` · `datafusion_sql::unparser::dialect::PostgreSqlDialect::use_array_keyword_for_array_literals` · datafusion-sql 55.1.0

```rust
fn use_array_keyword_for_array_literals(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [424, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
