# `sqlparser::dialect::mssql::MsSqlDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.mssql.MsSqlDialect.json).

<a id="op-24ee79ebd1c19d12454e0bf6"></a>
## MsSqlDialect

`struct` · `sqlparser::dialect::mssql::MsSqlDialect` · sqlparser 0.62.0

```rust
struct MsSqlDialect
```

Source: `src/dialect/mssql.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [Microsoft SQL Server](https://www.microsoft.com/en-us/sql-server/)

<a id="op-e2c1f71c66913e6820a967a0"></a>
## clone

`function` · `sqlparser::dialect::mssql::MsSqlDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MsSqlDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 26], "end": [31, 31], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/mssql.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93cd3444606ccbbedfb55e77"></a>
## cmp

`function` · `sqlparser::dialect::mssql::MsSqlDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MsSqlDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 72], "end": [31, 75], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/mssql.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db01b4ea64ad707f1b311bfa"></a>
## convert_type_before_value

`function` · `sqlparser::dialect::mssql::MsSqlDialect::convert_type_before_value` · sqlparser 0.62.0

```rust
fn convert_type_before_value(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL Server has `CONVERT(type, value)` instead of `CONVERT(value, type)`
<https://learn.microsoft.com/en-us/sql/t-sql/functions/cast-and-convert-transact-sql?view=sql-server-ver16>

<a id="op-230b1a8bdff2fa138b84722b"></a>
## default

`function` · `sqlparser::dialect::mssql::MsSqlDialect::default` · sqlparser 0.62.0

```rust
fn default() -> MsSqlDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 24], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/mssql.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70e51d1101b12e54756e8720"></a>
## deserialize

`function` · `sqlparser::dialect::mssql::MsSqlDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 56], "end": [32, 74], "filename": "src/dialect/mssql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/mssql.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f8f349f7e44163ada973abc"></a>
## eq

`function` · `sqlparser::dialect::mssql::MsSqlDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MsSqlDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 39], "end": [31, 48], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/mssql.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-385096d632125ca62d90bc51"></a>
## fmt

`function` · `sqlparser::dialect::mssql::MsSqlDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/mssql.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3e43bb6812c3b8098e1b212"></a>
## get_next_precedence

`function` · `sqlparser::dialect::mssql::MsSqlDialect::get_next_precedence` · sqlparser 0.62.0

```rust
fn get_next_precedence(&self, parser: &Parser<'_>) -> Option<Result<u8, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c455212db6a8dcda4b39335"></a>
## get_reserved_grantees_types

`function` · `sqlparser::dialect::mssql::MsSqlDialect::get_reserved_grantees_types` · sqlparser 0.62.0

```rust
fn get_reserved_grantees_types(&self) -> &[GranteesType]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:132`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://learn.microsoft.com/en-us/sql/relational-databases/security/authentication-access/server-level-roles>

<a id="op-188e864b87a3b5a5fedc22bd"></a>
## hash

`function` · `sqlparser::dialect::mssql::MsSqlDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 54], "end": [31, 58], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/mssql.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a89a5ed5f4112462e498bb8"></a>
## identifier_quote_style

`function` · `sqlparser::dialect::mssql::MsSqlDialect::identifier_quote_style` · sqlparser 0.62.0

```rust
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caa4902b7d29a4b7b156fc92"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::mssql::MsSqlDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d27eff9f5b7165612fc318dc"></a>
## is_identifier_part

`function` · `sqlparser::dialect::mssql::MsSqlDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3a4db69bc0be607232c05d1"></a>
## is_identifier_start

`function` · `sqlparser::dialect::mssql::MsSqlDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5c1c68ec0819fdbf6f559d7"></a>
## is_select_item_alias

`function` · `sqlparser::dialect::mssql::MsSqlDialect::is_select_item_alias` · sqlparser 0.62.0

```rust
fn is_select_item_alias(&self, explicit: bool, kw: &Keyword, parser: &mut Parser<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f68cb88d87e2c572e129d4d0"></a>
## is_table_factor_alias

`function` · `sqlparser::dialect::mssql::MsSqlDialect::is_table_factor_alias` · sqlparser 0.62.0

```rust
fn is_table_factor_alias(&self, explicit: bool, kw: &Keyword, parser: &mut Parser<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02872b1bb34da6a3240c76e8"></a>
## parse_statement

`function` · `sqlparser::dialect::mssql::MsSqlDialect::parse_statement` · sqlparser 0.62.0

```rust
fn parse_statement(&self, parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c8e6da2b236aa2093ab7016"></a>
## partial_cmp

`function` · `sqlparser::dialect::mssql::MsSqlDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MsSqlDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 60], "end": [31, 70], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/mssql.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50f8526c180acd295869a323"></a>
## serialize

`function` · `sqlparser::dialect::mssql::MsSqlDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 38], "end": [32, 54], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/mssql.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2fbd0610205c3dad4e8ca0e"></a>
## supports_boolean_literals

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_boolean_literals` · sqlparser 0.62.0

```rust
fn supports_boolean_literals(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

In MSSQL, there is no boolean type, and `true` and `false` are valid column names

<a id="op-fd5e524f68938e35c8e24cee"></a>
## supports_connect_by

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_connect_by` · sqlparser 0.62.0

```rust
fn supports_connect_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:74`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bdf8dae3b2b8e09b2915b0e"></a>
## supports_dollar_as_money_prefix

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_dollar_as_money_prefix` · sqlparser 0.62.0

```rust
fn supports_dollar_as_money_prefix(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:70`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL Server supports `$` as a prefix for money literals
<https://learn.microsoft.com/en-us/sql/t-sql/data-types/constants-transact-sql?view=sql-server-ver17#money-constants>

<a id="op-091d763f9da61a0ebae7cafb"></a>
## supports_end_transaction_modifier

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_end_transaction_modifier` · sqlparser 0.62.0

```rust
fn supports_end_transaction_modifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:107`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac8a08f3e1b67b97df0433d6"></a>
## supports_eq_alias_assignment

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_eq_alias_assignment` · sqlparser 0.62.0

```rust
fn supports_eq_alias_assignment(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:78`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21c4bf4e8d2f02d8dc23076c"></a>
## supports_named_fn_args_with_colon_operator

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_named_fn_args_with_colon_operator` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_colon_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:91`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e2b12e9f1e19b10f39bda27"></a>
## supports_named_fn_args_with_expr_name

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_named_fn_args_with_expr_name` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_expr_name(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:95`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b16803fdc0d90a9731c027e0"></a>
## supports_named_fn_args_with_rarrow_operator

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_named_fn_args_with_rarrow_operator` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_rarrow_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:99`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00afa0ed18966c8b2b7135af"></a>
## supports_nested_comments

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_nested_comments` · sqlparser 0.62.0

```rust
fn supports_nested_comments(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/slash-star-comment-transact-sql?view=sql-server-ver16>

<a id="op-88f4b38ea953a5fe02179814"></a>
## supports_object_name_double_dot_notation

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_object_name_double_dot_notation` · sqlparser 0.62.0

```rust
fn supports_object_name_double_dot_notation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:127`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://learn.microsoft.com/en-us/sql/t-sql/queries/from-transact-sql>

<a id="op-9435e36c94f6f8319655e4a1"></a>
## supports_outer_join_operator

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_outer_join_operator` · sqlparser 0.62.0

```rust
fn supports_outer_join_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a814a1209e2e660732aa13b4"></a>
## supports_set_stmt_without_operator

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_set_stmt_without_operator` · sqlparser 0.62.0

```rust
fn supports_set_stmt_without_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://learn.microsoft.com/en-us/sql/t-sql/statements/set-statements-transact-sql>

<a id="op-884d489691573f4566b6aeba"></a>
## supports_start_transaction_modifier

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_start_transaction_modifier` · sqlparser 0.62.0

```rust
fn supports_start_transaction_modifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b99a073821774e7b09a50d0b"></a>
## supports_table_versioning

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_table_versioning` · sqlparser 0.62.0

```rust
fn supports_table_versioning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://learn.microsoft.com/en-us/sql/relational-databases/tables/querying-data-in-a-system-versioned-temporal-table>

<a id="op-0a1525e1a3f52171ec1ceb0b"></a>
## supports_try_convert

`function` · `sqlparser::dialect::mssql::MsSqlDialect::supports_try_convert` · sqlparser 0.62.0

```rust
fn supports_try_convert(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::mssql::MsSqlDialect", "path": "MsSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [251, 2], "filename": "src/dialect/mssql.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/mssql.rs:82`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
