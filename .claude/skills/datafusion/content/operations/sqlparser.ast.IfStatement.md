# `sqlparser::ast::IfStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.IfStatement.json).

<a id="op-091928ba79df7343bc245430"></a>
## IfStatement

`struct` · `sqlparser::ast::IfStatement` · sqlparser 0.62.0

```rust
struct IfStatement
```

Source: `src/ast/mod.rs:2652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `IF` statement.

Example (BigQuery or Snowflake):
```sql
IF TRUE THEN
    SELECT 1;
    SELECT 2;
ELSEIF TRUE THEN
    SELECT 3;
ELSE
    SELECT 4;
END IF
```
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#if)
[Snowflake](https://docs.snowflake.com/en/sql-reference/snowflake-scripting/if)

Example (MSSQL):
```sql
IF 1=1 SELECT 1 ELSE SELECT 2
```
[MSSQL](https://learn.microsoft.com/en-us/sql/t-sql/language-elements/if-else-transact-sql?view=sql-server-ver16)

<a id="op-1849b5fb542b3585e57cff38"></a>
## clone

`function` · `sqlparser::ast::IfStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IfStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2649, 17], "end": [2649, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3add8977ee972bfb1b3fd7d"></a>
## cmp

`function` · `sqlparser::ast::IfStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IfStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2649, 51], "end": [2649, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-366920806dad60cd9ecd975f"></a>
## deserialize

`function` · `sqlparser::ast::IfStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2650, 49], "end": [2650, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a20ba05fc33dc4801e2d541"></a>
## else_block

`struct_field` · `sqlparser::ast::IfStatement::else_block` · sqlparser 0.62.0

```rust
else_block: Option<ConditionalStatementBlock>
```

Source: `src/ast/mod.rs:2658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `ELSE` block.

<a id="op-f39af6da04e08b75a95c114d"></a>
## elseif_blocks

`struct_field` · `sqlparser::ast::IfStatement::elseif_blocks` · sqlparser 0.62.0

```rust
elseif_blocks: Vec<ConditionalStatementBlock>
```

Source: `src/ast/mod.rs:2656`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional `ELSEIF` blocks.

<a id="op-a4bbc136290f8375def489ee"></a>
## end_token

`struct_field` · `sqlparser::ast::IfStatement::end_token` · sqlparser 0.62.0

```rust
end_token: Option<helpers::attached_token::AttachedToken>
```

Source: `src/ast/mod.rs:2660`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional trailing `END` token for the `IF` statement.

<a id="op-9a3eadd614aea0f3e3891cad"></a>
## eq

`function` · `sqlparser::ast::IfStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IfStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2649, 24], "end": [2649, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55e2c6ae94317fc6ba27852e"></a>
## fmt

`function` · `sqlparser::ast::IfStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2649, 10], "end": [2649, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1a5056ba5d78f0ac5859b7a"></a>
## fmt

`function` · `sqlparser::ast::IfStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2663, 1], "end": [2688, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41c517863836e08fd25518ef"></a>
## hash

`function` · `sqlparser::ast::IfStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2649, 56], "end": [2649, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4fcc7b46101142ade419cae"></a>
## if_block

`struct_field` · `sqlparser::ast::IfStatement::if_block` · sqlparser 0.62.0

```rust
if_block: ConditionalStatementBlock
```

Source: `src/ast/mod.rs:2654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The initial `IF` block containing the condition and statements.

<a id="op-2a8d03cc2bb12168613174eb"></a>
## partial_cmp

`function` · `sqlparser::ast::IfStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IfStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2649, 35], "end": [2649, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34e7923498ae551cba253e88"></a>
## serialize

`function` · `sqlparser::ast::IfStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2650, 38], "end": [2650, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c651b49d809dc52968dd682"></a>
## span

`function` · `sqlparser::ast::IfStatement::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "super::IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [735, 1], "end": [751, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:736`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10deff0aaaf8019464265789"></a>
## visit

`function` · `sqlparser::ast::IfStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2651, 40], "end": [2651, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d8f3cf69c911bb6946493f1"></a>
## visit

`function` · `sqlparser::ast::IfStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2651, 47], "end": [2651, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
