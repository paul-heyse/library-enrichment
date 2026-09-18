# `sqlparser::ast::ddl::AlterFunctionKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterFunctionKind.json).

<a id="op-f75e0505ce21b7d733eae6b6"></a>
## AlterFunctionKind

`enum` · `sqlparser::ast::ddl::AlterFunctionKind` · sqlparser 0.62.0

```rust
enum AlterFunctionKind
```

Source: `src/ast/ddl.rs:5387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function-like object type used by [`AlterFunction`](../operations/sqlparser.ast.ddl.AlterFunction.md#op-9f78c8ab8258ff26473fff44).

<a id="op-2f10a2575be324c4a45978eb"></a>
## Aggregate

`variant` · `sqlparser::ast::ddl::AlterFunctionKind::Aggregate` · sqlparser 0.62.0

```rust
Aggregate
```

Source: `src/ast/ddl.rs:5391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`AGGREGATE`

<a id="op-ccea81a58933eb43df830433"></a>
## Function

`variant` · `sqlparser::ast::ddl::AlterFunctionKind::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/ddl.rs:5389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FUNCTION`

<a id="op-a5241c1eb30a8774d4431b1a"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterFunctionKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterFunctionKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5384, 17], "end": [5384, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-025224b2a1615c85437b6725"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterFunctionKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterFunctionKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5384, 51], "end": [5384, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63b12aba7ca247b986201371"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterFunctionKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5385, 49], "end": [5385, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5385`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-122372ec235232637dbd8944"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterFunctionKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterFunctionKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5384, 24], "end": [5384, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c6555262a2cee6a587a1eae"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterFunctionKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5384, 10], "end": [5384, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7f52a23588a20759ea5a6ec"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterFunctionKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5394, 1], "end": [5401, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5395`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e2d02b5aa493600d446f253"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterFunctionKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5384, 56], "end": [5384, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ac210932566ee8d774aef99"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterFunctionKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterFunctionKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5384, 35], "end": [5384, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51d4b8a90c499e5c7d1f59ac"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterFunctionKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5385, 38], "end": [5385, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5385`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6347d4ae3347eb914b5c9d6b"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterFunctionKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5386, 47], "end": [5386, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e00a6b2968c5925d03175890"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterFunctionKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionKind", "path": "AlterFunctionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5386, 40], "end": [5386, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
