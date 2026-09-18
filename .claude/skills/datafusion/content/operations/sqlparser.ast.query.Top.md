# `sqlparser::ast::query::Top`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Top.json).

<a id="op-65cdb94e47754b1f8a77763e"></a>
## Top

`struct` · `sqlparser::ast::query::Top` · sqlparser 0.62.0

```rust
struct Top
```

Source: `src/ast/query.rs:3611`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MSSQL `TOP` clause options.

<a id="op-dfe27c8711a090ec2f6f9325"></a>
## clone

`function` · `sqlparser::ast::query::Top::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Top
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3607, 17], "end": [3607, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd6a3631b3a9edcdb0214dda"></a>
## cmp

`function` · `sqlparser::ast::query::Top::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Top) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3607, 51], "end": [3607, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b40aa6883358e75d809c1fd"></a>
## deserialize

`function` · `sqlparser::ast::query::Top::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3608, 49], "end": [3608, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41f4a29b706cf8fdbc45c6ec"></a>
## eq

`function` · `sqlparser::ast::query::Top::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Top) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3607, 24], "end": [3607, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-879c18a4c8ead1e2f9c5ed6f"></a>
## fmt

`function` · `sqlparser::ast::query::Top::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3632, 1], "end": [3647, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abe056127b5a7e35bad13397"></a>
## fmt

`function` · `sqlparser::ast::query::Top::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3607, 10], "end": [3607, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ba8d53bb0e7820d74d80b9f"></a>
## hash

`function` · `sqlparser::ast::query::Top::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3607, 56], "end": [3607, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cc0e8575162f1c386b4e4b0"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Top::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Top) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3607, 35], "end": [3607, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36fa3313bc8685471cd4a690"></a>
## percent

`struct_field` · `sqlparser::ast::query::Top::percent` · sqlparser 0.62.0

```rust
percent: bool
```

Source: `src/ast/query.rs:3616`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply `PERCENT` extension.

<a id="op-853d7e4b92dca5b53cbca68a"></a>
## quantity

`struct_field` · `sqlparser::ast::query::Top::quantity` · sqlparser 0.62.0

```rust
quantity: Option<TopQuantity>
```

Source: `src/ast/query.rs:3618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The optional quantity (expression or constant) following `TOP`.

<a id="op-1ff7eb2635cc09c5b81145a8"></a>
## serialize

`function` · `sqlparser::ast::query::Top::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3608, 38], "end": [3608, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e681e276dfec4cda94f47d8"></a>
## visit

`function` · `sqlparser::ast::query::Top::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3609, 40], "end": [3609, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3609`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd497261b26e21e8cd24bfa2"></a>
## visit

`function` · `sqlparser::ast::query::Top::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Top", "path": "Top"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3609, 47], "end": [3609, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3609`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b80ea42ad1995fecd5cff90f"></a>
## with_ties

`struct_field` · `sqlparser::ast::query::Top::with_ties` · sqlparser 0.62.0

```rust
with_ties: bool
```

Source: `src/ast/query.rs:3614`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL semantic equivalent of LIMIT but with same structure as FETCH.
MSSQL only.
