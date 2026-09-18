# `sqlparser::ast::LambdaFunctionParameter`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.LambdaFunctionParameter.json).

<a id="op-b569cc58c784a759262fb145"></a>
## LambdaFunctionParameter

`struct` · `sqlparser::ast::LambdaFunctionParameter` · sqlparser 0.62.0

```rust
struct LambdaFunctionParameter
```

Source: `src/ast/mod.rs:1507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A parameter to a lambda function, optionally with a data type.

<a id="op-d7188ef3e4427c7a6847491e"></a>
## clone

`function` · `sqlparser::ast::LambdaFunctionParameter::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LambdaFunctionParameter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1504, 17], "end": [1504, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:1504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96573472e09022f9d892ed6a"></a>
## cmp

`function` · `sqlparser::ast::LambdaFunctionParameter::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LambdaFunctionParameter) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1504, 51], "end": [1504, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:1504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fbc3eecd670393e816f3202"></a>
## data_type

`struct_field` · `sqlparser::ast::LambdaFunctionParameter::data_type` · sqlparser 0.62.0

```rust
data_type: Option<DataType>
```

Source: `src/ast/mod.rs:1512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The optional data type of the parameter
[Snowflake Syntax](https://docs.snowflake.com/en/sql-reference/functions/filter#arguments)

<a id="op-523ce1e5752dd052670cc433"></a>
## deserialize

`function` · `sqlparser::ast::LambdaFunctionParameter::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1505, 49], "end": [1505, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:1505`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e44540d55e2e5317f30092fd"></a>
## eq

`function` · `sqlparser::ast::LambdaFunctionParameter::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LambdaFunctionParameter) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1504, 24], "end": [1504, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:1504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35302b49d31281649537923f"></a>
## fmt

`function` · `sqlparser::ast::LambdaFunctionParameter::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1515, 1], "end": [1522, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:1516`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f42cca9d77210fdd03de0892"></a>
## fmt

`function` · `sqlparser::ast::LambdaFunctionParameter::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1504, 10], "end": [1504, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:1504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-736b35135064400e60eaeda5"></a>
## hash

`function` · `sqlparser::ast::LambdaFunctionParameter::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1504, 56], "end": [1504, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:1504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb781839ed66410e5c2b2ce6"></a>
## name

`struct_field` · `sqlparser::ast::LambdaFunctionParameter::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:1509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the parameter

<a id="op-3a5e054faa59c75285ad4848"></a>
## partial_cmp

`function` · `sqlparser::ast::LambdaFunctionParameter::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LambdaFunctionParameter) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1504, 35], "end": [1504, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:1504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ae851419f2a9d9fe4970eac"></a>
## serialize

`function` · `sqlparser::ast::LambdaFunctionParameter::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1505, 38], "end": [1505, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:1505`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-650ff8de9b4bda162d0ae937"></a>
## visit

`function` · `sqlparser::ast::LambdaFunctionParameter::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1506, 47], "end": [1506, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:1506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2acc5e3770e1333243075a3"></a>
## visit

`function` · `sqlparser::ast::LambdaFunctionParameter::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunctionParameter", "path": "LambdaFunctionParameter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1506, 40], "end": [1506, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:1506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
