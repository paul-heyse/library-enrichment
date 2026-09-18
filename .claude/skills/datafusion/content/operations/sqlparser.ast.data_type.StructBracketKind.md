# `sqlparser::ast::data_type::StructBracketKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.StructBracketKind.json).

<a id="op-3b6e1665c6d87fa9be0b4276"></a>
## StructBracketKind

`enum` · `sqlparser::ast::data_type::StructBracketKind` · sqlparser 0.62.0

```rust
enum StructBracketKind
```

Source: `src/ast/data_type.rs:900`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Type of brackets used for `STRUCT` literals.

<a id="op-12631bc02a6411290e91bb1d"></a>
## AngleBrackets

`variant` · `sqlparser::ast::data_type::StructBracketKind::AngleBrackets` · sqlparser 0.62.0

```rust
AngleBrackets
```

Source: `src/ast/data_type.rs:904`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Example: `STRUCT<a INT, b STRING>`

<a id="op-47c8ee9def91ce8fff38e7fc"></a>
## Parentheses

`variant` · `sqlparser::ast::data_type::StructBracketKind::Parentheses` · sqlparser 0.62.0

```rust
Parentheses
```

Source: `src/ast/data_type.rs:902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Example: `STRUCT(a INT, b STRING)`

<a id="op-9a7b7dc96bfda0e698dd6ffa"></a>
## clone

`function` · `sqlparser::ast::data_type::StructBracketKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> StructBracketKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::StructBracketKind", "path": "StructBracketKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [897, 17], "end": [897, 22], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:897`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6b375b6147e4c13c160472c"></a>
## cmp

`function` · `sqlparser::ast::data_type::StructBracketKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &StructBracketKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::StructBracketKind", "path": "StructBracketKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [897, 57], "end": [897, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:897`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8006d915675b88b738efc914"></a>
## deserialize

`function` · `sqlparser::ast::data_type::StructBracketKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::StructBracketKind", "path": "StructBracketKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [898, 49], "end": [898, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa66db4928bb4e3baa4c70aa"></a>
## eq

`function` · `sqlparser::ast::data_type::StructBracketKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &StructBracketKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::StructBracketKind", "path": "StructBracketKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [897, 30], "end": [897, 39], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:897`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72cf06be6017011b4686eb50"></a>
## fmt

`function` · `sqlparser::ast::data_type::StructBracketKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::StructBracketKind", "path": "StructBracketKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [897, 10], "end": [897, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:897`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c65b4ef21ded414d5d214fc4"></a>
## hash

`function` · `sqlparser::ast::data_type::StructBracketKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::StructBracketKind", "path": "StructBracketKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [897, 62], "end": [897, 66], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:897`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20ce14c4a00d3ba71fe74014"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::StructBracketKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &StructBracketKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::StructBracketKind", "path": "StructBracketKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [897, 41], "end": [897, 51], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:897`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12e40f6f2a40fd7a06a98077"></a>
## serialize

`function` · `sqlparser::ast::data_type::StructBracketKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::StructBracketKind", "path": "StructBracketKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [898, 38], "end": [898, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-776370c32f43f9570357705b"></a>
## visit

`function` · `sqlparser::ast::data_type::StructBracketKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::StructBracketKind", "path": "StructBracketKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [899, 40], "end": [899, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:899`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1778f0dc9acbda2f38be1bd"></a>
## visit

`function` · `sqlparser::ast::data_type::StructBracketKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::StructBracketKind", "path": "StructBracketKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [899, 47], "end": [899, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:899`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
