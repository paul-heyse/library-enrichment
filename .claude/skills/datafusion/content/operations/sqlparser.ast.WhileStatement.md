# `sqlparser::ast::WhileStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.WhileStatement.json).

<a id="op-88cee8466242f4c774d2a5f9"></a>
## WhileStatement

`struct` · `sqlparser::ast::WhileStatement` · sqlparser 0.62.0

```rust
struct WhileStatement
```

Source: `src/ast/mod.rs:2704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `WHILE` statement.

Example:
```sql
WHILE @@FETCH_STATUS = 0
BEGIN
   FETCH NEXT FROM c1 INTO @var1, @var2;
END
```

[MsSql](https://learn.microsoft.com/en-us/sql/t-sql/language-elements/while-transact-sql)

<a id="op-01c4d6106f7fd9d80ff7897e"></a>
## clone

`function` · `sqlparser::ast::WhileStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WhileStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2701, 17], "end": [2701, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1374156edb1e35631f57cc0c"></a>
## cmp

`function` · `sqlparser::ast::WhileStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WhileStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2701, 51], "end": [2701, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70330a04db7763817b2a15fb"></a>
## deserialize

`function` · `sqlparser::ast::WhileStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2702, 49], "end": [2702, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d85c04c33f41d778e8f6ae9"></a>
## eq

`function` · `sqlparser::ast::WhileStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WhileStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2701, 24], "end": [2701, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc86ae55895e10dcb80d604b"></a>
## fmt

`function` · `sqlparser::ast::WhileStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 1], "end": [2715, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2710`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8963b3ab420d18664b7b087"></a>
## fmt

`function` · `sqlparser::ast::WhileStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2701, 10], "end": [2701, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a42f786fd6c493d23644f26"></a>
## hash

`function` · `sqlparser::ast::WhileStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2701, 56], "end": [2701, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b130086d1d81122f12239020"></a>
## partial_cmp

`function` · `sqlparser::ast::WhileStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WhileStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2701, 35], "end": [2701, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb305a192ad0956b008c82b2"></a>
## serialize

`function` · `sqlparser::ast::WhileStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2702, 38], "end": [2702, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b892035e149a62a3fcf2b6a"></a>
## span

`function` · `sqlparser::ast::WhileStatement::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "super::WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [753, 1], "end": [759, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16ca337ea6369d81823ab828"></a>
## visit

`function` · `sqlparser::ast::WhileStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2703, 40], "end": [2703, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2703`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99b525ca5caba6b7bec6de7f"></a>
## visit

`function` · `sqlparser::ast::WhileStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2703, 47], "end": [2703, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2703`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f2240c935e322b47af72945"></a>
## while_block

`struct_field` · `sqlparser::ast::WhileStatement::while_block` · sqlparser 0.62.0

```rust
while_block: ConditionalStatementBlock
```

Source: `src/ast/mod.rs:2706`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Block executed while the condition holds.
