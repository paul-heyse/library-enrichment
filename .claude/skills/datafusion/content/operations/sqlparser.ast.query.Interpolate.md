# `sqlparser::ast::query::Interpolate`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Interpolate.json).

<a id="op-84f1d8a1273a5e4f6c644220"></a>
## Interpolate

`struct` · `sqlparser::ast::query::Interpolate` · sqlparser 0.62.0

```rust
struct Interpolate
```

Source: `src/ast/query.rs:3009`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INTERPOLATE` clause used with ClickHouse `WITH FILL` to compute missing values.

<a id="op-665bd4f01b931fbe701231c2"></a>
## clone

`function` · `sqlparser::ast::query::Interpolate::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Interpolate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "Interpolate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3005, 17], "end": [3005, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed0c6c32f881d6a928ec0d70"></a>
## cmp

`function` · `sqlparser::ast::query::Interpolate::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Interpolate) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "Interpolate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3005, 51], "end": [3005, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3de3dfaabed392eb7d521d0"></a>
## deserialize

`function` · `sqlparser::ast::query::Interpolate::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "Interpolate"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3006, 49], "end": [3006, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3006`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b6801d383997c51f07fb9f6"></a>
## eq

`function` · `sqlparser::ast::query::Interpolate::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Interpolate) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "Interpolate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3005, 24], "end": [3005, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80d43d57c2349ca10ff385e1"></a>
## exprs

`struct_field` · `sqlparser::ast::query::Interpolate::exprs` · sqlparser 0.62.0

```rust
exprs: Option<Vec<InterpolateExpr>>
```

Source: `src/ast/query.rs:3011`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of interpolation expressions.

<a id="op-c55b70f607d8c12e69eaecb8"></a>
## fmt

`function` · `sqlparser::ast::query::Interpolate::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "Interpolate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3005, 10], "end": [3005, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d085b31ebb36d634656019c"></a>
## hash

`function` · `sqlparser::ast::query::Interpolate::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "Interpolate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3005, 56], "end": [3005, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1c11366c277606404025869"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Interpolate::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Interpolate) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "Interpolate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3005, 35], "end": [3005, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83c839bfe7fd502de16622f3"></a>
## serialize

`function` · `sqlparser::ast::query::Interpolate::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "Interpolate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3006, 38], "end": [3006, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3006`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de0381abd94f29199a1a3325"></a>
## span

`function` · `sqlparser::ast::query::Interpolate::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "super::Interpolate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1298, 1], "end": [1304, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-035918b72d102bf687cf332c"></a>
## visit

`function` · `sqlparser::ast::query::Interpolate::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "Interpolate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3007, 47], "end": [3007, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3007`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6991820554a378c5dcd4832d"></a>
## visit

`function` · `sqlparser::ast::query::Interpolate::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Interpolate", "path": "Interpolate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3007, 40], "end": [3007, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3007`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
