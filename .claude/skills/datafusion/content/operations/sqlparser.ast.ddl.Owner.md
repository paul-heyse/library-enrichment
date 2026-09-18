# `sqlparser::ast::ddl::Owner`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.Owner.json).

<a id="op-da8a3bb274edd21f616cc84e"></a>
## Owner

`enum` · `sqlparser::ast::ddl::Owner` · sqlparser 0.62.0

```rust
enum Owner
```

Source: `src/ast/ddl.rs:650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

New owner specification for `ALTER TABLE ... OWNER TO ...`

<a id="op-15793e42d7ab1f35dab2a9fe"></a>
## CurrentRole

`variant` · `sqlparser::ast::ddl::Owner::CurrentRole` · sqlparser 0.62.0

```rust
CurrentRole
```

Source: `src/ast/ddl.rs:654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CURRENT_ROLE` keyword.

<a id="op-32b676223fb25938ce8b420f"></a>
## CurrentUser

`variant` · `sqlparser::ast::ddl::Owner::CurrentUser` · sqlparser 0.62.0

```rust
CurrentUser
```

Source: `src/ast/ddl.rs:656`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CURRENT_USER` keyword.

<a id="op-067b62e039a605ac83661f32"></a>
## Ident

`variant` · `sqlparser::ast::ddl::Owner::Ident` · sqlparser 0.62.0

```rust
Ident
```

Source: `src/ast/ddl.rs:652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A specific user/role identifier.

<a id="op-90e87efa88739314ce0c91e1"></a>
## SessionUser

`variant` · `sqlparser::ast::ddl::Owner::SessionUser` · sqlparser 0.62.0

```rust
SessionUser
```

Source: `src/ast/ddl.rs:658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SESSION_USER` keyword.

<a id="op-05c9aa67fcfbc7ae1024fcfb"></a>
## clone

`function` · `sqlparser::ast::ddl::Owner::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Owner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 17], "end": [646, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d49616ff1e27264344d6fa97"></a>
## cmp

`function` · `sqlparser::ast::ddl::Owner::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Owner) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 51], "end": [646, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1188b17a8ec91d5a1345d6a4"></a>
## deserialize

`function` · `sqlparser::ast::ddl::Owner::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [647, 49], "end": [647, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:647`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3123a6540b17b05784fc9c39"></a>
## eq

`function` · `sqlparser::ast::ddl::Owner::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Owner) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 24], "end": [646, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b8b0c839d634da31d7e2d2a"></a>
## fmt

`function` · `sqlparser::ast::ddl::Owner::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 10], "end": [646, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d92bfb2b2f4c02179c0d202e"></a>
## fmt

`function` · `sqlparser::ast::ddl::Owner::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [661, 1], "end": [670, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f547608f141e4b46bca7390"></a>
## hash

`function` · `sqlparser::ast::ddl::Owner::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 56], "end": [646, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-228b9dbc92fb64d259af48c0"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::Owner::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Owner) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 35], "end": [646, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f17ab0f0cd155eaed738565"></a>
## serialize

`function` · `sqlparser::ast::ddl::Owner::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [647, 38], "end": [647, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:647`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d5f1e8d5d28743143eed823"></a>
## visit

`function` · `sqlparser::ast::ddl::Owner::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [648, 40], "end": [648, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab33f184a8184ba061aa682b"></a>
## visit

`function` · `sqlparser::ast::ddl::Owner::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Owner", "path": "Owner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [648, 47], "end": [648, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
