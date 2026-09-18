# `sqlparser::ast::CreateTableLike`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateTableLike.json).

<a id="op-8b4d5e10f151041514b0bb24"></a>
## CreateTableLike

`struct` · `sqlparser::ast::CreateTableLike` · sqlparser 0.62.0

```rust
struct CreateTableLike
```

Source: `src/ast/mod.rs:11835`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the `LIKE` clause of a `CREATE TABLE` statement.

<a id="op-1b2b011635f5b1b24216245f"></a>
## clone

`function` · `sqlparser::ast::CreateTableLike::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateTableLike
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11831, 17], "end": [11831, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f820f084bc8ca61fd67224af"></a>
## cmp

`function` · `sqlparser::ast::CreateTableLike::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateTableLike) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11831, 51], "end": [11831, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-999aadaa38380ad5418608db"></a>
## defaults

`struct_field` · `sqlparser::ast::CreateTableLike::defaults` · sqlparser 0.62.0

```rust
defaults: Option<CreateTableLikeDefaults>
```

Source: `src/ast/mod.rs:11839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional behavior controlling whether defaults are copied.

<a id="op-ebd9815263be43cceefb0cb2"></a>
## deserialize

`function` · `sqlparser::ast::CreateTableLike::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11832, 49], "end": [11832, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91e09aa96416eb472d037f6a"></a>
## eq

`function` · `sqlparser::ast::CreateTableLike::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateTableLike) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11831, 24], "end": [11831, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59b05ead3da3eac5e2be3047"></a>
## fmt

`function` · `sqlparser::ast::CreateTableLike::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11842, 1], "end": [11850, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11843`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-970d6e26707a00554535346e"></a>
## fmt

`function` · `sqlparser::ast::CreateTableLike::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11831, 10], "end": [11831, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1f6ab3003402ef96b9be7eb"></a>
## hash

`function` · `sqlparser::ast::CreateTableLike::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11831, 56], "end": [11831, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fc346771ad30051119a8b34"></a>
## name

`struct_field` · `sqlparser::ast::CreateTableLike::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:11837`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The source table name to copy the schema from.

<a id="op-400ef27a10c246e4070f174c"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateTableLike::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateTableLike) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11831, 35], "end": [11831, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d381852a93ea172c670b37c"></a>
## serialize

`function` · `sqlparser::ast::CreateTableLike::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11832, 38], "end": [11832, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34293482c762e714a3e70954"></a>
## visit

`function` · `sqlparser::ast::CreateTableLike::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11833, 47], "end": [11833, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11833`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b4365edef9614e43666add7"></a>
## visit

`function` · `sqlparser::ast::CreateTableLike::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLike", "path": "CreateTableLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11833, 40], "end": [11833, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11833`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
