# `datafusion_sql::unparser::dialect::DuckDBDialect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.DuckDBDialect.json).

<a id="op-696c3a9c66eb3531d2501bbd"></a>
## DuckDBDialect

`struct` · `datafusion_sql::unparser::dialect::DuckDBDialect` · datafusion-sql 55.1.0

```rust
struct DuckDBDialect
```

Source: `src/unparser/dialect.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-091deca06bd60659b221c355"></a>
## character_length_style

`function` · `datafusion_sql::unparser::dialect::DuckDBDialect::character_length_style` · datafusion-sql 55.1.0

```rust
fn character_length_style(&self) -> CharacterLengthStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DuckDBDialect", "path": "DuckDBDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 1], "end": [556, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:513`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51cfe7f6581f1c04c1f856a6"></a>
## default

`function` · `datafusion_sql::unparser::dialect::DuckDBDialect::default` · datafusion-sql 55.1.0

```rust
fn default() -> DuckDBDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DuckDBDialect", "path": "DuckDBDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [494, 10], "end": [494, 17], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/dialect.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c1997475595c0fe05c9f9d7"></a>
## distinct_from_style

`function` · `datafusion_sql::unparser::dialect::DuckDBDialect::distinct_from_style` · datafusion-sql 55.1.0

```rust
fn distinct_from_style(&self) -> DistinctFromStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DuckDBDialect", "path": "DuckDBDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 1], "end": [556, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:553`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef4c7090b058ca6db24aab6b"></a>
## division_operator

`function` · `datafusion_sql::unparser::dialect::DuckDBDialect::division_operator` · datafusion-sql 55.1.0

```rust
fn division_operator(&self) -> BinaryOperator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DuckDBDialect", "path": "DuckDBDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 1], "end": [556, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:517`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4431f87564558dd0144943e"></a>
## identifier_quote_style

`function` · `datafusion_sql::unparser::dialect::DuckDBDialect::identifier_quote_style` · datafusion-sql 55.1.0

```rust
fn identifier_quote_style(&self, _: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DuckDBDialect", "path": "DuckDBDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 1], "end": [556, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:509`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c79ccbcef6c73c0c98b6a88"></a>
## new

`function` · `datafusion_sql::unparser::dialect::DuckDBDialect::new` · datafusion-sql 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DuckDBDialect", "path": "DuckDBDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [506, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab6e8d78d6809c270fcc0e16"></a>
## scalar_function_to_sql_overrides

`function` · `datafusion_sql::unparser::dialect::DuckDBDialect::scalar_function_to_sql_overrides` · datafusion-sql 55.1.0

```rust
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DuckDBDialect", "path": "DuckDBDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 1], "end": [556, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:532`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6eda7db478be2ed2772c3e36"></a>
## with_custom_scalar_overrides

`function` · `datafusion_sql::unparser::dialect::DuckDBDialect::with_custom_scalar_overrides` · datafusion-sql 55.1.0

```rust
fn with_custom_scalar_overrides(self, handlers: Vec<(&str, ScalarFnToSqlHandler)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DuckDBDialect", "path": "DuckDBDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 1], "end": [556, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "datafusion_sql::unparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "datafusion_sql::unparser::dialect::Dialect"}`

Source: `src/unparser/dialect.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
