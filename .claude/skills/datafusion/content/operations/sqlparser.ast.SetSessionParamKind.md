# `sqlparser::ast::SetSessionParamKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SetSessionParamKind.json).

<a id="op-e4c7fcae5616b94401aee64c"></a>
## SetSessionParamKind

`enum` · `sqlparser::ast::SetSessionParamKind` · sqlparser 0.62.0

```rust
enum SetSessionParamKind
```

Source: `src/ast/mod.rs:11079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Kind of session parameter being set by `SET SESSION`.

<a id="op-e3859b7884d758a1023cc90a"></a>
## Generic

`variant` · `sqlparser::ast::SetSessionParamKind::Generic` · sqlparser 0.62.0

```rust
Generic
```

Source: `src/ast/mod.rs:11081`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Generic session parameter (name/value pair).

<a id="op-a5a2766247e9ed3c72cb4e36"></a>
## IdentityInsert

`variant` · `sqlparser::ast::SetSessionParamKind::IdentityInsert` · sqlparser 0.62.0

```rust
IdentityInsert
```

Source: `src/ast/mod.rs:11083`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Identity insert related parameter.

<a id="op-480d95ba04b08d14116b2562"></a>
## Offsets

`variant` · `sqlparser::ast::SetSessionParamKind::Offsets` · sqlparser 0.62.0

```rust
Offsets
```

Source: `src/ast/mod.rs:11085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Offsets-related parameter.

<a id="op-f105079733c58d91c123e47c"></a>
## Statistics

`variant` · `sqlparser::ast::SetSessionParamKind::Statistics` · sqlparser 0.62.0

```rust
Statistics
```

Source: `src/ast/mod.rs:11087`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Statistics-related parameter.

<a id="op-f363d97d9dfd47fcb1fb4cff"></a>
## clone

`function` · `sqlparser::ast::SetSessionParamKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetSessionParamKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11075, 17], "end": [11075, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec781d95d75fcf89d3f7b57a"></a>
## cmp

`function` · `sqlparser::ast::SetSessionParamKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetSessionParamKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11075, 51], "end": [11075, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a47a56028520e328c653a654"></a>
## deserialize

`function` · `sqlparser::ast::SetSessionParamKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11076, 49], "end": [11076, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11076`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0047ca6d845a0f2b165df7c2"></a>
## eq

`function` · `sqlparser::ast::SetSessionParamKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetSessionParamKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11075, 24], "end": [11075, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28db8d821f24f11d88b9dc01"></a>
## fmt

`function` · `sqlparser::ast::SetSessionParamKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11090, 1], "end": [11099, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11091`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e38f127bfa0e99c246d2cc2d"></a>
## fmt

`function` · `sqlparser::ast::SetSessionParamKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11075, 10], "end": [11075, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d16d9617d294e59dda808e8"></a>
## hash

`function` · `sqlparser::ast::SetSessionParamKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11075, 56], "end": [11075, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db1757a3aba5edd7e3e22c0f"></a>
## partial_cmp

`function` · `sqlparser::ast::SetSessionParamKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetSessionParamKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11075, 35], "end": [11075, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb6c37c1017e37ebf958856a"></a>
## serialize

`function` · `sqlparser::ast::SetSessionParamKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11076, 38], "end": [11076, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11076`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6586c918b2cddac4e8d1bcfe"></a>
## visit

`function` · `sqlparser::ast::SetSessionParamKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11077, 40], "end": [11077, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11077`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0a725229b352cbf88af27aa"></a>
## visit

`function` · `sqlparser::ast::SetSessionParamKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamKind", "path": "SetSessionParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11077, 47], "end": [11077, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11077`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
