# `sqlparser::ast::data_type::CharLengthUnits`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.CharLengthUnits.json).

<a id="op-1625f121e6e9ba65e2caf646"></a>
## CharLengthUnits

`enum` · `sqlparser::ast::data_type::CharLengthUnits` · sqlparser 0.62.0

```rust
enum CharLengthUnits
```

Source: `src/ast/data_type.rs:1083`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Possible units for characters, initially based on 2016 ANSI [SQL Standard][1].

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#char-length-units

<a id="op-1014a8253c42c6be7e764fdc"></a>
## Characters

`variant` · `sqlparser::ast::data_type::CharLengthUnits::Characters` · sqlparser 0.62.0

```rust
Characters
```

Source: `src/ast/data_type.rs:1085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CHARACTERS unit

<a id="op-44252e9cac6f4ac3b525c390"></a>
## Octets

`variant` · `sqlparser::ast::data_type::CharLengthUnits::Octets` · sqlparser 0.62.0

```rust
Octets
```

Source: `src/ast/data_type.rs:1087`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

OCTETS unit

<a id="op-519a4da2f4214bab253f8b0c"></a>
## clone

`function` · `sqlparser::ast::data_type::CharLengthUnits::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CharLengthUnits
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1080, 23], "end": [1080, 28], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:1080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07729dc4d23be9fb98468328"></a>
## cmp

`function` · `sqlparser::ast::data_type::CharLengthUnits::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CharLengthUnits) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1080, 57], "end": [1080, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:1080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34746717638ecbb31e1e2321"></a>
## deserialize

`function` · `sqlparser::ast::data_type::CharLengthUnits::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1081, 49], "end": [1081, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:1081`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2346e9bcfb399834f90b668a"></a>
## eq

`function` · `sqlparser::ast::data_type::CharLengthUnits::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CharLengthUnits) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1080, 30], "end": [1080, 39], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:1080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-808289fe9cfb355dc8c71fa4"></a>
## fmt

`function` · `sqlparser::ast::data_type::CharLengthUnits::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1090, 1], "end": [1101, 2], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/data_type.rs:1091`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd35e7474008bf953001e180"></a>
## fmt

`function` · `sqlparser::ast::data_type::CharLengthUnits::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1080, 10], "end": [1080, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:1080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad33c52536719e0a8fa8e5b1"></a>
## hash

`function` · `sqlparser::ast::data_type::CharLengthUnits::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1080, 62], "end": [1080, 66], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:1080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48cf3e8e28ce8be61a2b63e9"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::CharLengthUnits::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CharLengthUnits) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1080, 41], "end": [1080, 51], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:1080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a568d6a0605e1337374fc869"></a>
## serialize

`function` · `sqlparser::ast::data_type::CharLengthUnits::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1081, 38], "end": [1081, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:1081`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-514ee3770b3e87e0cd4c3ef6"></a>
## visit

`function` · `sqlparser::ast::data_type::CharLengthUnits::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1082, 47], "end": [1082, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:1082`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b03bb7d76bed2fa45bdb55ec"></a>
## visit

`function` · `sqlparser::ast::data_type::CharLengthUnits::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharLengthUnits", "path": "CharLengthUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1082, 40], "end": [1082, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:1082`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
