# `sqlparser::ast::ddl::DeferrableInitial`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DeferrableInitial.json).

<a id="op-89e84bd0be6b0ec97fe86d11"></a>
## DeferrableInitial

`enum` · `sqlparser::ast::ddl::DeferrableInitial` · sqlparser 0.62.0

```rust
enum DeferrableInitial
```

Source: `src/ast/ddl.rs:2246`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Initial setting for deferrable constraints (`INITIALLY IMMEDIATE` or `INITIALLY DEFERRED`).

<a id="op-1756385422560cedb7f91f88"></a>
## Deferred

`variant` · `sqlparser::ast::ddl::DeferrableInitial::Deferred` · sqlparser 0.62.0

```rust
Deferred
```

Source: `src/ast/ddl.rs:2250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INITIALLY DEFERRED`

<a id="op-527b33b17f0bc31e5a4cbf8f"></a>
## Immediate

`variant` · `sqlparser::ast::ddl::DeferrableInitial::Immediate` · sqlparser 0.62.0

```rust
Immediate
```

Source: `src/ast/ddl.rs:2248`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INITIALLY IMMEDIATE`

<a id="op-f14624c5868887fa4ac22544"></a>
## clone

`function` · `sqlparser::ast::ddl::DeferrableInitial::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DeferrableInitial
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DeferrableInitial", "path": "DeferrableInitial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2243, 23], "end": [2243, 28], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0575ce163a60f271238b3a9a"></a>
## cmp

`function` · `sqlparser::ast::ddl::DeferrableInitial::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DeferrableInitial) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DeferrableInitial", "path": "DeferrableInitial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2243, 57], "end": [2243, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3492f14075820853edae3f76"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DeferrableInitial::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DeferrableInitial", "path": "DeferrableInitial"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2244, 49], "end": [2244, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d21ca3304fedee72a4c195fe"></a>
## eq

`function` · `sqlparser::ast::ddl::DeferrableInitial::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DeferrableInitial) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DeferrableInitial", "path": "DeferrableInitial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2243, 30], "end": [2243, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3127b27a13b04efc377d5fee"></a>
## fmt

`function` · `sqlparser::ast::ddl::DeferrableInitial::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DeferrableInitial", "path": "DeferrableInitial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2243, 10], "end": [2243, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0980c572255c9e3d27dd3ae3"></a>
## hash

`function` · `sqlparser::ast::ddl::DeferrableInitial::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DeferrableInitial", "path": "DeferrableInitial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2243, 62], "end": [2243, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c253207a21d1286c33cb0350"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DeferrableInitial::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DeferrableInitial) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DeferrableInitial", "path": "DeferrableInitial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2243, 41], "end": [2243, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-680d576cd8048004550f17a5"></a>
## serialize

`function` · `sqlparser::ast::ddl::DeferrableInitial::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DeferrableInitial", "path": "DeferrableInitial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2244, 38], "end": [2244, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cf79631f413d2161662037a"></a>
## visit

`function` · `sqlparser::ast::ddl::DeferrableInitial::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DeferrableInitial", "path": "DeferrableInitial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2245, 47], "end": [2245, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61b90c5515aad56cd80eb43a"></a>
## visit

`function` · `sqlparser::ast::ddl::DeferrableInitial::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DeferrableInitial", "path": "DeferrableInitial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2245, 40], "end": [2245, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
