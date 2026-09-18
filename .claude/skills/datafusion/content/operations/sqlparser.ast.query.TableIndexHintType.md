# `sqlparser::ast::query::TableIndexHintType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableIndexHintType.json).

<a id="op-b00b632f22688e94c16dcb79"></a>
## TableIndexHintType

`enum` · `sqlparser::ast::query::TableIndexHintType` · sqlparser 0.62.0

```rust
enum TableIndexHintType
```

Source: `src/ast/query.rs:1371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Type of index hint (e.g., `USE`, `IGNORE`, `FORCE`).

<a id="op-7acea570f41747b3c0c54254"></a>
## Force

`variant` · `sqlparser::ast::query::TableIndexHintType::Force` · sqlparser 0.62.0

```rust
Force
```

Source: `src/ast/query.rs:1377`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FORCE` hint.

<a id="op-95a504949251cdccf5ab1773"></a>
## Ignore

`variant` · `sqlparser::ast::query::TableIndexHintType::Ignore` · sqlparser 0.62.0

```rust
Ignore
```

Source: `src/ast/query.rs:1375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IGNORE` hint.

<a id="op-2b0211cdf470efe97281183e"></a>
## Use

`variant` · `sqlparser::ast::query::TableIndexHintType::Use` · sqlparser 0.62.0

```rust
Use
```

Source: `src/ast/query.rs:1373`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`USE` hint.

<a id="op-352b5a4c1a3b15480667a5c3"></a>
## clone

`function` · `sqlparser::ast::query::TableIndexHintType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableIndexHintType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1367, 17], "end": [1367, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-380df545fe3ef08dd68147f5"></a>
## cmp

`function` · `sqlparser::ast::query::TableIndexHintType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableIndexHintType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1367, 57], "end": [1367, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4fa7ef7e75cb5b2c609078c"></a>
## deserialize

`function` · `sqlparser::ast::query::TableIndexHintType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1368, 49], "end": [1368, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1368`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9c4dc7a2bd8566290bbd9c5"></a>
## eq

`function` · `sqlparser::ast::query::TableIndexHintType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableIndexHintType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1367, 30], "end": [1367, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-792d968a019598b8668843d7"></a>
## fmt

`function` · `sqlparser::ast::query::TableIndexHintType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1380, 1], "end": [1388, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1381`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-968c49dbdd727e1f31eeb79a"></a>
## fmt

`function` · `sqlparser::ast::query::TableIndexHintType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1367, 10], "end": [1367, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7af1c9b12f8cf206605737fe"></a>
## hash

`function` · `sqlparser::ast::query::TableIndexHintType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1367, 62], "end": [1367, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45fa812a1e5009246c8ee168"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableIndexHintType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableIndexHintType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1367, 41], "end": [1367, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55a9a1622ef1243ff72b8db3"></a>
## serialize

`function` · `sqlparser::ast::query::TableIndexHintType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1368, 38], "end": [1368, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1368`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a6f606fe5b4a2001870c58d"></a>
## visit

`function` · `sqlparser::ast::query::TableIndexHintType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1369, 47], "end": [1369, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1369`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d39694fb7ee8c8066fbda6c4"></a>
## visit

`function` · `sqlparser::ast::query::TableIndexHintType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintType", "path": "TableIndexHintType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1369, 40], "end": [1369, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1369`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
