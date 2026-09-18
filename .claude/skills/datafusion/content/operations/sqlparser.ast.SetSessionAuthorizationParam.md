# `sqlparser::ast::SetSessionAuthorizationParam`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SetSessionAuthorizationParam.json).

<a id="op-5ffbc69f70f4e9b33af4cb7e"></a>
## SetSessionAuthorizationParam

`struct` · `sqlparser::ast::SetSessionAuthorizationParam` · sqlparser 0.62.0

```rust
struct SetSessionAuthorizationParam
```

Source: `src/ast/mod.rs:11041`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a SET SESSION AUTHORIZATION statement

<a id="op-31760698e824ba92b3007b2c"></a>
## clone

`function` · `sqlparser::ast::SetSessionAuthorizationParam::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetSessionAuthorizationParam
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11038, 17], "end": [11038, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-055f8d6754c100c9cf732707"></a>
## cmp

`function` · `sqlparser::ast::SetSessionAuthorizationParam::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetSessionAuthorizationParam) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11038, 51], "end": [11038, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-265c4deb0256a38bbde17648"></a>
## deserialize

`function` · `sqlparser::ast::SetSessionAuthorizationParam::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11039, 49], "end": [11039, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11039`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05a9315c9ba8afb40729438d"></a>
## eq

`function` · `sqlparser::ast::SetSessionAuthorizationParam::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetSessionAuthorizationParam) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11038, 24], "end": [11038, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87e1774fb8de9291d71f6204"></a>
## fmt

`function` · `sqlparser::ast::SetSessionAuthorizationParam::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11038, 10], "end": [11038, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dffa9e9160f69116eda565b"></a>
## fmt

`function` · `sqlparser::ast::SetSessionAuthorizationParam::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11048, 1], "end": [11052, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11049`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb6732c00e789f023c97f6b5"></a>
## hash

`function` · `sqlparser::ast::SetSessionAuthorizationParam::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11038, 56], "end": [11038, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a259838cd863dab1ee685178"></a>
## kind

`struct_field` · `sqlparser::ast::SetSessionAuthorizationParam::kind` · sqlparser 0.62.0

```rust
kind: SetSessionAuthorizationParamKind
```

Source: `src/ast/mod.rs:11045`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The specific authorization parameter kind.

<a id="op-2445f0d506eaa3fc521d0f51"></a>
## partial_cmp

`function` · `sqlparser::ast::SetSessionAuthorizationParam::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetSessionAuthorizationParam) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11038, 35], "end": [11038, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b90d4c9a69b840c0c13f3ab"></a>
## scope

`struct_field` · `sqlparser::ast::SetSessionAuthorizationParam::scope` · sqlparser 0.62.0

```rust
scope: ContextModifier
```

Source: `src/ast/mod.rs:11043`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The scope for the `SET SESSION AUTHORIZATION` (e.g., GLOBAL/SESSION).

<a id="op-22e957e68bd5e9199c977a52"></a>
## serialize

`function` · `sqlparser::ast::SetSessionAuthorizationParam::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11039, 38], "end": [11039, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11039`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31d2cc78a68e37481994703f"></a>
## visit

`function` · `sqlparser::ast::SetSessionAuthorizationParam::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11040, 47], "end": [11040, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11040`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-852048e0d029478a1ab3840e"></a>
## visit

`function` · `sqlparser::ast::SetSessionAuthorizationParam::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParam", "path": "SetSessionAuthorizationParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11040, 40], "end": [11040, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11040`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
