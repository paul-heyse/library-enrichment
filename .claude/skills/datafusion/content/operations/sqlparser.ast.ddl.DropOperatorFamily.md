# `sqlparser::ast::ddl::DropOperatorFamily`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DropOperatorFamily.json).

<a id="op-c1b21d58f7c513d084706230"></a>
## DropOperatorFamily

`struct` · `sqlparser::ast::ddl::DropOperatorFamily` · sqlparser 0.62.0

```rust
struct DropOperatorFamily
```

Source: `src/ast/ddl.rs:5031`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP OPERATOR FAMILY` statement
See <https://www.postgresql.org/docs/current/sql-dropopfamily.html>

<a id="op-e042eb83012c4b4a63471f3c"></a>
## clone

`function` · `sqlparser::ast::ddl::DropOperatorFamily::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropOperatorFamily
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5028, 17], "end": [5028, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5028`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85a50cccf3c166fbd92c77d2"></a>
## cmp

`function` · `sqlparser::ast::ddl::DropOperatorFamily::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropOperatorFamily) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5028, 51], "end": [5028, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5028`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c39ed753b663ebf3294f3d5"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DropOperatorFamily::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5029, 49], "end": [5029, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5029`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bfc83a2041877f26ec98891"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::ddl::DropOperatorFamily::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/ddl.rs:5039`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CASCADE or RESTRICT`

<a id="op-4b53f007f7f3e1b6f34a6fe5"></a>
## eq

`function` · `sqlparser::ast::ddl::DropOperatorFamily::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropOperatorFamily) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5028, 24], "end": [5028, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5028`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad52fec612fff94d99cf8251"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropOperatorFamily::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5028, 10], "end": [5028, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5028`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f21866e3b575d9de5dd3cfd7"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropOperatorFamily::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5042, 1], "end": [5055, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5043`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f32937f0f9fdfcb77c43818"></a>
## hash

`function` · `sqlparser::ast::ddl::DropOperatorFamily::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5028, 56], "end": [5028, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5028`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77b7a5cddcc66afc6e6e2f2c"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::DropOperatorFamily::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:5033`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IF EXISTS` clause

<a id="op-c26d31ef754bf66a35003d71"></a>
## names

`struct_field` · `sqlparser::ast::ddl::DropOperatorFamily::names` · sqlparser 0.62.0

```rust
names: Vec<ast::ObjectName>
```

Source: `src/ast/ddl.rs:5035`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One or more operator families to drop

<a id="op-5955d677713289de14a19f51"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DropOperatorFamily::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropOperatorFamily) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5028, 35], "end": [5028, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5028`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46bc90fb178da5a8a01a900e"></a>
## serialize

`function` · `sqlparser::ast::ddl::DropOperatorFamily::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5029, 38], "end": [5029, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5029`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f526cd91476227a257343277"></a>
## span

`function` · `sqlparser::ast::ddl::DropOperatorFamily::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5057, 1], "end": [5061, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:5058`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d45f2414b2e9643732dc7d70"></a>
## using

`struct_field` · `sqlparser::ast::ddl::DropOperatorFamily::using` · sqlparser 0.62.0

```rust
using: ast::Ident
```

Source: `src/ast/ddl.rs:5037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index method (btree, hash, gist, gin, etc.)

<a id="op-7a25e1744925ff81e7580983"></a>
## visit

`function` · `sqlparser::ast::ddl::DropOperatorFamily::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5030, 47], "end": [5030, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5030`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8a152846176ed368944e45b"></a>
## visit

`function` · `sqlparser::ast::ddl::DropOperatorFamily::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5030, 40], "end": [5030, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5030`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
