# `sqlparser::ast::ddl::DropOperator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DropOperator.json).

<a id="op-e3d599bb19237e6d1726741e"></a>
## DropOperator

`struct` · `sqlparser::ast::ddl::DropOperator` · sqlparser 0.62.0

```rust
struct DropOperator
```

Source: `src/ast/ddl.rs:4972`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP OPERATOR` statement
See <https://www.postgresql.org/docs/current/sql-dropoperator.html>

<a id="op-1c82ad97b6581019a8eb5ebe"></a>
## clone

`function` · `sqlparser::ast::ddl::DropOperator::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropOperator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4969, 17], "end": [4969, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46a9dbc04b91c81f3ebae837"></a>
## cmp

`function` · `sqlparser::ast::ddl::DropOperator::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropOperator) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4969, 51], "end": [4969, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52e2baf982cf34863aa660c5"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DropOperator::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4970, 49], "end": [4970, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4970`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0aa5d63e5c86473084c1829c"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::ddl::DropOperator::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/ddl.rs:4978`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CASCADE or RESTRICT`

<a id="op-c6409a1753a93c659c8f5a84"></a>
## eq

`function` · `sqlparser::ast::ddl::DropOperator::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropOperator) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4969, 24], "end": [4969, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-946a4c0ce502965aa717d078"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4969, 10], "end": [4969, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9460c60f0dd8e168147ec92"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5006, 1], "end": [5018, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5007`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88f41bd9b0ef15d9e99c5972"></a>
## hash

`function` · `sqlparser::ast::ddl::DropOperator::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4969, 56], "end": [4969, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df57a0c7ed5d48dbe56874e8"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::DropOperator::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:4974`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IF EXISTS` clause

<a id="op-dacb2b9234bb16dd3e276401"></a>
## operators

`struct_field` · `sqlparser::ast::ddl::DropOperator::operators` · sqlparser 0.62.0

```rust
operators: Vec<DropOperatorSignature>
```

Source: `src/ast/ddl.rs:4976`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One or more operators to drop with their signatures

<a id="op-9cb8dcf92ce1a70513499654"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DropOperator::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropOperator) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4969, 35], "end": [4969, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8afcbbf86286eedb8524452"></a>
## serialize

`function` · `sqlparser::ast::ddl::DropOperator::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4970, 38], "end": [4970, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4970`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b051b3ad5fbc0d4cf4410741"></a>
## span

`function` · `sqlparser::ast::ddl::DropOperator::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5020, 1], "end": [5024, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:5021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25ed76fa61c176b0352f519a"></a>
## visit

`function` · `sqlparser::ast::ddl::DropOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4971, 40], "end": [4971, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4971`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6620df4a64ebc008c4bb591d"></a>
## visit

`function` · `sqlparser::ast::ddl::DropOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4971, 47], "end": [4971, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4971`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
