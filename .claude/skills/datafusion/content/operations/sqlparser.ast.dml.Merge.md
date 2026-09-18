# `sqlparser::ast::dml::Merge`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.Merge.json).

<a id="op-2388ba460453924eb8ea0ee8"></a>
## Merge

`struct` · `sqlparser::ast::dml::Merge` · sqlparser 0.62.0

```rust
struct Merge
```

Source: `src/ast/dml.rs:457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `MERGE` statement.

<a id="op-5e3918fe32a5e167265ff72b"></a>
## clauses

`struct_field` · `sqlparser::ast::dml::Merge::clauses` · sqlparser 0.62.0

```rust
clauses: Vec<MergeClause>
```

Source: `src/ast/dml.rs:473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the actions to perform when values match or do not match.

<a id="op-745bcebcd36382d3460b2995"></a>
## clone

`function` · `sqlparser::ast::dml::Merge::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Merge
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 17], "end": [454, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0efa381569982d0ec4c2893c"></a>
## cmp

`function` · `sqlparser::ast::dml::Merge::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Merge) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 51], "end": [454, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb12a3b1ee7b25b925b41627"></a>
## deserialize

`function` · `sqlparser::ast::dml::Merge::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 49], "end": [455, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2e59d19126e8ba1a8fab933"></a>
## eq

`function` · `sqlparser::ast::dml::Merge::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Merge) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 24], "end": [454, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-742bba341220d2595038d14d"></a>
## fmt

`function` · `sqlparser::ast::dml::Merge::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 1], "end": [500, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:479`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7e1c05cbbcfb1be424fdf2d"></a>
## fmt

`function` · `sqlparser::ast::dml::Merge::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 10], "end": [454, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc96e3e4c0688c011a25c240"></a>
## hash

`function` · `sqlparser::ast::dml::Merge::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 56], "end": [454, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecf09ff14aa07bfe81cd42e5"></a>
## into

`struct_field` · `sqlparser::ast::dml::Merge::into` · sqlparser 0.62.0

```rust
into: bool
```

Source: `src/ast/dml.rs:465`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

optional INTO keyword

<a id="op-586519d41e9aa324687bc140"></a>
## merge_token

`struct_field` · `sqlparser::ast::dml::Merge::merge_token` · sqlparser 0.62.0

```rust
merge_token: super::helpers::attached_token::AttachedToken
```

Source: `src/ast/dml.rs:459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `MERGE` token that starts the statement.

<a id="op-5b8e4f3aade1ecbebe67dd90"></a>
## on

`struct_field` · `sqlparser::ast::dml::Merge::on` · sqlparser 0.62.0

```rust
on: Box<super::Expr>
```

Source: `src/ast/dml.rs:471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the expression on which to join the target table and source

<a id="op-cf9ee92494625fb13ca4b56c"></a>
## optimizer_hints

`struct_field` · `sqlparser::ast::dml::Merge::optimizer_hints` · sqlparser 0.62.0

```rust
optimizer_hints: Vec<super::OptimizerHint>
```

Source: `src/ast/dml.rs:463`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Query optimizer hints

[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Comments.html#GUID-D316D545-89E2-4D54-977F-FC97815CD62E)

<a id="op-88dc7283dba0a43f2f35c8d5"></a>
## output

`struct_field` · `sqlparser::ast::dml::Merge::output` · sqlparser 0.62.0

```rust
output: Option<OutputClause>
```

Source: `src/ast/dml.rs:475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the output to save changes in MSSQL

<a id="op-686274e577291fd10e00dcb5"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::Merge::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Merge) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 35], "end": [454, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef6bcb2047e7a42b175b6efe"></a>
## serialize

`function` · `sqlparser::ast::dml::Merge::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 38], "end": [455, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f1aab1f0c93b1a32e7daa13"></a>
## source

`struct_field` · `sqlparser::ast::dml::Merge::source` · sqlparser 0.62.0

```rust
source: super::TableFactor
```

Source: `src/ast/dml.rs:469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the table or subquery to join with the target table

<a id="op-ea314c02348cf868a982cb83"></a>
## span

`function` · `sqlparser::ast::dml::Merge::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "super::Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [987, 1], "end": [996, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:988`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e387cd6dcf333051ad9f7941"></a>
## table

`struct_field` · `sqlparser::ast::dml::Merge::table` · sqlparser 0.62.0

```rust
table: super::TableFactor
```

Source: `src/ast/dml.rs:467`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the table to merge

<a id="op-7ab05b353baf4ccda14faecd"></a>
## visit

`function` · `sqlparser::ast::dml::Merge::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 40], "end": [456, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7e6912d731ea558f2d0eb0c"></a>
## visit

`function` · `sqlparser::ast::dml::Merge::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 47], "end": [456, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
