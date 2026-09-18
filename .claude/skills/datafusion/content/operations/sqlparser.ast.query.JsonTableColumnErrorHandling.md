# `sqlparser::ast::query::JsonTableColumnErrorHandling`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.JsonTableColumnErrorHandling.json).

<a id="op-106f9cdd6000bc778e191ece"></a>
## JsonTableColumnErrorHandling

`enum` · `sqlparser::ast::query::JsonTableColumnErrorHandling` · sqlparser 0.62.0

```rust
enum JsonTableColumnErrorHandling
```

Source: `src/ast/query.rs:4094`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Stores the error handling clause of a `JSON_TABLE` table valued function:
{NULL | DEFAULT json_string | ERROR} ON {ERROR | EMPTY }
Error/empty-value handling for `JSON_TABLE` columns.

<a id="op-0f6cd588a4918bf8fe6f1812"></a>
## Default

`variant` · `sqlparser::ast::query::JsonTableColumnErrorHandling::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/query.rs:4098`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DEFAULT <value>` — use the provided `Value` as a default.

<a id="op-b7f0b35385895e0e96782caf"></a>
## Error

`variant` · `sqlparser::ast::query::JsonTableColumnErrorHandling::Error` · sqlparser 0.62.0

```rust
Error
```

Source: `src/ast/query.rs:4100`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ERROR` — raise an error.

<a id="op-b93e6d346423a3b1230fe2ed"></a>
## Null

`variant` · `sqlparser::ast::query::JsonTableColumnErrorHandling::Null` · sqlparser 0.62.0

```rust
Null
```

Source: `src/ast/query.rs:4096`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NULL` — return NULL when the path does not match.

<a id="op-ede7c93491f51b8d8be9be04"></a>
## clone

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> JsonTableColumnErrorHandling
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4090, 17], "end": [4090, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4090`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e80258ca45008ac759e90152"></a>
## cmp

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &JsonTableColumnErrorHandling) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4090, 51], "end": [4090, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4090`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38cabdb143472cad07bdea00"></a>
## deserialize

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4092, 49], "end": [4092, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4092`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9425f5c1f198d406388f1e59"></a>
## eq

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &JsonTableColumnErrorHandling) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4090, 24], "end": [4090, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4090`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccf1471c769429371daeb8d4"></a>
## fmt

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4103, 1], "end": [4113, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:4104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d74189a6f301c28e8166cbe4"></a>
## fmt

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4090, 10], "end": [4090, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4090`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4cf23de08e4c327ca69e8a7"></a>
## hash

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4090, 56], "end": [4090, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4090`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6520e92142c9adf4b8c9d047"></a>
## partial_cmp

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &JsonTableColumnErrorHandling) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4090, 35], "end": [4090, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4090`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3bf9c4354d367088e82db3c"></a>
## serialize

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4092, 38], "end": [4092, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4092`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f6225a807626a724e2c792e"></a>
## visit

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4091, 47], "end": [4091, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4091`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef0fcc8970541280c32682b4"></a>
## visit

`function` · `sqlparser::ast::query::JsonTableColumnErrorHandling::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumnErrorHandling", "path": "JsonTableColumnErrorHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4091, 40], "end": [4091, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4091`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
