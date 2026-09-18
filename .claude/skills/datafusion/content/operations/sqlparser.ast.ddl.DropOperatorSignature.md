# `sqlparser::ast::ddl::DropOperatorSignature`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DropOperatorSignature.json).

<a id="op-8b9caecbaf786c13f64e8cfd"></a>
## DropOperatorSignature

`struct` · `sqlparser::ast::ddl::DropOperatorSignature` · sqlparser 0.62.0

```rust
struct DropOperatorSignature
```

Source: `src/ast/ddl.rs:4985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator signature for a `DROP OPERATOR` statement

<a id="op-53717eb75d7eb64fbda09655"></a>
## clone

`function` · `sqlparser::ast::ddl::DropOperatorSignature::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropOperatorSignature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4982, 17], "end": [4982, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19e4297535cb4717ca33f8b6"></a>
## cmp

`function` · `sqlparser::ast::ddl::DropOperatorSignature::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropOperatorSignature) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4982, 51], "end": [4982, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc4f9d472ec4bd6ee27c5db7"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DropOperatorSignature::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4983, 49], "end": [4983, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4983`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35ed5046ad08908de5087ea4"></a>
## eq

`function` · `sqlparser::ast::ddl::DropOperatorSignature::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropOperatorSignature) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4982, 24], "end": [4982, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48cbffc62eb2b83a81ece505"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropOperatorSignature::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4982, 10], "end": [4982, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db73bee05fccbc4db0d495c1"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropOperatorSignature::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4994, 1], "end": [5004, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4995`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b87890c272c91b946dd1225"></a>
## hash

`function` · `sqlparser::ast::ddl::DropOperatorSignature::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4982, 56], "end": [4982, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1a46aee45abb401bedae526"></a>
## left_type

`struct_field` · `sqlparser::ast::ddl::DropOperatorSignature::left_type` · sqlparser 0.62.0

```rust
left_type: Option<ast::DataType>
```

Source: `src/ast/ddl.rs:4989`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left operand type

<a id="op-08461a716ce1c57a89f7cfde"></a>
## name

`struct_field` · `sqlparser::ast::ddl::DropOperatorSignature::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator name

<a id="op-2e522dab1b4e2cf0fda37fb3"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DropOperatorSignature::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropOperatorSignature) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4982, 35], "end": [4982, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49af91e8b6ef287f52989173"></a>
## right_type

`struct_field` · `sqlparser::ast::ddl::DropOperatorSignature::right_type` · sqlparser 0.62.0

```rust
right_type: ast::DataType
```

Source: `src/ast/ddl.rs:4991`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Right operand type

<a id="op-a5dc89573ee18072bd754752"></a>
## serialize

`function` · `sqlparser::ast::ddl::DropOperatorSignature::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4983, 38], "end": [4983, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4983`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5c29aea59deeab14081a037"></a>
## visit

`function` · `sqlparser::ast::ddl::DropOperatorSignature::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4984, 47], "end": [4984, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4984`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0a0382e133c86c83217a9d0"></a>
## visit

`function` · `sqlparser::ast::ddl::DropOperatorSignature::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorSignature", "path": "DropOperatorSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4984, 40], "end": [4984, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4984`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
