# `sqlparser::dialect::clickhouse::ClickHouseDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.clickhouse.ClickHouseDialect.json).

<a id="op-6cd9a6eea119cd7df12dfe44"></a>
## ClickHouseDialect

`struct` · `sqlparser::dialect::clickhouse::ClickHouseDialect` · sqlparser 0.62.0

```rust
struct ClickHouseDialect
```

Source: `src/dialect/clickhouse.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [ClickHouse](https://clickhouse.com/).

<a id="op-05a2dc9508e2bee0bab6959b"></a>
## clone

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ClickHouseDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 26], "end": [21, 31], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/clickhouse.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81736387bca1db39d6153964"></a>
## cmp

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ClickHouseDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 72], "end": [21, 75], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/clickhouse.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d2eb0a72b2db312dfd493f1"></a>
## default

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::default` · sqlparser 0.62.0

```rust
fn default() -> ClickHouseDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 17], "end": [21, 24], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/clickhouse.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-194261e16a25821ce23607dd"></a>
## describe_requires_table_keyword

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::describe_requires_table_keyword` · sqlparser 0.62.0

```rust
fn describe_requires_table_keyword(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:43`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dce505d58a9dfeeebedb6e44"></a>
## deserialize

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 56], "end": [22, 74], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/clickhouse.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa3c0b337be57d576d9e02c3"></a>
## eq

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ClickHouseDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 39], "end": [21, 48], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/clickhouse.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ef67bc59a5fe2548f1b3e97"></a>
## fmt

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 10], "end": [21, 15], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/clickhouse.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8515613292dc7d57121ce7ba"></a>
## hash

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 54], "end": [21, 58], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/clickhouse.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1894cce79bb6688e1eb534b"></a>
## is_identifier_part

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc6122fdcb32b6c5a5b867f1"></a>
## is_identifier_start

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:26`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03c4c8df8c3029d658a13d0c"></a>
## partial_cmp

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ClickHouseDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 60], "end": [21, 70], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/clickhouse.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adaadca8cb57bf8e676d66ec"></a>
## require_interval_qualifier

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::require_interval_qualifier` · sqlparser 0.62.0

```rust
fn require_interval_qualifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:47`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46fae962c11552d180a51d43"></a>
## serialize

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 38], "end": [22, 54], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/clickhouse.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4c38229ba966ff8d4d88eb6"></a>
## supports_array_join_syntax

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_array_join_syntax` · sqlparser 0.62.0

```rust
fn supports_array_join_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:71`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c74eaf83eb0d5c79ef6cb156"></a>
## supports_comma_separated_trim

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_comma_separated_trim` · sqlparser 0.62.0

```rust
fn supports_comma_separated_trim(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-492e66d2880c13a983caeafa"></a>
## supports_dictionary_syntax

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_dictionary_syntax` · sqlparser 0.62.0

```rust
fn supports_dictionary_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:79`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39c504d14a77dd1d17eacb3e"></a>
## supports_from_first_select

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_from_first_select` · sqlparser 0.62.0

```rust
fn supports_from_first_select(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:88`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-821aee91d32f40bd55fbc982"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d65f258450a3479637d9284b"></a>
## supports_group_by_with_modifier

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_group_by_with_modifier` · sqlparser 0.62.0

```rust
fn supports_group_by_with_modifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/en/sql-reference/statements/select/group-by#rollup-modifier>

<a id="op-f29eee2df8d6dd3ab0cc3aac"></a>
## supports_insert_format

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_insert_format` · sqlparser 0.62.0

```rust
fn supports_insert_format(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:59`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b4c708afc04cee14bb4e535"></a>
## supports_insert_table_function

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_insert_table_function` · sqlparser 0.62.0

```rust
fn supports_insert_table_function(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cab3299c601bdaa63ea21ace"></a>
## supports_interpolate

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_interpolate` · sqlparser 0.62.0

```rust
fn supports_interpolate(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:134`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/en/sql-reference/statements/select/order-by#order-by-expr-with-fill-modifier>

<a id="op-b4c8112d914c4a30df97bedf"></a>
## supports_lambda_functions

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_lambda_functions` · sqlparser 0.62.0

```rust
fn supports_lambda_functions(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:84`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/en/sql-reference/functions#higher-order-functions---operator-and-lambdaparams-expr-function>

<a id="op-73889132154c297f9b31b23b"></a>
## supports_limit_by

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_limit_by` · sqlparser 0.62.0

```rust
fn supports_limit_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/en/sql-reference/statements/select/limit-by>

<a id="op-d46037587b7bbb666fae45ee"></a>
## supports_limit_comma

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_limit_comma` · sqlparser 0.62.0

```rust
fn supports_limit_comma(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:51`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ec6d22991bc6ec2cef17611"></a>
## supports_nested_comments

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_nested_comments` · sqlparser 0.62.0

```rust
fn supports_nested_comments(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Supported since 2020.
See <https://clickhouse.com/docs/whats-new/changelog/2020#backward-incompatible-change-2>

<a id="op-27c264cd00f515e7e837ea0d"></a>
## supports_numeric_literal_underscores

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_numeric_literal_underscores` · sqlparser 0.62.0

```rust
fn supports_numeric_literal_underscores(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:63`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9f55758aa77624c80d15291"></a>
## supports_optimize_table

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_optimize_table` · sqlparser 0.62.0

```rust
fn supports_optimize_table(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/en/sql-reference/statements/optimize>

<a id="op-10ddbc22aba94fba84f03130"></a>
## supports_order_by_all

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_order_by_all` · sqlparser 0.62.0

```rust
fn supports_order_by_all(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:93`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/en/sql-reference/statements/select/order-by>

<a id="op-90cd6263fd3cfa1e8a34393e"></a>
## supports_partition_by_after_order_by

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_partition_by_after_order_by` · sqlparser 0.62.0

```rust
fn supports_partition_by_after_order_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:67`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5c0b19a3728ba6f4e50fae1"></a>
## supports_prewhere

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_prewhere` · sqlparser 0.62.0

```rust
fn supports_prewhere(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/en/sql-reference/statements/select/prewhere>

<a id="op-c4325b4f8fe56a1e28908d79"></a>
## supports_select_format

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_select_format` · sqlparser 0.62.0

```rust
fn supports_select_format(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/en/sql-reference/statements/select/format>

<a id="op-b6295cdef4666819227065c4"></a>
## supports_select_wildcard_except

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_select_wildcard_except` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_except(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c140d23bc0244b3938ba07a"></a>
## supports_select_wildcard_replace

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_select_wildcard_replace` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_replace(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:149`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/sql-reference/statements/select#replace>

<a id="op-d0f9bdd803d9767c84945133"></a>
## supports_settings

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_settings` · sqlparser 0.62.0

```rust
fn supports_settings(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/en/sql-reference/statements/select#settings-in-select-query>

<a id="op-d310221b3afd2d8f3feb1b7d"></a>
## supports_string_literal_backslash_escape

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_string_literal_backslash_escape` · sqlparser 0.62.0

```rust
fn supports_string_literal_backslash_escape(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:35`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2e1014fb8059e64d393d520"></a>
## supports_with_fill

`function` · `sqlparser::dialect::clickhouse::ClickHouseDialect::supports_with_fill` · sqlparser 0.62.0

```rust
fn supports_with_fill(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::clickhouse::ClickHouseDialect", "path": "ClickHouseDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [156, 2], "filename": "src/dialect/clickhouse.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/clickhouse.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://clickhouse.com/docs/en/sql-reference/statements/select/order-by#order-by-expr-with-fill-modifier>
