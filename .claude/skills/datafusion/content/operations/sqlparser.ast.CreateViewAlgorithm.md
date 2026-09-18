# `sqlparser::ast::CreateViewAlgorithm`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateViewAlgorithm.json).

<a id="op-824a6f6d1f7d9ce4393b6a51"></a>
## CreateViewAlgorithm

`enum` · `sqlparser::ast::CreateViewAlgorithm` · sqlparser 0.62.0

```rust
enum CreateViewAlgorithm
```

Source: `src/ast/mod.rs:10474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL `CREATE VIEW` algorithm parameter: [ALGORITHM = {UNDEFINED | MERGE | TEMPTABLE}]
MySQL `CREATE VIEW` algorithm options.

<a id="op-b979218ae95c24fef7bfdf1d"></a>
## Merge

`variant` · `sqlparser::ast::CreateViewAlgorithm::Merge` · sqlparser 0.62.0

```rust
Merge
```

Source: `src/ast/mod.rs:10478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MERGE` algorithm.

<a id="op-602d8976f9dd8ff138ff546d"></a>
## TempTable

`variant` · `sqlparser::ast::CreateViewAlgorithm::TempTable` · sqlparser 0.62.0

```rust
TempTable
```

Source: `src/ast/mod.rs:10480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TEMPTABLE` algorithm.

<a id="op-ffb1d2bc92d44b2e5d2171ae"></a>
## Undefined

`variant` · `sqlparser::ast::CreateViewAlgorithm::Undefined` · sqlparser 0.62.0

```rust
Undefined
```

Source: `src/ast/mod.rs:10476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`UNDEFINED` algorithm.

<a id="op-1ccb172e893458348625a7b8"></a>
## clone

`function` · `sqlparser::ast::CreateViewAlgorithm::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateViewAlgorithm
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10470, 17], "end": [10470, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6ea1202e8689009cd77c804"></a>
## cmp

`function` · `sqlparser::ast::CreateViewAlgorithm::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateViewAlgorithm) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10470, 51], "end": [10470, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e714c79697c9b3f2ee2d7a4b"></a>
## deserialize

`function` · `sqlparser::ast::CreateViewAlgorithm::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10471, 49], "end": [10471, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69dbcad9ea56365a7432ed29"></a>
## eq

`function` · `sqlparser::ast::CreateViewAlgorithm::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateViewAlgorithm) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10470, 24], "end": [10470, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cafed9a9562ced7cc0ede15"></a>
## fmt

`function` · `sqlparser::ast::CreateViewAlgorithm::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10470, 10], "end": [10470, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-596d326c87a8df863ea6e0c9"></a>
## fmt

`function` · `sqlparser::ast::CreateViewAlgorithm::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10483, 1], "end": [10491, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10484`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68f9af34bddfc796d7c81379"></a>
## hash

`function` · `sqlparser::ast::CreateViewAlgorithm::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10470, 56], "end": [10470, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e09858b9f4f2ae9e5402b8be"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateViewAlgorithm::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateViewAlgorithm) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10470, 35], "end": [10470, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c78a906bb4087869a1382733"></a>
## serialize

`function` · `sqlparser::ast::CreateViewAlgorithm::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10471, 38], "end": [10471, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-131156ca13115e7bace29ae1"></a>
## visit

`function` · `sqlparser::ast::CreateViewAlgorithm::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10472, 40], "end": [10472, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdce9f98dbd5a9b84841c746"></a>
## visit

`function` · `sqlparser::ast::CreateViewAlgorithm::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewAlgorithm", "path": "CreateViewAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10472, 47], "end": [10472, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
