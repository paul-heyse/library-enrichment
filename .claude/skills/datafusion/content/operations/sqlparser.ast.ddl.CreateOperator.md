# `sqlparser::ast::ddl::CreateOperator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateOperator.json).

<a id="op-14168759e4f60463e5defd95"></a>
## CreateOperator

`struct` · `sqlparser::ast::ddl::CreateOperator` · sqlparser 0.62.0

```rust
struct CreateOperator
```

Source: `src/ast/ddl.rs:4752`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE OPERATOR statement
See <https://www.postgresql.org/docs/current/sql-createoperator.html>

<a id="op-24e5669eaba8c710c93bf0fb"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateOperator::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateOperator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4749, 17], "end": [4749, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18db48bf1c945337a19be6db"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateOperator::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateOperator) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4749, 51], "end": [4749, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e748197c844d9fe2b0cdc50"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateOperator::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4750, 49], "end": [4750, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf7b2b411228e11cb9cce3b2"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateOperator::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateOperator) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4749, 24], "end": [4749, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2da5862a44bb453c671b8ef2"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4749, 10], "end": [4749, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b92c58799c07ba9faffabaa"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4799, 1], "end": [4824, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4800`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a1e28188894ce37499f43a6"></a>
## function

`struct_field` · `sqlparser::ast::ddl::CreateOperator::function` · sqlparser 0.62.0

```rust
function: ast::ObjectName
```

Source: `src/ast/ddl.rs:4756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FUNCTION or PROCEDURE parameter (function name)

<a id="op-e36c710530ec5d6fd05fe358"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateOperator::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4749, 56], "end": [4749, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e41fe1562c4c398374ec7a6"></a>
## is_procedure

`struct_field` · `sqlparser::ast::ddl::CreateOperator::is_procedure` · sqlparser 0.62.0

```rust
is_procedure: bool
```

Source: `src/ast/ddl.rs:4758`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether PROCEDURE keyword was used (vs FUNCTION)

<a id="op-6bf5bec439d5b035b829031b"></a>
## left_arg

`struct_field` · `sqlparser::ast::ddl::CreateOperator::left_arg` · sqlparser 0.62.0

```rust
left_arg: Option<ast::DataType>
```

Source: `src/ast/ddl.rs:4760`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LEFTARG parameter (left operand type)

<a id="op-6c84c10a3583ec31aa486c17"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateOperator::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator name (can be schema-qualified)

<a id="op-a4ddef4713a91802604558b1"></a>
## options

`struct_field` · `sqlparser::ast::ddl::CreateOperator::options` · sqlparser 0.62.0

```rust
options: Vec<OperatorOption>
```

Source: `src/ast/ddl.rs:4764`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator options (COMMUTATOR, NEGATOR, RESTRICT, JOIN, HASHES, MERGES)

<a id="op-7145cb0f2c29eca5ab690741"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateOperator::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateOperator) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4749, 35], "end": [4749, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69a2adad7aa35f5f3d9b7640"></a>
## right_arg

`struct_field` · `sqlparser::ast::ddl::CreateOperator::right_arg` · sqlparser 0.62.0

```rust
right_arg: Option<ast::DataType>
```

Source: `src/ast/ddl.rs:4762`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

RIGHTARG parameter (right operand type)

<a id="op-2f8b8457530e0470b30ea501"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateOperator::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4750, 38], "end": [4750, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb77759ffb754e6415474003"></a>
## span

`function` · `sqlparser::ast::ddl::CreateOperator::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "crate::ast::CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2495, 1], "end": [2499, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2496`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bf9565e7ad853b36761197d"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4751, 47], "end": [4751, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91fb8e835ebc9bca232983d9"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4751, 40], "end": [4751, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
