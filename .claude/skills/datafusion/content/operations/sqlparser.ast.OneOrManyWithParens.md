# `sqlparser::ast::OneOrManyWithParens`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.OneOrManyWithParens.json).

<a id="op-466d4716a8e184af91481f1a"></a>
## OneOrManyWithParens

`enum` · `sqlparser::ast::OneOrManyWithParens` · sqlparser 0.62.0

```rust
enum OneOrManyWithParens<T>
```

Source: `src/ast/mod.rs:1568`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Encapsulates the common pattern in SQL where either one unparenthesized item
such as an identifier or expression is permitted, or multiple of the same
item in a parenthesized list. For accessing items regardless of the form,
`OneOrManyWithParens` implements `Deref<Target = [T]>` and `IntoIterator`,
so you can call slice methods on it and iterate over items
# Examples
Accessing as a slice:
```
# use sqlparser::ast::OneOrManyWithParens;
let one = OneOrManyWithParens::One("a");

assert_eq!(one[0], "a");
assert_eq!(one.len(), 1);
```
Iterating:
```
# use sqlparser::ast::OneOrManyWithParens;
let one = OneOrManyWithParens::One("a");
let many = OneOrManyWithParens::Many(vec!["a", "b"]);

assert_eq!(one.into_iter().chain(many).collect::<Vec<_>>(), vec!["a", "a", "b"] );
```

<a id="op-084103cd2a7832f3df10b985"></a>
## IntoIter

`assoc_type` · `sqlparser::ast::OneOrManyWithParens::IntoIter` · sqlparser 0.62.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1675, 1], "end": [1692, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/ast/mod.rs:1678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6494023c3abe2dfe92a2301a"></a>
## Item

`assoc_type` · `sqlparser::ast::OneOrManyWithParens::Item` · sqlparser 0.62.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1675, 1], "end": [1692, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/ast/mod.rs:1676`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bccee856a8fe5b09b9a673f"></a>
## Many

`variant` · `sqlparser::ast::OneOrManyWithParens::Many` · sqlparser 0.62.0

```rust
Many
```

Source: `src/ast/mod.rs:1572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One or more `T`s, parenthesized.

<a id="op-da7b4499a1e0c84bc174876b"></a>
## One

`variant` · `sqlparser::ast::OneOrManyWithParens::One` · sqlparser 0.62.0

```rust
One
```

Source: `src/ast/mod.rs:1570`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single `T`, unparenthesized.

<a id="op-fac84f9ded40e5d6c8ac3859"></a>
## Target

`assoc_type` · `sqlparser::ast::OneOrManyWithParens::Target` · sqlparser 0.62.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1575, 1], "end": [1584, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ast/mod.rs:1576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e9c54c3dcaf0f60fe5bb6bb"></a>
## as_ref

`function` · `sqlparser::ast::OneOrManyWithParens::as_ref` · sqlparser 0.62.0

```rust
fn as_ref(&self) -> &[T]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1586, 1], "end": [1590, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"generic": "T"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/ast/mod.rs:1587`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac888d2d120709169dfcfeb2"></a>
## clone

`function` · `sqlparser::ast::OneOrManyWithParens::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OneOrManyWithParens<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1565, 17], "end": [1565, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:1565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56e60257e7bf18b5c427df8b"></a>
## cmp

`function` · `sqlparser::ast::OneOrManyWithParens::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OneOrManyWithParens<T>) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Ord", "path": "$crate::cmp::Ord"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1565, 51], "end": [1565, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:1565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a131191fccb5f40fade19018"></a>
## deref

`function` · `sqlparser::ast::OneOrManyWithParens::deref` · sqlparser 0.62.0

```rust
fn deref(&self) -> &[T]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1575, 1], "end": [1584, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ast/mod.rs:1578`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17568d11d2df1be3a5c02cfc"></a>
## deserialize

`function` · `sqlparser::ast::OneOrManyWithParens::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "_serde::Deserialize"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [1566, 49], "end": [1566, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:1566`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fb85545574fa8bce9ccb81b"></a>
## eq

`function` · `sqlparser::ast::OneOrManyWithParens::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OneOrManyWithParens<T>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1565, 24], "end": [1565, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:1565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cae3ea51870de5a5eba14014"></a>
## fmt

`function` · `sqlparser::ast::OneOrManyWithParens::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Display", "path": "fmt::Display"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [1694, 1], "end": [1706, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:1698`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f533404b8ffcb090335fcfc8"></a>
## fmt

`function` · `sqlparser::ast::OneOrManyWithParens::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1565, 10], "end": [1565, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:1565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5721215b11ad6df16609a03f"></a>
## hash

`function` · `sqlparser::ast::OneOrManyWithParens::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "$crate::hash::Hash"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1565, 56], "end": [1565, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:1565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbf24cf0a823ffc0bed19462"></a>
## into_iter

`function` · `sqlparser::ast::OneOrManyWithParens::into_iter` · sqlparser 0.62.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1675, 1], "end": [1692, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/ast/mod.rs:1680`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdeca4335f3b26aa96b38926"></a>
## partial_cmp

`function` · `sqlparser::ast::OneOrManyWithParens::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OneOrManyWithParens<T>) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "$crate::cmp::PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1565, 35], "end": [1565, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:1565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-913107f21786ece22c686398"></a>
## serialize

`function` · `sqlparser::ast::OneOrManyWithParens::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "_serde::Serialize"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [1566, 38], "end": [1566, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:1566`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a00d5157f2f7d8044a4bb81"></a>
## visit

`function` · `sqlparser::ast::OneOrManyWithParens::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "sqlparser::ast::VisitMut"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1567, 47], "end": [1567, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:1567`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3b5f57b5d6c02ccfa5ba42f"></a>
## visit

`function` · `sqlparser::ast::OneOrManyWithParens::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParens", "path": "OneOrManyWithParens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "sqlparser::ast::Visit"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1567, 40], "end": [1567, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:1567`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
