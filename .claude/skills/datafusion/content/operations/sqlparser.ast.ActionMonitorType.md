# `sqlparser::ast::ActionMonitorType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ActionMonitorType.json).

<a id="op-aead6db890d157343ee77405"></a>
## ActionMonitorType

`enum` · `sqlparser::ast::ActionMonitorType` · sqlparser 0.62.0

```rust
enum ActionMonitorType
```

Source: `src/ast/mod.rs:7388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/sql/grant-privilege>
under `globalPrivileges` in the `MONITOR` privilege.

<a id="op-104b1c9f9ef74f9dff926fb0"></a>
## Execution

`variant` · `sqlparser::ast::ActionMonitorType::Execution` · sqlparser 0.62.0

```rust
Execution
```

Source: `src/ast/mod.rs:7390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Monitor execution.

<a id="op-f870e1f87f2284f1d81fb8f8"></a>
## Security

`variant` · `sqlparser::ast::ActionMonitorType::Security` · sqlparser 0.62.0

```rust
Security
```

Source: `src/ast/mod.rs:7392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Monitor security.

<a id="op-ae6e0756066c07b1982158f9"></a>
## Usage

`variant` · `sqlparser::ast::ActionMonitorType::Usage` · sqlparser 0.62.0

```rust
Usage
```

Source: `src/ast/mod.rs:7394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Monitor usage.

<a id="op-25c77f0bd0b2b48eeb892a9a"></a>
## clone

`function` · `sqlparser::ast::ActionMonitorType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ActionMonitorType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7383, 17], "end": [7383, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7383`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0faea58582f5e82f0dbf2fc"></a>
## cmp

`function` · `sqlparser::ast::ActionMonitorType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ActionMonitorType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7383, 51], "end": [7383, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7383`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00e0359a6189688f2647b6a1"></a>
## deserialize

`function` · `sqlparser::ast::ActionMonitorType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7384, 49], "end": [7384, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfaeb1b9afb8edc6436159b4"></a>
## eq

`function` · `sqlparser::ast::ActionMonitorType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ActionMonitorType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7383, 24], "end": [7383, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7383`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c78ecb5c5f63791b708778f"></a>
## fmt

`function` · `sqlparser::ast::ActionMonitorType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7397, 1], "end": [7405, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7398`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2ecc0cbab2f01ab3f2c6a34"></a>
## fmt

`function` · `sqlparser::ast::ActionMonitorType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7383, 10], "end": [7383, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7383`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9e6852cacf593eb6e8c51ff"></a>
## hash

`function` · `sqlparser::ast::ActionMonitorType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7383, 56], "end": [7383, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7383`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18af33ae4444a8454f3e3740"></a>
## partial_cmp

`function` · `sqlparser::ast::ActionMonitorType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ActionMonitorType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7383, 35], "end": [7383, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7383`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31a1b1da16ec0731d34c9cdc"></a>
## serialize

`function` · `sqlparser::ast::ActionMonitorType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7384, 38], "end": [7384, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfb9de4cdbfa1693460a29ed"></a>
## visit

`function` · `sqlparser::ast::ActionMonitorType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7385, 40], "end": [7385, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7385`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebba56f2c2519bba9d65e335"></a>
## visit

`function` · `sqlparser::ast::ActionMonitorType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionMonitorType", "path": "ActionMonitorType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7385, 47], "end": [7385, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7385`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
