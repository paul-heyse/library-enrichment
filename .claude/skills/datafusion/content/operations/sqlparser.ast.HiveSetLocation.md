# `sqlparser::ast::HiveSetLocation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HiveSetLocation.json).

<a id="op-144930319ad9f227a66d6072"></a>
## HiveSetLocation

`struct` · `sqlparser::ast::HiveSetLocation` · sqlparser 0.62.0

```rust
struct HiveSetLocation
```

Source: `src/ast/mod.rs:10428`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive-specific `SET LOCATION` helper used in some `LOAD DATA` statements.

<a id="op-bdb7757aed6e543752903a20"></a>
## clone

`function` · `sqlparser::ast::HiveSetLocation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HiveSetLocation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10424, 17], "end": [10424, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a405f8a45d6dfc4ff9a6d237"></a>
## cmp

`function` · `sqlparser::ast::HiveSetLocation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HiveSetLocation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10424, 51], "end": [10424, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f9726e7598336dcb9730387"></a>
## deserialize

`function` · `sqlparser::ast::HiveSetLocation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10425, 49], "end": [10425, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10425`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0afda5285992fef48eeea7ea"></a>
## eq

`function` · `sqlparser::ast::HiveSetLocation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HiveSetLocation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10424, 24], "end": [10424, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40f766fdd94708d9efc8bb0a"></a>
## fmt

`function` · `sqlparser::ast::HiveSetLocation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10424, 10], "end": [10424, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c34ec55f9ff5b8a5e552bbca"></a>
## fmt

`function` · `sqlparser::ast::HiveSetLocation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10435, 1], "end": [10442, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10436`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-899499c4acdf58ab33f1c637"></a>
## has_set

`struct_field` · `sqlparser::ast::HiveSetLocation::has_set` · sqlparser 0.62.0

```rust
has_set: bool
```

Source: `src/ast/mod.rs:10430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `SET` keyword was present.

<a id="op-02744347f6c05381cbc5ee61"></a>
## hash

`function` · `sqlparser::ast::HiveSetLocation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10424, 56], "end": [10424, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db25a790c4941d352730c876"></a>
## location

`struct_field` · `sqlparser::ast::HiveSetLocation::location` · sqlparser 0.62.0

```rust
location: Ident
```

Source: `src/ast/mod.rs:10432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The location identifier.

<a id="op-12e40d28efb980e0c2173572"></a>
## partial_cmp

`function` · `sqlparser::ast::HiveSetLocation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HiveSetLocation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10424, 35], "end": [10424, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b49dc719b62a3cc0860d24d2"></a>
## serialize

`function` · `sqlparser::ast::HiveSetLocation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10425, 38], "end": [10425, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10425`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-398af5686e755b1e904ed2b3"></a>
## visit

`function` · `sqlparser::ast::HiveSetLocation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10426, 40], "end": [10426, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8aa0e1e191e8bf0e18976ad4"></a>
## visit

`function` · `sqlparser::ast::HiveSetLocation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveSetLocation", "path": "HiveSetLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10426, 47], "end": [10426, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
