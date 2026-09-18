# `sqlparser::ast::ddl::AlterTypeAddValuePosition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTypeAddValuePosition.json).

<a id="op-41294ca66b6182418d7c9b32"></a>
## AlterTypeAddValuePosition

`enum` · `sqlparser::ast::ddl::AlterTypeAddValuePosition` · sqlparser 0.62.0

```rust
enum AlterTypeAddValuePosition
```

Source: `src/ast/ddl.rs:1111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [AlterTypeAddValue](../operations/sqlparser.ast.ddl.AlterTypeAddValue.md#op-044d206525d82f53918166fe)

<a id="op-e2a7744d5acd1c7e25cae027"></a>
## After

`variant` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::After` · sqlparser 0.62.0

```rust
After
```

Source: `src/ast/ddl.rs:1115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Place the new value after the given neighbor value.

<a id="op-e366b311a281baec928727da"></a>
## Before

`variant` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::Before` · sqlparser 0.62.0

```rust
Before
```

Source: `src/ast/ddl.rs:1113`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Place the new value before the given neighbor value.

<a id="op-a299659800ed4b1f2948e014"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterTypeAddValuePosition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValuePosition", "path": "AlterTypeAddValuePosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1108, 17], "end": [1108, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8caaf1d893f3948d05f493d3"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterTypeAddValuePosition) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValuePosition", "path": "AlterTypeAddValuePosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1108, 51], "end": [1108, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a837f8e8512b15ec090bfd9b"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValuePosition", "path": "AlterTypeAddValuePosition"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1109, 49], "end": [1109, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48382e2a2cb75025f767d048"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterTypeAddValuePosition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValuePosition", "path": "AlterTypeAddValuePosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1108, 24], "end": [1108, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cad35ee8b665196f9b94c844"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValuePosition", "path": "AlterTypeAddValuePosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1108, 10], "end": [1108, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d92017bec6cc7188cfbc5d4c"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValuePosition", "path": "AlterTypeAddValuePosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1108, 56], "end": [1108, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43f9512c832593a03b4483f4"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterTypeAddValuePosition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValuePosition", "path": "AlterTypeAddValuePosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1108, 35], "end": [1108, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33acf49def8fe49951bc86c9"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValuePosition", "path": "AlterTypeAddValuePosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1109, 38], "end": [1109, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2131f09d2751776d7e5cf87b"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValuePosition", "path": "AlterTypeAddValuePosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1110, 47], "end": [1110, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1110`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47a871a38ea730c8f9ced0e4"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTypeAddValuePosition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValuePosition", "path": "AlterTypeAddValuePosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1110, 40], "end": [1110, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1110`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
