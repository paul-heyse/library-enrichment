# `sqlparser::ast::VacuumStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.VacuumStatement.json).

<a id="op-574dd0a79feb0717be73bb3f"></a>
## VacuumStatement

`struct` · `sqlparser::ast::VacuumStatement` · sqlparser 0.62.0

```rust
struct VacuumStatement
```

Source: `src/ast/mod.rs:11908`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Re-sorts rows and reclaims space in either a specified table or all tables in the current database

'''sql
VACUUM [ FULL | SORT ONLY | DELETE ONLY | REINDEX | RECLUSTER ] [ \[ table_name \] [ TO threshold PERCENT ] \[ BOOST \] ]
'''
[Redshift](https://docs.aws.amazon.com/redshift/latest/dg/r_VACUUM_command.html)

<a id="op-15a8a5988f6367c21876526a"></a>
## boost

`struct_field` · `sqlparser::ast::VacuumStatement::boost` · sqlparser 0.62.0

```rust
boost: bool
```

Source: `src/ast/mod.rs:11924`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `BOOST` was specified.

<a id="op-30a838bf18b4082ab32e7eed"></a>
## clone

`function` · `sqlparser::ast::VacuumStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> VacuumStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11905, 17], "end": [11905, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6b0c64c6663964d03b2c45a"></a>
## cmp

`function` · `sqlparser::ast::VacuumStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &VacuumStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11905, 51], "end": [11905, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4865ee354a69b30cd46a039"></a>
## delete_only

`struct_field` · `sqlparser::ast::VacuumStatement::delete_only` · sqlparser 0.62.0

```rust
delete_only: bool
```

Source: `src/ast/mod.rs:11914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `DELETE ONLY` was specified.

<a id="op-cc10897a0fcc524879e5baad"></a>
## deserialize

`function` · `sqlparser::ast::VacuumStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11906, 49], "end": [11906, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11906`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19ea9abb7457819542d3a7c5"></a>
## eq

`function` · `sqlparser::ast::VacuumStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &VacuumStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11905, 24], "end": [11905, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3fdfe8e4dcae995f6b84135"></a>
## fmt

`function` · `sqlparser::ast::VacuumStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11927, 1], "end": [11949, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dccb8ebf35cf8c5cea3f6a5d"></a>
## fmt

`function` · `sqlparser::ast::VacuumStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11905, 10], "end": [11905, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-721ebdc60abead608c3a2d94"></a>
## full

`struct_field` · `sqlparser::ast::VacuumStatement::full` · sqlparser 0.62.0

```rust
full: bool
```

Source: `src/ast/mod.rs:11910`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `FULL` was specified.

<a id="op-5b7db22a9c8a8773e738cd26"></a>
## hash

`function` · `sqlparser::ast::VacuumStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11905, 56], "end": [11905, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a7ef082f5ea6bf6ccae57f1"></a>
## partial_cmp

`function` · `sqlparser::ast::VacuumStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &VacuumStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11905, 35], "end": [11905, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f19202413ce328b42ef3217f"></a>
## recluster

`struct_field` · `sqlparser::ast::VacuumStatement::recluster` · sqlparser 0.62.0

```rust
recluster: bool
```

Source: `src/ast/mod.rs:11918`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `RECLUSTER` was specified.

<a id="op-82d69d5f0d03118de50283ff"></a>
## reindex

`struct_field` · `sqlparser::ast::VacuumStatement::reindex` · sqlparser 0.62.0

```rust
reindex: bool
```

Source: `src/ast/mod.rs:11916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `REINDEX` was specified.

<a id="op-10c34019c2fe59fbd3fa0a47"></a>
## serialize

`function` · `sqlparser::ast::VacuumStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11906, 38], "end": [11906, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11906`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5667980b000243ee00ed6f1"></a>
## sort_only

`struct_field` · `sqlparser::ast::VacuumStatement::sort_only` · sqlparser 0.62.0

```rust
sort_only: bool
```

Source: `src/ast/mod.rs:11912`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `SORT ONLY` was specified.

<a id="op-54dfb1847030f846848a2904"></a>
## table_name

`struct_field` · `sqlparser::ast::VacuumStatement::table_name` · sqlparser 0.62.0

```rust
table_name: Option<ObjectName>
```

Source: `src/ast/mod.rs:11920`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional table to run `VACUUM` on.

<a id="op-ccd4ee306c6b15ca71afcd63"></a>
## threshold

`struct_field` · `sqlparser::ast::VacuumStatement::threshold` · sqlparser 0.62.0

```rust
threshold: Option<ValueWithSpan>
```

Source: `src/ast/mod.rs:11922`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional threshold value (percent) for `TO threshold PERCENT`.

<a id="op-4de036aa60f8e5ce6e3f2ad1"></a>
## visit

`function` · `sqlparser::ast::VacuumStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11907, 40], "end": [11907, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11907`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc163e179a94011c613ce362"></a>
## visit

`function` · `sqlparser::ast::VacuumStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11907, 47], "end": [11907, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11907`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
