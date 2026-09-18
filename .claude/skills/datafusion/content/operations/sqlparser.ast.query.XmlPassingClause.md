# `sqlparser::ast::query::XmlPassingClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.XmlPassingClause.json).

<a id="op-3b84de447d15f18590b0e5f0"></a>
## XmlPassingClause

`struct` · `sqlparser::ast::query::XmlPassingClause` · sqlparser 0.62.0

```rust
struct XmlPassingClause
```

Source: `src/ast/query.rs:4297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The PASSING clause for XMLTABLE
The PASSING clause for `XMLTABLE`.

<a id="op-fefeb5f6842f6f24acfb4984"></a>
## arguments

`struct_field` · `sqlparser::ast::query::XmlPassingClause::arguments` · sqlparser 0.62.0

```rust
arguments: Vec<XmlPassingArgument>
```

Source: `src/ast/query.rs:4299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The list of passed arguments.

<a id="op-10922e766fe193c373aabe9e"></a>
## clone

`function` · `sqlparser::ast::query::XmlPassingClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> XmlPassingClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4293, 17], "end": [4293, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c92cc393e334d48629e1f3a3"></a>
## cmp

`function` · `sqlparser::ast::query::XmlPassingClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &XmlPassingClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4293, 51], "end": [4293, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee15653ca88cfb2ec019b7c1"></a>
## deserialize

`function` · `sqlparser::ast::query::XmlPassingClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4295, 49], "end": [4295, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4295`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb887f567221c4817d809df1"></a>
## eq

`function` · `sqlparser::ast::query::XmlPassingClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &XmlPassingClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4293, 24], "end": [4293, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8714abb1c804692c42a94906"></a>
## fmt

`function` · `sqlparser::ast::query::XmlPassingClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4293, 10], "end": [4293, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ee66a38738732b8a8bbb497"></a>
## fmt

`function` · `sqlparser::ast::query::XmlPassingClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4302, 1], "end": [4309, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:4303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cd45a722f64865a465c7241"></a>
## hash

`function` · `sqlparser::ast::query::XmlPassingClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4293, 56], "end": [4293, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e085b2c80aa0a86617be32f"></a>
## partial_cmp

`function` · `sqlparser::ast::query::XmlPassingClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &XmlPassingClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4293, 35], "end": [4293, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfc3eab52b18e5b6a535464c"></a>
## serialize

`function` · `sqlparser::ast::query::XmlPassingClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4295, 38], "end": [4295, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4295`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ddcc111ca81c282be4e9ffc"></a>
## visit

`function` · `sqlparser::ast::query::XmlPassingClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4294, 47], "end": [4294, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4294`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb7b42bc35918d7672161c99"></a>
## visit

`function` · `sqlparser::ast::query::XmlPassingClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlPassingClause", "path": "XmlPassingClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4294, 40], "end": [4294, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4294`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
