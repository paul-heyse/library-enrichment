# `sqlparser::ast::ddl::CreateOperatorFamily`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateOperatorFamily.json).

<a id="op-13422976d883ab714093da11"></a>
## CreateOperatorFamily

`struct` · `sqlparser::ast::ddl::CreateOperatorFamily` · sqlparser 0.62.0

```rust
struct CreateOperatorFamily
```

Source: `src/ast/ddl.rs:4772`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE OPERATOR FAMILY statement
See <https://www.postgresql.org/docs/current/sql-createopfamily.html>

<a id="op-ebea676007dc3b431b42be1c"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateOperatorFamily
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4769, 17], "end": [4769, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54188d03491b7944ac24baf0"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateOperatorFamily) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4769, 51], "end": [4769, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81029794af96408173708485"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4770, 49], "end": [4770, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4770`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eef6371c7c43204b48e37b33"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateOperatorFamily) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4769, 24], "end": [4769, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c5a8e359624d825dfb97674"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4769, 10], "end": [4769, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c023343d43fd9b80ddd8aabe"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4826, 1], "end": [4834, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03df06a87bfcde71c2679d9c"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4769, 56], "end": [4769, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05a5c1fff0cc5a0b7a38a954"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateOperatorFamily::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator family name (can be schema-qualified)

<a id="op-c4a831174069a3c64d2065e8"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateOperatorFamily) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4769, 35], "end": [4769, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-423b791445db918c974cef11"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4770, 38], "end": [4770, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4770`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdc020763d480e1e1aeb6e11"></a>
## span

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "crate::ast::CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2501, 1], "end": [2505, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a342f196f7dd1311f78801b"></a>
## using

`struct_field` · `sqlparser::ast::ddl::CreateOperatorFamily::using` · sqlparser 0.62.0

```rust
using: ast::Ident
```

Source: `src/ast/ddl.rs:4776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index method (btree, hash, gist, gin, etc.)

<a id="op-05e81466f1bf39588088b117"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4771, 40], "end": [4771, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13cd1ba43be10df26ea0151b"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateOperatorFamily::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4771, 47], "end": [4771, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
