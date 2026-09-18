# `sqlparser::ast::dml::OutputClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.OutputClause.json).

<a id="op-fc895a09dc6771d433345e94"></a>
## OutputClause

`enum` · `sqlparser::ast::dml::OutputClause` · sqlparser 0.62.0

```rust
enum OutputClause
```

Source: `src/ast/dml.rs:758`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `OUTPUT` clause on `MERGE`, `INSERT`, `UPDATE`, or `DELETE` (MSSQL).

Example:
OUTPUT $action, deleted.* INTO dbo.temp_products;
<https://learn.microsoft.com/en-us/sql/t-sql/queries/output-clause-transact-sql>

<a id="op-87c6bf6bc19444360216611c"></a>
## Output

`variant` · `sqlparser::ast::dml::OutputClause::Output` · sqlparser 0.62.0

```rust
Output
```

Source: `src/ast/dml.rs:760`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OUTPUT` clause

<a id="op-bc083057842283502f0ef65e"></a>
## Returning

`variant` · `sqlparser::ast::dml::OutputClause::Returning` · sqlparser 0.62.0

```rust
Returning
```

Source: `src/ast/dml.rs:769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RETURNING` clause

<a id="op-0c1c64244c7d7f118d0e2569"></a>
## clone

`function` · `sqlparser::ast::dml::OutputClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OutputClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [755, 17], "end": [755, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f631773dabfa2139aa2a88a"></a>
## cmp

`function` · `sqlparser::ast::dml::OutputClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OutputClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [755, 51], "end": [755, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-327c4db56855d64891f61473"></a>
## deserialize

`function` · `sqlparser::ast::dml::OutputClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [756, 49], "end": [756, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ffecb4e79db716c4b6ad79c"></a>
## eq

`function` · `sqlparser::ast::dml::OutputClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OutputClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [755, 24], "end": [755, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e33f453206082845927f3db"></a>
## fmt

`function` · `sqlparser::ast::dml::OutputClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 1], "end": [802, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:778`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2cf85eab4383cf72e49b9d2"></a>
## fmt

`function` · `sqlparser::ast::dml::OutputClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [755, 10], "end": [755, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19b47ffdbe91ce660233be46"></a>
## hash

`function` · `sqlparser::ast::dml::OutputClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [755, 56], "end": [755, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-371bd6e5ae739d518c629e6e"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::OutputClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OutputClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [755, 35], "end": [755, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50d36281e069fee0f690aab9"></a>
## serialize

`function` · `sqlparser::ast::dml::OutputClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [756, 38], "end": [756, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29fb727d90f6da5ba93d6b47"></a>
## span

`function` · `sqlparser::ast::dml::OutputClause::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "super::OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2558, 1], "end": [2579, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2559`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-751e041614e8d897e53ec623"></a>
## visit

`function` · `sqlparser::ast::dml::OutputClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [757, 47], "end": [757, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:757`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5e3857c24b21677987b0713"></a>
## visit

`function` · `sqlparser::ast::dml::OutputClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::OutputClause", "path": "OutputClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [757, 40], "end": [757, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:757`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
