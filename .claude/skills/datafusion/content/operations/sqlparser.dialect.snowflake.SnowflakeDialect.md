# `sqlparser::dialect::snowflake::SnowflakeDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.snowflake.SnowflakeDialect.json).

<a id="op-10b3cb75005f3e9e32defb32"></a>
## SnowflakeDialect

`struct` · `sqlparser::dialect::snowflake::SnowflakeDialect` · sqlparser 0.62.0

```rust
struct SnowflakeDialect
```

Source: `src/dialect/snowflake.rs:135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [Snowflake](https://www.snowflake.com/)

<a id="op-efb59f331cdb760371cd56de"></a>
## allow_extract_custom

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::allow_extract_custom` · sqlparser 0.62.0

```rust
fn allow_extract_custom(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:443`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7ad2abedf4ef537132ac6e0"></a>
## allow_extract_single_quotes

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::allow_extract_single_quotes` · sqlparser 0.62.0

```rust
fn allow_extract_single_quotes(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:447`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bab1e26dfc96cd8f38d99338"></a>
## clone

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SnowflakeDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 26], "end": [133, 31], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/snowflake.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06fb28653bbf524e9e50328d"></a>
## cmp

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SnowflakeDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 72], "end": [133, 75], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/snowflake.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdcbb58c854c6e34e9812fb4"></a>
## default

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::default` · sqlparser 0.62.0

```rust
fn default() -> SnowflakeDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 17], "end": [133, 24], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/snowflake.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e448afd300b2b7ae417895af"></a>
## describe_requires_table_keyword

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::describe_requires_table_keyword` · sqlparser 0.62.0

```rust
fn describe_requires_table_keyword(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14e91334cc8e37990533b40b"></a>
## deserialize

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 56], "end": [134, 74], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/snowflake.rs:134`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf76e8549646692674abbfc9"></a>
## eq

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SnowflakeDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 39], "end": [133, 48], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/snowflake.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-057ac095ce444bca51f69e62"></a>
## fmt

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 10], "end": [133, 15], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/snowflake.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8df814a1068a4189814d6380"></a>
## get_next_precedence

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::get_next_precedence` · sqlparser 0.62.0

```rust
fn get_next_precedence(&self, parser: &Parser<'_>) -> Option<Result<u8, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a36e8a4325ad3926ccf8fee"></a>
## get_reserved_keywords_for_select_item_operator

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::get_reserved_keywords_for_select_item_operator` · sqlparser 0.62.0

```rust
fn get_reserved_keywords_for_select_item_operator(&self) -> &[Keyword]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:625`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://docs.snowflake.com/en/sql-reference/constructs/connect-by>

<a id="op-78c84a3ba21b3caf07fe065c"></a>
## hash

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 54], "end": [133, 58], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/snowflake.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9cf3bc78d7394eacb1e2ac1"></a>
## is_column_alias

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::is_column_alias` · sqlparser 0.62.0

```rust
fn is_column_alias(&self, kw: &Keyword, parser: &mut Parser<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-675d5efe806e31dc6f535e31"></a>
## is_identifier_generating_function_name

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::is_identifier_generating_function_name` · sqlparser 0.62.0

```rust
fn is_identifier_generating_function_name(&self, ident: &Ident, name_parts: &[ObjectNamePart]) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfedec87dfc6975209c7538e"></a>
## is_identifier_part

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6c7e59a0d1ba26ab8cd6f32"></a>
## is_identifier_start

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dfdc20c9f6fb96a8901d730"></a>
## is_reserved_for_identifier

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::is_reserved_for_identifier` · sqlparser 0.62.0

```rust
fn is_reserved_for_identifier(&self, kw: Keyword) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41120c497be99e6c7bf7d646"></a>
## is_table_alias

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::is_table_alias` · sqlparser 0.62.0

```rust
fn is_table_alias(&self, kw: &Keyword, parser: &mut Parser<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:521`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74df995fb5f94839d65ef2b3"></a>
## is_table_factor

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::is_table_factor` · sqlparser 0.62.0

```rust
fn is_table_factor(&self, kw: &Keyword, parser: &mut Parser<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:605`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-576e56aa012524851a863c2a"></a>
## parse_column_option

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::parse_column_option` · sqlparser 0.62.0

```rust
fn parse_column_option(&self, parser: &mut Parser<'_>) -> Result<Option<Result<Option<ColumnOption>, ParserError>>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:400`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72e3274e767ff7c41bcf5458"></a>
## parse_statement

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::parse_statement` · sqlparser 0.62.0

```rust
fn parse_statement(&self, parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:248`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18716d23c21114e07634381f"></a>
## partial_cmp

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SnowflakeDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 60], "end": [133, 70], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/snowflake.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1478b39688bfb81271a623cc"></a>
## serialize

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 38], "end": [134, 54], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/snowflake.rs:134`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-471ddcc6fb8ee9f225a518dd"></a>
## supports_array_typedef_without_element_type

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_array_typedef_without_element_type` · sqlparser 0.62.0

```rust
fn supports_array_typedef_without_element_type(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:234`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://docs.snowflake.com/en/sql-reference/data-types-semistructured#array)

<a id="op-ca1d12b0a9394652959467f4"></a>
## supports_comma_separated_drop_column_list

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_comma_separated_drop_column_list` · sqlparser 0.62.0

```rust
fn supports_comma_separated_drop_column_list(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fadb8d987be0d4b26e44ce09"></a>
## supports_comma_separated_trim

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_comma_separated_trim` · sqlparser 0.62.0

```rust
fn supports_comma_separated_trim(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e7fff806d11dc430c19c2b9"></a>
## supports_comment_on

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_comment_on` · sqlparser 0.62.0

```rust
fn supports_comment_on(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:214`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://docs.snowflake.com/en/sql-reference/sql/comment)

<a id="op-00459cb1eeb80f5498e5c8a2"></a>
## supports_connect_by

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_connect_by` · sqlparser 0.62.0

```rust
fn supports_connect_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7a4c8b4bbdf8063eb4c953f"></a>
## supports_create_view_comment_syntax

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_create_view_comment_syntax` · sqlparser 0.62.0

```rust
fn supports_create_view_comment_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://docs.snowflake.com/en/sql-reference/sql/create-view#optional-parameters)

<a id="op-51d77db7c72835a8e8985164"></a>
## supports_dictionary_syntax

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_dictionary_syntax` · sqlparser 0.62.0

```rust
fn supports_dictionary_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:198`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7da0c1d4b3974ba826811165"></a>
## supports_execute_immediate

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_execute_immediate` · sqlparser 0.62.0

```rust
fn supports_execute_immediate(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:186`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/sql/execute-immediate>

<a id="op-80da9a7f0e1c4177d65ff284"></a>
## supports_extract_comma_syntax

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_extract_comma_syntax` · sqlparser 0.62.0

```rust
fn supports_extract_comma_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://docs.snowflake.com/en/sql-reference/functions/extract)

<a id="op-78f8724b0d0327c8cbeb2b79"></a>
## supports_from_trailing_commas

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_from_trailing_commas` · sqlparser 0.62.0

```rust
fn supports_from_trailing_commas(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1a34d8f486514437ed114e5"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://docs.snowflake.com/en/sql-reference/constructs/group-by>

<a id="op-e0ea26b96a15e07e4e04737f"></a>
## supports_lambda_functions

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_lambda_functions` · sqlparser 0.62.0

```rust
fn supports_lambda_functions(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/user-guide/querying-semistructured#label-higher-order-functions>

<a id="op-5b92fa88fc6f6406101bb094"></a>
## supports_left_associative_joins_without_parens

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_left_associative_joins_without_parens` · sqlparser 0.62.0

```rust
fn supports_left_associative_joins_without_parens(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0407da0fa6b167bf9d06fa0"></a>
## supports_match_recognize

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_match_recognize` · sqlparser 0.62.0

```rust
fn supports_match_recognize(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-153d21faf5be8706f6ca4b76"></a>
## supports_object_name_double_dot_notation

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_object_name_double_dot_notation` · sqlparser 0.62.0

```rust
fn supports_object_name_double_dot_notation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cec017d480c135254d860fe"></a>
## supports_outer_join_operator

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_outer_join_operator` · sqlparser 0.62.0

```rust
fn supports_outer_join_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:177`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/constructs/where#joins-in-the-where-clause>

<a id="op-b1fc6213c67e6b6e2746f0ae"></a>
## supports_parens_around_table_factor

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_parens_around_table_factor` · sqlparser 0.62.0

```rust
fn supports_parens_around_table_factor(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://docs.snowflake.com/en/sql-reference/constructs/from)

<a id="op-49c7686308419bdb7ad2b587"></a>
## supports_parenthesized_set_variables

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_parenthesized_set_variables` · sqlparser 0.62.0

```rust
fn supports_parenthesized_set_variables(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:209`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://docs.snowflake.com/en/sql-reference/sql/set#syntax)

<a id="op-1e2e487605addc5912bb1b48"></a>
## supports_partiql

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_partiql` · sqlparser 0.62.0

```rust
fn supports_partiql(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d68362d4b7df6409ec0cd156"></a>
## supports_projection_trailing_commas

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_projection_trailing_commas` · sqlparser 0.62.0

```rust
fn supports_projection_trailing_commas(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:143`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c7c2cb649f38eea684d727e"></a>
## supports_select_expr_star

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_select_expr_star` · sqlparser 0.62.0

```rust
fn supports_select_expr_star(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bb59590ba7b856e1642b585"></a>
## supports_select_wildcard_exclude

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_select_wildcard_exclude` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_exclude(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5e9875029bac6c6bbed61eb"></a>
## supports_select_wildcard_ilike

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_select_wildcard_ilike` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_ilike(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:668`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a id="op-c3a71e1df0839de260609f98"></a>
## supports_select_wildcard_rename

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_select_wildcard_rename` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_rename(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a id="op-7c3b7827c7bdb471f4d68a08"></a>
## supports_select_wildcard_replace

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_select_wildcard_replace` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_replace(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a id="op-28f0784a3f84d09d0d339182"></a>
## supports_semantic_view_table_factor

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_semantic_view_table_factor` · sqlparser 0.62.0

```rust
fn supports_semantic_view_table_factor(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32ed01957b6e38a65b15f1a4"></a>
## supports_show_like_before_in

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_show_like_before_in` · sqlparser 0.62.0

```rust
fn supports_show_like_before_in(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake expects the `LIKE` option before the `IN` option,
for example: <https://docs.snowflake.com/en/sql-reference/sql/show-views#syntax>

<a id="op-32a57160bf8b0b9a092043a3"></a>
## supports_space_separated_column_options

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_space_separated_column_options` · sqlparser 0.62.0

```rust
fn supports_space_separated_column_options(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:629`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d56569d1139a1da924d0a8da"></a>
## supports_string_literal_backslash_escape

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_string_literal_backslash_escape` · sqlparser 0.62.0

```rust
fn supports_string_literal_backslash_escape(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcccd4d39a4a43b8f41a0058"></a>
## supports_subquery_as_function_arg

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_subquery_as_function_arg` · sqlparser 0.62.0

```rust
fn supports_subquery_as_function_arg(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:224`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://docs.snowflake.com/en/sql-reference/functions/flatten)

<a id="op-707daffe25ff76f533cd7ac9"></a>
## supports_table_versioning

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_table_versioning` · sqlparser 0.62.0

```rust
fn supports_table_versioning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:615`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See: <https://docs.snowflake.com/en/sql-reference/constructs/at-before>

<a id="op-71a6b22b1e22519dfef54afb"></a>
## supports_values_as_table_factor

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_values_as_table_factor` · sqlparser 0.62.0

```rust
fn supports_values_as_table_factor(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://docs.snowflake.com/en/sql-reference/constructs/values)

<a id="op-145dbf4c1a27625d470529d4"></a>
## supports_window_function_null_treatment_arg

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_window_function_null_treatment_arg` · sqlparser 0.62.0

```rust
fn supports_window_function_null_treatment_arg(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8a1db607e554db315033369"></a>
## supports_within_after_array_aggregation

`function` · `sqlparser::dialect::snowflake::SnowflakeDialect::supports_within_after_array_aggregation` · sqlparser 0.62.0

```rust
fn supports_within_after_array_aggregation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::snowflake::SnowflakeDialect", "path": "SnowflakeDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [685, 2], "filename": "src/dialect/snowflake.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/snowflake.rs:172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
