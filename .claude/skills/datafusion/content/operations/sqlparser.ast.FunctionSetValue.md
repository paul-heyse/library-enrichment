# `sqlparser::ast::FunctionSetValue`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionSetValue.json).

<a id="op-8b3628e3c578506e8c3dae1b"></a>
## FunctionSetValue

`enum` · `sqlparser::ast::FunctionSetValue` · sqlparser 0.62.0

```rust
enum FunctionSetValue
```

Source: `src/ast/mod.rs:10001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Value for a SET configuration parameter in a CREATE FUNCTION statement.

[PostgreSQL](https://www.postgresql.org/docs/current/sql-createfunction.html)

<a id="op-f3ad3dcb2092113d101bb0d3"></a>
## Default

`variant` · `sqlparser::ast::FunctionSetValue::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/mod.rs:10003`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SET param = DEFAULT / SET param TO DEFAULT

<a id="op-1f71eb77defd8c7ca97efb6c"></a>
## FromCurrent

`variant` · `sqlparser::ast::FunctionSetValue::FromCurrent` · sqlparser 0.62.0

```rust
FromCurrent
```

Source: `src/ast/mod.rs:10007`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SET param FROM CURRENT

<a id="op-d94dccaf1470645b9b75603a"></a>
## Values

`variant` · `sqlparser::ast::FunctionSetValue::Values` · sqlparser 0.62.0

```rust
Values
```

Source: `src/ast/mod.rs:10005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SET param = value1, value2, ...

<a id="op-0f3d1c54d0eb606cc67f0d20"></a>
## clone

`function` · `sqlparser::ast::FunctionSetValue::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionSetValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSetValue", "path": "FunctionSetValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9998, 17], "end": [9998, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01f34d5e781ceac0b5eafd46"></a>
## cmp

`function` · `sqlparser::ast::FunctionSetValue::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionSetValue) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSetValue", "path": "FunctionSetValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9998, 51], "end": [9998, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9dfe7855d006f18bf71aee5"></a>
## deserialize

`function` · `sqlparser::ast::FunctionSetValue::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSetValue", "path": "FunctionSetValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9999, 49], "end": [9999, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c2eb8fd3d15977d63accaca"></a>
## eq

`function` · `sqlparser::ast::FunctionSetValue::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionSetValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSetValue", "path": "FunctionSetValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9998, 24], "end": [9998, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23cec1f912ceb16d2161eb1f"></a>
## fmt

`function` · `sqlparser::ast::FunctionSetValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSetValue", "path": "FunctionSetValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9998, 10], "end": [9998, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71b20291b46e3e90b715e098"></a>
## hash

`function` · `sqlparser::ast::FunctionSetValue::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSetValue", "path": "FunctionSetValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9998, 56], "end": [9998, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5dd47682772736263aeba02"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionSetValue::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionSetValue) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSetValue", "path": "FunctionSetValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9998, 35], "end": [9998, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a728559cbe8def7816d4f269"></a>
## serialize

`function` · `sqlparser::ast::FunctionSetValue::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSetValue", "path": "FunctionSetValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9999, 38], "end": [9999, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13622cbf0f2b3f42076d9799"></a>
## visit

`function` · `sqlparser::ast::FunctionSetValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSetValue", "path": "FunctionSetValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10000, 47], "end": [10000, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10000`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46e6ee4ec7ed427432ab9604"></a>
## visit

`function` · `sqlparser::ast::FunctionSetValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSetValue", "path": "FunctionSetValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10000, 40], "end": [10000, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10000`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
