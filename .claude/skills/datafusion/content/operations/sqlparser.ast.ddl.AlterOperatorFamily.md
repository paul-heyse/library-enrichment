# `sqlparser::ast::ddl::AlterOperatorFamily`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterOperatorFamily.json).

<a id="op-2199620fb97e7c93e0221bb9"></a>
## AlterOperatorFamily

`struct` · `sqlparser::ast::ddl::AlterOperatorFamily` · sqlparser 0.62.0

```rust
struct AlterOperatorFamily
```

Source: `src/ast/ddl.rs:5221`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALTER OPERATOR FAMILY` statement
See <https://www.postgresql.org/docs/current/sql-alteropfamily.html>

<a id="op-8b1c2a8d2a46b9118f3b4ccd"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterOperatorFamily
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5218, 17], "end": [5218, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfdfa2220f49d6086ad92ec0"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterOperatorFamily) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5218, 51], "end": [5218, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb2eab94fc538c477efccd90"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5219, 49], "end": [5219, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8d97cc2b5a7bc786a06fcd1"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterOperatorFamily) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5218, 24], "end": [5218, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bb6a9f48b3cf970e1659e1b"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5218, 10], "end": [5218, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e4af535dcb5eaae4cf98883"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5259, 1], "end": [5268, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0ae0f0b07698f4e146c3ea1"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5218, 56], "end": [5218, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5adc86fe9e5ea694a2b822a"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterOperatorFamily::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:5223`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator family name (can be schema-qualified)

<a id="op-aee3736ccae3b75714877f3a"></a>
## operation

`struct_field` · `sqlparser::ast::ddl::AlterOperatorFamily::operation` · sqlparser 0.62.0

```rust
operation: AlterOperatorFamilyOperation
```

Source: `src/ast/ddl.rs:5227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The operation to perform

<a id="op-9e2949c0dbf81009fe2c0941"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterOperatorFamily) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5218, 35], "end": [5218, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a11eb7ce8f16c504f9587a2"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5219, 38], "end": [5219, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd92fb21a7af7713b1e41058"></a>
## span

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5292, 1], "end": [5296, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:5293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6477430f265ee13dcae49d05"></a>
## using

`struct_field` · `sqlparser::ast::ddl::AlterOperatorFamily::using` · sqlparser 0.62.0

```rust
using: ast::Ident
```

Source: `src/ast/ddl.rs:5225`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index method (btree, hash, gist, gin, etc.)

<a id="op-5112f8de75c5772c57a4131b"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5220, 47], "end": [5220, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5220`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64a97952521f346daabbebff"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperatorFamily::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5220, 40], "end": [5220, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5220`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
