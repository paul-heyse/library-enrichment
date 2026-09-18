# `sqlparser::dialect::sqlite::SQLiteDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.sqlite.SQLiteDialect.json).

<a id="op-259947aa56697b5c1b719c5f"></a>
## SQLiteDialect

`struct` · `sqlparser::dialect::sqlite::SQLiteDialect` · sqlparser 0.62.0

```rust
struct SQLiteDialect
```

Source: `src/dialect/sqlite.rs:35`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [SQLite](https://www.sqlite.org)

This dialect allows columns in a
[`CREATE TABLE`](https://sqlite.org/lang_createtable.html) statement with no
type specified, as in `CREATE TABLE t1 (a)`. In the AST, these columns will
have the data type [`Unspecified`](crate::ast::DataType::Unspecified).

<a id="op-b571a8e433fac0d6a86756a0"></a>
## clone

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SQLiteDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 26], "end": [33, 31], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/sqlite.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a150eb30e905b709cbc5ea56"></a>
## cmp

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SQLiteDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 72], "end": [33, 75], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/sqlite.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2135a473c4dbc45e233a013"></a>
## default

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::default` · sqlparser 0.62.0

```rust
fn default() -> SQLiteDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 24], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/sqlite.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1c949756c514f7ccb05df4c"></a>
## deserialize

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 56], "end": [34, 74], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/sqlite.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9eca5c5e89fa4723cd3d5aff"></a>
## eq

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SQLiteDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 39], "end": [33, 48], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/sqlite.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90d96f49fa2b109e6550c3b1"></a>
## fmt

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/sqlite.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9018283a9b320b4ed6d18769"></a>
## hash

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 54], "end": [33, 58], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/sqlite.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb56ec3db65db060ceba7e64"></a>
## identifier_quote_style

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::identifier_quote_style` · sqlparser 0.62.0

```rust
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86e3d22cf10ad83490240944"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b0f8ddee5a2181e9c7b2e4d"></a>
## is_identifier_part

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:65`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00e7cab8b037d23d786ce789"></a>
## is_identifier_start

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:49`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45f1c2d207a4dd76491571a3"></a>
## parse_infix

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::parse_infix` · sqlparser 0.62.0

```rust
fn parse_infix(&self, parser: &mut parser::Parser<'_>, expr: &ast::Expr, _precedence: u8) -> Option<Result<ast::Expr, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:78`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-494d1fc71e5bd1bae1cfb3ee"></a>
## parse_statement

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::parse_statement` · sqlparser 0.62.0

```rust
fn parse_statement(&self, parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:69`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1550226b9e7f4a5ec73fdfd"></a>
## partial_cmp

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SQLiteDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 60], "end": [33, 70], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/sqlite.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62cb07f5bc836bda37d993a0"></a>
## serialize

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 38], "end": [34, 54], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/sqlite.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f6bd018969a6d27210f3c84"></a>
## supports_asc_desc_in_column_definition

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::supports_asc_desc_in_column_definition` · sqlparser 0.62.0

```rust
fn supports_asc_desc_in_column_definition(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:110`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-551eadf41105e1904a18c4e9"></a>
## supports_comma_separated_trim

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::supports_comma_separated_trim` · sqlparser 0.62.0

```rust
fn supports_comma_separated_trim(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ace172b90b7a54da02fe8f8"></a>
## supports_dollar_placeholder

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::supports_dollar_placeholder` · sqlparser 0.62.0

```rust
fn supports_dollar_placeholder(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48f072f4c607fb06ce01dda1"></a>
## supports_filter_during_aggregation

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::supports_filter_during_aggregation` · sqlparser 0.62.0

```rust
fn supports_filter_during_aggregation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:57`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-151293d6f848a22347302f5b"></a>
## supports_in_empty_list

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::supports_in_empty_list` · sqlparser 0.62.0

```rust
fn supports_in_empty_list(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94d6c8a2f9effc67c0a2433b"></a>
## supports_limit_comma

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::supports_limit_comma` · sqlparser 0.62.0

```rust
fn supports_limit_comma(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:106`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9075b8fc82416da5b219ee1"></a>
## supports_notnull_operator

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::supports_notnull_operator` · sqlparser 0.62.0

```rust
fn supports_notnull_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQLite supports `NOTNULL` as aliases for `IS NOT NULL`
See: <https://sqlite.org/syntax/expr.html>

<a id="op-75b70841f09d32574a95a47a"></a>
## supports_start_transaction_modifier

`function` · `sqlparser::dialect::sqlite::SQLiteDialect::supports_start_transaction_modifier` · sqlparser 0.62.0

```rust
fn supports_start_transaction_modifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::sqlite::SQLiteDialect", "path": "SQLiteDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [127, 2], "filename": "src/dialect/sqlite.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/sqlite.rs:61`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
