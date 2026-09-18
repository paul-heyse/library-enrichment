# `sqlparser::ast::query::ForClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ForClause.json).

<a id="op-ff6df7b4984e7108f97bde0f"></a>
## ForClause

`enum` · `sqlparser::ast::query::ForClause` · sqlparser 0.62.0

```rust
enum ForClause
```

Source: `src/ast/query.rs:3835`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR XML` or `FOR JSON` clause (MSSQL): formats the output of a query as XML or JSON.

<a id="op-a586c2bb111ff54fb2537679"></a>
## Browse

`variant` · `sqlparser::ast::query::ForClause::Browse` · sqlparser 0.62.0

```rust
Browse
```

Source: `src/ast/query.rs:3837`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR BROWSE` clause.

<a id="op-0a35d00963a8200b71698e9c"></a>
## Json

`variant` · `sqlparser::ast::query::ForClause::Json` · sqlparser 0.62.0

```rust
Json
```

Source: `src/ast/query.rs:3839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR JSON ...` clause and its options.

<a id="op-5786eeea465beadc901233c1"></a>
## Xml

`variant` · `sqlparser::ast::query::ForClause::Xml` · sqlparser 0.62.0

```rust
Xml
```

Source: `src/ast/query.rs:3850`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR XML ...` clause and its options.

<a id="op-edc860592da5db86e00ef608"></a>
## clone

`function` · `sqlparser::ast::query::ForClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ForClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3832, 17], "end": [3832, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c14d8644be10c6ebbf522af6"></a>
## cmp

`function` · `sqlparser::ast::query::ForClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ForClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3832, 51], "end": [3832, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc66ba656b4912bcc2a9469b"></a>
## deserialize

`function` · `sqlparser::ast::query::ForClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3833, 49], "end": [3833, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3833`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81b95042cb50e2dca261b5e3"></a>
## eq

`function` · `sqlparser::ast::query::ForClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ForClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3832, 24], "end": [3832, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c4a52d12d718b72ab74ac80"></a>
## fmt

`function` · `sqlparser::ast::query::ForClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3864, 1], "end": [3912, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddacac48fd9f591c0e2a7fc3"></a>
## fmt

`function` · `sqlparser::ast::query::ForClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3832, 10], "end": [3832, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8add4057919bb409e419831"></a>
## hash

`function` · `sqlparser::ast::query::ForClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3832, 56], "end": [3832, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1730c02eb230619e11892496"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ForClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ForClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3832, 35], "end": [3832, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02c00234bb9c3a24207f62ed"></a>
## serialize

`function` · `sqlparser::ast::query::ForClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3833, 38], "end": [3833, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3833`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1738d979313643614f7b842a"></a>
## visit

`function` · `sqlparser::ast::query::ForClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3834, 47], "end": [3834, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3834`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65481db9803cbc5d5681d931"></a>
## visit

`function` · `sqlparser::ast::query::ForClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForClause", "path": "ForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3834, 40], "end": [3834, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3834`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
