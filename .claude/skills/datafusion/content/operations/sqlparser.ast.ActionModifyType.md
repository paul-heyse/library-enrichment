# `sqlparser::ast::ActionModifyType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ActionModifyType.json).

<a id="op-df59eaccd51777f9bafb70a3"></a>
## ActionModifyType

`enum` · `sqlparser::ast::ActionModifyType` · sqlparser 0.62.0

```rust
enum ActionModifyType
```

Source: `src/ast/mod.rs:7361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/sql/grant-privilege>
under `globalPrivileges` in the `MODIFY` privilege.

<a id="op-d51e2a38e82938e93a042518"></a>
## LogLevel

`variant` · `sqlparser::ast::ActionModifyType::LogLevel` · sqlparser 0.62.0

```rust
LogLevel
```

Source: `src/ast/mod.rs:7363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modify log level.

<a id="op-65e737f230c709e9d9f5f064"></a>
## SessionLogLevel

`variant` · `sqlparser::ast::ActionModifyType::SessionLogLevel` · sqlparser 0.62.0

```rust
SessionLogLevel
```

Source: `src/ast/mod.rs:7367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modify session log level.

<a id="op-151f0bf07d7945c5be55c8e2"></a>
## SessionTraceLevel

`variant` · `sqlparser::ast::ActionModifyType::SessionTraceLevel` · sqlparser 0.62.0

```rust
SessionTraceLevel
```

Source: `src/ast/mod.rs:7369`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modify session trace level.

<a id="op-f717c87453f441f2f70b0b9d"></a>
## TraceLevel

`variant` · `sqlparser::ast::ActionModifyType::TraceLevel` · sqlparser 0.62.0

```rust
TraceLevel
```

Source: `src/ast/mod.rs:7365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modify trace level.

<a id="op-6744f4d3cd891d5a03b7395e"></a>
## clone

`function` · `sqlparser::ast::ActionModifyType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ActionModifyType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7356, 17], "end": [7356, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb1dbd14b5b89f4bda6a14d0"></a>
## cmp

`function` · `sqlparser::ast::ActionModifyType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ActionModifyType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7356, 51], "end": [7356, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caf653416b68ff983afdc44c"></a>
## deserialize

`function` · `sqlparser::ast::ActionModifyType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7357, 49], "end": [7357, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7d72faf42667293cbb9cc00"></a>
## eq

`function` · `sqlparser::ast::ActionModifyType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ActionModifyType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7356, 24], "end": [7356, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14b4ce200f7b109b05488b61"></a>
## fmt

`function` · `sqlparser::ast::ActionModifyType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7372, 1], "end": [7381, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7373`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f1fb5440a9f9544cb31a5d9"></a>
## fmt

`function` · `sqlparser::ast::ActionModifyType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7356, 10], "end": [7356, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec4328badcac2eddba7ea591"></a>
## hash

`function` · `sqlparser::ast::ActionModifyType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7356, 56], "end": [7356, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1d48c5d2aaa96d7a67ec272"></a>
## partial_cmp

`function` · `sqlparser::ast::ActionModifyType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ActionModifyType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7356, 35], "end": [7356, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19a54886bdca9baf0a72badd"></a>
## serialize

`function` · `sqlparser::ast::ActionModifyType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7357, 38], "end": [7357, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23457f1a3a03180e9525477b"></a>
## visit

`function` · `sqlparser::ast::ActionModifyType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7358, 40], "end": [7358, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7358`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4856af74013bb51e0d700208"></a>
## visit

`function` · `sqlparser::ast::ActionModifyType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionModifyType", "path": "ActionModifyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7358, 47], "end": [7358, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7358`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
