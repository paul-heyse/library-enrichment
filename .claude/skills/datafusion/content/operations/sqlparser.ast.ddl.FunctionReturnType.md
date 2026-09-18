# `sqlparser::ast::ddl::FunctionReturnType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.FunctionReturnType.json).

<a id="op-dcb8dd0b714c65f596eb168f"></a>
## FunctionReturnType

`enum` · `sqlparser::ast::ddl::FunctionReturnType` · sqlparser 0.62.0

```rust
enum FunctionReturnType
```

Source: `src/ast/ddl.rs:3547`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The return type of a `CREATE FUNCTION` statement.

<a id="op-d84727d56315be0075081d09"></a>
## DataType

`variant` · `sqlparser::ast::ddl::FunctionReturnType::DataType` · sqlparser 0.62.0

```rust
DataType
```

Source: `src/ast/ddl.rs:3549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RETURNS <type>`

<a id="op-160bbfae116629c4baf56a0b"></a>
## SetOf

`variant` · `sqlparser::ast::ddl::FunctionReturnType::SetOf` · sqlparser 0.62.0

```rust
SetOf
```

Source: `src/ast/ddl.rs:3553`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RETURNS SETOF <type>`

[PostgreSQL](https://www.postgresql.org/docs/current/sql-createfunction.html)

<a id="op-2f159ad6a838ba173c92ea09"></a>
## clone

`function` · `sqlparser::ast::ddl::FunctionReturnType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionReturnType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3544, 17], "end": [3544, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-995a11150a0e3d6b6ada97a8"></a>
## cmp

`function` · `sqlparser::ast::ddl::FunctionReturnType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionReturnType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3544, 51], "end": [3544, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-281c906659369b1897af5e08"></a>
## deserialize

`function` · `sqlparser::ast::ddl::FunctionReturnType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3545, 49], "end": [3545, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3545`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3d8dedf353cf51c214fe220"></a>
## eq

`function` · `sqlparser::ast::ddl::FunctionReturnType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionReturnType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3544, 24], "end": [3544, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ba065626b471d0de80f0c73"></a>
## fmt

`function` · `sqlparser::ast::ddl::FunctionReturnType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3544, 10], "end": [3544, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3403bbce08c334eb871d610"></a>
## fmt

`function` · `sqlparser::ast::ddl::FunctionReturnType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3556, 1], "end": [3563, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3557`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85a95525b4c260fafaf31529"></a>
## hash

`function` · `sqlparser::ast::ddl::FunctionReturnType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3544, 56], "end": [3544, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47fcd136e72f1455c38016cb"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::FunctionReturnType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionReturnType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3544, 35], "end": [3544, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8785ee8de3a0c042f4c96e37"></a>
## serialize

`function` · `sqlparser::ast::ddl::FunctionReturnType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3545, 38], "end": [3545, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3545`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e10db5a72fe5728224f3073"></a>
## visit

`function` · `sqlparser::ast::ddl::FunctionReturnType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3546, 40], "end": [3546, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3546`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78b138afb3920eeff077872d"></a>
## visit

`function` · `sqlparser::ast::ddl::FunctionReturnType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::FunctionReturnType", "path": "FunctionReturnType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3546, 47], "end": [3546, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3546`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
