# `sqlparser::ast::NamedParenthesizedList`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.NamedParenthesizedList.json).

<a id="op-d1ad498aeb82ef1b0d9f641c"></a>
## NamedParenthesizedList

`struct` · `sqlparser::ast::NamedParenthesizedList` · sqlparser 0.62.0

```rust
struct NamedParenthesizedList
```

Source: `src/ast/mod.rs:10558`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Key/Value, where the value is a (optionally named) list of identifiers

```sql
UNION = (tbl_name[,tbl_name]...)
ENGINE = ReplicatedMergeTree('/table_name','{replica}', ver)
ENGINE = SummingMergeTree([columns])
```

<a id="op-fefae643e5b5056080f04d6b"></a>
## clone

`function` · `sqlparser::ast::NamedParenthesizedList::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> NamedParenthesizedList
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NamedParenthesizedList", "path": "NamedParenthesizedList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10548, 17], "end": [10548, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b22b5f0b213ddf81a199b27"></a>
## cmp

`function` · `sqlparser::ast::NamedParenthesizedList::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &NamedParenthesizedList) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NamedParenthesizedList", "path": "NamedParenthesizedList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10548, 51], "end": [10548, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8a1ec7f7edf5558f7f7f0d2"></a>
## deserialize

`function` · `sqlparser::ast::NamedParenthesizedList::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NamedParenthesizedList", "path": "NamedParenthesizedList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10549, 49], "end": [10549, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63f6caf74e58ca9802bf9657"></a>
## eq

`function` · `sqlparser::ast::NamedParenthesizedList::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &NamedParenthesizedList) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NamedParenthesizedList", "path": "NamedParenthesizedList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10548, 24], "end": [10548, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4839d0bf0d28117f0dff074f"></a>
## fmt

`function` · `sqlparser::ast::NamedParenthesizedList::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NamedParenthesizedList", "path": "NamedParenthesizedList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10548, 10], "end": [10548, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5746e541b7d179039aba659a"></a>
## hash

`function` · `sqlparser::ast::NamedParenthesizedList::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NamedParenthesizedList", "path": "NamedParenthesizedList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10548, 56], "end": [10548, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ad4500c8c0ce229495dd54"></a>
## key

`struct_field` · `sqlparser::ast::NamedParenthesizedList::key` · sqlparser 0.62.0

```rust
key: Ident
```

Source: `src/ast/mod.rs:10560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The option key (identifier) for this named list.

<a id="op-c93eb87b15ca48e90489eb39"></a>
## name

`struct_field` · `sqlparser::ast::NamedParenthesizedList::name` · sqlparser 0.62.0

```rust
name: Option<Ident>
```

Source: `src/ast/mod.rs:10562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional secondary name associated with the key.

<a id="op-eba5685528ad2210507024de"></a>
## partial_cmp

`function` · `sqlparser::ast::NamedParenthesizedList::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &NamedParenthesizedList) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NamedParenthesizedList", "path": "NamedParenthesizedList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10548, 35], "end": [10548, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90da05009dcf3b66c70b6b66"></a>
## serialize

`function` · `sqlparser::ast::NamedParenthesizedList::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NamedParenthesizedList", "path": "NamedParenthesizedList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10549, 38], "end": [10549, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d31a682b3f0ade64ca2bf70"></a>
## values

`struct_field` · `sqlparser::ast::NamedParenthesizedList::values` · sqlparser 0.62.0

```rust
values: Vec<Ident>
```

Source: `src/ast/mod.rs:10564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The list of identifier values for the key.

<a id="op-6697f0ea3d2223d91a094761"></a>
## visit

`function` · `sqlparser::ast::NamedParenthesizedList::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NamedParenthesizedList", "path": "NamedParenthesizedList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10550, 40], "end": [10550, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bda3e1c491836a328ce6b0e3"></a>
## visit

`function` · `sqlparser::ast::NamedParenthesizedList::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NamedParenthesizedList", "path": "NamedParenthesizedList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10550, 47], "end": [10550, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
