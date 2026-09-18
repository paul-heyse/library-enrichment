# `sqlparser::ast::CommentDef`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CommentDef.json).

<a id="op-4c76a50086e832b42860824d"></a>
## CommentDef

`enum` · `sqlparser::ast::CommentDef` · sqlparser 0.62.0

```rust
enum CommentDef
```

Source: `src/ast/mod.rs:10672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Helper to indicate if a comment includes the `=` in the display form

<a id="op-19fdbff0694a66ba2e64bbf7"></a>
## WithEq

`variant` · `sqlparser::ast::CommentDef::WithEq` · sqlparser 0.62.0

```rust
WithEq
```

Source: `src/ast/mod.rs:10675`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Includes `=` when printing the comment, as `COMMENT = 'comment'`
Does not include `=` when printing the comment, as `COMMENT 'comment'`

<a id="op-8b9504073502ea937de1465a"></a>
## WithoutEq

`variant` · `sqlparser::ast::CommentDef::WithoutEq` · sqlparser 0.62.0

```rust
WithoutEq
```

Source: `src/ast/mod.rs:10677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Comment variant that omits the `=` when displayed.

<a id="op-c5dc982c7672cf0074ce78f0"></a>
## clone

`function` · `sqlparser::ast::CommentDef::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CommentDef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10669, 17], "end": [10669, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10669`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4aa3107f3566db5394f6dcaf"></a>
## cmp

`function` · `sqlparser::ast::CommentDef::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CommentDef) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10669, 51], "end": [10669, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10669`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75ffe2d8a69acd9b1161b0f0"></a>
## deserialize

`function` · `sqlparser::ast::CommentDef::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10670, 49], "end": [10670, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10670`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bad6cb64e0c4891a6ef82a44"></a>
## eq

`function` · `sqlparser::ast::CommentDef::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CommentDef) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10669, 24], "end": [10669, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10669`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c9c29ac9e5ef96b07a9eb5e"></a>
## fmt

`function` · `sqlparser::ast::CommentDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10669, 10], "end": [10669, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10669`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b025f6c2b5c9b17d60f99ab"></a>
## fmt

`function` · `sqlparser::ast::CommentDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10680, 1], "end": [10686, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10681`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f63a69a2607c67e60a08707"></a>
## hash

`function` · `sqlparser::ast::CommentDef::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10669, 56], "end": [10669, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10669`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e1cefd3a67641f2e67415fc"></a>
## partial_cmp

`function` · `sqlparser::ast::CommentDef::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CommentDef) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10669, 35], "end": [10669, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10669`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb4aec95cf806bc385dec88b"></a>
## serialize

`function` · `sqlparser::ast::CommentDef::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10670, 38], "end": [10670, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10670`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b7489e0b6f9ac3a21d0aa41"></a>
## visit

`function` · `sqlparser::ast::CommentDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10671, 40], "end": [10671, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10671`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d8a69c38843021b0274c41c"></a>
## visit

`function` · `sqlparser::ast::CommentDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentDef", "path": "CommentDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10671, 47], "end": [10671, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10671`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
