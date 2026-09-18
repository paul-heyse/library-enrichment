# `sqlparser::dialect::postgresql::PostgreSqlDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.postgresql.PostgreSqlDialect.json).

<a id="op-39fe295d5fdf505b9c9457a2"></a>
## PostgreSqlDialect

`struct` · `sqlparser::dialect::postgresql::PostgreSqlDialect` · sqlparser 0.62.0

```rust
struct PostgreSqlDialect
```

Source: `src/dialect/postgresql.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [PostgreSQL](https://www.postgresql.org/)

<a id="op-2cb5d40017d2dae4419cecfb"></a>
## allow_extract_custom

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::allow_extract_custom` · sqlparser 0.62.0

```rust
fn allow_extract_custom(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:187`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e00b0882104a65d95d4f332"></a>
## allow_extract_single_quotes

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::allow_extract_single_quotes` · sqlparser 0.62.0

```rust
fn allow_extract_single_quotes(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:191`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58cc1f031a929d25d0bdb65f"></a>
## clone

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> PostgreSqlDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 26], "end": [39, 31], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/postgresql.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1336fc18b9a1a61c18b50514"></a>
## cmp

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &PostgreSqlDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 72], "end": [39, 75], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/postgresql.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9367785e6d282968d639f0bb"></a>
## default

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::default` · sqlparser 0.62.0

```rust
fn default() -> PostgreSqlDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 24], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/postgresql.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3ebc35e631ec74916d61b44"></a>
## deserialize

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 56], "end": [40, 74], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/postgresql.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccba332f196bf1662c90f638"></a>
## eq

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &PostgreSqlDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 39], "end": [39, 48], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/postgresql.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54ad01cdadd65c55b88e3346"></a>
## fmt

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/postgresql.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8944329b04cc4c6e46ee98e3"></a>
## get_next_precedence

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::get_next_precedence` · sqlparser 0.62.0

```rust
fn get_next_precedence(&self, parser: &Parser<'_>) -> Option<Result<u8, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fe63bc50476c60b9af8da09"></a>
## hash

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 54], "end": [39, 58], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/postgresql.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e69586a8cbfc82e7e99fd8a"></a>
## identifier_quote_style

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::identifier_quote_style` · sqlparser 0.62.0

```rust
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3eeeca1fe19d2985ff94dbc"></a>
## is_custom_operator_part

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::is_custom_operator_part` · sqlparser 0.62.0

```rust
fn is_custom_operator_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:95`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://www.postgresql.org/docs/current/sql-createoperator.html>

<a id="op-ca8e7f34f5653db6281257c1"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:66`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e50d2db744d4f8f99db3d71"></a>
## is_identifier_part

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:76`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b793aef065cfedd163d6624"></a>
## is_identifier_start

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:70`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a42a21abb2898fda55f72084"></a>
## is_reserved_for_identifier

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::is_reserved_for_identifier` · sqlparser 0.62.0

```rust
fn is_reserved_for_identifier(&self, kw: Keyword) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:86`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76123a7ed10d3d13f6d33056"></a>
## partial_cmp

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &PostgreSqlDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 60], "end": [39, 70], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/postgresql.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-157cb1de4a2cbe86cded64f0"></a>
## prec_value

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::prec_value` · sqlparser 0.62.0

```rust
fn prec_value(&self, prec: Precedence) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5fa1865226d0a29bc342ca2"></a>
## serialize

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 38], "end": [40, 54], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/postgresql.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfe7f3051262308b7238094c"></a>
## supports_alter_column_type_using

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_alter_column_type_using` · sqlparser 0.62.0

```rust
fn supports_alter_column_type_using(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:285`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8ff36fcdfbfa5a9281843fc"></a>
## supports_array_typedef_with_brackets

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_array_typedef_with_brackets` · sqlparser 0.62.0

```rust
fn supports_array_typedef_with_brackets(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:273`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://www.postgresql.org/docs/current/arrays.html#ARRAYS-DECLARATION>

<a id="op-61522901b69bf4b5bb6aa23c"></a>
## supports_bitwise_shift_operators

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_bitwise_shift_operators` · sqlparser 0.62.0

```rust
fn supports_bitwise_shift_operators(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:216`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf9e65aa51ab497200eb6dd4"></a>
## supports_comma_separated_trim

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_comma_separated_trim` · sqlparser 0.62.0

```rust
fn supports_comma_separated_trim(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a9b8c61880670fc8ab5ea4b"></a>
## supports_comment_on

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_comment_on` · sqlparser 0.62.0

```rust
fn supports_comment_on(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:221`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

see <https://www.postgresql.org/docs/current/sql-comment.html>

<a id="op-dba6511b1f778b9d260cafda"></a>
## supports_comment_optimizer_hint

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_comment_optimizer_hint` · sqlparser 0.62.0

```rust
fn supports_comment_optimizer_hint(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:326`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Postgres supports query optimizer hints via the `pg_hint_plan` extension,
using the same comment-prefixed-with-`+` syntax as MySQL and Oracle.

See <https://github.com/ossc-db/pg_hint_plan>

<a id="op-aec21f3043f212ebb32962d8"></a>
## supports_create_index_with_clause

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_create_index_with_clause` · sqlparser 0.62.0

```rust
fn supports_create_index_with_clause(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ea319f9e511e0b4b3f6c492"></a>
## supports_create_table_like_parenthesized

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_create_table_like_parenthesized` · sqlparser 0.62.0

```rust
fn supports_create_table_like_parenthesized(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4880e5c8013dcc3d8ea7844"></a>
## supports_empty_projections

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_empty_projections` · sqlparser 0.62.0

```rust
fn supports_empty_projections(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return true if the dialect supports empty projections in SELECT statements

Example
```sql
SELECT from table_name
```

<a id="op-847e49c20c165f69e9a74738"></a>
## supports_explain_with_utility_options

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_explain_with_utility_options` · sqlparser 0.62.0

```rust
fn supports_explain_with_utility_options(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

see <https://www.postgresql.org/docs/current/sql-explain.html>

<a id="op-dae00268de61fb1314962340"></a>
## supports_factorial_operator

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_factorial_operator` · sqlparser 0.62.0

```rust
fn supports_factorial_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

see <https://www.postgresql.org/docs/13/functions-math.html>

<a id="op-e223ce8450c2966b657945ce"></a>
## supports_filter_during_aggregation

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_filter_during_aggregation` · sqlparser 0.62.0

```rust
fn supports_filter_during_aggregation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-869df205b65b13ffcd502688"></a>
## supports_geometric_types

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_geometric_types` · sqlparser 0.62.0

```rust
fn supports_geometric_types(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:277`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39cf331a32ef8f8c9382580a"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:160`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2667e8542d0666f4fd8ffdb"></a>
## supports_insert_table_alias

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_insert_table_alias` · sqlparser 0.62.0

```rust
fn supports_insert_table_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d7aa912c3c3e5154856cd5a"></a>
## supports_interval_options

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_interval_options` · sqlparser 0.62.0

```rust
fn supports_interval_options(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:298`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[Postgres] supports optional field and precision options for `INTERVAL` data type.

[Postgres]: https://www.postgresql.org/docs/17/datatype-datetime.html

<a id="op-8d1d6b0a39a8d972088b7159"></a>
## supports_listen_notify

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_listen_notify` · sqlparser 0.62.0

```rust
fn supports_listen_notify(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:207`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

see <https://www.postgresql.org/docs/current/sql-listen.html>
see <https://www.postgresql.org/docs/current/sql-unlisten.html>
see <https://www.postgresql.org/docs/current/sql-notify.html>

<a id="op-3c235b739c4a8b6306d96fe6"></a>
## supports_load_extension

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_load_extension` · sqlparser 0.62.0

```rust
fn supports_load_extension(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://www.postgresql.org/docs/current/sql-load.html>

<a id="op-54c1cc6afd234cf170f02b6c"></a>
## supports_named_fn_args_with_colon_operator

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_named_fn_args_with_colon_operator` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_colon_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://www.postgresql.org/docs/current/functions-json.html>

Required to support the colon in:
```sql
SELECT json_object('a': 'b')
```

<a id="op-edc040b2e6a84636546d2ce0"></a>
## supports_named_fn_args_with_expr_name

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_named_fn_args_with_expr_name` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_expr_name(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:246`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://www.postgresql.org/docs/current/functions-json.html>

Required to support the label in:
```sql
SELECT json_object('label': 'value')
```

<a id="op-985c2c3316fe092790b35ab8"></a>
## supports_nested_comments

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_nested_comments` · sqlparser 0.62.0

```rust
fn supports_nested_comments(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10f5191fd04389313ec2f902"></a>
## supports_notnull_operator

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_notnull_operator` · sqlparser 0.62.0

```rust
fn supports_notnull_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Postgres supports `NOTNULL` as an alias for `IS NOT NULL`
See: <https://www.postgresql.org/docs/17/functions-comparison.html>

<a id="op-be4c7f050afb41a72abd1eed"></a>
## supports_numeric_literal_underscores

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_numeric_literal_underscores` · sqlparser 0.62.0

```rust
fn supports_numeric_literal_underscores(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b1321e8b224913ade18871b"></a>
## supports_select_wildcard_with_alias

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_select_wildcard_with_alias` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_with_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d150027f49fbf7bd7925cd07"></a>
## supports_set_names

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_set_names` · sqlparser 0.62.0

```rust
fn supports_set_names(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:281`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e58de333c276258d93075c9"></a>
## supports_string_escape_constant

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_string_escape_constant` · sqlparser 0.62.0

```rust
fn supports_string_escape_constant(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28866956002220ee04195c45"></a>
## supports_unicode_string_literal

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_unicode_string_literal` · sqlparser 0.62.0

```rust
fn supports_unicode_string_literal(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:82`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4c7dbe0a30ee2bb26d13d15"></a>
## supports_xml_expressions

`function` · `sqlparser::dialect::postgresql::PostgreSqlDialect::supports_xml_expressions` · sqlparser 0.62.0

```rust
fn supports_xml_expressions(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::postgresql::PostgreSqlDialect", "path": "PostgreSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [329, 2], "filename": "src/dialect/postgresql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/postgresql.rs:318`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
