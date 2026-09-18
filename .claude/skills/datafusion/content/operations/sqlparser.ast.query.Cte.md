# `sqlparser::ast::query::Cte`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Cte.json).

<a id="op-d15f90443aa034df9bab24b5"></a>
## Cte

`struct` · `sqlparser::ast::query::Cte` · sqlparser 0.62.0

```rust
struct Cte
```

Source: `src/ast/query.rs:805`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single CTE (used after `WITH`): `<alias> [(col1, col2, ...)] AS <materialized> ( <query> )`
The names in the column list before `AS`, when specified, replace the names
of the columns returned by the query. The parser does not validate that the
number of columns in the query matches the number of columns in the query.

<a id="op-9188c04ca58a9131fddc9f23"></a>
## alias

`struct_field` · `sqlparser::ast::query::Cte::alias` · sqlparser 0.62.0

```rust
alias: TableAlias
```

Source: `src/ast/query.rs:807`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The CTE alias (name introduced before the `AS` keyword).

<a id="op-620cc31a3efe0d6633fdca13"></a>
## clone

`function` · `sqlparser::ast::query::Cte::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Cte
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 17], "end": [802, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:802`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d4742b1bccd9783f7c62298"></a>
## closing_paren_token

`struct_field` · `sqlparser::ast::query::Cte::closing_paren_token` · sqlparser 0.62.0

```rust
closing_paren_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/query.rs:815`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Token for the closing parenthesis of the CTE definition.

<a id="op-273a3977f900dc72703dbee2"></a>
## cmp

`function` · `sqlparser::ast::query::Cte::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Cte) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 51], "end": [802, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:802`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cbad03c7f8f280de99a86eb"></a>
## deserialize

`function` · `sqlparser::ast::query::Cte::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [803, 49], "end": [803, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-567bdf8c467a192a410a33cd"></a>
## eq

`function` · `sqlparser::ast::query::Cte::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Cte) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 24], "end": [802, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:802`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f9dc4d8e99cd97be5d6001e"></a>
## fmt

`function` · `sqlparser::ast::query::Cte::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [818, 1], "end": [845, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8be43d029fe4dec53b40eb20"></a>
## fmt

`function` · `sqlparser::ast::query::Cte::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 10], "end": [802, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:802`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49447a1118176137faa7d1da"></a>
## from

`struct_field` · `sqlparser::ast::query::Cte::from` · sqlparser 0.62.0

```rust
from: Option<Ident>
```

Source: `src/ast/query.rs:811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `FROM` identifier for materialized CTEs.

<a id="op-aff77f285c496c7958c17539"></a>
## hash

`function` · `sqlparser::ast::query::Cte::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 56], "end": [802, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:802`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7aaa8a50663f06d197eacda"></a>
## materialized

`struct_field` · `sqlparser::ast::query::Cte::materialized` · sqlparser 0.62.0

```rust
materialized: Option<CteAsMaterialized>
```

Source: `src/ast/query.rs:813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `AS MATERIALIZED` / `AS NOT MATERIALIZED` hint.

<a id="op-7ed276c25511704f963a4a8b"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Cte::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Cte) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 35], "end": [802, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:802`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1270c1bcf243d1865ed85bf2"></a>
## query

`struct_field` · `sqlparser::ast::query::Cte::query` · sqlparser 0.62.0

```rust
query: Box<Query>
```

Source: `src/ast/query.rs:809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The query that defines the CTE body.

<a id="op-a0a95844c5ac3c725d758988"></a>
## serialize

`function` · `sqlparser::ast::query::Cte::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [803, 38], "end": [803, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94b1f8ba32fe1510146ca26a"></a>
## span

`function` · `sqlparser::ast::query::Cte::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "super::Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [214, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:198`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f602172579a94b6d24bcb2f7"></a>
## visit

`function` · `sqlparser::ast::query::Cte::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [804, 40], "end": [804, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:804`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc4f7e44830988d32a47b9a8"></a>
## visit

`function` · `sqlparser::ast::query::Cte::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Cte", "path": "Cte"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [804, 47], "end": [804, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:804`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
