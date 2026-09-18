# `sqlparser::ast::query::TableSampleQuantity`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableSampleQuantity.json).

<a id="op-a9988c8ca842c827f56193c1"></a>
## TableSampleQuantity

`struct` · `sqlparser::ast::query::TableSampleQuantity` · sqlparser 0.62.0

```rust
struct TableSampleQuantity
```

Source: `src/ast/query.rs:1793`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Quantity for a `TABLESAMPLE` clause (e.g. `10 PERCENT` or `(10)`).

<a id="op-417b13482e7a296a6e8de19e"></a>
## clone

`function` · `sqlparser::ast::query::TableSampleQuantity::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableSampleQuantity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1789, 17], "end": [1789, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5e688029e8332097fe82d7b"></a>
## cmp

`function` · `sqlparser::ast::query::TableSampleQuantity::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableSampleQuantity) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1789, 51], "end": [1789, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae1417f921501d09b8e1b5be"></a>
## deserialize

`function` · `sqlparser::ast::query::TableSampleQuantity::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1790, 49], "end": [1790, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-125a7279eca98dbb152d9e6f"></a>
## eq

`function` · `sqlparser::ast::query::TableSampleQuantity::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableSampleQuantity) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1789, 24], "end": [1789, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d41ef5ab3e572fddec91a92"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleQuantity::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1789, 10], "end": [1789, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcea054297a7ceae0e478c5d"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleQuantity::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1802, 1], "end": [1816, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d336dc084f98cf399571af4d"></a>
## hash

`function` · `sqlparser::ast::query::TableSampleQuantity::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1789, 56], "end": [1789, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16e897ef43b0a55e31ab61be"></a>
## parenthesized

`struct_field` · `sqlparser::ast::query::TableSampleQuantity::parenthesized` · sqlparser 0.62.0

```rust
parenthesized: bool
```

Source: `src/ast/query.rs:1795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the quantity was wrapped in parentheses.

<a id="op-1eba074026badb6a5d52c1e6"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableSampleQuantity::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableSampleQuantity) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1789, 35], "end": [1789, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f176f7630fc1128e9529f283"></a>
## serialize

`function` · `sqlparser::ast::query::TableSampleQuantity::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1790, 38], "end": [1790, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c46978c71ee89efd40126ca3"></a>
## unit

`struct_field` · `sqlparser::ast::query::TableSampleQuantity::unit` · sqlparser 0.62.0

```rust
unit: Option<TableSampleUnit>
```

Source: `src/ast/query.rs:1799`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional unit (e.g. `PERCENT`, `ROWS`).

<a id="op-4e47b16a9ef134cb36fbb456"></a>
## value

`struct_field` · `sqlparser::ast::query::TableSampleQuantity::value` · sqlparser 0.62.0

```rust
value: Expr
```

Source: `src/ast/query.rs:1797`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The numeric expression specifying the quantity.

<a id="op-0abdcb7b3196cf8c9d6436cb"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleQuantity::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1791, 47], "end": [1791, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1791`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-786d72cf75103036e1313198"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleQuantity::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleQuantity", "path": "TableSampleQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1791, 40], "end": [1791, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1791`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
