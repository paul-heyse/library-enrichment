# `sqlparser::ast::query::SelectFlavor`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SelectFlavor.json).

<a id="op-0f91c832bb97c26433661d0d"></a>
## SelectFlavor

`enum` · `sqlparser::ast::query::SelectFlavor` · sqlparser 0.62.0

```rust
enum SelectFlavor
```

Source: `src/ast/query.rs:328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

What did this select look like?

<a id="op-ec4d5074a554ba9a9700a104"></a>
## FromFirst

`variant` · `sqlparser::ast::query::SelectFlavor::FromFirst` · sqlparser 0.62.0

```rust
FromFirst
```

Source: `src/ast/query.rs:332`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FROM ... SELECT *`

<a id="op-80f31e1b8155da85adfb20b5"></a>
## FromFirstNoSelect

`variant` · `sqlparser::ast::query::SelectFlavor::FromFirstNoSelect` · sqlparser 0.62.0

```rust
FromFirstNoSelect
```

Source: `src/ast/query.rs:334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FROM *`

<a id="op-ea096bb9bd0ebe8e63f0da51"></a>
## Standard

`variant` · `sqlparser::ast::query::SelectFlavor::Standard` · sqlparser 0.62.0

```rust
Standard
```

Source: `src/ast/query.rs:330`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SELECT *`

<a id="op-67efffc85a46c92e03e93b98"></a>
## clone

`function` · `sqlparser::ast::query::SelectFlavor::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SelectFlavor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectFlavor", "path": "SelectFlavor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 17], "end": [325, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97b2cf36a3828ba6ba238700"></a>
## cmp

`function` · `sqlparser::ast::query::SelectFlavor::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SelectFlavor) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectFlavor", "path": "SelectFlavor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 57], "end": [325, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65172d46b22119e6908e8f08"></a>
## deserialize

`function` · `sqlparser::ast::query::SelectFlavor::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectFlavor", "path": "SelectFlavor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 49], "end": [326, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:326`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72d240f24853a5139cdd677b"></a>
## eq

`function` · `sqlparser::ast::query::SelectFlavor::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SelectFlavor) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectFlavor", "path": "SelectFlavor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 30], "end": [325, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9758df3e38823107421ebf04"></a>
## fmt

`function` · `sqlparser::ast::query::SelectFlavor::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectFlavor", "path": "SelectFlavor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 10], "end": [325, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f8b4cee7b8b56edf0f0ac1a"></a>
## hash

`function` · `sqlparser::ast::query::SelectFlavor::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectFlavor", "path": "SelectFlavor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 62], "end": [325, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9b177d740a645179fc6fcd7"></a>
## partial_cmp

`function` · `sqlparser::ast::query::SelectFlavor::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SelectFlavor) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectFlavor", "path": "SelectFlavor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 41], "end": [325, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0a3980babef2a6e202ec464"></a>
## serialize

`function` · `sqlparser::ast::query::SelectFlavor::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectFlavor", "path": "SelectFlavor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 38], "end": [326, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:326`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4451b453e7216a0d283ac723"></a>
## visit

`function` · `sqlparser::ast::query::SelectFlavor::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectFlavor", "path": "SelectFlavor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [327, 40], "end": [327, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:327`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-638f564744d712e94968e8c5"></a>
## visit

`function` · `sqlparser::ast::query::SelectFlavor::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectFlavor", "path": "SelectFlavor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [327, 47], "end": [327, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:327`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
