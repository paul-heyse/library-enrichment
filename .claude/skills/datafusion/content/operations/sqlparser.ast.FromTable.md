# `sqlparser::ast::FromTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FromTable.json).

<a id="op-72ce4e776f4567150274980d"></a>
## FromTable

`enum` · `sqlparser::ast::FromTable` · sqlparser 0.62.0

```rust
enum FromTable
```

Source: `src/ast/mod.rs:3221`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `FROM` clause within a `DELETE` statement.

Syntax
```sql
[FROM] table
```

<a id="op-35a5ca1ccda23e87aa2edac4"></a>
## WithFromKeyword

`variant` · `sqlparser::ast::FromTable::WithFromKeyword` · sqlparser 0.62.0

```rust
WithFromKeyword
```

Source: `src/ast/mod.rs:3223`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An explicit `FROM` keyword was specified.

<a id="op-5c5642c2aded5f963d2ab3c2"></a>
## WithoutKeyword

`variant` · `sqlparser::ast::FromTable::WithoutKeyword` · sqlparser 0.62.0

```rust
WithoutKeyword
```

Source: `src/ast/mod.rs:3226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

BigQuery: `FROM` keyword was omitted.
<https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#delete_statement>

<a id="op-c2c9b4812a756be76097bd4a"></a>
## clone

`function` · `sqlparser::ast::FromTable::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FromTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3218, 17], "end": [3218, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:3218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3c69666bab6c8e6ba74017f"></a>
## cmp

`function` · `sqlparser::ast::FromTable::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FromTable) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3218, 51], "end": [3218, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:3218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2a80ebaaece9bf5fb10385d"></a>
## deserialize

`function` · `sqlparser::ast::FromTable::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3219, 49], "end": [3219, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:3219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36686f0238d2fcea58b3355c"></a>
## eq

`function` · `sqlparser::ast::FromTable::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FromTable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3218, 24], "end": [3218, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:3218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33db6dde1c21b8ec573f458c"></a>
## fmt

`function` · `sqlparser::ast::FromTable::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3218, 10], "end": [3218, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:3218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7845598c5e5ca418e191ca5"></a>
## fmt

`function` · `sqlparser::ast::FromTable::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3228, 1], "end": [3239, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:3229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5423ec547d73c547c8057e90"></a>
## hash

`function` · `sqlparser::ast::FromTable::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3218, 56], "end": [3218, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:3218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb1668a3ab8bdc0ccd02ab0d"></a>
## partial_cmp

`function` · `sqlparser::ast::FromTable::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FromTable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3218, 35], "end": [3218, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:3218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d0794bf63d5f48dc96af194"></a>
## serialize

`function` · `sqlparser::ast::FromTable::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3219, 38], "end": [3219, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:3219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09e1bbc11298ab9570b97214"></a>
## span

`function` · `sqlparser::ast::FromTable::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "super::FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [998, 1], "end": [1005, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-165c60c2b9245e2f200cb220"></a>
## visit

`function` · `sqlparser::ast::FromTable::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3220, 40], "end": [3220, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:3220`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c694bae5e181e452572749de"></a>
## visit

`function` · `sqlparser::ast::FromTable::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FromTable", "path": "FromTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3220, 47], "end": [3220, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:3220`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
