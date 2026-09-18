# `sqlparser::ast::ddl::CreateOperatorClass`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateOperatorClass.json).

<a id="op-a0d1f80e3d579e67b42b3acd"></a>
## CreateOperatorClass

`struct` · `sqlparser::ast::ddl::CreateOperatorClass` · sqlparser 0.62.0

```rust
struct CreateOperatorClass
```

Source: `src/ast/ddl.rs:4784`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE OPERATOR CLASS statement
See <https://www.postgresql.org/docs/current/sql-createopclass.html>

<a id="op-3be28c79e5a63b9722189473"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateOperatorClass::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateOperatorClass
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4781, 17], "end": [4781, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d53376a2f4d3047761054e4"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateOperatorClass::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateOperatorClass) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4781, 51], "end": [4781, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e58bc413e3c213f0ef6f886"></a>
## default

`struct_field` · `sqlparser::ast::ddl::CreateOperatorClass::default` · sqlparser 0.62.0

```rust
default: bool
```

Source: `src/ast/ddl.rs:4788`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether this is the default operator class for the type

<a id="op-4039753a150abdedbc73b4c2"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateOperatorClass::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4782, 49], "end": [4782, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4782`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b043c20e49dbada9b124b5bb"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateOperatorClass::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateOperatorClass) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4781, 24], "end": [4781, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-462078cb91eef6136f33f609"></a>
## family

`struct_field` · `sqlparser::ast::ddl::CreateOperatorClass::family` · sqlparser 0.62.0

```rust
family: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:4794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional operator family name

<a id="op-1ea6c040a00ff29b88a9d8a7"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateOperatorClass::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4781, 10], "end": [4781, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc274ebead0585281edf909b"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateOperatorClass::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4836, 1], "end": [4848, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4837`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cad63308348424f564aa89b7"></a>
## for_type

`struct_field` · `sqlparser::ast::ddl::CreateOperatorClass::for_type` · sqlparser 0.62.0

```rust
for_type: ast::DataType
```

Source: `src/ast/ddl.rs:4790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The data type

<a id="op-962e3bed0777dd8388050a8b"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateOperatorClass::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4781, 56], "end": [4781, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76cf1550a9a4aadf3cfdc329"></a>
## items

`struct_field` · `sqlparser::ast::ddl::CreateOperatorClass::items` · sqlparser 0.62.0

```rust
items: Vec<OperatorClassItem>
```

Source: `src/ast/ddl.rs:4796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of operator class items (operators, functions, storage)

<a id="op-3d525516cae88a5655e3721a"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateOperatorClass::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4786`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator class name (can be schema-qualified)

<a id="op-82eccc682bfbbead432778c0"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateOperatorClass::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateOperatorClass) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4781, 35], "end": [4781, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-729981a47170158979c3429a"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateOperatorClass::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4782, 38], "end": [4782, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4782`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-187a281de4921c5342fee3f6"></a>
## span

`function` · `sqlparser::ast::ddl::CreateOperatorClass::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "crate::ast::CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2507, 1], "end": [2511, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc2a79fde0a205b8b13453b3"></a>
## using

`struct_field` · `sqlparser::ast::ddl::CreateOperatorClass::using` · sqlparser 0.62.0

```rust
using: ast::Ident
```

Source: `src/ast/ddl.rs:4792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index method (btree, hash, gist, gin, etc.)

<a id="op-ade3d21939a07056a76254d6"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateOperatorClass::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4783, 47], "end": [4783, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cec0e685df879beca512387f"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateOperatorClass::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4783, 40], "end": [4783, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
