# `sqlparser::ast::query::TableIndexHintForClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableIndexHintForClause.json).

<a id="op-b6618b92ad479a49f30727ab"></a>
## TableIndexHintForClause

`enum` · `sqlparser::ast::query::TableIndexHintForClause` · sqlparser 0.62.0

```rust
enum TableIndexHintForClause
```

Source: `src/ast/query.rs:1414`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Which clause the table index hint applies to.

<a id="op-f6750199e62940b05e952f54"></a>
## GroupBy

`variant` · `sqlparser::ast::query::TableIndexHintForClause::GroupBy` · sqlparser 0.62.0

```rust
GroupBy
```

Source: `src/ast/query.rs:1420`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply the hint to `GROUP BY` clauses.

<a id="op-6ab9436cf05fb83711249a57"></a>
## Join

`variant` · `sqlparser::ast::query::TableIndexHintForClause::Join` · sqlparser 0.62.0

```rust
Join
```

Source: `src/ast/query.rs:1416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply the hint to JOIN clauses.

<a id="op-2cad540233a9702835547331"></a>
## OrderBy

`variant` · `sqlparser::ast::query::TableIndexHintForClause::OrderBy` · sqlparser 0.62.0

```rust
OrderBy
```

Source: `src/ast/query.rs:1418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply the hint to `ORDER BY` clauses.

<a id="op-afa652099b63e5140cc27c8c"></a>
## clone

`function` · `sqlparser::ast::query::TableIndexHintForClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableIndexHintForClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1410, 17], "end": [1410, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-240f1acfb5b6d297496eceb7"></a>
## cmp

`function` · `sqlparser::ast::query::TableIndexHintForClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableIndexHintForClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1410, 57], "end": [1410, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7960710960315304ff8007b9"></a>
## deserialize

`function` · `sqlparser::ast::query::TableIndexHintForClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1411, 49], "end": [1411, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1411`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b3f14a8e511efc4483e6d0f"></a>
## eq

`function` · `sqlparser::ast::query::TableIndexHintForClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableIndexHintForClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1410, 30], "end": [1410, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82328f5467518961dcaa3689"></a>
## fmt

`function` · `sqlparser::ast::query::TableIndexHintForClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1423, 1], "end": [1431, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdaa9ec9726103ff33ae96ee"></a>
## fmt

`function` · `sqlparser::ast::query::TableIndexHintForClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1410, 10], "end": [1410, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19957de543f65d2de51ad838"></a>
## hash

`function` · `sqlparser::ast::query::TableIndexHintForClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1410, 62], "end": [1410, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed97861f1c220baa1179141a"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableIndexHintForClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableIndexHintForClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1410, 41], "end": [1410, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51971711194c1303a7a91987"></a>
## serialize

`function` · `sqlparser::ast::query::TableIndexHintForClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1411, 38], "end": [1411, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1411`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e4a9d7547d80885d04ea3d2"></a>
## visit

`function` · `sqlparser::ast::query::TableIndexHintForClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1412, 47], "end": [1412, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1412`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8002f1934565e69b545f182"></a>
## visit

`function` · `sqlparser::ast::query::TableIndexHintForClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHintForClause", "path": "TableIndexHintForClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1412, 40], "end": [1412, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1412`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
