# `sqlparser::ast::Subscript`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Subscript.json).

<a id="op-4081a86d895fcbc411ea485c"></a>
## Subscript

`enum` · `sqlparser::ast::Subscript` · sqlparser 0.62.0

```rust
enum Subscript
```

Source: `src/ast/mod.rs:1387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The contents inside the `[` and `]` in a subscript expression.

<a id="op-ed358a85b65765499058c6a7"></a>
## Index

`variant` · `sqlparser::ast::Subscript::Index` · sqlparser 0.62.0

```rust
Index
```

Source: `src/ast/mod.rs:1389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Accesses the element of the array at the given index.

<a id="op-e3ad5937f719fbf6d75f6867"></a>
## Slice

`variant` · `sqlparser::ast::Subscript::Slice` · sqlparser 0.62.0

```rust
Slice
```

Source: `src/ast/mod.rs:1415`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Accesses a slice of an array on PostgreSQL, e.g.

```plaintext
=> select (array[1,2,3,4,5,6])[2:5];
-----------
{2,3,4,5}
```

The lower and/or upper bound can be omitted to slice from the start or
end of the array respectively.

See <https://www.postgresql.org/docs/current/arrays.html#ARRAYS-ACCESSING>.

Also supports an optional "stride" as the last element (this is not
supported by postgres), e.g.

```plaintext
=> select (array[1,2,3,4,5,6])[1:6:2];
-----------
{1,3,5}
```

<a id="op-ee2bbda618a299b890e0c773"></a>
## clone

`function` · `sqlparser::ast::Subscript::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Subscript
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1384, 17], "end": [1384, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:1384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0c522578fb56c26c2322853"></a>
## cmp

`function` · `sqlparser::ast::Subscript::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Subscript) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1384, 51], "end": [1384, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:1384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e402422d3d16f6bcb315d65"></a>
## deserialize

`function` · `sqlparser::ast::Subscript::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1385, 49], "end": [1385, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:1385`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80823f877520591ee2f4ad73"></a>
## eq

`function` · `sqlparser::ast::Subscript::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Subscript) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1384, 24], "end": [1384, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:1384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26ede0da6603d7291921c08f"></a>
## fmt

`function` · `sqlparser::ast::Subscript::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1425, 1], "end": [1449, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:1426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e927944525cdfb151a9d6341"></a>
## fmt

`function` · `sqlparser::ast::Subscript::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1384, 10], "end": [1384, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:1384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53b7cde3daed116df56172b2"></a>
## hash

`function` · `sqlparser::ast::Subscript::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1384, 56], "end": [1384, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:1384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12ab8bcb78a4629ce7087058"></a>
## partial_cmp

`function` · `sqlparser::ast::Subscript::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Subscript) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1384, 35], "end": [1384, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:1384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbca33e60a4e9adf649ec6c9"></a>
## serialize

`function` · `sqlparser::ast::Subscript::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1385, 38], "end": [1385, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:1385`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1450d7b7a0e7819685eb5c96"></a>
## span

`function` · `sqlparser::ast::Subscript::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "super::Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1670, 1], "end": [1689, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1671`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ceee77cf6d80bcd4eb4f1370"></a>
## visit

`function` · `sqlparser::ast::Subscript::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1386, 47], "end": [1386, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:1386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1112a5bb3f233c4bf9a2b11"></a>
## visit

`function` · `sqlparser::ast::Subscript::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Subscript", "path": "Subscript"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1386, 40], "end": [1386, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:1386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
