# `sqlparser::ast::HiveDistributionStyle`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HiveDistributionStyle.json).

<a id="op-a77d410e1850849fc8f673bb"></a>
## HiveDistributionStyle

`enum` · `sqlparser::ast::HiveDistributionStyle` · sqlparser 0.62.0

```rust
enum HiveDistributionStyle
```

Source: `src/ast/mod.rs:8539`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Distribution style options for Hive tables.

<a id="op-982047921dade66aa45f5823"></a>
## NONE

`variant` · `sqlparser::ast::HiveDistributionStyle::NONE` · sqlparser 0.62.0

```rust
NONE
```

Source: `src/ast/mod.rs:8555`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No distribution style specified.

<a id="op-01df7fe9e696fa22896ee21d"></a>
## PARTITIONED

`variant` · `sqlparser::ast::HiveDistributionStyle::PARTITIONED` · sqlparser 0.62.0

```rust
PARTITIONED
```

Source: `src/ast/mod.rs:8541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Partitioned distribution with the given columns.

<a id="op-b9f659a02e2be0505f609bec"></a>
## SKEWED

`variant` · `sqlparser::ast::HiveDistributionStyle::SKEWED` · sqlparser 0.62.0

```rust
SKEWED
```

Source: `src/ast/mod.rs:8546`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Skewed distribution definition.

<a id="op-67764fc3ad036449ac1e091a"></a>
## clone

`function` · `sqlparser::ast::HiveDistributionStyle::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HiveDistributionStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDistributionStyle", "path": "HiveDistributionStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8535, 17], "end": [8535, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93fb6042d9798b6b156e4c76"></a>
## cmp

`function` · `sqlparser::ast::HiveDistributionStyle::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HiveDistributionStyle) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDistributionStyle", "path": "HiveDistributionStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8535, 51], "end": [8535, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28e838761b8cf24f6cdc2d44"></a>
## deserialize

`function` · `sqlparser::ast::HiveDistributionStyle::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDistributionStyle", "path": "HiveDistributionStyle"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8536, 49], "end": [8536, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8536`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01db842c0bed5fa3cb2f150c"></a>
## eq

`function` · `sqlparser::ast::HiveDistributionStyle::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HiveDistributionStyle) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDistributionStyle", "path": "HiveDistributionStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8535, 24], "end": [8535, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcb56ad629bf997ab4183ca9"></a>
## fmt

`function` · `sqlparser::ast::HiveDistributionStyle::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDistributionStyle", "path": "HiveDistributionStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8535, 10], "end": [8535, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30944ac9d99317eb5b1486fc"></a>
## hash

`function` · `sqlparser::ast::HiveDistributionStyle::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDistributionStyle", "path": "HiveDistributionStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8535, 56], "end": [8535, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef882076b10de54f96cabbd6"></a>
## partial_cmp

`function` · `sqlparser::ast::HiveDistributionStyle::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HiveDistributionStyle) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDistributionStyle", "path": "HiveDistributionStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8535, 35], "end": [8535, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45dbfed72d9ea0ed1cf6690b"></a>
## serialize

`function` · `sqlparser::ast::HiveDistributionStyle::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDistributionStyle", "path": "HiveDistributionStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8536, 38], "end": [8536, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8536`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11fe0e12934c549c8eb63c3a"></a>
## visit

`function` · `sqlparser::ast::HiveDistributionStyle::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDistributionStyle", "path": "HiveDistributionStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8537, 40], "end": [8537, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8537`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3278c75e4d42e793d516f4e"></a>
## visit

`function` · `sqlparser::ast::HiveDistributionStyle::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDistributionStyle", "path": "HiveDistributionStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8537, 47], "end": [8537, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8537`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
