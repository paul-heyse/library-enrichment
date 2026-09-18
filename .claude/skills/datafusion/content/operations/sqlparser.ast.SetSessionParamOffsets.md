# `sqlparser::ast::SetSessionParamOffsets`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SetSessionParamOffsets.json).

<a id="op-8bac23f925df45633a9de2b5"></a>
## SetSessionParamOffsets

`struct` · `sqlparser::ast::SetSessionParamOffsets` · sqlparser 0.62.0

```rust
struct SetSessionParamOffsets
```

Source: `src/ast/mod.rs:11139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Offsets-related session parameter with keywords and a value.

<a id="op-34beda89dda2b0ec4e345fa7"></a>
## clone

`function` · `sqlparser::ast::SetSessionParamOffsets::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetSessionParamOffsets
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11135, 17], "end": [11135, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b72ad08cb6ca140b4ee2065f"></a>
## cmp

`function` · `sqlparser::ast::SetSessionParamOffsets::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetSessionParamOffsets) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11135, 51], "end": [11135, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-398e9219a967601b6291f58b"></a>
## deserialize

`function` · `sqlparser::ast::SetSessionParamOffsets::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11136, 49], "end": [11136, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63859efeca3771cd5d4e6795"></a>
## eq

`function` · `sqlparser::ast::SetSessionParamOffsets::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetSessionParamOffsets) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11135, 24], "end": [11135, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-689765a77911fc0439c34ea7"></a>
## fmt

`function` · `sqlparser::ast::SetSessionParamOffsets::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11146, 1], "end": [11155, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1dd8519e397236f2ae53a37"></a>
## fmt

`function` · `sqlparser::ast::SetSessionParamOffsets::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11135, 10], "end": [11135, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76ab182796c14b23c83f39f4"></a>
## hash

`function` · `sqlparser::ast::SetSessionParamOffsets::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11135, 56], "end": [11135, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7de19c4e911f852bd3d7068d"></a>
## keywords

`struct_field` · `sqlparser::ast::SetSessionParamOffsets::keywords` · sqlparser 0.62.0

```rust
keywords: Vec<String>
```

Source: `src/ast/mod.rs:11141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Keywords specifying which offsets to modify.

<a id="op-d1a167a068856c6a79c2e447"></a>
## partial_cmp

`function` · `sqlparser::ast::SetSessionParamOffsets::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetSessionParamOffsets) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11135, 35], "end": [11135, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-598f2084fd5c4b3c81cf9bfc"></a>
## serialize

`function` · `sqlparser::ast::SetSessionParamOffsets::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11136, 38], "end": [11136, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59e19787ca0420ab1b468c35"></a>
## value

`struct_field` · `sqlparser::ast::SetSessionParamOffsets::value` · sqlparser 0.62.0

```rust
value: SessionParamValue
```

Source: `src/ast/mod.rs:11143`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Value (ON/OFF) for the offsets setting.

<a id="op-88fcc0b2ffee583ee413838f"></a>
## visit

`function` · `sqlparser::ast::SetSessionParamOffsets::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11137, 47], "end": [11137, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad673e4b1c929e334bd77241"></a>
## visit

`function` · `sqlparser::ast::SetSessionParamOffsets::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamOffsets", "path": "SetSessionParamOffsets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11137, 40], "end": [11137, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
