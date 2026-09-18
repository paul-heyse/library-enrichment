# `sqlparser::ast::ddl::AlterConnectorOwner`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterConnectorOwner.json).

<a id="op-96595ead1d1580efb8319628"></a>
## AlterConnectorOwner

`enum` · `sqlparser::ast::ddl::AlterConnectorOwner` · sqlparser 0.62.0

```rust
enum AlterConnectorOwner
```

Source: `src/ast/ddl.rs:676`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

New connector owner specification for `ALTER CONNECTOR ... OWNER TO ...`

<a id="op-823879a55d32b04aab9284de"></a>
## Role

`variant` · `sqlparser::ast::ddl::AlterConnectorOwner::Role` · sqlparser 0.62.0

```rust
Role
```

Source: `src/ast/ddl.rs:680`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ROLE <ident>` connector owner.

<a id="op-d7e1102ea85c64abb36e305a"></a>
## User

`variant` · `sqlparser::ast::ddl::AlterConnectorOwner::User` · sqlparser 0.62.0

```rust
User
```

Source: `src/ast/ddl.rs:678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`USER <ident>` connector owner.

<a id="op-88a01d2bdad97932a2690d6b"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterConnectorOwner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 17], "end": [672, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b196063617dee012640e3d81"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterConnectorOwner) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 51], "end": [672, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03b3740bfecfa1b3468732e8"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [673, 49], "end": [673, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c375ebcb1f1e5e0d10a9b54b"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterConnectorOwner) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 24], "end": [672, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c88845879c8e9b634294dbf5"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 10], "end": [672, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db6e049a187ac15c48227772"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 1], "end": [690, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:684`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ac1d84e9b725a38933187e6"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 56], "end": [672, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2839306120d3347f3255b3f"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterConnectorOwner) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 35], "end": [672, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88711173aa92cc0003b5eba1"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [673, 38], "end": [673, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-154e80e6b028c691b1e1e2ae"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 47], "end": [674, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:674`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57bc2b34c0d4f0633ebbe4af"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterConnectorOwner::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterConnectorOwner", "path": "AlterConnectorOwner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 40], "end": [674, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:674`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
