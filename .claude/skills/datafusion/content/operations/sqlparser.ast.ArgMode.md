# `sqlparser::ast::ArgMode`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ArgMode.json).

<a id="op-bfbb73fceea13f91de5f0e65"></a>
## ArgMode

`enum` · `sqlparser::ast::ArgMode` · sqlparser 0.62.0

```rust
enum ArgMode
```

Source: `src/ast/mod.rs:9928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The mode of an argument in CREATE FUNCTION.

<a id="op-d4623cde6c5e22c474752a16"></a>
## In

`variant` · `sqlparser::ast::ArgMode::In` · sqlparser 0.62.0

```rust
In
```

Source: `src/ast/mod.rs:9930`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IN` mode.

<a id="op-f8ab52796a53f224aab9e34a"></a>
## InOut

`variant` · `sqlparser::ast::ArgMode::InOut` · sqlparser 0.62.0

```rust
InOut
```

Source: `src/ast/mod.rs:9934`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INOUT` mode.

<a id="op-98a28176e629fb0a09218433"></a>
## Out

`variant` · `sqlparser::ast::ArgMode::Out` · sqlparser 0.62.0

```rust
Out
```

Source: `src/ast/mod.rs:9932`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OUT` mode.

<a id="op-10e4aba6e5ebbc0dca4e89eb"></a>
## Variadic

`variant` · `sqlparser::ast::ArgMode::Variadic` · sqlparser 0.62.0

```rust
Variadic
```

Source: `src/ast/mod.rs:9936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`VARIADIC` mode.

<a id="op-0c06e89172dbf8536139140b"></a>
## clone

`function` · `sqlparser::ast::ArgMode::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ArgMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9925, 17], "end": [9925, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-643bd4ed039cf5871f878140"></a>
## cmp

`function` · `sqlparser::ast::ArgMode::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ArgMode) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9925, 51], "end": [9925, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cfb4af9bf2d3637c9c8e8a4"></a>
## deserialize

`function` · `sqlparser::ast::ArgMode::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9926, 49], "end": [9926, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6be29f0c1028acb437ccda93"></a>
## eq

`function` · `sqlparser::ast::ArgMode::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ArgMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9925, 24], "end": [9925, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3231c82e5707154a76798438"></a>
## fmt

`function` · `sqlparser::ast::ArgMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9925, 10], "end": [9925, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f584d1272e5100f38fad2dc8"></a>
## fmt

`function` · `sqlparser::ast::ArgMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9939, 1], "end": [9948, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9940`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4922b84ae3a3b7919f9a93d7"></a>
## hash

`function` · `sqlparser::ast::ArgMode::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9925, 56], "end": [9925, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64f80e1f873350de4171e22a"></a>
## partial_cmp

`function` · `sqlparser::ast::ArgMode::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ArgMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9925, 35], "end": [9925, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4c1d95badab8f8564f694ee"></a>
## serialize

`function` · `sqlparser::ast::ArgMode::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9926, 38], "end": [9926, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-053515595bed15946185b305"></a>
## visit

`function` · `sqlparser::ast::ArgMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9927, 47], "end": [9927, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9927`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-062dbc98960c28082c0cf40c"></a>
## visit

`function` · `sqlparser::ast::ArgMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ArgMode", "path": "ArgMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9927, 40], "end": [9927, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9927`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
