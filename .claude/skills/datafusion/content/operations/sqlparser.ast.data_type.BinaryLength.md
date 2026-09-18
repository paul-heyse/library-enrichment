# `sqlparser::ast::data_type::BinaryLength`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.BinaryLength.json).

<a id="op-e3028c4acdd799c1d8cccd94"></a>
## BinaryLength

`enum` · `sqlparser::ast::data_type::BinaryLength` · sqlparser 0.62.0

```rust
enum BinaryLength
```

Source: `src/ast/data_type.rs:1109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Information about [binary length][1], including length and possibly unit.

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#binary-length

<a id="op-6aec177232ecb197ef13ae90"></a>
## IntegerLength

`variant` · `sqlparser::ast::data_type::BinaryLength::IntegerLength` · sqlparser 0.62.0

```rust
IntegerLength
```

Source: `src/ast/data_type.rs:1111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Integer length for binary types (e.g. `VARBINARY(100)`).

<a id="op-b73bb5afba75c63cb0db1301"></a>
## Max

`variant` · `sqlparser::ast::data_type::BinaryLength::Max` · sqlparser 0.62.0

```rust
Max
```

Source: `src/ast/data_type.rs:1116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

VARBINARY(MAX) used in T-SQL (Microsoft SQL Server).

<a id="op-5b98c184c80e3f28acb1ea8d"></a>
## clone

`function` · `sqlparser::ast::data_type::BinaryLength::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> BinaryLength
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1103, 23], "end": [1103, 28], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:1103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3c9cecfcdcca9e2202b7903"></a>
## cmp

`function` · `sqlparser::ast::data_type::BinaryLength::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &BinaryLength) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1103, 57], "end": [1103, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:1103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3f5cd4f9a9ec2297e4683bc"></a>
## deserialize

`function` · `sqlparser::ast::data_type::BinaryLength::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1104, 49], "end": [1104, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:1104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54432947365fba87abd1c53f"></a>
## eq

`function` · `sqlparser::ast::data_type::BinaryLength::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &BinaryLength) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1103, 30], "end": [1103, 39], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:1103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d75673c1acc39a90c82bdca"></a>
## fmt

`function` · `sqlparser::ast::data_type::BinaryLength::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1103, 10], "end": [1103, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:1103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa5db95af22a4673ca08500b"></a>
## fmt

`function` · `sqlparser::ast::data_type::BinaryLength::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1119, 1], "end": [1131, 2], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/data_type.rs:1120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2e87ef574777a2bcb0408de"></a>
## hash

`function` · `sqlparser::ast::data_type::BinaryLength::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1103, 62], "end": [1103, 66], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:1103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3ddcb0eaa0276df0c38f0af"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::BinaryLength::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &BinaryLength) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1103, 41], "end": [1103, 51], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:1103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-967f97455e6e40a38f2e1dcd"></a>
## serialize

`function` · `sqlparser::ast::data_type::BinaryLength::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1104, 38], "end": [1104, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:1104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f14c5201ed40c27c16ef5be"></a>
## visit

`function` · `sqlparser::ast::data_type::BinaryLength::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1105, 40], "end": [1105, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:1105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84a65810233b30a884307d2b"></a>
## visit

`function` · `sqlparser::ast::data_type::BinaryLength::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::BinaryLength", "path": "BinaryLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1105, 47], "end": [1105, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:1105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
