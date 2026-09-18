# `sqlparser::ast::FunctionDeterminismSpecifier`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionDeterminismSpecifier.json).

<a id="op-ec6a947d828e77e886abd035"></a>
## FunctionDeterminismSpecifier

`enum` · `sqlparser::ast::FunctionDeterminismSpecifier` · sqlparser 0.62.0

```rust
enum FunctionDeterminismSpecifier
```

Source: `src/ast/mod.rs:10088`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[BigQuery] Determinism specifier used in a UDF definition.

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#syntax_11

<a id="op-86784798641c49fb5d732dd2"></a>
## Deterministic

`variant` · `sqlparser::ast::FunctionDeterminismSpecifier::Deterministic` · sqlparser 0.62.0

```rust
Deterministic
```

Source: `src/ast/mod.rs:10090`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function is deterministic.

<a id="op-6ce69b53c9f0b834ae968441"></a>
## NotDeterministic

`variant` · `sqlparser::ast::FunctionDeterminismSpecifier::NotDeterministic` · sqlparser 0.62.0

```rust
NotDeterministic
```

Source: `src/ast/mod.rs:10092`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function is not deterministic.

<a id="op-639421d40d09ceaf6d2ed580"></a>
## clone

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionDeterminismSpecifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10085, 17], "end": [10085, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30a07ef16284f20f0275608c"></a>
## cmp

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionDeterminismSpecifier) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10085, 51], "end": [10085, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f348f4d525d05052a3a00b45"></a>
## deserialize

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10086, 49], "end": [10086, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73b960645145c311448837fa"></a>
## eq

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionDeterminismSpecifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10085, 24], "end": [10085, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d0dd66823ce47f633b18dba"></a>
## fmt

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10095, 1], "end": [10106, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10096`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-895467e8e459230b4feb83b7"></a>
## fmt

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10085, 10], "end": [10085, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62e2925ec71d9ed2d75d2eb3"></a>
## hash

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10085, 56], "end": [10085, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-772fc2191549c6ac6b646872"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionDeterminismSpecifier) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10085, 35], "end": [10085, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c532bf0f0d0aed526bbbf2d"></a>
## serialize

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10086, 38], "end": [10086, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69ee6c1432abfd1147e2dbca"></a>
## visit

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10087, 47], "end": [10087, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10087`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82dbbdcdd7d1862efb4ace56"></a>
## visit

`function` · `sqlparser::ast::FunctionDeterminismSpecifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDeterminismSpecifier", "path": "FunctionDeterminismSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10087, 40], "end": [10087, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10087`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
