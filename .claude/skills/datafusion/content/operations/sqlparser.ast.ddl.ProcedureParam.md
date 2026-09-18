# `sqlparser::ast::ddl::ProcedureParam`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ProcedureParam.json).

<a id="op-c167eaf8e4535690a4dafccc"></a>
## ProcedureParam

`struct` · `sqlparser::ast::ddl::ProcedureParam` · sqlparser 0.62.0

```rust
struct ProcedureParam
```

Source: `src/ast/ddl.rs:1502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A parameter of a stored procedure or function declaration.

<a id="op-34d05669e394e43b04ba8a1a"></a>
## clone

`function` · `sqlparser::ast::ddl::ProcedureParam::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ProcedureParam
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 17], "end": [1498, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df57f95f3af50ab0e0ba3d5d"></a>
## cmp

`function` · `sqlparser::ast::ddl::ProcedureParam::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ProcedureParam) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 51], "end": [1498, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73579ee6a5029e20e00c4b80"></a>
## data_type

`struct_field` · `sqlparser::ast::ddl::ProcedureParam::data_type` · sqlparser 0.62.0

```rust
data_type: ast::DataType
```

Source: `src/ast/ddl.rs:1506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parameter data type.

<a id="op-5c7176087be797f58299008a"></a>
## default

`struct_field` · `sqlparser::ast::ddl::ProcedureParam::default` · sqlparser 0.62.0

```rust
default: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:1510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional default expression for the parameter.

<a id="op-b119c24fc24fee1680eff05c"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ProcedureParam::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1499, 49], "end": [1499, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1499`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bb26ac277799e4f4d6a78ba"></a>
## eq

`function` · `sqlparser::ast::ddl::ProcedureParam::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ProcedureParam) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 24], "end": [1498, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c5a4e5a1918bb3b5e4f57bf"></a>
## fmt

`function` · `sqlparser::ast::ddl::ProcedureParam::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 10], "end": [1498, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a14fcdfab0bfa475f0d90ced"></a>
## fmt

`function` · `sqlparser::ast::ddl::ProcedureParam::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1513, 1], "end": [1527, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1514`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bbd73394e8a0b55ae15cf78"></a>
## hash

`function` · `sqlparser::ast::ddl::ProcedureParam::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 56], "end": [1498, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4edf994109218976e4ab3c7"></a>
## mode

`struct_field` · `sqlparser::ast::ddl::ProcedureParam::mode` · sqlparser 0.62.0

```rust
mode: Option<ast::ArgMode>
```

Source: `src/ast/ddl.rs:1508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional mode (`IN`, `OUT`, `INOUT`, etc.).

<a id="op-d661715d38cfc089d3aa09ce"></a>
## name

`struct_field` · `sqlparser::ast::ddl::ProcedureParam::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:1504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parameter name.

<a id="op-c69d25e0d82e63488d725b7a"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ProcedureParam::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ProcedureParam) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1498, 35], "end": [1498, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5a5637548bc960e7c7961ad"></a>
## serialize

`function` · `sqlparser::ast::ddl::ProcedureParam::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1499, 38], "end": [1499, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1499`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f63e3e1712a935247b915ca"></a>
## visit

`function` · `sqlparser::ast::ddl::ProcedureParam::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1500, 40], "end": [1500, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1500`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a4eabac2cb2f9ebb21e68a6"></a>
## visit

`function` · `sqlparser::ast::ddl::ProcedureParam::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ProcedureParam", "path": "ProcedureParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1500, 47], "end": [1500, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1500`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
