# `sqlparser::ast::dcl::SecondaryRoles`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.SecondaryRoles.json).

<a id="op-3e8ae496d041c5b742554de6"></a>
## SecondaryRoles

`enum` · `sqlparser::ast::dcl::SecondaryRoles` · sqlparser 0.62.0

```rust
enum SecondaryRoles
```

Source: `src/ast/dcl.rs:287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `SECONDARY ROLES` USE variant
See: <https://docs.snowflake.com/en/sql-reference/sql/use-secondary-roles>

<a id="op-b11b9aa9f7aa9690a0a46e3f"></a>
## All

`variant` · `sqlparser::ast::dcl::SecondaryRoles::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/dcl.rs:289`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use all secondary roles.

<a id="op-b7069947d5c979e6001ba647"></a>
## List

`variant` · `sqlparser::ast::dcl::SecondaryRoles::List` · sqlparser 0.62.0

```rust
List
```

Source: `src/ast/dcl.rs:293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Explicit list of secondary roles.

<a id="op-f698b1c05bb838553ba79534"></a>
## None

`variant` · `sqlparser::ast::dcl::SecondaryRoles::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/dcl.rs:291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use no secondary roles.

<a id="op-6ec2787cd5c6e01d159f8114"></a>
## clone

`function` · `sqlparser::ast::dcl::SecondaryRoles::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SecondaryRoles
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 17], "end": [284, 22], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dcl.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eff9ff0ce0b36f633014b3c3"></a>
## cmp

`function` · `sqlparser::ast::dcl::SecondaryRoles::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SecondaryRoles) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 51], "end": [284, 54], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dcl.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7d648dc9fe467b9c95fb071"></a>
## deserialize

`function` · `sqlparser::ast::dcl::SecondaryRoles::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 49], "end": [285, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dcl.rs:285`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-996d0581804e7232b6e79997"></a>
## eq

`function` · `sqlparser::ast::dcl::SecondaryRoles::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SecondaryRoles) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 24], "end": [284, 33], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dcl.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bb3f34ee34ec4bb0780a3a6"></a>
## fmt

`function` · `sqlparser::ast::dcl::SecondaryRoles::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 10], "end": [284, 15], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dcl.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbae47c6bfa9894d0da6becf"></a>
## fmt

`function` · `sqlparser::ast::dcl::SecondaryRoles::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 1], "end": [304, 2], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dcl.rs:297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1967790ff9ebf3733690d3e6"></a>
## hash

`function` · `sqlparser::ast::dcl::SecondaryRoles::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 56], "end": [284, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dcl.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-333bed637173a6c17915e294"></a>
## partial_cmp

`function` · `sqlparser::ast::dcl::SecondaryRoles::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SecondaryRoles) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 35], "end": [284, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dcl.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-791bc15ddb0da16a288cc6ba"></a>
## serialize

`function` · `sqlparser::ast::dcl::SecondaryRoles::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 38], "end": [285, 47], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dcl.rs:285`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14d6ce4e00605bd78dfde83a"></a>
## visit

`function` · `sqlparser::ast::dcl::SecondaryRoles::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 40], "end": [286, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dcl.rs:286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d9988dfbbd05fb8927b2a68"></a>
## visit

`function` · `sqlparser::ast::dcl::SecondaryRoles::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::SecondaryRoles", "path": "SecondaryRoles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 47], "end": [286, 55], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dcl.rs:286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
