# `sqlparser::ast::MinMaxValue`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.MinMaxValue.json).

<a id="op-2ea98eb941e10211af9427a9"></a>
## MinMaxValue

`enum` · `sqlparser::ast::MinMaxValue` · sqlparser 0.62.0

```rust
enum MinMaxValue
```

Source: `src/ast/mod.rs:6680`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Can use to describe options in  create sequence or table column type identity
[ MINVALUE minvalue | NO MINVALUE ] [ MAXVALUE maxvalue | NO MAXVALUE ]

<a id="op-7200a802bc5a36f1cc57bcc9"></a>
## Empty

`variant` · `sqlparser::ast::MinMaxValue::Empty` · sqlparser 0.62.0

```rust
Empty
```

Source: `src/ast/mod.rs:6682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Clause is not specified.

<a id="op-13454864caee5bf122d9138d"></a>
## None

`variant` · `sqlparser::ast::MinMaxValue::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/mod.rs:6684`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

NO MINVALUE / NO MAXVALUE.

<a id="op-05cfb22771c6f6b434dd16be"></a>
## Some

`variant` · `sqlparser::ast::MinMaxValue::Some` · sqlparser 0.62.0

```rust
Some
```

Source: `src/ast/mod.rs:6686`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MINVALUE <expr>` / `MAXVALUE <expr>`.

<a id="op-f640b98c5f4c8e54d118ed36"></a>
## clone

`function` · `sqlparser::ast::MinMaxValue::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MinMaxValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MinMaxValue", "path": "MinMaxValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6677, 17], "end": [6677, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d4585533675edced1582055"></a>
## cmp

`function` · `sqlparser::ast::MinMaxValue::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MinMaxValue) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MinMaxValue", "path": "MinMaxValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6677, 51], "end": [6677, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57c519d83728351f62f16fa7"></a>
## deserialize

`function` · `sqlparser::ast::MinMaxValue::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MinMaxValue", "path": "MinMaxValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6678, 49], "end": [6678, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ef59acd29b6aa3492ba53c5"></a>
## eq

`function` · `sqlparser::ast::MinMaxValue::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MinMaxValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MinMaxValue", "path": "MinMaxValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6677, 24], "end": [6677, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d950fae87bed560b2ba1386"></a>
## fmt

`function` · `sqlparser::ast::MinMaxValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MinMaxValue", "path": "MinMaxValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6677, 10], "end": [6677, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb9a77fbb453657f96cf603f"></a>
## hash

`function` · `sqlparser::ast::MinMaxValue::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MinMaxValue", "path": "MinMaxValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6677, 56], "end": [6677, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c08fff20cf34565fe05424f9"></a>
## partial_cmp

`function` · `sqlparser::ast::MinMaxValue::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MinMaxValue) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MinMaxValue", "path": "MinMaxValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6677, 35], "end": [6677, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-010ce9c648c9fd04c50cdf47"></a>
## serialize

`function` · `sqlparser::ast::MinMaxValue::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MinMaxValue", "path": "MinMaxValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6678, 38], "end": [6678, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e431808ce5c22c95305ec2b4"></a>
## visit

`function` · `sqlparser::ast::MinMaxValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MinMaxValue", "path": "MinMaxValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6679, 47], "end": [6679, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6679`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7c07e2a12d3873685eb3aee"></a>
## visit

`function` · `sqlparser::ast::MinMaxValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MinMaxValue", "path": "MinMaxValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6679, 40], "end": [6679, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6679`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
