# `sqlparser::ast::ddl::AlterCollation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterCollation.json).

<a id="op-5fa0f5e3e2306c881b7c593e"></a>
## AlterCollation

`struct` · `sqlparser::ast::ddl::AlterCollation` · sqlparser 0.62.0

```rust
struct AlterCollation
```

Source: `src/ast/ddl.rs:4574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ALTER COLLATION statement.
Note: this is a PostgreSQL-specific statement.

<a id="op-af7c1b5942a9cca0f3721e08"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterCollation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterCollation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4571, 17], "end": [4571, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10aef8338cf388a7197a6109"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterCollation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterCollation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4571, 51], "end": [4571, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3af56093f00fd2b70c70c792"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterCollation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4572, 49], "end": [4572, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-867fcfb887e206eecbe331a9"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterCollation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterCollation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4571, 24], "end": [4571, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c08b36bccbd6c9acc719191f"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterCollation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4571, 10], "end": [4571, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f19969f05f88bf48891ae34f"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterCollation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4631, 1], "end": [4635, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4632`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d0529c504ef728776381307"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterCollation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4571, 56], "end": [4571, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6642754229de8da428530eaa"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterCollation::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the collation being altered.

<a id="op-eeb14a3d4ffff55386b26953"></a>
## operation

`struct_field` · `sqlparser::ast::ddl::AlterCollation::operation` · sqlparser 0.62.0

```rust
operation: AlterCollationOperation
```

Source: `src/ast/ddl.rs:4578`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The operation to perform on the collation.

<a id="op-2f4824e3bac9951488b8af0e"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterCollation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterCollation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4571, 35], "end": [4571, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5286149b17bee0fbfd66b85d"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterCollation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4572, 38], "end": [4572, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5ac1518e364cda69ab4aa52"></a>
## span

`function` · `sqlparser::ast::ddl::AlterCollation::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4637, 1], "end": [4641, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:4638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c2ff462c34a303dc4e4eb94"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterCollation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4573, 40], "end": [4573, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7542467deb61deae9703107"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterCollation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4573, 47], "end": [4573, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
