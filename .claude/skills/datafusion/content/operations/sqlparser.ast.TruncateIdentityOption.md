# `sqlparser::ast::TruncateIdentityOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TruncateIdentityOption.json).

<a id="op-b5df93ae5904af639840ac52"></a>
## TruncateIdentityOption

`enum` · `sqlparser::ast::TruncateIdentityOption` · sqlparser 0.62.0

```rust
enum TruncateIdentityOption
```

Source: `src/ast/mod.rs:6623`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL identity option for TRUNCATE table
[ RESTART IDENTITY | CONTINUE IDENTITY ]

<a id="op-79a403685d75821b9b5f949d"></a>
## Continue

`variant` · `sqlparser::ast::TruncateIdentityOption::Continue` · sqlparser 0.62.0

```rust
Continue
```

Source: `src/ast/mod.rs:6627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Continue identity values (CONTINUE IDENTITY).

<a id="op-63f051d95a5ea154b6b5339f"></a>
## Restart

`variant` · `sqlparser::ast::TruncateIdentityOption::Restart` · sqlparser 0.62.0

```rust
Restart
```

Source: `src/ast/mod.rs:6625`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Restart identity values (RESTART IDENTITY).

<a id="op-25472b0ddb777f947f452eaf"></a>
## clone

`function` · `sqlparser::ast::TruncateIdentityOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TruncateIdentityOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateIdentityOption", "path": "TruncateIdentityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6620, 17], "end": [6620, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4e77f87ba07f95e66eb9f28"></a>
## cmp

`function` · `sqlparser::ast::TruncateIdentityOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TruncateIdentityOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateIdentityOption", "path": "TruncateIdentityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6620, 51], "end": [6620, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-556167f148d39fe88755c081"></a>
## deserialize

`function` · `sqlparser::ast::TruncateIdentityOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateIdentityOption", "path": "TruncateIdentityOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6621, 49], "end": [6621, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c235e1fdb37004bdb03b970a"></a>
## eq

`function` · `sqlparser::ast::TruncateIdentityOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TruncateIdentityOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateIdentityOption", "path": "TruncateIdentityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6620, 24], "end": [6620, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5f419a7f17f1bed830c84ea"></a>
## fmt

`function` · `sqlparser::ast::TruncateIdentityOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateIdentityOption", "path": "TruncateIdentityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6620, 10], "end": [6620, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e44756d05df960d76e45ffd5"></a>
## hash

`function` · `sqlparser::ast::TruncateIdentityOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateIdentityOption", "path": "TruncateIdentityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6620, 56], "end": [6620, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa7147f1cf638fffb87936f1"></a>
## partial_cmp

`function` · `sqlparser::ast::TruncateIdentityOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TruncateIdentityOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateIdentityOption", "path": "TruncateIdentityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6620, 35], "end": [6620, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bbc87b87233b3459dfaf11d"></a>
## serialize

`function` · `sqlparser::ast::TruncateIdentityOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateIdentityOption", "path": "TruncateIdentityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6621, 38], "end": [6621, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0add7d25da22a45bb051c0aa"></a>
## visit

`function` · `sqlparser::ast::TruncateIdentityOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateIdentityOption", "path": "TruncateIdentityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6622, 40], "end": [6622, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7c0446d302b68d51e187d41"></a>
## visit

`function` · `sqlparser::ast::TruncateIdentityOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateIdentityOption", "path": "TruncateIdentityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6622, 47], "end": [6622, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
