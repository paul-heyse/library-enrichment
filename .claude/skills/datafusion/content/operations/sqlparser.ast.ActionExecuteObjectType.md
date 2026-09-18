# `sqlparser::ast::ActionExecuteObjectType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ActionExecuteObjectType.json).

<a id="op-68f3ad5cdca1b91b947890e7"></a>
## ActionExecuteObjectType

`enum` · `sqlparser::ast::ActionExecuteObjectType` · sqlparser 0.62.0

```rust
enum ActionExecuteObjectType
```

Source: `src/ast/mod.rs:7295`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/sql/grant-privilege>
under `globalPrivileges` in the `EXECUTE` privilege.

<a id="op-714cefab5aa5bb60b8a7cb77"></a>
## Alert

`variant` · `sqlparser::ast::ActionExecuteObjectType::Alert` · sqlparser 0.62.0

```rust
Alert
```

Source: `src/ast/mod.rs:7297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Alert object.

<a id="op-1fbd386f2cd31604e72f0aa5"></a>
## DataMetricFunction

`variant` · `sqlparser::ast::ActionExecuteObjectType::DataMetricFunction` · sqlparser 0.62.0

```rust
DataMetricFunction
```

Source: `src/ast/mod.rs:7299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Data metric function object.

<a id="op-6bbdb47343bd71d0e9e72ad5"></a>
## ManagedAlert

`variant` · `sqlparser::ast::ActionExecuteObjectType::ManagedAlert` · sqlparser 0.62.0

```rust
ManagedAlert
```

Source: `src/ast/mod.rs:7301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Managed alert object.

<a id="op-6e0d71376b271168c29161e0"></a>
## ManagedTask

`variant` · `sqlparser::ast::ActionExecuteObjectType::ManagedTask` · sqlparser 0.62.0

```rust
ManagedTask
```

Source: `src/ast/mod.rs:7303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Managed task object.

<a id="op-39fe182c13e078d9dfc23547"></a>
## Task

`variant` · `sqlparser::ast::ActionExecuteObjectType::Task` · sqlparser 0.62.0

```rust
Task
```

Source: `src/ast/mod.rs:7305`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Task object.

<a id="op-531d84b9b5280947bf2a15e0"></a>
## clone

`function` · `sqlparser::ast::ActionExecuteObjectType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ActionExecuteObjectType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7290, 17], "end": [7290, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0833ceadfd2e62dbb781b807"></a>
## cmp

`function` · `sqlparser::ast::ActionExecuteObjectType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ActionExecuteObjectType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7290, 51], "end": [7290, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8953f07a7a867dd3b73e3056"></a>
## deserialize

`function` · `sqlparser::ast::ActionExecuteObjectType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7291, 49], "end": [7291, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7c5505fe3487cc932c7a789"></a>
## eq

`function` · `sqlparser::ast::ActionExecuteObjectType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ActionExecuteObjectType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7290, 24], "end": [7290, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-399d9863f00494fb9c844109"></a>
## fmt

`function` · `sqlparser::ast::ActionExecuteObjectType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7308, 1], "end": [7318, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7309`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c03e18dd6ee997d3e4fd3ee0"></a>
## fmt

`function` · `sqlparser::ast::ActionExecuteObjectType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7290, 10], "end": [7290, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-138e2738618907ef05bd2194"></a>
## hash

`function` · `sqlparser::ast::ActionExecuteObjectType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7290, 56], "end": [7290, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5329794d70f04c221ccfee3"></a>
## partial_cmp

`function` · `sqlparser::ast::ActionExecuteObjectType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ActionExecuteObjectType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7290, 35], "end": [7290, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0df86c894615ea69b6b64fe3"></a>
## serialize

`function` · `sqlparser::ast::ActionExecuteObjectType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7291, 38], "end": [7291, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-958bf26121558a55d8bed72c"></a>
## visit

`function` · `sqlparser::ast::ActionExecuteObjectType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7292, 40], "end": [7292, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df5770c316bc81bbf8e99eae"></a>
## visit

`function` · `sqlparser::ast::ActionExecuteObjectType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionExecuteObjectType", "path": "ActionExecuteObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7292, 47], "end": [7292, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
