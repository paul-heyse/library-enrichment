# `sqlparser::ast::Tag`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Tag.json).

<a id="op-0b9e7be0260cd55139aefcd2"></a>
## Tag

`struct` · `sqlparser::ast::Tag` · sqlparser 0.62.0

```rust
struct Tag
```

Source: `src/ast/mod.rs:10629`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `WITH TAG ( tag_name = '<tag_value>', ...)`

<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-7b3a9ec351b43a2446651d8f"></a>
## clone

`function` · `sqlparser::ast::Tag::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Tag
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10626, 17], "end": [10626, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63b79be12b94adae4a1172ef"></a>
## cmp

`function` · `sqlparser::ast::Tag::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Tag) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10626, 51], "end": [10626, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f55543b6965055757254c513"></a>
## deserialize

`function` · `sqlparser::ast::Tag::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10627, 49], "end": [10627, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9aedd24121983fef670b7796"></a>
## eq

`function` · `sqlparser::ast::Tag::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Tag) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10626, 24], "end": [10626, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02c53a054a0e0e6357f36d15"></a>
## fmt

`function` · `sqlparser::ast::Tag::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10643, 1], "end": [10647, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4047146d3858935970225a2"></a>
## fmt

`function` · `sqlparser::ast::Tag::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10626, 10], "end": [10626, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5324983c3b580c6521b5a96b"></a>
## hash

`function` · `sqlparser::ast::Tag::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10626, 56], "end": [10626, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c27bef9ec2354b13e4ee6ea"></a>
## key

`struct_field` · `sqlparser::ast::Tag::key` · sqlparser 0.62.0

```rust
key: ObjectName
```

Source: `src/ast/mod.rs:10631`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The tag key (can be qualified).

<a id="op-155192b5c0e12cc4da561272"></a>
## new

`function` · `sqlparser::ast::Tag::new` · sqlparser 0.62.0

```rust
fn new(key: ObjectName, value: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10636, 1], "end": [10641, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:10638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new `Tag` with the given key and value.

<a id="op-90e0ce1b007264d0f65f74f9"></a>
## partial_cmp

`function` · `sqlparser::ast::Tag::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Tag) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10626, 35], "end": [10626, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89875f9cf63d414e191910a4"></a>
## serialize

`function` · `sqlparser::ast::Tag::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10627, 38], "end": [10627, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79a43df010d65c9c035f4d29"></a>
## value

`struct_field` · `sqlparser::ast::Tag::value` · sqlparser 0.62.0

```rust
value: String
```

Source: `src/ast/mod.rs:10633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The tag value as a string.

<a id="op-47bc223807f7710caf4ba7f4"></a>
## visit

`function` · `sqlparser::ast::Tag::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10628, 40], "end": [10628, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcf00674ad2f1607ad882eae"></a>
## visit

`function` · `sqlparser::ast::Tag::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Tag", "path": "Tag"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10628, 47], "end": [10628, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
