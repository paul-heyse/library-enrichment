# `sqlparser::ast::ObjectNamePartFunction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ObjectNamePartFunction.json).

<a id="op-b7567806b882dfc2848ddf8c"></a>
## ObjectNamePartFunction

`struct` · `sqlparser::ast::ObjectNamePartFunction` · sqlparser 0.62.0

```rust
struct ObjectNamePartFunction
```

Source: `src/ast/mod.rs:452`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An object name part that consists of a function that dynamically
constructs identifiers.

- [Snowflake](https://docs.snowflake.com/en/sql-reference/identifier-literal)

<a id="op-f8df71d84fb4bbe3fa0f0ec3"></a>
## args

`struct_field` · `sqlparser::ast::ObjectNamePartFunction::args` · sqlparser 0.62.0

```rust
args: Vec<FunctionArg>
```

Source: `src/ast/mod.rs:456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function arguments used to compute the identifier.

<a id="op-362c1d5eb66915b9e4175986"></a>
## clone

`function` · `sqlparser::ast::ObjectNamePartFunction::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ObjectNamePartFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 17], "end": [449, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cce002038731bd4224bbeeb0"></a>
## cmp

`function` · `sqlparser::ast::ObjectNamePartFunction::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ObjectNamePartFunction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 51], "end": [449, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98748fab32092eb87952fb10"></a>
## deserialize

`function` · `sqlparser::ast::ObjectNamePartFunction::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [450, 49], "end": [450, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:450`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3190de3e468a4d2b3ac54b37"></a>
## eq

`function` · `sqlparser::ast::ObjectNamePartFunction::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ObjectNamePartFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 24], "end": [449, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89dd526d040a3466e95cfea2"></a>
## fmt

`function` · `sqlparser::ast::ObjectNamePartFunction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 10], "end": [449, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d70a07b6c8813d860ed43e79"></a>
## fmt

`function` · `sqlparser::ast::ObjectNamePartFunction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 1], "end": [464, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef3313e2bff3533a32bca7fe"></a>
## hash

`function` · `sqlparser::ast::ObjectNamePartFunction::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 56], "end": [449, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d64d8bd2c55db32e6408e6ab"></a>
## name

`struct_field` · `sqlparser::ast::ObjectNamePartFunction::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The function name that produces the object name part.

<a id="op-1e0fc17357b1ae3c2ce9938c"></a>
## partial_cmp

`function` · `sqlparser::ast::ObjectNamePartFunction::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ObjectNamePartFunction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 35], "end": [449, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8573cc091dda1d718a6879f"></a>
## serialize

`function` · `sqlparser::ast::ObjectNamePartFunction::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [450, 38], "end": [450, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:450`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-353a85f394c1ebf60540e6c0"></a>
## visit

`function` · `sqlparser::ast::ObjectNamePartFunction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 47], "end": [451, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:451`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-794084382444b7f59ebf1efc"></a>
## visit

`function` · `sqlparser::ast::ObjectNamePartFunction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectNamePartFunction", "path": "ObjectNamePartFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 40], "end": [451, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:451`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
