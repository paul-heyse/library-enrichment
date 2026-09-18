# `sqlparser::ast::InsertAliases`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.InsertAliases.json).

<a id="op-a17c04ce330614ede55ab2ad"></a>
## InsertAliases

`struct` · `sqlparser::ast::InsertAliases` · sqlparser 0.62.0

```rust
struct InsertAliases
```

Source: `src/ast/mod.rs:6705`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional aliases for `INSERT` targets: row alias and optional column aliases.

<a id="op-6bf11deccf93772cc1d0afac"></a>
## clone

`function` · `sqlparser::ast::InsertAliases::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> InsertAliases
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InsertAliases", "path": "InsertAliases"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6701, 17], "end": [6701, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9bc6f7edfc332fe75080f1c"></a>
## cmp

`function` · `sqlparser::ast::InsertAliases::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &InsertAliases) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InsertAliases", "path": "InsertAliases"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6701, 51], "end": [6701, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12ed0b41934ea1c9a9dcef6a"></a>
## col_aliases

`struct_field` · `sqlparser::ast::InsertAliases::col_aliases` · sqlparser 0.62.0

```rust
col_aliases: Option<Vec<Ident>>
```

Source: `src/ast/mod.rs:6709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of column aliases for the inserted values.

<a id="op-2742cdc3b455c78d53f377f2"></a>
## deserialize

`function` · `sqlparser::ast::InsertAliases::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InsertAliases", "path": "InsertAliases"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6702, 49], "end": [6702, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a334a5a22ddf6386d1629c28"></a>
## eq

`function` · `sqlparser::ast::InsertAliases::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &InsertAliases) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InsertAliases", "path": "InsertAliases"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6701, 24], "end": [6701, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37f48b915a61719fcacbd2e2"></a>
## fmt

`function` · `sqlparser::ast::InsertAliases::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InsertAliases", "path": "InsertAliases"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6701, 10], "end": [6701, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28f24e9dddafaedc35e983b7"></a>
## hash

`function` · `sqlparser::ast::InsertAliases::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InsertAliases", "path": "InsertAliases"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6701, 56], "end": [6701, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2053e8320da990431668e10"></a>
## partial_cmp

`function` · `sqlparser::ast::InsertAliases::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &InsertAliases) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InsertAliases", "path": "InsertAliases"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6701, 35], "end": [6701, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ee6efea32c4f28f6c3c926f"></a>
## row_alias

`struct_field` · `sqlparser::ast::InsertAliases::row_alias` · sqlparser 0.62.0

```rust
row_alias: ObjectName
```

Source: `src/ast/mod.rs:6707`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Row alias (table-style alias) for the inserted values.

<a id="op-ad8f938e8fb545afdc515926"></a>
## serialize

`function` · `sqlparser::ast::InsertAliases::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InsertAliases", "path": "InsertAliases"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6702, 38], "end": [6702, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36e6f9ebce84e1f2128160d4"></a>
## visit

`function` · `sqlparser::ast::InsertAliases::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InsertAliases", "path": "InsertAliases"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6703, 47], "end": [6703, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6703`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffc946816f87e754baed33fd"></a>
## visit

`function` · `sqlparser::ast::InsertAliases::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InsertAliases", "path": "InsertAliases"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6703, 40], "end": [6703, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6703`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
