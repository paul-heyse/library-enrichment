# `sqlparser::ast::dcl::SetConfigValue`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.SetConfigValue.json).

<a id="op-97dd91100be8d8e8549b1a43"></a>
## SetConfigValue

`enum` · `sqlparser::ast::dcl::SetConfigValue` · sqlparser 0.62.0

```rust
enum SetConfigValue
```

Source: `src/ast/dcl.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SET config value option:
* SET `configuration_parameter` { TO | = } { `value` | DEFAULT }
* SET `configuration_parameter` FROM CURRENT

<a id="op-5a846de2d8af273cc7538584"></a>
## Default

`variant` · `sqlparser::ast::dcl::SetConfigValue::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/dcl.rs:121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use the default value.

<a id="op-bbf0db2958609054c871468d"></a>
## FromCurrent

`variant` · `sqlparser::ast::dcl::SetConfigValue::FromCurrent` · sqlparser 0.62.0

```rust
FromCurrent
```

Source: `src/ast/dcl.rs:123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use the current value (`FROM CURRENT`).

<a id="op-d3aef001c5ee66fee9185597"></a>
## Value

`variant` · `sqlparser::ast::dcl::SetConfigValue::Value` · sqlparser 0.62.0

```rust
Value
```

Source: `src/ast/dcl.rs:125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set to the provided expression value.

<a id="op-f10c0d78edf4ee2d5fd6f08d"></a>
## clone

`function` · `sqlparser::ast::dcl::SetConfigValue::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetConfigValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SetConfigValue", "path": "SetConfigValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 17], "end": [116, 22], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dcl.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-594c3b68a2da36d0f30fbcf6"></a>
## cmp

`function` · `sqlparser::ast::dcl::SetConfigValue::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetConfigValue) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SetConfigValue", "path": "SetConfigValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 51], "end": [116, 54], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dcl.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df013f3339ad67d70ad11d2e"></a>
## deserialize

`function` · `sqlparser::ast::dcl::SetConfigValue::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SetConfigValue", "path": "SetConfigValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 49], "end": [117, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dcl.rs:117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c97d508879047f14da95101"></a>
## eq

`function` · `sqlparser::ast::dcl::SetConfigValue::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetConfigValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SetConfigValue", "path": "SetConfigValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 24], "end": [116, 33], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dcl.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a36c86550f8e801d52b2496b"></a>
## fmt

`function` · `sqlparser::ast::dcl::SetConfigValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SetConfigValue", "path": "SetConfigValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 10], "end": [116, 15], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dcl.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73730bcca49e2d8464bdd9b1"></a>
## hash

`function` · `sqlparser::ast::dcl::SetConfigValue::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SetConfigValue", "path": "SetConfigValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 56], "end": [116, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dcl.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82e562544f6b2a6e85a56286"></a>
## partial_cmp

`function` · `sqlparser::ast::dcl::SetConfigValue::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetConfigValue) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SetConfigValue", "path": "SetConfigValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 35], "end": [116, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dcl.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63f91b5ed211de288304c819"></a>
## serialize

`function` · `sqlparser::ast::dcl::SetConfigValue::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SetConfigValue", "path": "SetConfigValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 38], "end": [117, 47], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dcl.rs:117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98e3952b896b5e455d06d114"></a>
## visit

`function` · `sqlparser::ast::dcl::SetConfigValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SetConfigValue", "path": "SetConfigValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 40], "end": [118, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dcl.rs:118`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb0e1ac28d289d56d1ee277b"></a>
## visit

`function` · `sqlparser::ast::dcl::SetConfigValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SetConfigValue", "path": "SetConfigValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 47], "end": [118, 55], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dcl.rs:118`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
