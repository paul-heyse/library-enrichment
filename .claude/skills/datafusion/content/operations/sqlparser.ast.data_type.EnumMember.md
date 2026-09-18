# `sqlparser::ast::data_type::EnumMember`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.EnumMember.json).

<a id="op-e0400e2688abf98f51f9e2e8"></a>
## EnumMember

`enum` · `sqlparser::ast::data_type::EnumMember` · sqlparser 0.62.0

```rust
enum EnumMember
```

Source: `src/ast/data_type.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A member of an ENUM type.

<a id="op-e7c31f6898750c97e4b6a604"></a>
## Name

`variant` · `sqlparser::ast::data_type::EnumMember::Name` · sqlparser 0.62.0

```rust
Name
```

Source: `src/ast/data_type.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Just a name.

<a id="op-8e20e66a3c725972aad83f6e"></a>
## NamedValue

`variant` · `sqlparser::ast::data_type::EnumMember::NamedValue` · sqlparser 0.62.0

```rust
NamedValue
```

Source: `src/ast/data_type.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse allows to specify an integer value for each enum value.

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/data-types/enum)

<a id="op-f6f0a719562dbbfe9f9ab2a2"></a>
## clone

`function` · `sqlparser::ast::data_type::EnumMember::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> EnumMember
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::EnumMember", "path": "EnumMember"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 22], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf6bcdf34f6b8b99244d3dc0"></a>
## cmp

`function` · `sqlparser::ast::data_type::EnumMember::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &EnumMember) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::EnumMember", "path": "EnumMember"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 51], "end": [32, 54], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b40321d6f9dcd83fb0461a6"></a>
## deserialize

`function` · `sqlparser::ast::data_type::EnumMember::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::EnumMember", "path": "EnumMember"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 49], "end": [33, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad1f0faa099de0384cb16720"></a>
## eq

`function` · `sqlparser::ast::data_type::EnumMember::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &EnumMember) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::EnumMember", "path": "EnumMember"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 24], "end": [32, 33], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fec612724ed869c5e7cb994"></a>
## fmt

`function` · `sqlparser::ast::data_type::EnumMember::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::EnumMember", "path": "EnumMember"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-511cc0f1fd9e56108d5bcfd0"></a>
## hash

`function` · `sqlparser::ast::data_type::EnumMember::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::EnumMember", "path": "EnumMember"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 56], "end": [32, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01b2bbf479edd97ccc857f14"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::EnumMember::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &EnumMember) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::EnumMember", "path": "EnumMember"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 35], "end": [32, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8ad70693f604adc7c40c3a1"></a>
## serialize

`function` · `sqlparser::ast::data_type::EnumMember::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::EnumMember", "path": "EnumMember"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 38], "end": [33, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60f8cb74197a136b06d3daa7"></a>
## visit

`function` · `sqlparser::ast::data_type::EnumMember::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::EnumMember", "path": "EnumMember"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 47], "end": [34, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b95f32fc5f88d8974d1b87ed"></a>
## visit

`function` · `sqlparser::ast::data_type::EnumMember::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::EnumMember", "path": "EnumMember"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 40], "end": [34, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
