# `sqlparser::dialect::oracle::OracleDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.oracle.OracleDialect.json).

<a id="op-dc3af44003351feced7fd59d"></a>
## OracleDialect

`struct` · `sqlparser::dialect::oracle::OracleDialect` · sqlparser 0.62.0

```rust
struct OracleDialect
```

Source: `src/dialect/oracle.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [Oracle Databases](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/index.html)

<a id="op-f37e03d00b9aa6a5e6ff33e5"></a>
## clone

`function` · `sqlparser::dialect::oracle::OracleDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OracleDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 26], "end": [30, 31], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/oracle.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec5aff11cfb17fa1acbff59a"></a>
## cmp

`function` · `sqlparser::dialect::oracle::OracleDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OracleDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 72], "end": [30, 75], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/oracle.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b384a0aaee3a66a9d55fb096"></a>
## default

`function` · `sqlparser::dialect::oracle::OracleDialect::default` · sqlparser 0.62.0

```rust
fn default() -> OracleDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 24], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/oracle.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-636892d063bc805e33084294"></a>
## deserialize

`function` · `sqlparser::dialect::oracle::OracleDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 56], "end": [31, 74], "filename": "src/dialect/oracle.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/oracle.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5d979e540d91379c2e8fc0a"></a>
## eq

`function` · `sqlparser::dialect::oracle::OracleDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OracleDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 39], "end": [30, 48], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/oracle.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecc125954a64641131d6033f"></a>
## fmt

`function` · `sqlparser::dialect::oracle::OracleDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/oracle.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8154972b705e131c96b5f0ce"></a>
## get_next_precedence

`function` · `sqlparser::dialect::oracle::OracleDialect::get_next_precedence` · sqlparser 0.62.0

```rust
fn get_next_precedence(&self, parser: &Parser<'_>) -> Option<Result<u8, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:88`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6e7ec1dac14aa3eeed68ab1"></a>
## get_reserved_keywords_for_select_item_operator

`function` · `sqlparser::dialect::oracle::OracleDialect::get_reserved_keywords_for_select_item_operator` · sqlparser 0.62.0

```rust
fn get_reserved_keywords_for_select_item_operator(&self) -> &[Keyword]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57253c6a72a209d535148dd5"></a>
## hash

`function` · `sqlparser::dialect::oracle::OracleDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 54], "end": [30, 58], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/oracle.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91d69eb6452541920f42b678"></a>
## identifier_quote_style

`function` · `sqlparser::dialect::oracle::OracleDialect::identifier_quote_style` · sqlparser 0.62.0

```rust
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fe31e262fdda9371fea8b8b"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::oracle::OracleDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f55a9f9179715defed6d3f4"></a>
## is_identifier_part

`function` · `sqlparser::dialect::oracle::OracleDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:48`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa2e96f9fa62a31b80916cae"></a>
## is_identifier_start

`function` · `sqlparser::dialect::oracle::OracleDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:44`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b3aa7a1bde44820f2086d76"></a>
## partial_cmp

`function` · `sqlparser::dialect::oracle::OracleDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OracleDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 60], "end": [30, 70], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/oracle.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5997da5fb2bc4112c4c393fa"></a>
## serialize

`function` · `sqlparser::dialect::oracle::OracleDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 38], "end": [31, 54], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/oracle.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92f218b83b6ad5f599e9bf36"></a>
## supports_boolean_literals

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_boolean_literals` · sqlparser 0.62.0

```rust
fn supports_boolean_literals(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92c81b518add21b8839815a3"></a>
## supports_comment_on

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_comment_on` · sqlparser 0.62.0

```rust
fn supports_comment_on(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:76`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59932cf734d904909e66689a"></a>
## supports_comment_optimizer_hint

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_comment_optimizer_hint` · sqlparser 0.62.0

```rust
fn supports_comment_optimizer_hint(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:110`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bdbac7b620bd96b643a22ef"></a>
## supports_connect_by

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_connect_by` · sqlparser 0.62.0

```rust
fn supports_connect_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4380dd857aeb95710d2bf039"></a>
## supports_create_table_select

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_create_table_select` · sqlparser 0.62.0

```rust
fn supports_create_table_select(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:80`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17d85f35aa25f4b528561f98"></a>
## supports_execute_immediate

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_execute_immediate` · sqlparser 0.62.0

```rust
fn supports_execute_immediate(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80c7059c2f4c0aec509e9b43"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-feafaf596be0bd71389c4871"></a>
## supports_insert_table_alias

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_insert_table_alias` · sqlparser 0.62.0

```rust
fn supports_insert_table_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b26acc7c657dc58ef87dac36"></a>
## supports_insert_table_query

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_insert_table_query` · sqlparser 0.62.0

```rust
fn supports_insert_table_query(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/INSERT.html#GUID-903F8043-0254-4EE9-ACC1-CB8AC0AF3423__I2126242>

<a id="op-894cdd547c884f74ded44587"></a>
## supports_match_recognize

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_match_recognize` · sqlparser 0.62.0

```rust
fn supports_match_recognize(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9005babad007221956075c7e"></a>
## supports_outer_join_operator

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_outer_join_operator` · sqlparser 0.62.0

```rust
fn supports_outer_join_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:52`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0221bef3d94b704e4de70d8"></a>
## supports_quote_delimited_string

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_quote_delimited_string` · sqlparser 0.62.0

```rust
fn supports_quote_delimited_string(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:106`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d1c178cee5c9a5e05661b5d"></a>
## supports_set_stmt_without_operator

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_set_stmt_without_operator` · sqlparser 0.62.0

```rust
fn supports_set_stmt_without_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:84`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bcd2adf69658620b4ecf4d3"></a>
## supports_window_function_null_treatment_arg

`function` · `sqlparser::dialect::oracle::OracleDialect::supports_window_function_null_treatment_arg` · sqlparser 0.62.0

```rust
fn supports_window_function_null_treatment_arg(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::oracle::OracleDialect", "path": "OracleDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [122, 2], "filename": "src/dialect/oracle.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/oracle.rs:68`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
