# `sqlparser::ast::TableAliasWithoutColumns`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TableAliasWithoutColumns.json).

<a id="op-36e13510d15df59fae07e111"></a>
## TableAliasWithoutColumns

`struct` · `sqlparser::ast::TableAliasWithoutColumns` · sqlparser 0.62.0

```rust
struct TableAliasWithoutColumns
```

Source: `src/ast/mod.rs:6716`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for an `INSERT` table; i.e. the table to be inserted into

<a id="op-bef157957cc5a620b9924a6a"></a>
## alias

`struct_field` · `sqlparser::ast::TableAliasWithoutColumns::alias` · sqlparser 0.62.0

```rust
alias: Ident
```

Source: `src/ast/mod.rs:6720`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

the alias name itself

<a id="op-7abe806f20c5c019567a2532"></a>
## clone

`function` · `sqlparser::ast::TableAliasWithoutColumns::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableAliasWithoutColumns
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableAliasWithoutColumns", "path": "TableAliasWithoutColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6712, 17], "end": [6712, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7aa2e922d25ac4e7f69cb7f"></a>
## cmp

`function` · `sqlparser::ast::TableAliasWithoutColumns::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableAliasWithoutColumns) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableAliasWithoutColumns", "path": "TableAliasWithoutColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6712, 51], "end": [6712, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-912e5eb639e2f667f7e5f99b"></a>
## deserialize

`function` · `sqlparser::ast::TableAliasWithoutColumns::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableAliasWithoutColumns", "path": "TableAliasWithoutColumns"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6713, 49], "end": [6713, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f74590a9945e982ddf00a42c"></a>
## eq

`function` · `sqlparser::ast::TableAliasWithoutColumns::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableAliasWithoutColumns) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableAliasWithoutColumns", "path": "TableAliasWithoutColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6712, 24], "end": [6712, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34d409f189ec4271538e0e82"></a>
## explicit

`struct_field` · `sqlparser::ast::TableAliasWithoutColumns::explicit` · sqlparser 0.62.0

```rust
explicit: bool
```

Source: `src/ast/mod.rs:6718`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` if the aliases was explicitly introduced with the "AS" keyword

<a id="op-57f0517c406dd7b475e5b001"></a>
## fmt

`function` · `sqlparser::ast::TableAliasWithoutColumns::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableAliasWithoutColumns", "path": "TableAliasWithoutColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6712, 10], "end": [6712, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10003817451774a5a5d229bb"></a>
## hash

`function` · `sqlparser::ast::TableAliasWithoutColumns::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableAliasWithoutColumns", "path": "TableAliasWithoutColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6712, 56], "end": [6712, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecec909078cb7f7959890513"></a>
## partial_cmp

`function` · `sqlparser::ast::TableAliasWithoutColumns::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableAliasWithoutColumns) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableAliasWithoutColumns", "path": "TableAliasWithoutColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6712, 35], "end": [6712, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2c69364fa3b75978f8e9703"></a>
## serialize

`function` · `sqlparser::ast::TableAliasWithoutColumns::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableAliasWithoutColumns", "path": "TableAliasWithoutColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6713, 38], "end": [6713, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f7a6dbffd16c810af60abeb"></a>
## visit

`function` · `sqlparser::ast::TableAliasWithoutColumns::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableAliasWithoutColumns", "path": "TableAliasWithoutColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6714, 47], "end": [6714, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6714`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed765f709df67653afc6935d"></a>
## visit

`function` · `sqlparser::ast::TableAliasWithoutColumns::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableAliasWithoutColumns", "path": "TableAliasWithoutColumns"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6714, 40], "end": [6714, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6714`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
