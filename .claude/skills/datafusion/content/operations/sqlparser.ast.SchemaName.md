# `sqlparser::ast::SchemaName`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SchemaName.json).

<a id="op-02506ec6008fadf67ba79d12"></a>
## SchemaName

`enum` · `sqlparser::ast::SchemaName` · sqlparser 0.62.0

```rust
enum SchemaName
```

Source: `src/ast/mod.rs:10293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Schema possible naming variants ([1]).

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#schema-definition

<a id="op-1826b3bf5f020565fd621538"></a>
## NamedAuthorization

`variant` · `sqlparser::ast::SchemaName::NamedAuthorization` · sqlparser 0.62.0

```rust
NamedAuthorization
```

Source: `src/ast/mod.rs:10299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Both schema name and authorization identifier specified: `<schema name>  AUTHORIZATION <schema authorization identifier>`.

<a id="op-c08fd34faee8a02ea79b27cb"></a>
## Simple

`variant` · `sqlparser::ast::SchemaName::Simple` · sqlparser 0.62.0

```rust
Simple
```

Source: `src/ast/mod.rs:10295`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only schema name specified: `<schema name>`.

<a id="op-a8532a3c5c853620356ad4bd"></a>
## UnnamedAuthorization

`variant` · `sqlparser::ast::SchemaName::UnnamedAuthorization` · sqlparser 0.62.0

```rust
UnnamedAuthorization
```

Source: `src/ast/mod.rs:10297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only authorization identifier specified: `AUTHORIZATION <schema authorization identifier>`.

<a id="op-e99637fbc7f7461820f8f24d"></a>
## clone

`function` · `sqlparser::ast::SchemaName::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SchemaName
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10290, 17], "end": [10290, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-298410ec86be98069bce106a"></a>
## cmp

`function` · `sqlparser::ast::SchemaName::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SchemaName) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10290, 51], "end": [10290, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10ac2d25171f47cd74e2c1ec"></a>
## deserialize

`function` · `sqlparser::ast::SchemaName::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10291, 49], "end": [10291, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d13d1cc997eb16dfd3a19dc2"></a>
## eq

`function` · `sqlparser::ast::SchemaName::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SchemaName) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10290, 24], "end": [10290, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-876cde0c48e0b13dcbe33d65"></a>
## fmt

`function` · `sqlparser::ast::SchemaName::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10290, 10], "end": [10290, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-970610e426c63f236308bb4c"></a>
## fmt

`function` · `sqlparser::ast::SchemaName::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10302, 1], "end": [10316, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46003f390c4f7e0e0550c712"></a>
## hash

`function` · `sqlparser::ast::SchemaName::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10290, 56], "end": [10290, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cc634c3a5b0bd3022937845"></a>
## partial_cmp

`function` · `sqlparser::ast::SchemaName::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SchemaName) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10290, 35], "end": [10290, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae8cde304bb7a53c1c6d742d"></a>
## serialize

`function` · `sqlparser::ast::SchemaName::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10291, 38], "end": [10291, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99fbd6b838a875b2e263b0a7"></a>
## visit

`function` · `sqlparser::ast::SchemaName::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10292, 47], "end": [10292, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f570c7fdb82f89e7518998c8"></a>
## visit

`function` · `sqlparser::ast::SchemaName::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SchemaName", "path": "SchemaName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10292, 40], "end": [10292, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
