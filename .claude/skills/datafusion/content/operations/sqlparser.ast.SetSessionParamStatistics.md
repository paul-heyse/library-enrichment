# `sqlparser::ast::SetSessionParamStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SetSessionParamStatistics.json).

<a id="op-0049d0dfc37a3fc64fa95176"></a>
## SetSessionParamStatistics

`struct` · `sqlparser::ast::SetSessionParamStatistics` · sqlparser 0.62.0

```rust
struct SetSessionParamStatistics
```

Source: `src/ast/mod.rs:11161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Statistics-related session parameter specifying topic and value.

<a id="op-423eae3b619ca8437036d608"></a>
## clone

`function` · `sqlparser::ast::SetSessionParamStatistics::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetSessionParamStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11157, 17], "end": [11157, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db91b27ef70de05ce8f1a362"></a>
## cmp

`function` · `sqlparser::ast::SetSessionParamStatistics::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetSessionParamStatistics) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11157, 51], "end": [11157, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6771ef20a2471ae6c3af70fd"></a>
## deserialize

`function` · `sqlparser::ast::SetSessionParamStatistics::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11158, 49], "end": [11158, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11158`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-617409096d858706a26c90ac"></a>
## eq

`function` · `sqlparser::ast::SetSessionParamStatistics::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetSessionParamStatistics) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11157, 24], "end": [11157, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ea6bcfcb0690ebccef85e62"></a>
## fmt

`function` · `sqlparser::ast::SetSessionParamStatistics::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11157, 10], "end": [11157, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-daadebda69ec0ba5b1238e2d"></a>
## fmt

`function` · `sqlparser::ast::SetSessionParamStatistics::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11168, 1], "end": [11172, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11169`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9414658f166aa51ead3f9c83"></a>
## hash

`function` · `sqlparser::ast::SetSessionParamStatistics::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11157, 56], "end": [11157, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32caabe01aa76fa7d854e8d8"></a>
## partial_cmp

`function` · `sqlparser::ast::SetSessionParamStatistics::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetSessionParamStatistics) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11157, 35], "end": [11157, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b31996d7b7c6ffafcb45c4c0"></a>
## serialize

`function` · `sqlparser::ast::SetSessionParamStatistics::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11158, 38], "end": [11158, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11158`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5aaa5c73fe5d3e1fea83e0c2"></a>
## topic

`struct_field` · `sqlparser::ast::SetSessionParamStatistics::topic` · sqlparser 0.62.0

```rust
topic: SessionParamStatsTopic
```

Source: `src/ast/mod.rs:11163`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Statistics topic to set (IO/PROFILE/TIME/XML).

<a id="op-7926ab5cd8483bb26e7b2d95"></a>
## value

`struct_field` · `sqlparser::ast::SetSessionParamStatistics::value` · sqlparser 0.62.0

```rust
value: SessionParamValue
```

Source: `src/ast/mod.rs:11165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Value (ON/OFF) for the statistics topic.

<a id="op-d3ecc07c6d5bfe8ad5a4b800"></a>
## visit

`function` · `sqlparser::ast::SetSessionParamStatistics::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11159, 47], "end": [11159, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f344e0ace1659efe36dc7f42"></a>
## visit

`function` · `sqlparser::ast::SetSessionParamStatistics::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamStatistics", "path": "SetSessionParamStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11159, 40], "end": [11159, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
