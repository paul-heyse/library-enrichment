# `sqlparser::ast::OperateFunctionArg`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.OperateFunctionArg.json).

<a id="op-0d95ff3f308b2da669e0df11"></a>
## OperateFunctionArg

`struct` · `sqlparser::ast::OperateFunctionArg` · sqlparser 0.62.0

```rust
struct OperateFunctionArg
```

Source: `src/ast/mod.rs:9875`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function argument in CREATE OR DROP FUNCTION.

<a id="op-74bead93b92e72543b9428fe"></a>
## clone

`function` · `sqlparser::ast::OperateFunctionArg::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OperateFunctionArg
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9872, 17], "end": [9872, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9872`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8885b43410282028fff2567b"></a>
## cmp

`function` · `sqlparser::ast::OperateFunctionArg::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OperateFunctionArg) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9872, 51], "end": [9872, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9872`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26b85ce4aa75026febb659e6"></a>
## data_type

`struct_field` · `sqlparser::ast::OperateFunctionArg::data_type` · sqlparser 0.62.0

```rust
data_type: DataType
```

Source: `src/ast/mod.rs:9881`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The data type of the argument.

<a id="op-3965a9dcb597007af2fc651c"></a>
## default_expr

`struct_field` · `sqlparser::ast::OperateFunctionArg::default_expr` · sqlparser 0.62.0

```rust
default_expr: Option<Expr>
```

Source: `src/ast/mod.rs:9883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional default expression for the argument.

<a id="op-4a48caf45d20848d16a913be"></a>
## deserialize

`function` · `sqlparser::ast::OperateFunctionArg::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9873, 49], "end": [9873, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9873`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-821a9ad2ce708ae3706a848a"></a>
## eq

`function` · `sqlparser::ast::OperateFunctionArg::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OperateFunctionArg) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9872, 24], "end": [9872, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9872`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3db9f7c6294b1b3d58cefabf"></a>
## fmt

`function` · `sqlparser::ast::OperateFunctionArg::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9872, 10], "end": [9872, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9872`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f5895640ec71c0a3b452783"></a>
## fmt

`function` · `sqlparser::ast::OperateFunctionArg::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9908, 1], "end": [9922, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9909`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bae22457b18e62ef709d6692"></a>
## hash

`function` · `sqlparser::ast::OperateFunctionArg::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9872, 56], "end": [9872, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9872`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db3dcca412b5401ad83e82c2"></a>
## mode

`struct_field` · `sqlparser::ast::OperateFunctionArg::mode` · sqlparser 0.62.0

```rust
mode: Option<ArgMode>
```

Source: `src/ast/mod.rs:9877`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional argument mode (`IN`, `OUT`, `INOUT`).

<a id="op-f804c1a61672a8161e905603"></a>
## name

`struct_field` · `sqlparser::ast::OperateFunctionArg::name` · sqlparser 0.62.0

```rust
name: Option<Ident>
```

Source: `src/ast/mod.rs:9879`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional argument identifier/name.

<a id="op-03cff7cd6cb93e1f98552c66"></a>
## partial_cmp

`function` · `sqlparser::ast::OperateFunctionArg::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OperateFunctionArg) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9872, 35], "end": [9872, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9872`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62281f59b3e4a7d386aaa321"></a>
## serialize

`function` · `sqlparser::ast::OperateFunctionArg::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9873, 38], "end": [9873, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9873`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6152145a479f2587f2d1a2d9"></a>
## unnamed

`function` · `sqlparser::ast::OperateFunctionArg::unnamed` · sqlparser 0.62.0

```rust
fn unnamed(data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9886, 1], "end": [9906, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:9888`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns an unnamed argument.

<a id="op-0a2e02510a4898e3891ee539"></a>
## visit

`function` · `sqlparser::ast::OperateFunctionArg::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9874, 47], "end": [9874, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf9b7e139d70deb8972a2ab6"></a>
## visit

`function` · `sqlparser::ast::OperateFunctionArg::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9874, 40], "end": [9874, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d54d60ff632d81343fa3b63b"></a>
## with_name

`function` · `sqlparser::ast::OperateFunctionArg::with_name` · sqlparser 0.62.0

```rust
fn with_name(name: &str, data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9886, 1], "end": [9906, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:9898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns an argument with name.
