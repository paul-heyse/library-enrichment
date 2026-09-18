# `sqlparser::ast::FunctionDesc`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionDesc.json).

<a id="op-885dc3c8a132d2506644632d"></a>
## FunctionDesc

`struct` · `sqlparser::ast::FunctionDesc` · sqlparser 0.62.0

```rust
struct FunctionDesc
```

Source: `src/ast/mod.rs:9854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Generic function description for DROP FUNCTION and CREATE TRIGGER.

<a id="op-4ec955333cf948cb9710266b"></a>
## args

`struct_field` · `sqlparser::ast::FunctionDesc::args` · sqlparser 0.62.0

```rust
args: Option<Vec<OperateFunctionArg>>
```

Source: `src/ast/mod.rs:9858`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of function arguments.

<a id="op-1db047ffbb78873f57e306fc"></a>
## clone

`function` · `sqlparser::ast::FunctionDesc::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionDesc
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9851, 17], "end": [9851, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9851`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23c0e38d1fba19fcb89f0abb"></a>
## cmp

`function` · `sqlparser::ast::FunctionDesc::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionDesc) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9851, 51], "end": [9851, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9851`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9171f0f51b24f33311f9cd6"></a>
## deserialize

`function` · `sqlparser::ast::FunctionDesc::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9852, 49], "end": [9852, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9852`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92412a559deefbf1ffe046ea"></a>
## eq

`function` · `sqlparser::ast::FunctionDesc::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionDesc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9851, 24], "end": [9851, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9851`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68ae3e526a100822ae5d3339"></a>
## fmt

`function` · `sqlparser::ast::FunctionDesc::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9861, 1], "end": [9869, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9862`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1e959c946f069fecbfa699f"></a>
## fmt

`function` · `sqlparser::ast::FunctionDesc::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9851, 10], "end": [9851, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9851`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a6f819bcf64ba7b82fa6e92"></a>
## hash

`function` · `sqlparser::ast::FunctionDesc::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9851, 56], "end": [9851, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9851`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8455eddb5ab2ea18643a0861"></a>
## name

`struct_field` · `sqlparser::ast::FunctionDesc::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:9856`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The function name.

<a id="op-39d9a6550655e5df3e2c21ee"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionDesc::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionDesc) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9851, 35], "end": [9851, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9851`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8309854d9294b79adce5c07"></a>
## serialize

`function` · `sqlparser::ast::FunctionDesc::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9852, 38], "end": [9852, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9852`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-604b1c09798a3aefde714e52"></a>
## visit

`function` · `sqlparser::ast::FunctionDesc::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9853, 40], "end": [9853, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9853`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a63b00024d10ff6102b21a90"></a>
## visit

`function` · `sqlparser::ast::FunctionDesc::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionDesc", "path": "FunctionDesc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9853, 47], "end": [9853, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9853`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
