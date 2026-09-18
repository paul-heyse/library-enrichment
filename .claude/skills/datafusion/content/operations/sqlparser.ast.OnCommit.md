# `sqlparser::ast::OnCommit`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.OnCommit.json).

<a id="op-eee292dd16b93847e8c81c5d"></a>
## OnCommit

`enum` · `sqlparser::ast::OnCommit` · sqlparser 0.62.0

```rust
enum OnCommit
```

Source: `src/ast/mod.rs:9308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Action to take `ON COMMIT` for temporary tables.

<a id="op-136ea9697f8d4ad384c40a9d"></a>
## DeleteRows

`variant` · `sqlparser::ast::OnCommit::DeleteRows` · sqlparser 0.62.0

```rust
DeleteRows
```

Source: `src/ast/mod.rs:9310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Delete rows on commit.

<a id="op-029fbddc101182c17461a7f6"></a>
## Drop

`variant` · `sqlparser::ast::OnCommit::Drop` · sqlparser 0.62.0

```rust
Drop
```

Source: `src/ast/mod.rs:9314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Drop the table on commit.

<a id="op-c97dc4a75bd08f29f83a404f"></a>
## PreserveRows

`variant` · `sqlparser::ast::OnCommit::PreserveRows` · sqlparser 0.62.0

```rust
PreserveRows
```

Source: `src/ast/mod.rs:9312`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Preserve rows on commit.

<a id="op-9d6c6244d571dab5e5fe6218"></a>
## clone

`function` · `sqlparser::ast::OnCommit::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OnCommit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnCommit", "path": "OnCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9304, 23], "end": [9304, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b2168bf0dab387cfd3e6e09"></a>
## cmp

`function` · `sqlparser::ast::OnCommit::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OnCommit) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnCommit", "path": "OnCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9304, 57], "end": [9304, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d9c9d707bf757567d706ac6"></a>
## deserialize

`function` · `sqlparser::ast::OnCommit::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnCommit", "path": "OnCommit"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9305, 49], "end": [9305, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9305`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0df0cafe801e139599c09298"></a>
## eq

`function` · `sqlparser::ast::OnCommit::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OnCommit) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnCommit", "path": "OnCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9304, 30], "end": [9304, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2f780ea0a1128f6ff4d39dd"></a>
## fmt

`function` · `sqlparser::ast::OnCommit::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnCommit", "path": "OnCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9304, 10], "end": [9304, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aaa60c7895d2ce9de8427e3a"></a>
## hash

`function` · `sqlparser::ast::OnCommit::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnCommit", "path": "OnCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9304, 62], "end": [9304, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6872a184174cf248f5e4aad3"></a>
## partial_cmp

`function` · `sqlparser::ast::OnCommit::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OnCommit) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnCommit", "path": "OnCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9304, 41], "end": [9304, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32f4f7a489295ebb6cd28d96"></a>
## serialize

`function` · `sqlparser::ast::OnCommit::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnCommit", "path": "OnCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9305, 38], "end": [9305, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9305`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fad6aa56f23163f1da18af3"></a>
## visit

`function` · `sqlparser::ast::OnCommit::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnCommit", "path": "OnCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9306, 47], "end": [9306, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c82fe54c1114795bf56869ba"></a>
## visit

`function` · `sqlparser::ast::OnCommit::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnCommit", "path": "OnCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9306, 40], "end": [9306, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
