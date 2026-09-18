# `sqlparser::ast::CreateServerOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateServerOption.json).

<a id="op-4ec0f56b2b6365ea130c244a"></a>
## CreateServerOption

`struct` · `sqlparser::ast::CreateServerOption` · sqlparser 0.62.0

```rust
struct CreateServerOption
```

Source: `src/ast/mod.rs:9001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A key/value option for `CREATE SERVER`.

<a id="op-4f0ea4f2f0f3e048e23d6964"></a>
## clone

`function` · `sqlparser::ast::CreateServerOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateServerOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8998, 17], "end": [8998, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e7379cc1f37e5b7b0ef9a02"></a>
## cmp

`function` · `sqlparser::ast::CreateServerOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateServerOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8998, 51], "end": [8998, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-733ecf004d0e93b44d80d086"></a>
## deserialize

`function` · `sqlparser::ast::CreateServerOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8999, 49], "end": [8999, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a460820db0874c69c21f2537"></a>
## eq

`function` · `sqlparser::ast::CreateServerOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateServerOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8998, 24], "end": [8998, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-914cd21b4d1449ecf1eb337b"></a>
## fmt

`function` · `sqlparser::ast::CreateServerOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8998, 10], "end": [8998, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc80b53e3ebdccc03ea5d272"></a>
## fmt

`function` · `sqlparser::ast::CreateServerOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9008, 1], "end": [9012, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9009`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ae11fb0789e44539a20e37f"></a>
## hash

`function` · `sqlparser::ast::CreateServerOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8998, 56], "end": [8998, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5415ae63aeb427dfa9c3e132"></a>
## key

`struct_field` · `sqlparser::ast::CreateServerOption::key` · sqlparser 0.62.0

```rust
key: Ident
```

Source: `src/ast/mod.rs:9003`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Option key identifier.

<a id="op-523245fcafc2095e7b137d28"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateServerOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateServerOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8998, 35], "end": [8998, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-197adf2528de8dcb43e097d0"></a>
## serialize

`function` · `sqlparser::ast::CreateServerOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8999, 38], "end": [8999, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2360b3173d1dc50db7dc4a8"></a>
## value

`struct_field` · `sqlparser::ast::CreateServerOption::value` · sqlparser 0.62.0

```rust
value: Ident
```

Source: `src/ast/mod.rs:9005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Option value identifier.

<a id="op-49646e180f718233af480383"></a>
## visit

`function` · `sqlparser::ast::CreateServerOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9000, 40], "end": [9000, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9000`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e130cca3f6dde4dd63ce277b"></a>
## visit

`function` · `sqlparser::ast::CreateServerOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerOption", "path": "CreateServerOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9000, 47], "end": [9000, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9000`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
