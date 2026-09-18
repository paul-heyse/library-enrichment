# `sqlparser::ast::Assignment`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Assignment.json).

<a id="op-2ff006d8b5aa9f3f9bd2cc0a"></a>
## Assignment

`struct` · `sqlparser::ast::Assignment` · sqlparser 0.62.0

```rust
struct Assignment
```

Source: `src/ast/mod.rs:7811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL assignment `foo = expr` as used in SQLUpdate

<a id="op-f2a9c202332a63da9bd727e9"></a>
## clone

`function` · `sqlparser::ast::Assignment::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Assignment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7808, 17], "end": [7808, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5f15653f3e4f0b29708616e"></a>
## cmp

`function` · `sqlparser::ast::Assignment::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Assignment) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7808, 51], "end": [7808, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62c8baf6a16ba98545f02ef8"></a>
## deserialize

`function` · `sqlparser::ast::Assignment::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7809, 49], "end": [7809, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b27459132d15ae65dcde454d"></a>
## eq

`function` · `sqlparser::ast::Assignment::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Assignment) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7808, 24], "end": [7808, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17055577263b00a5a5ca8dca"></a>
## fmt

`function` · `sqlparser::ast::Assignment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7808, 10], "end": [7808, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a66d4b695036f4cb973e1ef"></a>
## fmt

`function` · `sqlparser::ast::Assignment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7818, 1], "end": [7822, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8420ca655bcc5535fdf92cdd"></a>
## hash

`function` · `sqlparser::ast::Assignment::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7808, 56], "end": [7808, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5569b933a1e46010334b42e0"></a>
## partial_cmp

`function` · `sqlparser::ast::Assignment::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Assignment) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7808, 35], "end": [7808, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3b2997750200dc926dddadf"></a>
## serialize

`function` · `sqlparser::ast::Assignment::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7809, 38], "end": [7809, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-231e28b782697f0d9e45aa46"></a>
## span

`function` · `sqlparser::ast::Assignment::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "super::Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1433, 1], "end": [1439, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1434`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c95595def7eea0e2fa25edf2"></a>
## target

`struct_field` · `sqlparser::ast::Assignment::target` · sqlparser 0.62.0

```rust
target: AssignmentTarget
```

Source: `src/ast/mod.rs:7813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The left-hand side of the assignment.

<a id="op-db3ed5a6e8951112b72adfe6"></a>
## value

`struct_field` · `sqlparser::ast::Assignment::value` · sqlparser 0.62.0

```rust
value: Expr
```

Source: `src/ast/mod.rs:7815`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression assigned to the target.

<a id="op-6c12638bfdb1ba39e54abb5b"></a>
## visit

`function` · `sqlparser::ast::Assignment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7810, 40], "end": [7810, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7810`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d360d919f457d624827fc767"></a>
## visit

`function` · `sqlparser::ast::Assignment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Assignment", "path": "Assignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7810, 47], "end": [7810, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7810`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
