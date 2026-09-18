# `sqlparser::ast::ConditionalStatementBlock`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ConditionalStatementBlock.json).

<a id="op-6d364edf1d9ed5606c36d180"></a>
## ConditionalStatementBlock

`struct` · `sqlparser::ast::ConditionalStatementBlock` · sqlparser 0.62.0

```rust
struct ConditionalStatementBlock
```

Source: `src/ast/mod.rs:2744`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A block within a [Statement::Case](../operations/sqlparser.ast.Statement.md#op-fa906e2a69b8ff6269f8f9eb) or [Statement::If](../operations/sqlparser.ast.Statement.md#op-91544b25fd3e9a33e5de011c) or [Statement::While](../operations/sqlparser.ast.Statement.md#op-a61a71e42600cd57ed03ba56)-like statement

Example 1:
```sql
WHEN EXISTS(SELECT 1) THEN SELECT 1;
```

Example 2:
```sql
IF TRUE THEN SELECT 1; SELECT 2;
```

Example 3:
```sql
ELSE SELECT 1; SELECT 2;
```

Example 4:
```sql
WHILE @@FETCH_STATUS = 0
BEGIN
   FETCH NEXT FROM c1 INTO @var1, @var2;
END
```

<a id="op-5b6f91ea6eb6b9101bbc3e6b"></a>
## clone

`function` · `sqlparser::ast::ConditionalStatementBlock::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ConditionalStatementBlock
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2741, 17], "end": [2741, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd69b7cabc83fb7ab1e4c1da"></a>
## cmp

`function` · `sqlparser::ast::ConditionalStatementBlock::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ConditionalStatementBlock) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2741, 51], "end": [2741, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-811cb68e46723c6187655425"></a>
## condition

`struct_field` · `sqlparser::ast::ConditionalStatementBlock::condition` · sqlparser 0.62.0

```rust
condition: Option<Expr>
```

Source: `src/ast/mod.rs:2748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional condition expression for the block.

<a id="op-cc2e1b8f3dc92e6b547e5aee"></a>
## conditional_statements

`struct_field` · `sqlparser::ast::ConditionalStatementBlock::conditional_statements` · sqlparser 0.62.0

```rust
conditional_statements: ConditionalStatements
```

Source: `src/ast/mod.rs:2752`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The statements contained in this conditional block.

<a id="op-430a860546fd3837fbab8268"></a>
## deserialize

`function` · `sqlparser::ast::ConditionalStatementBlock::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2742, 49], "end": [2742, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2742`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f07c75d8003e23f64b39315"></a>
## eq

`function` · `sqlparser::ast::ConditionalStatementBlock::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ConditionalStatementBlock) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2741, 24], "end": [2741, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3866f230209d252c20204358"></a>
## fmt

`function` · `sqlparser::ast::ConditionalStatementBlock::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2762, 1], "end": [2787, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2763`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a831cfa2346f582265363bcc"></a>
## fmt

`function` · `sqlparser::ast::ConditionalStatementBlock::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2741, 10], "end": [2741, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64cf8ea2b49b884d1d789e4b"></a>
## hash

`function` · `sqlparser::ast::ConditionalStatementBlock::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2741, 56], "end": [2741, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c48737339e7e8d32121346a0"></a>
## partial_cmp

`function` · `sqlparser::ast::ConditionalStatementBlock::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ConditionalStatementBlock) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2741, 35], "end": [2741, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fe662e633516bd36907235a"></a>
## serialize

`function` · `sqlparser::ast::ConditionalStatementBlock::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2742, 38], "end": [2742, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2742`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-265d72111594fa98cea4401d"></a>
## span

`function` · `sqlparser::ast::ConditionalStatementBlock::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "super::ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 1], "end": [788, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f05d17ec370cd7c7f27a6178"></a>
## start_token

`struct_field` · `sqlparser::ast::ConditionalStatementBlock::start_token` · sqlparser 0.62.0

```rust
start_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/mod.rs:2746`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Token representing the start of the block (e.g., WHEN/IF/WHILE).

<a id="op-4df2ee5d8ee2db5ff05d7902"></a>
## statements

`function` · `sqlparser::ast::ConditionalStatementBlock::statements` · sqlparser 0.62.0

```rust
fn statements(&self) -> &Vec<Statement>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2755, 1], "end": [2760, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:2757`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Get the statements in this conditional block.

<a id="op-5db1b7fe6733ee12ef407092"></a>
## then_token

`struct_field` · `sqlparser::ast::ConditionalStatementBlock::then_token` · sqlparser 0.62.0

```rust
then_token: Option<helpers::attached_token::AttachedToken>
```

Source: `src/ast/mod.rs:2750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional token for the `THEN` keyword.

<a id="op-866291b83e84e0ed098e2ef8"></a>
## visit

`function` · `sqlparser::ast::ConditionalStatementBlock::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2743, 47], "end": [2743, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a60b1ec0a2975544cd79151c"></a>
## visit

`function` · `sqlparser::ast::ConditionalStatementBlock::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatementBlock", "path": "ConditionalStatementBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2743, 40], "end": [2743, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
