# `sqlparser::ast::query::XmlPassingArgument`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.XmlPassingArgument.json).

<a id="op-4b23a63d2c953acc3b3d60bf"></a>
## XmlPassingArgument

`struct` · `sqlparser::ast::query::XmlPassingArgument` · sqlparser 0.62.0

```rust
struct XmlPassingArgument
```

Source: `src/ast/query.rs:4270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Argument passed in the XMLTABLE PASSING clause
Argument passed in the `XMLTABLE PASSING` clause.

<a id="op-0e31419f301fda2ccd25e002"></a>
## alias

`struct_field` · `sqlparser::ast::query::XmlPassingArgument::alias` · sqlparser 0.62.0

```rust
alias: Option<Ident>
```

Source: `src/ast/query.rs:4274`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the argument.

<a id="op-be2441653a8ac16895ecc2d3"></a>
## by_value

`struct_field` · `sqlparser::ast::query::XmlPassingArgument::by_value` · sqlparser 0.62.0

```rust
by_value: bool
```

Source: `src/ast/query.rs:4276`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` if `BY VALUE` is specified for the argument.

<a id="op-acc31eb3b05672acc857a4f2"></a>
## clone

`function` · `sqlparser::ast::query::XmlPassingArgument::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> XmlPassingArgument
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4266, 17], "end": [4266, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fc25fce41d6de5a266fa9bd"></a>
## cmp

`function` · `sqlparser::ast::query::XmlPassingArgument::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &XmlPassingArgument) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4266, 51], "end": [4266, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b68e05fc62a4688ba8d3939"></a>
## deserialize

`function` · `sqlparser::ast::query::XmlPassingArgument::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4268, 49], "end": [4268, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b29198572ffb21c0f085ad28"></a>
## eq

`function` · `sqlparser::ast::query::XmlPassingArgument::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &XmlPassingArgument) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4266, 24], "end": [4266, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46a845faa6088849323a2d0b"></a>
## expr

`struct_field` · `sqlparser::ast::query::XmlPassingArgument::expr` · sqlparser 0.62.0

```rust
expr: Expr
```

Source: `src/ast/query.rs:4272`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression to pass to the XML table.

<a id="op-dbaf4570227e04ca588d45e0"></a>
## fmt

`function` · `sqlparser::ast::query::XmlPassingArgument::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4266, 10], "end": [4266, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e50acb8c183834d2463128df"></a>
## fmt

`function` · `sqlparser::ast::query::XmlPassingArgument::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4279, 1], "end": [4290, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:4280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87ed372da95f6f43376a302a"></a>
## hash

`function` · `sqlparser::ast::query::XmlPassingArgument::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4266, 56], "end": [4266, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cacc9e48189f88d928611053"></a>
## partial_cmp

`function` · `sqlparser::ast::query::XmlPassingArgument::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &XmlPassingArgument) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4266, 35], "end": [4266, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43dec1675c8a264e412c8773"></a>
## serialize

`function` · `sqlparser::ast::query::XmlPassingArgument::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4268, 38], "end": [4268, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8ee40fb2d56ab172373b149"></a>
## visit

`function` · `sqlparser::ast::query::XmlPassingArgument::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4267, 47], "end": [4267, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1d21c59421da9848e74e3c5"></a>
## visit

`function` · `sqlparser::ast::query::XmlPassingArgument::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingArgument", "path": "XmlPassingArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4267, 40], "end": [4267, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
