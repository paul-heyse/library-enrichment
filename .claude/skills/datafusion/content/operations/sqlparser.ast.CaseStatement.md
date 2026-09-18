# `sqlparser::ast::CaseStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CaseStatement.json).

<a id="op-08227235c0301d99dde5b635"></a>
## CaseStatement

`struct` · `sqlparser::ast::CaseStatement` · sqlparser 0.62.0

```rust
struct CaseStatement
```

Source: `src/ast/mod.rs:2579`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `CASE` statement.

Examples:
```sql
CASE
    WHEN EXISTS(SELECT 1)
        THEN SELECT 1 FROM T;
    WHEN EXISTS(SELECT 2)
        THEN SELECT 1 FROM U;
    ELSE
        SELECT 1 FROM V;
END CASE;
```

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#case_search_expression)
[Snowflake](https://docs.snowflake.com/en/sql-reference/snowflake-scripting/case)

<a id="op-d644cbd4044f887bd72a407c"></a>
## case_token

`struct_field` · `sqlparser::ast::CaseStatement::case_token` · sqlparser 0.62.0

```rust
case_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/mod.rs:2581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `CASE` token that starts the statement.

<a id="op-05a54f0bbb00a96634b94f0f"></a>
## clone

`function` · `sqlparser::ast::CaseStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CaseStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2576, 17], "end": [2576, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71835cb251bea37a0df02d88"></a>
## cmp

`function` · `sqlparser::ast::CaseStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CaseStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2576, 51], "end": [2576, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ccbdf16fac62d3b9f57af20"></a>
## deserialize

`function` · `sqlparser::ast::CaseStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2577, 49], "end": [2577, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdd8a1765a4db3611683631d"></a>
## else_block

`struct_field` · `sqlparser::ast::CaseStatement::else_block` · sqlparser 0.62.0

```rust
else_block: Option<ConditionalStatementBlock>
```

Source: `src/ast/mod.rs:2587`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `ELSE` block for the `CASE` statement.

<a id="op-f8b7b12203641ca28f165b4b"></a>
## end_case_token

`struct_field` · `sqlparser::ast::CaseStatement::end_case_token` · sqlparser 0.62.0

```rust
end_case_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/mod.rs:2589`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The last token of the statement (`END` or `CASE`).

<a id="op-2c561aaa2d28f79d6413478d"></a>
## eq

`function` · `sqlparser::ast::CaseStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CaseStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2576, 24], "end": [2576, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dc128a6ade712c5a0334c80"></a>
## fmt

`function` · `sqlparser::ast::CaseStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2576, 10], "end": [2576, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92777086067cc0303c88dd5a"></a>
## fmt

`function` · `sqlparser::ast::CaseStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2592, 1], "end": [2626, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0138005c081cbcb8e5b1042"></a>
## hash

`function` · `sqlparser::ast::CaseStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2576, 56], "end": [2576, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9612dc5a96ecd838461a7db6"></a>
## match_expr

`struct_field` · `sqlparser::ast::CaseStatement::match_expr` · sqlparser 0.62.0

```rust
match_expr: Option<Expr>
```

Source: `src/ast/mod.rs:2583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional expression to match against in `CASE ... WHEN`.

<a id="op-899fc7b4e2bf5b88ca847e7f"></a>
## partial_cmp

`function` · `sqlparser::ast::CaseStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CaseStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2576, 35], "end": [2576, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b07d3b48755dd577313f6756"></a>
## serialize

`function` · `sqlparser::ast::CaseStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2577, 38], "end": [2577, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d983e575bea813d84960fda9"></a>
## span

`function` · `sqlparser::ast::CaseStatement::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "super::CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [721, 1], "end": [733, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:722`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6934c1db52a211cb23b7c758"></a>
## visit

`function` · `sqlparser::ast::CaseStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2578, 47], "end": [2578, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2578`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d010f408919a2edf68017bb7"></a>
## visit

`function` · `sqlparser::ast::CaseStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2578, 40], "end": [2578, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2578`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81d484f6811997d8720f1be6"></a>
## when_blocks

`struct_field` · `sqlparser::ast::CaseStatement::when_blocks` · sqlparser 0.62.0

```rust
when_blocks: Vec<ConditionalStatementBlock>
```

Source: `src/ast/mod.rs:2585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `WHEN ... THEN` blocks of the `CASE` statement.
