# `sqlparser::ast::query::TableAlias`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableAlias.json).

<a id="op-1c41671feae251420ea7d00f"></a>
## TableAlias

`struct` · `sqlparser::ast::query::TableAlias` · sqlparser 0.62.0

```rust
struct TableAlias
```

Source: `src/ast/query.rs:2527`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An alias for a table reference, optionally including an explicit `AS` and column names.

<a id="op-7f85cdfe4286e86026086053"></a>
## at

`struct_field` · `sqlparser::ast::query::TableAlias::at` · sqlparser 0.62.0

```rust
at: Option<Ident>
```

Source: `src/ast/query.rs:2541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional PartiQL index alias declared with `AT`. For example:
```sql
SELECT element, index FROM bar AS b, b.data.scalar_array AS element AT index
```
See: <https://docs.aws.amazon.com/redshift/latest/dg/query-super.html>

<a id="op-e1bc555fa86961434ce04e40"></a>
## clone

`function` · `sqlparser::ast::query::TableAlias::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableAlias
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2523, 17], "end": [2523, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-593cfc4cae66a2e24fe4ad05"></a>
## cmp

`function` · `sqlparser::ast::query::TableAlias::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableAlias) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2523, 51], "end": [2523, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e687dd194a28246d4cfc75cd"></a>
## columns

`struct_field` · `sqlparser::ast::query::TableAlias::columns` · sqlparser 0.62.0

```rust
columns: Vec<TableAliasColumnDef>
```

Source: `src/ast/query.rs:2535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional column aliases declared in parentheses after the table alias.

<a id="op-cc9d65324b027a3daba88b92"></a>
## deserialize

`function` · `sqlparser::ast::query::TableAlias::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2524, 49], "end": [2524, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2266538ce87eaa65ba001d90"></a>
## eq

`function` · `sqlparser::ast::query::TableAlias::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableAlias) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2523, 24], "end": [2523, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e9355025ce56552667144b1"></a>
## explicit

`struct_field` · `sqlparser::ast::query::TableAlias::explicit` · sqlparser 0.62.0

```rust
explicit: bool
```

Source: `src/ast/query.rs:2531`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tells whether the alias was introduced with an explicit, preceding "AS"
keyword, e.g. `AS name`. Typically, the keyword is preceding the name
(e.g. `.. FROM table AS t ..`).

<a id="op-525f811cd35de3213798be10"></a>
## fmt

`function` · `sqlparser::ast::query::TableAlias::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2523, 10], "end": [2523, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d12eb380d3abadd44af748f"></a>
## fmt

`function` · `sqlparser::ast::query::TableAlias::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2544, 1], "end": [2555, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2545`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42fe0881c430b62a7d8ea606"></a>
## hash

`function` · `sqlparser::ast::query::TableAlias::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2523, 56], "end": [2523, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c7c3b33613cd01317e48ed1"></a>
## name

`struct_field` · `sqlparser::ast::query::TableAlias::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/query.rs:2533`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Alias identifier for the table.

<a id="op-d019b11ca4600fb67ecc2091"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableAlias::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableAlias) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2523, 35], "end": [2523, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-726feb944d030aad7177fa4d"></a>
## serialize

`function` · `sqlparser::ast::query::TableAlias::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2524, 38], "end": [2524, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df9e4c0aaf4ad1c74b140f6d"></a>
## span

`function` · `sqlparser::ast::query::TableAlias::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "super::TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2183, 1], "end": [2197, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dffeda11ef3fb37d4307d82"></a>
## visit

`function` · `sqlparser::ast::query::TableAlias::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2525, 47], "end": [2525, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91428a5bfe7cec60103403d5"></a>
## visit

`function` · `sqlparser::ast::query::TableAlias::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAlias", "path": "TableAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2525, 40], "end": [2525, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
