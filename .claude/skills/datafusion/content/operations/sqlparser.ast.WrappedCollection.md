# `sqlparser::ast::WrappedCollection`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.WrappedCollection.json).

<a id="op-b75f35a2be62f301e04c2f14"></a>
## WrappedCollection

`enum` · `sqlparser::ast::WrappedCollection` · sqlparser 0.62.0

```rust
enum WrappedCollection<T>
```

Source: `src/ast/mod.rs:10705`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Helper to indicate if a collection should be wrapped by a symbol in the display form

[`Display`] is implemented for every [`Vec<T>`] where `T: Display`.
The string output is a comma separated list for the vec items

# Examples
```
# use sqlparser::ast::WrappedCollection;
let items = WrappedCollection::Parentheses(vec!["one", "two", "three"]);
assert_eq!("(one, two, three)", items.to_string());

let items = WrappedCollection::NoWrapping(vec!["one", "two", "three"]);
assert_eq!("one, two, three", items.to_string());
```

Unresolved upstream links (retained, not inferred): ``Vec<T>``, ``Display``.

<a id="op-98215c0ea1c62a1c7954b61e"></a>
## NoWrapping

`variant` · `sqlparser::ast::WrappedCollection::NoWrapping` · sqlparser 0.62.0

```rust
NoWrapping
```

Source: `src/ast/mod.rs:10707`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Print the collection without wrapping symbols, as `item, item, item`

<a id="op-037f5a937f330ad76ad14a9c"></a>
## Parentheses

`variant` · `sqlparser::ast::WrappedCollection::Parentheses` · sqlparser 0.62.0

```rust
Parentheses
```

Source: `src/ast/mod.rs:10709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Wraps the collection in Parentheses, as `(item, item, item)`

<a id="op-e0a128ab4f720b933c226c6b"></a>
## clone

`function` · `sqlparser::ast::WrappedCollection::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WrappedCollection<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10702, 17], "end": [10702, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f99bf05009be180e35e416a"></a>
## cmp

`function` · `sqlparser::ast::WrappedCollection::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WrappedCollection<T>) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Ord", "path": "$crate::cmp::Ord"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10702, 51], "end": [10702, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86184ce30f966384d4767f20"></a>
## deserialize

`function` · `sqlparser::ast::WrappedCollection::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "_serde::Deserialize"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [10703, 49], "end": [10703, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10703`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9db326b55e73f460eaba1010"></a>
## eq

`function` · `sqlparser::ast::WrappedCollection::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WrappedCollection<T>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10702, 24], "end": [10702, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ce1604a99082303b9996b8c"></a>
## fmt

`function` · `sqlparser::ast::WrappedCollection::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [10712, 1], "end": [10726, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10716`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e927aa43f1d35fadbd8b9025"></a>
## fmt

`function` · `sqlparser::ast::WrappedCollection::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10702, 10], "end": [10702, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f614b5da0f7ea31997c57c9f"></a>
## hash

`function` · `sqlparser::ast::WrappedCollection::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "$crate::hash::Hash"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10702, 56], "end": [10702, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c78992ede784c27b4ed8328"></a>
## partial_cmp

`function` · `sqlparser::ast::WrappedCollection::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WrappedCollection<T>) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "$crate::cmp::PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10702, 35], "end": [10702, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04d6e7cf13412e19bd72d71b"></a>
## serialize

`function` · `sqlparser::ast::WrappedCollection::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "_serde::Serialize"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [10703, 38], "end": [10703, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10703`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-352981bda8ec99b3259b53b1"></a>
## visit

`function` · `sqlparser::ast::WrappedCollection::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "sqlparser::ast::VisitMut"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10704, 47], "end": [10704, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee5fa8c115584e3323f090de"></a>
## visit

`function` · `sqlparser::ast::WrappedCollection::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::WrappedCollection", "path": "WrappedCollection"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "sqlparser::ast::Visit"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10704, 40], "end": [10704, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
