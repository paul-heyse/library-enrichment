# `sqlparser::dialect::mysql::MySqlDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.mysql.MySqlDialect.json).

<a id="op-75b128347e7235854a9a461e"></a>
## MySqlDialect

`struct` · `sqlparser::dialect::mysql::MySqlDialect` · sqlparser 0.62.0

```rust
struct MySqlDialect
```

Source: `src/dialect/mysql.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [MySQL](https://www.mysql.com/)

<a id="op-0e998279f8e322615bfc9fee"></a>
## clone

`function` · `sqlparser::dialect::mysql::MySqlDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MySqlDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 26], "end": [38, 31], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/mysql.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ba139dc3ec611650e834561"></a>
## cmp

`function` · `sqlparser::dialect::mysql::MySqlDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MySqlDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 72], "end": [38, 75], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/mysql.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c32ec5f300f0cfdc53f012ca"></a>
## default

`function` · `sqlparser::dialect::mysql::MySqlDialect::default` · sqlparser 0.62.0

```rust
fn default() -> MySqlDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 24], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/mysql.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e7e69bdd544fafc3d310a4a"></a>
## deserialize

`function` · `sqlparser::dialect::mysql::MySqlDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 56], "end": [39, 74], "filename": "src/dialect/mysql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/mysql.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5ad4b35a7aa09722cb566db"></a>
## eq

`function` · `sqlparser::dialect::mysql::MySqlDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MySqlDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 39], "end": [38, 48], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/mysql.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85ac1a9ad003d559c3604a0c"></a>
## fmt

`function` · `sqlparser::dialect::mysql::MySqlDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/mysql.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9771eeb585c5914271f1d233"></a>
## hash

`function` · `sqlparser::dialect::mysql::MySqlDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 54], "end": [38, 58], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/mysql.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e639723453010931d61656d"></a>
## identifier_quote_style

`function` · `sqlparser::dialect::mysql::MySqlDialect::identifier_quote_style` · sqlparser 0.62.0

```rust
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:66`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6d8b9b691243755662b8a92"></a>
## ignores_wildcard_escapes

`function` · `sqlparser::dialect::mysql::MySqlDialect::ignores_wildcard_escapes` · sqlparser 0.62.0

```rust
fn ignores_wildcard_escapes(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:80`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a29cbf101753432a48a1319"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::mysql::MySqlDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74dbfba8fde3933815d4fd90"></a>
## is_identifier_part

`function` · `sqlparser::dialect::mysql::MySqlDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c0f6b2a97dc26dd02ddac65"></a>
## is_identifier_start

`function` · `sqlparser::dialect::mysql::MySqlDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:43`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf710d584215f67c5b8f9ca8"></a>
## is_table_factor_alias

`function` · `sqlparser::dialect::mysql::MySqlDialect::is_table_factor_alias` · sqlparser 0.62.0

```rust
fn is_table_factor_alias(&self, explicit: bool, kw: &Keyword, _parser: &mut Parser<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:152`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-daa17f94e90acb757c34061e"></a>
## parse_infix

`function` · `sqlparser::dialect::mysql::MySqlDialect::parse_infix` · sqlparser 0.62.0

```rust
fn parse_infix(&self, parser: &mut parser::Parser<'_>, expr: &ast::Expr, _precedence: u8) -> Option<Result<ast::Expr, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:97`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-740d49404e1dd4dbfb53e22f"></a>
## parse_statement

`function` · `sqlparser::dialect::mysql::MySqlDialect::parse_statement` · sqlparser 0.62.0

```rust
fn parse_statement(&self, parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9227d987be2a23d61606ff92"></a>
## partial_cmp

`function` · `sqlparser::dialect::mysql::MySqlDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MySqlDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 60], "end": [38, 70], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/mysql.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2afd892e9afd5f779ca2e696"></a>
## require_interval_qualifier

`function` · `sqlparser::dialect::mysql::MySqlDialect::require_interval_qualifier` · sqlparser 0.62.0

```rust
fn require_interval_qualifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5a22b9ea430cca5e5fc1eeb"></a>
## requires_single_line_comment_whitespace

`function` · `sqlparser::dialect::mysql::MySqlDialect::requires_single_line_comment_whitespace` · sqlparser 0.62.0

```rust
fn requires_single_line_comment_whitespace(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2f7c3e16c93f71c99cd19bf"></a>
## serialize

`function` · `sqlparser::dialect::mysql::MySqlDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 38], "end": [39, 54], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/mysql.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28e6f7f2fe9d2d80b26ac117"></a>
## supports_binary_kw_as_cast

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_binary_kw_as_cast` · sqlparser 0.62.0

```rust
fn supports_binary_kw_as_cast(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:202`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Deprecated functionality by MySQL but still supported
See: <https://dev.mysql.com/doc/refman/8.4/en/cast-functions.html#operator_binary>

<a id="op-021c601d2864c8b5843b0935"></a>
## supports_bitwise_shift_operators

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_bitwise_shift_operators` · sqlparser 0.62.0

```rust
fn supports_bitwise_shift_operators(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:88`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1425d4d04cffd7ca3b66ff00"></a>
## supports_comma_separated_set_assignments

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_comma_separated_set_assignments` · sqlparser 0.62.0

```rust
fn supports_comma_separated_set_assignments(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f1b4d2b1e1f1a00b84c0f87"></a>
## supports_comment_optimizer_hint

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_comment_optimizer_hint` · sqlparser 0.62.0

```rust
fn supports_comment_optimizer_hint(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a780e442ffed99833982b595"></a>
## supports_constraint_keyword_without_name

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_constraint_keyword_without_name` · sqlparser 0.62.0

```rust
fn supports_constraint_keyword_without_name(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:211`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://dev.mysql.com/doc/refman/8.4/en/create-table.html>

<a id="op-6f1c9484d71ad2028b31b989"></a>
## supports_create_table_select

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_create_table_select` · sqlparser 0.62.0

```rust
fn supports_create_table_select(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://dev.mysql.com/doc/refman/8.4/en/create-table-select.html>

<a id="op-6249c2c8149b2639eadf3018"></a>
## supports_cross_join_constraint

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_cross_join_constraint` · sqlparser 0.62.0

```rust
fn supports_cross_join_constraint(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:191`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faa83c1ff1d79495c4d57139"></a>
## supports_data_type_signed_suffix

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_data_type_signed_suffix` · sqlparser 0.62.0

```rust
fn supports_data_type_signed_suffix(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:187`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd21dcde005dc7ca68d5d6cc"></a>
## supports_double_ampersand_operator

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_double_ampersand_operator` · sqlparser 0.62.0

```rust
fn supports_double_ampersand_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://dev.mysql.com/doc/refman/8.4/en/expressions.html>

<a id="op-107d5ab287b97d83fa768cae"></a>
## supports_insert_set

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_insert_set` · sqlparser 0.62.0

```rust
fn supports_insert_set(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://dev.mysql.com/doc/refman/8.4/en/insert.html>

<a id="op-ca0d7b22f06c10d23516a93b"></a>
## supports_key_column_option

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_key_column_option` · sqlparser 0.62.0

```rust
fn supports_key_column_option(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:216`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://dev.mysql.com/doc/refman/8.4/en/create-table.html>

<a id="op-09b1236969993ae88d315507"></a>
## supports_limit_comma

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_limit_comma` · sqlparser 0.62.0

```rust
fn supports_limit_comma(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:134`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e64eb891f41e1991db9623be"></a>
## supports_match_against

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_match_against` · sqlparser 0.62.0

```rust
fn supports_match_against(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30d89b33689ff17ede608fb5"></a>
## supports_multiline_comment_hints

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_multiline_comment_hints` · sqlparser 0.62.0

```rust
fn supports_multiline_comment_hints(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:93`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

see <https://dev.mysql.com/doc/refman/8.4/en/comments.html>

<a id="op-c152ba82e59284637f8cadce"></a>
## supports_numeric_prefix

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_numeric_prefix` · sqlparser 0.62.0

```rust
fn supports_numeric_prefix(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:84`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ec199be367abe041bc84e70"></a>
## supports_select_modifiers

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_select_modifiers` · sqlparser 0.62.0

```rust
fn supports_select_modifiers(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3007858a1851018f63aab9ea"></a>
## supports_set_names

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_set_names` · sqlparser 0.62.0

```rust
fn supports_set_names(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc45334f9b30153ece3b4fa4"></a>
## supports_string_literal_backslash_escape

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_string_literal_backslash_escape` · sqlparser 0.62.0

```rust
fn supports_string_literal_backslash_escape(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:71`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-188be1096b9333ce5bc4f267"></a>
## supports_string_literal_concatenation

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_string_literal_concatenation` · sqlparser 0.62.0

```rust
fn supports_string_literal_concatenation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:76`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

see <https://dev.mysql.com/doc/refman/8.4/en/string-functions.html#function_concat>

<a id="op-bc500830bb8578466b65db75"></a>
## supports_table_hints

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_table_hints` · sqlparser 0.62.0

```rust
fn supports_table_hints(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:158`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62a412ad2c2f306b2db56362"></a>
## supports_update_order_by

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_update_order_by` · sqlparser 0.62.0

```rust
fn supports_update_order_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://dev.mysql.com/doc/refman/8.4/en/update.html>

<a id="op-c324bbdc20d1ec5cc567ca80"></a>
## supports_user_host_grantee

`function` · `sqlparser::dialect::mysql::MySqlDialect::supports_user_host_grantee` · sqlparser 0.62.0

```rust
fn supports_user_host_grantee(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mysql::MySqlDialect", "path": "MySqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [219, 2], "filename": "src/dialect/mysql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mysql.rs:148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
