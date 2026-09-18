# `sqlparser::ast::data_type::TimezoneInfo`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.TimezoneInfo.json).

<a id="op-9cadd924b5645ed9866a5731"></a>
## TimezoneInfo

`enum` · `sqlparser::ast::data_type::TimezoneInfo` · sqlparser 0.62.0

```rust
enum TimezoneInfo
```

Source: `src/ast/data_type.rs:914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Timestamp and Time data types information about TimeZone formatting.

This is more related to a display information than real differences between each variant. To
guarantee compatibility with the input query we must maintain its exact information.

<a id="op-cefba01a46039cb275b3fe39"></a>
## None

`variant` · `sqlparser::ast::data_type::TimezoneInfo::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/data_type.rs:916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No information about time zone, e.g. TIMESTAMP

<a id="op-e6d1ae551a65b8309bb22cff"></a>
## Tz

`variant` · `sqlparser::ast::data_type::TimezoneInfo::Tz` · sqlparser 0.62.0

```rust
Tz
```

Source: `src/ast/data_type.rs:930`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Postgresql specific `WITH TIME ZONE` formatting, for both TIME and TIMESTAMP, e.g. TIMETZ, [Postgresql]

[Postgresql]: https://www.postgresql.org/docs/current/datatype-datetime.html

<a id="op-0c1867571f17f877f7e59fe2"></a>
## WithTimeZone

`variant` · `sqlparser::ast::data_type::TimezoneInfo::WithTimeZone` · sqlparser 0.62.0

```rust
WithTimeZone
```

Source: `src/ast/data_type.rs:921`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Temporal type 'WITH TIME ZONE', e.g. TIMESTAMP WITH TIME ZONE, [SQL Standard], [Oracle]

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#datetime-type
[Oracle]: https://docs.oracle.com/en/database/oracle/oracle-database/12.2/nlspg/datetime-data-types-and-time-zone-support.html#GUID-3F1C388E-C651-43D5-ADBC-1A49E5C2CA05

<a id="op-836c99c36bed93c8ac8cd6d5"></a>
## WithoutTimeZone

`variant` · `sqlparser::ast::data_type::TimezoneInfo::WithoutTimeZone` · sqlparser 0.62.0

```rust
WithoutTimeZone
```

Source: `src/ast/data_type.rs:926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Temporal type 'WITHOUT TIME ZONE', e.g. TIME WITHOUT TIME ZONE, [SQL Standard], [Postgresql]

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#datetime-type
[Postgresql]: https://www.postgresql.org/docs/current/datatype-datetime.html

<a id="op-4b2a3a0635864aea31167c62"></a>
## clone

`function` · `sqlparser::ast::data_type::TimezoneInfo::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TimezoneInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [911, 23], "end": [911, 28], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:911`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d946e8167d6993f9da088c57"></a>
## cmp

`function` · `sqlparser::ast::data_type::TimezoneInfo::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TimezoneInfo) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [911, 57], "end": [911, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:911`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdbf23617257eb4fcb06542a"></a>
## deserialize

`function` · `sqlparser::ast::data_type::TimezoneInfo::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [912, 49], "end": [912, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:912`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d023c93943021ad373aa727"></a>
## eq

`function` · `sqlparser::ast::data_type::TimezoneInfo::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TimezoneInfo) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [911, 30], "end": [911, 39], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:911`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13aa6478bacb4b1109ad3ea6"></a>
## fmt

`function` · `sqlparser::ast::data_type::TimezoneInfo::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [911, 10], "end": [911, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:911`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-879e8aa49fafbf8c50eda453"></a>
## fmt

`function` · `sqlparser::ast::data_type::TimezoneInfo::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 1], "end": [953, 2], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/data_type.rs:934`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-324921f9e95e7edaed788a26"></a>
## hash

`function` · `sqlparser::ast::data_type::TimezoneInfo::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [911, 62], "end": [911, 66], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:911`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e730ffd40f5e0980cba6e30"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::TimezoneInfo::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TimezoneInfo) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [911, 41], "end": [911, 51], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:911`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28877fa87875f320ec0e6a60"></a>
## serialize

`function` · `sqlparser::ast::data_type::TimezoneInfo::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [912, 38], "end": [912, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:912`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae2a173807ba898c0372c5f8"></a>
## visit

`function` · `sqlparser::ast::data_type::TimezoneInfo::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [913, 47], "end": [913, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4c189f0c9a9314a44822646"></a>
## visit

`function` · `sqlparser::ast::data_type::TimezoneInfo::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::TimezoneInfo", "path": "TimezoneInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [913, 40], "end": [913, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
