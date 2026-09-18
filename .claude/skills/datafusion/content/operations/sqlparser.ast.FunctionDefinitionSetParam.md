# `sqlparser::ast::FunctionDefinitionSetParam`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionDefinitionSetParam.json).

<a id="op-ae2a3f6431de8e55bf356a32"></a>
## FunctionDefinitionSetParam

`struct` · `sqlparser::ast::FunctionDefinitionSetParam` · sqlparser 0.62.0

```rust
struct FunctionDefinitionSetParam
```

Source: `src/ast/mod.rs:10016`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A SET configuration_parameter clause in a CREATE FUNCTION statement.

[PostgreSQL](https://www.postgresql.org/docs/current/sql-createfunction.html)

<a id="op-48162a1248a5859337261b99"></a>
## clone

`function` · `sqlparser::ast::FunctionDefinitionSetParam::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionDefinitionSetParam
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10013, 17], "end": [10013, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10013`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a643f95c024f7d74273f70f"></a>
## cmp

`function` · `sqlparser::ast::FunctionDefinitionSetParam::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionDefinitionSetParam) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10013, 51], "end": [10013, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10013`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63c39dba5485a688bda3a4ad"></a>
## deserialize

`function` · `sqlparser::ast::FunctionDefinitionSetParam::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10014, 49], "end": [10014, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d792886cdc60ef500aa8001"></a>
## eq

`function` · `sqlparser::ast::FunctionDefinitionSetParam::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionDefinitionSetParam) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10013, 24], "end": [10013, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10013`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0556efd1efdd98637a381dc8"></a>
## fmt

`function` · `sqlparser::ast::FunctionDefinitionSetParam::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10013, 10], "end": [10013, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10013`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48a6161bd05341d8d94fe701"></a>
## fmt

`function` · `sqlparser::ast::FunctionDefinitionSetParam::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10023, 1], "end": [10034, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57de3fd8d97107df1a7733ce"></a>
## hash

`function` · `sqlparser::ast::FunctionDefinitionSetParam::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10013, 56], "end": [10013, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10013`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f653ee5b7a98a076d0881e2c"></a>
## name

`struct_field` · `sqlparser::ast::FunctionDefinitionSetParam::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:10018`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the configuration parameter.

<a id="op-e889f4d811c62af97c32b81d"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionDefinitionSetParam::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionDefinitionSetParam) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10013, 35], "end": [10013, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10013`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c0b37af91644a3f3fd3df10"></a>
## serialize

`function` · `sqlparser::ast::FunctionDefinitionSetParam::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10014, 38], "end": [10014, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f61b3c07a5f11b0df5ea2c6e"></a>
## value

`struct_field` · `sqlparser::ast::FunctionDefinitionSetParam::value` · sqlparser 0.62.0

```rust
value: FunctionSetValue
```

Source: `src/ast/mod.rs:10020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The value to set for the parameter.

<a id="op-b77c83dc46e801c48b14637e"></a>
## visit

`function` · `sqlparser::ast::FunctionDefinitionSetParam::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10015, 47], "end": [10015, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10015`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4a2518852e5da6628a13e0e"></a>
## visit

`function` · `sqlparser::ast::FunctionDefinitionSetParam::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDefinitionSetParam", "path": "FunctionDefinitionSetParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10015, 40], "end": [10015, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10015`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
