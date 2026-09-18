# `sqlparser::ast::dcl::ResetConfig`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.ResetConfig.json).

<a id="op-66c5205b212473e20acea79d"></a>
## ResetConfig

`enum` · `sqlparser::ast::dcl::ResetConfig` · sqlparser 0.62.0

```rust
enum ResetConfig
```

Source: `src/ast/dcl.rs:134`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

RESET config option:
* RESET `configuration_parameter`
* RESET ALL

<a id="op-d2fe446caed22cc5f0fd1631"></a>
## ALL

`variant` · `sqlparser::ast::dcl::ResetConfig::ALL` · sqlparser 0.62.0

```rust
ALL
```

Source: `src/ast/dcl.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Reset all configuration parameters.

<a id="op-14d6d1329e140c90a4d4732b"></a>
## ConfigName

`variant` · `sqlparser::ast::dcl::ResetConfig::ConfigName` · sqlparser 0.62.0

```rust
ConfigName
```

Source: `src/ast/dcl.rs:138`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Reset the named configuration parameter.

<a id="op-7c3d3b648511f72bce41d9d2"></a>
## clone

`function` · `sqlparser::ast::dcl::ResetConfig::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ResetConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::ResetConfig", "path": "ResetConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 17], "end": [131, 22], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dcl.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-732a37a992a09f8e7b1297e9"></a>
## cmp

`function` · `sqlparser::ast::dcl::ResetConfig::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ResetConfig) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::ResetConfig", "path": "ResetConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 51], "end": [131, 54], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dcl.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eaa1a8a90779a639c589d3e2"></a>
## deserialize

`function` · `sqlparser::ast::dcl::ResetConfig::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::ResetConfig", "path": "ResetConfig"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 49], "end": [132, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dcl.rs:132`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f458fd999fdae3cc67715fd"></a>
## eq

`function` · `sqlparser::ast::dcl::ResetConfig::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ResetConfig) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::ResetConfig", "path": "ResetConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 24], "end": [131, 33], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dcl.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddc70eb567d98497000e3849"></a>
## fmt

`function` · `sqlparser::ast::dcl::ResetConfig::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::ResetConfig", "path": "ResetConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 10], "end": [131, 15], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dcl.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e38a4337b86ae1148670b0c6"></a>
## hash

`function` · `sqlparser::ast::dcl::ResetConfig::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::ResetConfig", "path": "ResetConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 56], "end": [131, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dcl.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb517dd58c431a6dbae989f0"></a>
## partial_cmp

`function` · `sqlparser::ast::dcl::ResetConfig::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ResetConfig) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::ResetConfig", "path": "ResetConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 35], "end": [131, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dcl.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5e307d3658a664a82d9eb27"></a>
## serialize

`function` · `sqlparser::ast::dcl::ResetConfig::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::ResetConfig", "path": "ResetConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 38], "end": [132, 47], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dcl.rs:132`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f28a031e5a97b26c3513067"></a>
## visit

`function` · `sqlparser::ast::dcl::ResetConfig::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::ResetConfig", "path": "ResetConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 47], "end": [133, 55], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dcl.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-987f14f0fb1a7dea332b1810"></a>
## visit

`function` · `sqlparser::ast::dcl::ResetConfig::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::ResetConfig", "path": "ResetConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 40], "end": [133, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dcl.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
