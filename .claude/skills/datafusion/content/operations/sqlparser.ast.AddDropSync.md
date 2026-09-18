# `sqlparser::ast::AddDropSync`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AddDropSync.json).

<a id="op-26f44da93bc2b3fcd4fb8063"></a>
## AddDropSync

`enum` · `sqlparser::ast::AddDropSync` · sqlparser 0.62.0

```rust
enum AddDropSync
```

Source: `src/ast/mod.rs:2436`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Indicates partition operation type for partition management statements.

<a id="op-45590186cbaa54912e57d8c3"></a>
## ADD

`variant` · `sqlparser::ast::AddDropSync::ADD` · sqlparser 0.62.0

```rust
ADD
```

Source: `src/ast/mod.rs:2438`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Add partitions.

<a id="op-6bb32498e24c730ff3ab3a85"></a>
## DROP

`variant` · `sqlparser::ast::AddDropSync::DROP` · sqlparser 0.62.0

```rust
DROP
```

Source: `src/ast/mod.rs:2440`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Drop partitions.

<a id="op-a41d90eb123360293c319d47"></a>
## SYNC

`variant` · `sqlparser::ast::AddDropSync::SYNC` · sqlparser 0.62.0

```rust
SYNC
```

Source: `src/ast/mod.rs:2442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sync partitions.

<a id="op-f5b18878139234b6b65153c1"></a>
## clone

`function` · `sqlparser::ast::AddDropSync::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AddDropSync
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2432, 23], "end": [2432, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c477118849648815c7a3a17a"></a>
## cmp

`function` · `sqlparser::ast::AddDropSync::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AddDropSync) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2432, 57], "end": [2432, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb4b6f682406f73cd9bef272"></a>
## deserialize

`function` · `sqlparser::ast::AddDropSync::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2433, 49], "end": [2433, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff7955489e36c235f440c9cc"></a>
## eq

`function` · `sqlparser::ast::AddDropSync::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AddDropSync) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2432, 30], "end": [2432, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06d1d1ea138fffa4223abbdc"></a>
## fmt

`function` · `sqlparser::ast::AddDropSync::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2445, 1], "end": [2453, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bba4115a2da979c07ef3a01"></a>
## fmt

`function` · `sqlparser::ast::AddDropSync::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2432, 10], "end": [2432, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0e184f560a2def964aaf54e"></a>
## hash

`function` · `sqlparser::ast::AddDropSync::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2432, 62], "end": [2432, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92f344329aec39e30ae44ff3"></a>
## partial_cmp

`function` · `sqlparser::ast::AddDropSync::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AddDropSync) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2432, 41], "end": [2432, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4797d230be7bee25e7603b4"></a>
## serialize

`function` · `sqlparser::ast::AddDropSync::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2433, 38], "end": [2433, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15399ac9d6a4eab222f520f1"></a>
## visit

`function` · `sqlparser::ast::AddDropSync::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2434, 47], "end": [2434, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2434`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a08ef7aeea157f9744b6a4a9"></a>
## visit

`function` · `sqlparser::ast::AddDropSync::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AddDropSync", "path": "AddDropSync"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2434, 40], "end": [2434, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2434`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
