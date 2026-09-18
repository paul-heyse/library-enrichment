# `sqlparser::dialect::redshift::RedshiftSqlDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.redshift.RedshiftSqlDialect.json).

<a id="op-7eed4bb426ee920704f5734b"></a>
## RedshiftSqlDialect

`struct` · `sqlparser::dialect::redshift::RedshiftSqlDialect` · sqlparser 0.62.0

```rust
struct RedshiftSqlDialect
```

Source: `src/dialect/redshift.rs:27`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [RedShift](https://aws.amazon.com/redshift/)

<a id="op-9a52c1b7439d3068272ce24d"></a>
## allow_extract_single_quotes

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::allow_extract_single_quotes` · sqlparser 0.62.0

```rust
fn allow_extract_single_quotes(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:132`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b02d2e84b835154e664a51c"></a>
## clone

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RedshiftSqlDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 26], "end": [25, 31], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/redshift.rs:25`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b437ed487d20f13c47fae2bb"></a>
## cmp

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RedshiftSqlDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 72], "end": [25, 75], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/redshift.rs:25`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c030bc1d666576bc7075ccf"></a>
## convert_type_before_value

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::convert_type_before_value` · sqlparser 0.62.0

```rust
fn convert_type_before_value(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:97`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

redshift has `CONVERT(type, value)` instead of `CONVERT(value, type)`
<https://docs.aws.amazon.com/redshift/latest/dg/r_CONVERT_function.html>

<a id="op-0091cf43752a2c7c8797b708"></a>
## default

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::default` · sqlparser 0.62.0

```rust
fn default() -> RedshiftSqlDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 17], "end": [25, 24], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/redshift.rs:25`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1569e2cea0ff3ab6eb99a232"></a>
## deserialize

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 56], "end": [26, 74], "filename": "src/dialect/redshift.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/redshift.rs:26`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69ccfe32ea647c0213e97c53"></a>
## eq

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RedshiftSqlDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 39], "end": [25, 48], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/redshift.rs:25`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bbc8dadfc727d89be31e34a"></a>
## fmt

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 10], "end": [25, 15], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/redshift.rs:25`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f97ddcc4640e612369c77ac8"></a>
## hash

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 54], "end": [25, 58], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/redshift.rs:25`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8680b939e4a132d644921231"></a>
## is_identifier_part

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:89`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11d3eed7d9c04ab438f2a556"></a>
## is_identifier_start

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:83`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5575144420422dd5b5a9267f"></a>
## is_nested_delimited_identifier_start

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::is_nested_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_nested_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:44`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determine if a character starts a potential nested quoted identifier.
Example: RedShift supports the following quote styles to all mean the same thing:
```sql
SELECT 1 AS foo;
SELECT 1 AS "foo";
SELECT 1 AS [foo];
SELECT 1 AS ["foo"];
```

<a id="op-7de38cdcea2eeea6faff2edc"></a>
## partial_cmp

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RedshiftSqlDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 60], "end": [25, 70], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/redshift.rs:25`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-448fb1edfc09c58c6a9a4e19"></a>
## peek_nested_delimited_identifier_quotes

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::peek_nested_delimited_identifier_quotes` · sqlparser 0.62.0

```rust
fn peek_nested_delimited_identifier_quotes(&self, chars: Peekable<Chars<'_>>) -> Option<(char, Option<char>)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:59`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only applicable whenever [`Self::is_nested_delimited_identifier_start`](../operations/sqlparser.dialect.redshift.RedshiftSqlDialect.md#op-5575144420422dd5b5a9267f) returns true
If the next sequence of tokens potentially represent a nested identifier, then this method
returns a tuple containing the outer quote style, and if present, the inner (nested) quote style.

Example (Redshift):
```text
`["foo"]` => Some(`[`, Some(`"`))
`[foo]` => Some(`[`, None)
`[0]` => None
`"foo"` => None
```

<a id="op-6aaef26cf013a8a83cdc5f50"></a>
## serialize

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 38], "end": [26, 54], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/redshift.rs:26`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93f5aa07d9c5535809fe02ca"></a>
## supports_array_typedef_with_brackets

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_array_typedef_with_brackets` · sqlparser 0.62.0

```rust
fn supports_array_typedef_with_brackets(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:128`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48a9d7f80b1dec76540b51ae"></a>
## supports_bitwise_shift_operators

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_bitwise_shift_operators` · sqlparser 0.62.0

```rust
fn supports_bitwise_shift_operators(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e134171383ecde2e18dead6a"></a>
## supports_connect_by

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_connect_by` · sqlparser 0.62.0

```rust
fn supports_connect_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9de0d9af24fb9de5a87cae9"></a>
## supports_create_table_like_parenthesized

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_create_table_like_parenthesized` · sqlparser 0.62.0

```rust
fn supports_create_table_like_parenthesized(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88ffcc314e185c5c27a69981"></a>
## supports_geometric_types

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_geometric_types` · sqlparser 0.62.0

```rust
fn supports_geometric_types(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eff6909e2f25277276434af3"></a>
## supports_partiql

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_partiql` · sqlparser 0.62.0

```rust
fn supports_partiql(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift supports PartiQL: <https://docs.aws.amazon.com/redshift/latest/dg/super-overview.html>

<a id="op-7e4e48c9c3b6f3e5fc33641a"></a>
## supports_select_exclude

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_select_exclude` · sqlparser 0.62.0

```rust
fn supports_select_exclude(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:150`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d1c99e00033599c6edf111c"></a>
## supports_select_wildcard_exclude

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_select_wildcard_exclude` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_exclude(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:140`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e080fc09771a5ce8c670855"></a>
## supports_select_wildcard_with_alias

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_select_wildcard_with_alias` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_with_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift supports aliasing wildcard expressions:
<https://docs.aws.amazon.com/redshift/latest/dg/r_SELECT_list.html>

<a id="op-ed9ba94272785dd383eba6e0"></a>
## supports_string_escape_constant

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_string_escape_constant` · sqlparser 0.62.0

```rust
fn supports_string_escape_constant(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc38984a4dece901bc4b8c75"></a>
## supports_string_literal_backslash_escape

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_string_literal_backslash_escape` · sqlparser 0.62.0

```rust
fn supports_string_literal_backslash_escape(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ffed95bfe4dcc58d157b134"></a>
## supports_string_literal_concatenation_with_newline

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_string_literal_concatenation_with_newline` · sqlparser 0.62.0

```rust
fn supports_string_literal_concatenation_with_newline(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:158`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f248694a5b86b2139f540004"></a>
## supports_top_before_distinct

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_top_before_distinct` · sqlparser 0.62.0

```rust
fn supports_top_before_distinct(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:107`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift expects the `TOP` option before the `ALL/DISTINCT` option:
<https://docs.aws.amazon.com/redshift/latest/dg/r_SELECT_list.html#r_SELECT_list-parameters>

<a id="op-5f87dafc734d524c218725ad"></a>
## supports_window_function_null_treatment_arg

`function` · `sqlparser::dialect::redshift::RedshiftSqlDialect::supports_window_function_null_treatment_arg` · sqlparser 0.62.0

```rust
fn supports_window_function_null_treatment_arg(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::redshift::RedshiftSqlDialect", "path": "RedshiftSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [165, 2], "filename": "src/dialect/redshift.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/redshift.rs:162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
