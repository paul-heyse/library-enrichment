# `sqlparser::ast::HiveRowFormat`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HiveRowFormat.json).

<a id="op-5f2aa0c0929ce61aa4e0c372"></a>
## HiveRowFormat

`enum` · `sqlparser::ast::HiveRowFormat` · sqlparser 0.62.0

```rust
enum HiveRowFormat
```

Source: `src/ast/mod.rs:8562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Row format specification for Hive tables (SERDE or DELIMITED).

<a id="op-5dc7d8e8d44432c1cffe3cef"></a>
## DELIMITED

`variant` · `sqlparser::ast::HiveRowFormat::DELIMITED` · sqlparser 0.62.0

```rust
DELIMITED
```

Source: `src/ast/mod.rs:8569`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Delimited row format with one or more delimiter specifications.

<a id="op-9c82815180e70a08f53ff75a"></a>
## SERDE

`variant` · `sqlparser::ast::HiveRowFormat::SERDE` · sqlparser 0.62.0

```rust
SERDE
```

Source: `src/ast/mod.rs:8564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SerDe class specification with the implementing class name.

<a id="op-85a686477a7e4fdd6cfc4c9f"></a>
## clone

`function` · `sqlparser::ast::HiveRowFormat::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HiveRowFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowFormat", "path": "HiveRowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8558, 17], "end": [8558, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8558`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4797c533e80ba3fb4851e09"></a>
## cmp

`function` · `sqlparser::ast::HiveRowFormat::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HiveRowFormat) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowFormat", "path": "HiveRowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8558, 51], "end": [8558, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8558`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25d7d1a69ee3fb9da12b6632"></a>
## deserialize

`function` · `sqlparser::ast::HiveRowFormat::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowFormat", "path": "HiveRowFormat"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8559, 49], "end": [8559, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8559`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d583b645fba53057de4f0408"></a>
## eq

`function` · `sqlparser::ast::HiveRowFormat::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HiveRowFormat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowFormat", "path": "HiveRowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8558, 24], "end": [8558, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8558`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-100a63b22fd7ffc89d33e445"></a>
## fmt

`function` · `sqlparser::ast::HiveRowFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowFormat", "path": "HiveRowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8558, 10], "end": [8558, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8558`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46da93a9003d457e93c14483"></a>
## hash

`function` · `sqlparser::ast::HiveRowFormat::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowFormat", "path": "HiveRowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8558, 56], "end": [8558, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8558`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d1ca869f193de134cc8ab16"></a>
## partial_cmp

`function` · `sqlparser::ast::HiveRowFormat::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HiveRowFormat) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowFormat", "path": "HiveRowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8558, 35], "end": [8558, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8558`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96be30627c8cbd34f0c1342e"></a>
## serialize

`function` · `sqlparser::ast::HiveRowFormat::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowFormat", "path": "HiveRowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8559, 38], "end": [8559, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8559`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ea6476c9c53eb3bf4be0455"></a>
## visit

`function` · `sqlparser::ast::HiveRowFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowFormat", "path": "HiveRowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8560, 40], "end": [8560, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b19261a2307775f565626a2"></a>
## visit

`function` · `sqlparser::ast::HiveRowFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowFormat", "path": "HiveRowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8560, 47], "end": [8560, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
