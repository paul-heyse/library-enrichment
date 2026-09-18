# `sqlparser::ast::PrintStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.PrintStatement.json).

<a id="op-a70cbe72c419809d42bbd2c5"></a>
## PrintStatement

`struct` · `sqlparser::ast::PrintStatement` · sqlparser 0.62.0

```rust
struct PrintStatement
```

Source: `src/ast/mod.rs:11287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PRINT` statement for producing debug/output messages.

<a id="op-b7852553fa377fe8b058519e"></a>
## clone

`function` · `sqlparser::ast::PrintStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> PrintStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11283, 17], "end": [11283, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e9c0c618013d7948acb54a1"></a>
## cmp

`function` · `sqlparser::ast::PrintStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &PrintStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11283, 51], "end": [11283, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b78bd47fd22fa13ca5134789"></a>
## deserialize

`function` · `sqlparser::ast::PrintStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11284, 49], "end": [11284, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acf9b7facc4dd5accd65551d"></a>
## eq

`function` · `sqlparser::ast::PrintStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &PrintStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11283, 24], "end": [11283, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2273a85655408a343e030618"></a>
## fmt

`function` · `sqlparser::ast::PrintStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11292, 1], "end": [11296, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-569bdbd255ca5421158938f4"></a>
## fmt

`function` · `sqlparser::ast::PrintStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11283, 10], "end": [11283, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a8ea99e6529b08aec7343cd"></a>
## hash

`function` · `sqlparser::ast::PrintStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11283, 56], "end": [11283, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9038562d7af128f1b5f24a01"></a>
## message

`struct_field` · `sqlparser::ast::PrintStatement::message` · sqlparser 0.62.0

```rust
message: Box<Expr>
```

Source: `src/ast/mod.rs:11289`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression producing the message to print.

<a id="op-072ca2237b1e56ccbb3404d5"></a>
## partial_cmp

`function` · `sqlparser::ast::PrintStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &PrintStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11283, 35], "end": [11283, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9e6b8ad280790f97737dfcf"></a>
## serialize

`function` · `sqlparser::ast::PrintStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11284, 38], "end": [11284, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e93530b3ed50170c9daa87d"></a>
## visit

`function` · `sqlparser::ast::PrintStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11285, 40], "end": [11285, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11285`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1cd99651a42e735334ed230"></a>
## visit

`function` · `sqlparser::ast::PrintStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11285, 47], "end": [11285, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11285`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
