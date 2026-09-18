# `sqlparser::ast::trigger::TriggerExecBody`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.trigger.TriggerExecBody.json).

<a id="op-1249b45cc6794be0f999c8c0"></a>
## TriggerExecBody

`struct` · `sqlparser::ast::trigger::TriggerExecBody` · sqlparser 0.62.0

```rust
struct TriggerExecBody
```

Source: `src/ast/trigger.rs:168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This keyword immediately precedes the declaration of one or two relation names that provide access to the transition relations of the triggering statement

<a id="op-ca3b571d08daefeb4af491d7"></a>
## clone

`function` · `sqlparser::ast::trigger::TriggerExecBody::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TriggerExecBody
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 17], "end": [165, 22], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/trigger.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aba6b549138c940ac14d7f44"></a>
## cmp

`function` · `sqlparser::ast::trigger::TriggerExecBody::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TriggerExecBody) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 51], "end": [165, 54], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/trigger.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17c54c8a2bdf76345b7270f5"></a>
## deserialize

`function` · `sqlparser::ast::trigger::TriggerExecBody::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 49], "end": [166, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/trigger.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea060329230fbed5a0fee6fd"></a>
## eq

`function` · `sqlparser::ast::trigger::TriggerExecBody::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TriggerExecBody) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 24], "end": [165, 33], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/trigger.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a385e1cbfe543173f481cfa6"></a>
## exec_type

`struct_field` · `sqlparser::ast::trigger::TriggerExecBody::exec_type` · sqlparser 0.62.0

```rust
exec_type: TriggerExecBodyType
```

Source: `src/ast/trigger.rs:170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the body is a `FUNCTION` or `PROCEDURE` invocation.

<a id="op-04b76b2dd31ac664a4b314bc"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerExecBody::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 10], "end": [165, 15], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/trigger.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-733e1f6de05420bc48e287fd"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerExecBody::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [184, 2], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/trigger.rs:176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8886995e4cdd53b2feb79450"></a>
## func_desc

`struct_field` · `sqlparser::ast::trigger::TriggerExecBody::func_desc` · sqlparser 0.62.0

```rust
func_desc: FunctionDesc
```

Source: `src/ast/trigger.rs:172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Description of the function/procedure to execute.

<a id="op-c22fd0a48c984fbf452a044c"></a>
## hash

`function` · `sqlparser::ast::trigger::TriggerExecBody::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 56], "end": [165, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/trigger.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e8cf13a9a211962186abc2f"></a>
## partial_cmp

`function` · `sqlparser::ast::trigger::TriggerExecBody::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TriggerExecBody) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 35], "end": [165, 45], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/trigger.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-238e1af291467882270713b1"></a>
## serialize

`function` · `sqlparser::ast::trigger::TriggerExecBody::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 38], "end": [166, 47], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/trigger.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35ab17dc396146b5db9706f0"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerExecBody::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 47], "end": [167, 55], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/trigger.rs:167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71bcd3c0d5e9d716b722c142"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerExecBody::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBody", "path": "TriggerExecBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 40], "end": [167, 45], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/trigger.rs:167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
