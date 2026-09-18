# `sqlparser::ast::FunctionArguments`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionArguments.json).

<a id="op-6aac3cf81821657c8b4900ce"></a>
## FunctionArguments

`enum` · `sqlparser::ast::FunctionArguments` · sqlparser 0.62.0

```rust
enum FunctionArguments
```

Source: `src/ast/mod.rs:8138`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The arguments passed to a function call.

<a id="op-1f95f529484538814b07b61b"></a>
## List

`variant` · `sqlparser::ast::FunctionArguments::List` · sqlparser 0.62.0

```rust
List
```

Source: `src/ast/mod.rs:8147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A normal function argument list, including any clauses within it such as
`DISTINCT` or `ORDER BY`.

<a id="op-0456770ea428bb8f60aa313a"></a>
## None

`variant` · `sqlparser::ast::FunctionArguments::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/mod.rs:8141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Used for special functions like `CURRENT_TIMESTAMP` that are invoked
without parentheses.

<a id="op-c4a5b6d97dddae1cd806be20"></a>
## Subquery

`variant` · `sqlparser::ast::FunctionArguments::Subquery` · sqlparser 0.62.0

```rust
Subquery
```

Source: `src/ast/mod.rs:8144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

On some dialects, a subquery can be passed without surrounding
parentheses if it's the sole argument to the function.

<a id="op-2508736ec9807cfb75c24caf"></a>
## clone

`function` · `sqlparser::ast::FunctionArguments::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionArguments
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8135, 17], "end": [8135, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a79667bedfa4000120f544fd"></a>
## cmp

`function` · `sqlparser::ast::FunctionArguments::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionArguments) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8135, 51], "end": [8135, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33d114f48e27a3cb74ae0434"></a>
## deserialize

`function` · `sqlparser::ast::FunctionArguments::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8136, 49], "end": [8136, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36074ff92bf585c486a5cc3f"></a>
## eq

`function` · `sqlparser::ast::FunctionArguments::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionArguments) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8135, 24], "end": [8135, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a561deaa560ab12cf6c6058"></a>
## fmt

`function` · `sqlparser::ast::FunctionArguments::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8135, 10], "end": [8135, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca8b04a457bb993734be4683"></a>
## fmt

`function` · `sqlparser::ast::FunctionArguments::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8150, 1], "end": [8158, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8151`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a15d25247de6d655b86dbfc5"></a>
## hash

`function` · `sqlparser::ast::FunctionArguments::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8135, 56], "end": [8135, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb42d3c5cd5cb61a76807664"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionArguments::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionArguments) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8135, 35], "end": [8135, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e1e4eee8dda54fee7d9458d"></a>
## serialize

`function` · `sqlparser::ast::FunctionArguments::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8136, 38], "end": [8136, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec3d1abde9de14f4d1508265"></a>
## span

`function` · `sqlparser::ast::FunctionArguments::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "super::FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1759, 1], "end": [1767, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1760`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c72850f3bb71cd772d78f75"></a>
## visit

`function` · `sqlparser::ast::FunctionArguments::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8137, 40], "end": [8137, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fb9b11f02e251f3ce5584b1"></a>
## visit

`function` · `sqlparser::ast::FunctionArguments::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArguments", "path": "FunctionArguments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8137, 47], "end": [8137, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
