# `sqlparser::ast::data_type::IntervalFields`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.IntervalFields.json).

<a id="op-b6dd859f176b9781491b678c"></a>
## IntervalFields

`enum` · `sqlparser::ast::data_type::IntervalFields` · sqlparser 0.62.0

```rust
enum IntervalFields
```

Source: `src/ast/data_type.rs:961`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fields for [Postgres] `INTERVAL` type.

[Postgres]: https://www.postgresql.org/docs/17/datatype-datetime.html

<a id="op-c808472fa36a27805de10d0f"></a>
## Day

`variant` · `sqlparser::ast::data_type::IntervalFields::Day` · sqlparser 0.62.0

```rust
Day
```

Source: `src/ast/data_type.rs:967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DAY` field

<a id="op-0e3eb6c1610d40ea743be6ff"></a>
## DayToHour

`variant` · `sqlparser::ast::data_type::IntervalFields::DayToHour` · sqlparser 0.62.0

```rust
DayToHour
```

Source: `src/ast/data_type.rs:977`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DAY TO HOUR` field

<a id="op-2e3b8f70b7a9fc0eb8d1a100"></a>
## DayToMinute

`variant` · `sqlparser::ast::data_type::IntervalFields::DayToMinute` · sqlparser 0.62.0

```rust
DayToMinute
```

Source: `src/ast/data_type.rs:979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DAY TO MINUTE` field

<a id="op-9184563d3bcbba45a9195fa0"></a>
## DayToSecond

`variant` · `sqlparser::ast::data_type::IntervalFields::DayToSecond` · sqlparser 0.62.0

```rust
DayToSecond
```

Source: `src/ast/data_type.rs:981`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DAY TO SECOND` field

<a id="op-fdf11f314f49e0e6c357e801"></a>
## Hour

`variant` · `sqlparser::ast::data_type::IntervalFields::Hour` · sqlparser 0.62.0

```rust
Hour
```

Source: `src/ast/data_type.rs:969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`HOUR` field

<a id="op-cbfd95d47924766959b8d2c3"></a>
## HourToMinute

`variant` · `sqlparser::ast::data_type::IntervalFields::HourToMinute` · sqlparser 0.62.0

```rust
HourToMinute
```

Source: `src/ast/data_type.rs:983`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`HOUR TO MINUTE` field

<a id="op-7bc43ebf694b519c027a24d0"></a>
## HourToSecond

`variant` · `sqlparser::ast::data_type::IntervalFields::HourToSecond` · sqlparser 0.62.0

```rust
HourToSecond
```

Source: `src/ast/data_type.rs:985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`HOUR TO SECOND` field

<a id="op-b181722d74a358a311571cc2"></a>
## Minute

`variant` · `sqlparser::ast::data_type::IntervalFields::Minute` · sqlparser 0.62.0

```rust
Minute
```

Source: `src/ast/data_type.rs:971`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MINUTE` field

<a id="op-1692e056244c7364d9d22e8a"></a>
## MinuteToSecond

`variant` · `sqlparser::ast::data_type::IntervalFields::MinuteToSecond` · sqlparser 0.62.0

```rust
MinuteToSecond
```

Source: `src/ast/data_type.rs:987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MINUTE TO SECOND` field

<a id="op-80b9bee6bd84515618e1e7f3"></a>
## Month

`variant` · `sqlparser::ast::data_type::IntervalFields::Month` · sqlparser 0.62.0

```rust
Month
```

Source: `src/ast/data_type.rs:965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MONTH` field

<a id="op-62f9c540a4b581cd1a12f1a1"></a>
## Second

`variant` · `sqlparser::ast::data_type::IntervalFields::Second` · sqlparser 0.62.0

```rust
Second
```

Source: `src/ast/data_type.rs:973`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SECOND` field

<a id="op-224dd19a7156de6ff304f04e"></a>
## Year

`variant` · `sqlparser::ast::data_type::IntervalFields::Year` · sqlparser 0.62.0

```rust
Year
```

Source: `src/ast/data_type.rs:963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`YEAR` field

<a id="op-e28b195ea26d52efa2010eb2"></a>
## YearToMonth

`variant` · `sqlparser::ast::data_type::IntervalFields::YearToMonth` · sqlparser 0.62.0

```rust
YearToMonth
```

Source: `src/ast/data_type.rs:975`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`YEAR TO MONTH` field

<a id="op-21ffec344a6bbca080e48c4a"></a>
## clone

`function` · `sqlparser::ast::data_type::IntervalFields::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IntervalFields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [958, 23], "end": [958, 28], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66c57298bfa2ec1a35ab6ab8"></a>
## cmp

`function` · `sqlparser::ast::data_type::IntervalFields::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IntervalFields) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [958, 57], "end": [958, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33cf5769a6204ea654cadd3f"></a>
## deserialize

`function` · `sqlparser::ast::data_type::IntervalFields::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [959, 49], "end": [959, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:959`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c1716ae7c7f9eecc5f23ed6"></a>
## eq

`function` · `sqlparser::ast::data_type::IntervalFields::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IntervalFields) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [958, 30], "end": [958, 39], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55632b762d57253eef2b0a16"></a>
## fmt

`function` · `sqlparser::ast::data_type::IntervalFields::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [958, 10], "end": [958, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adfcb68ab3d36b2b950b9a46"></a>
## fmt

`function` · `sqlparser::ast::data_type::IntervalFields::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1008, 2], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/data_type.rs:991`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9edc6982f5a87abf431ffdc9"></a>
## hash

`function` · `sqlparser::ast::data_type::IntervalFields::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [958, 62], "end": [958, 66], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dd889594829d4ff0833ddca"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::IntervalFields::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IntervalFields) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [958, 41], "end": [958, 51], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a7ac6f7cdaaa9b26e392fcc"></a>
## serialize

`function` · `sqlparser::ast::data_type::IntervalFields::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [959, 38], "end": [959, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:959`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-beb815ea461cb0f4c3ee20eb"></a>
## visit

`function` · `sqlparser::ast::data_type::IntervalFields::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [960, 47], "end": [960, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:960`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d242df5b154c7acd8664e6b0"></a>
## visit

`function` · `sqlparser::ast::data_type::IntervalFields::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::IntervalFields", "path": "IntervalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [960, 40], "end": [960, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:960`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
