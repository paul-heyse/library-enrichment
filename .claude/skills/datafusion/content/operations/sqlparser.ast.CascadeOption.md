# `sqlparser::ast::CascadeOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CascadeOption.json).

<a id="op-af8b7a15eab770b1ff06bb03"></a>
## CascadeOption

`enum` · `sqlparser::ast::CascadeOption` · sqlparser 0.62.0

```rust
enum CascadeOption
```

Source: `src/ast/mod.rs:6635`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Cascade/restrict option for Postgres TRUNCATE table, MySQL GRANT/REVOKE, etc.
[ CASCADE | RESTRICT ]

<a id="op-cfee627f8e3ad6c666d2fc70"></a>
## Cascade

`variant` · `sqlparser::ast::CascadeOption::Cascade` · sqlparser 0.62.0

```rust
Cascade
```

Source: `src/ast/mod.rs:6637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply cascading action (e.g., CASCADE).

<a id="op-8d2a7374b0761e5dbac23ef4"></a>
## Restrict

`variant` · `sqlparser::ast::CascadeOption::Restrict` · sqlparser 0.62.0

```rust
Restrict
```

Source: `src/ast/mod.rs:6639`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Restrict the action (e.g., RESTRICT).

<a id="op-93101d413284a322c6b9b8e5"></a>
## clone

`function` · `sqlparser::ast::CascadeOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CascadeOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6632, 17], "end": [6632, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6632`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8eece6a9f01aa8baa6988ea1"></a>
## cmp

`function` · `sqlparser::ast::CascadeOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CascadeOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6632, 51], "end": [6632, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6632`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afa9d7b49f195cc7aafc559f"></a>
## deserialize

`function` · `sqlparser::ast::CascadeOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6633, 49], "end": [6633, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8a4175201d478cd520d0599"></a>
## eq

`function` · `sqlparser::ast::CascadeOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CascadeOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6632, 24], "end": [6632, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6632`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-380b9c60ba263fb904de300c"></a>
## fmt

`function` · `sqlparser::ast::CascadeOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6642, 1], "end": [6649, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0543f81c5556c8e6d9a4115"></a>
## fmt

`function` · `sqlparser::ast::CascadeOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6632, 10], "end": [6632, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6632`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de9d9653c6c4eeb8a303e710"></a>
## hash

`function` · `sqlparser::ast::CascadeOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6632, 56], "end": [6632, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6632`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd8d0d02dadab86518fb727e"></a>
## partial_cmp

`function` · `sqlparser::ast::CascadeOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CascadeOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6632, 35], "end": [6632, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6632`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f6574773bbee20ef00a2b70"></a>
## serialize

`function` · `sqlparser::ast::CascadeOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6633, 38], "end": [6633, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6acc1b775df6ab8e9afd508d"></a>
## visit

`function` · `sqlparser::ast::CascadeOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6634, 40], "end": [6634, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6634`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce35c6c73f38b394cbe3976d"></a>
## visit

`function` · `sqlparser::ast::CascadeOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CascadeOption", "path": "CascadeOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6634, 47], "end": [6634, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6634`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
