# `sqlparser::ast::query::Values`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Values.json).

<a id="op-c0c4487557959b64d59ca513"></a>
## Values

`struct` · `sqlparser::ast::query::Values` · sqlparser 0.62.0

```rust
struct Values
```

Source: `src/ast/query.rs:3653`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An explicit `VALUES` clause and its rows.

<a id="op-9bdd324bc41e2d736613614d"></a>
## clone

`function` · `sqlparser::ast::query::Values::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Values
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3649, 17], "end": [3649, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5bd00efb2707f4e9c61b67c"></a>
## cmp

`function` · `sqlparser::ast::query::Values::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Values) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3649, 51], "end": [3649, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-151767b93e6786c3eb70b279"></a>
## deserialize

`function` · `sqlparser::ast::query::Values::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3650, 49], "end": [3650, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b98dc455c01912229db4c617"></a>
## eq

`function` · `sqlparser::ast::query::Values::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Values) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3649, 24], "end": [3649, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9b2ac58c00f6828ec654b3e"></a>
## explicit_row

`struct_field` · `sqlparser::ast::query::Values::explicit_row` · sqlparser 0.62.0

```rust
explicit_row: bool
```

Source: `src/ast/query.rs:3656`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Was there an explicit `ROW` keyword (MySQL)?
<https://dev.mysql.com/doc/refman/8.0/en/values.html>

<a id="op-c60da8ea71023f70723e7fd6"></a>
## fmt

`function` · `sqlparser::ast::query::Values::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3664, 1], "end": [3680, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3665`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e626103fe967b1979b3aaa1a"></a>
## fmt

`function` · `sqlparser::ast::query::Values::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3649, 10], "end": [3649, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5622f03983bd4b1f0b3165ad"></a>
## hash

`function` · `sqlparser::ast::query::Values::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3649, 56], "end": [3649, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16ddb4ef0e44acde25f997f6"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Values::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Values) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3649, 35], "end": [3649, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c08e438addb3206924c6791d"></a>
## rows

`struct_field` · `sqlparser::ast::query::Values::rows` · sqlparser 0.62.0

```rust
rows: Vec<Parens<Vec<Expr>>>
```

Source: `src/ast/query.rs:3661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The list of rows, each row is a list of expressions.

<a id="op-cc2092123ec9e8e95352b323"></a>
## serialize

`function` · `sqlparser::ast::query::Values::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3650, 38], "end": [3650, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d37fed449168cd292e320f8a"></a>
## span

`function` · `sqlparser::ast::query::Values::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "super::Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [254, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fc551d89083a8a9217a3460"></a>
## value_keyword

`struct_field` · `sqlparser::ast::query::Values::value_keyword` · sqlparser 0.62.0

```rust
value_keyword: bool
```

Source: `src/ast/query.rs:3659`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` if `VALUE` (singular) keyword was used instead of `VALUES`.
<https://dev.mysql.com/doc/refman/9.2/en/insert.html>

<a id="op-121d513881520bc82a2e812d"></a>
## visit

`function` · `sqlparser::ast::query::Values::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3651, 40], "end": [3651, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cfcfa4815f6b14a7fa6bdb9"></a>
## visit

`function` · `sqlparser::ast::query::Values::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3651, 47], "end": [3651, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
