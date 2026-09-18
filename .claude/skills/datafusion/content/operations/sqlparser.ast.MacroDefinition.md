# `sqlparser::ast::MacroDefinition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.MacroDefinition.json).

<a id="op-e28a5d7c8e463304bbe8e103"></a>
## MacroDefinition

`enum` · `sqlparser::ast::MacroDefinition` · sqlparser 0.62.0

```rust
enum MacroDefinition
```

Source: `src/ast/mod.rs:10270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Definition for a DuckDB macro: either an expression or a table-producing query.

<a id="op-e105f91d3ef01f87b739fb51"></a>
## Expr

`variant` · `sqlparser::ast::MacroDefinition::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/ast/mod.rs:10272`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The macro is defined as an expression.

<a id="op-235f8f2f5f3501b0cd37deed"></a>
## Table

`variant` · `sqlparser::ast::MacroDefinition::Table` · sqlparser 0.62.0

```rust
Table
```

Source: `src/ast/mod.rs:10274`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The macro is defined as a table (query).

<a id="op-36b937449046f7848c1169c2"></a>
## clone

`function` · `sqlparser::ast::MacroDefinition::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MacroDefinition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10266, 17], "end": [10266, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fbdaa445079406b3c0b0579"></a>
## cmp

`function` · `sqlparser::ast::MacroDefinition::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MacroDefinition) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10266, 51], "end": [10266, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14a0747668bff1d82c8b6af9"></a>
## deserialize

`function` · `sqlparser::ast::MacroDefinition::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10267, 49], "end": [10267, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36b28aa6ae1c0d5de0c818ca"></a>
## eq

`function` · `sqlparser::ast::MacroDefinition::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MacroDefinition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10266, 24], "end": [10266, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-095fa57ea24dd3776e9f9b75"></a>
## fmt

`function` · `sqlparser::ast::MacroDefinition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10266, 10], "end": [10266, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb707eabc10d7d9e03342395"></a>
## fmt

`function` · `sqlparser::ast::MacroDefinition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10277, 1], "end": [10285, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10278`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c94d13e2bd2c8b998c7915a"></a>
## hash

`function` · `sqlparser::ast::MacroDefinition::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10266, 56], "end": [10266, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3f84ba0e8b0acc48172955c"></a>
## partial_cmp

`function` · `sqlparser::ast::MacroDefinition::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MacroDefinition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10266, 35], "end": [10266, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-269aed3217b5aee75df21f7c"></a>
## serialize

`function` · `sqlparser::ast::MacroDefinition::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10267, 38], "end": [10267, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-457994d105e0ca55494b74ef"></a>
## visit

`function` · `sqlparser::ast::MacroDefinition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10268, 47], "end": [10268, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b465c97d8a413d76bdefa958"></a>
## visit

`function` · `sqlparser::ast::MacroDefinition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MacroDefinition", "path": "MacroDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10268, 40], "end": [10268, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
