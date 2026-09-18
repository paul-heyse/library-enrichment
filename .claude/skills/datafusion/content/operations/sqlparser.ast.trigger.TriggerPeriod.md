# `sqlparser::ast::trigger::TriggerPeriod`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.trigger.TriggerPeriod.json).

<a id="op-55a268be1ed21517db13692c"></a>
## TriggerPeriod

`enum` · `sqlparser::ast::trigger::TriggerPeriod` · sqlparser 0.62.0

```rust
enum TriggerPeriod
```

Source: `src/ast/trigger.rs:123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Trigger period

<a id="op-ed26cfdf7117d1076a8f8391"></a>
## After

`variant` · `sqlparser::ast::trigger::TriggerPeriod::After` · sqlparser 0.62.0

```rust
After
```

Source: `src/ast/trigger.rs:127`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The trigger fires once for the triggering SQL statement

<a id="op-cb2312b0f608034caa5c55ef"></a>
## Before

`variant` · `sqlparser::ast::trigger::TriggerPeriod::Before` · sqlparser 0.62.0

```rust
Before
```

Source: `src/ast/trigger.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The trigger fires before the triggering event

<a id="op-6598a0912e79e91d9d5bedda"></a>
## For

`variant` · `sqlparser::ast::trigger::TriggerPeriod::For` · sqlparser 0.62.0

```rust
For
```

Source: `src/ast/trigger.rs:125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The trigger fires once for each row affected by the triggering event

<a id="op-9fc33dffcb822d52759d5670"></a>
## InsteadOf

`variant` · `sqlparser::ast::trigger::TriggerPeriod::InsteadOf` · sqlparser 0.62.0

```rust
InsteadOf
```

Source: `src/ast/trigger.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The trigger fires instead of the triggering event

<a id="op-6b363fe9c8f585edd6f9797b"></a>
## clone

`function` · `sqlparser::ast::trigger::TriggerPeriod::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TriggerPeriod
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 17], "end": [120, 22], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/trigger.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19d9e7934e22b08e06800e7f"></a>
## cmp

`function` · `sqlparser::ast::trigger::TriggerPeriod::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TriggerPeriod) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 57], "end": [120, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/trigger.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a524eb35355c8966ad0eb05"></a>
## deserialize

`function` · `sqlparser::ast::trigger::TriggerPeriod::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 49], "end": [121, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/trigger.rs:121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7272c2158366ef4a006c38b"></a>
## eq

`function` · `sqlparser::ast::trigger::TriggerPeriod::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TriggerPeriod) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 30], "end": [120, 39], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/trigger.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-448043548979ef4a23c45634"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerPeriod::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 10], "end": [120, 15], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/trigger.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6f2155da7b171d9ec819b05"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerPeriod::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [143, 2], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/trigger.rs:135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67af21cf20054134d3e318fd"></a>
## hash

`function` · `sqlparser::ast::trigger::TriggerPeriod::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 62], "end": [120, 66], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/trigger.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6657c067cec544be96056504"></a>
## partial_cmp

`function` · `sqlparser::ast::trigger::TriggerPeriod::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TriggerPeriod) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 41], "end": [120, 51], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/trigger.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52de0407889f4362e4905808"></a>
## serialize

`function` · `sqlparser::ast::trigger::TriggerPeriod::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 38], "end": [121, 47], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/trigger.rs:121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-689886644979248437d9cd8c"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerPeriod::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 40], "end": [122, 45], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/trigger.rs:122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b015fab1432fd26e3fb490f8"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerPeriod::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerPeriod", "path": "TriggerPeriod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 47], "end": [122, 55], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/trigger.rs:122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
