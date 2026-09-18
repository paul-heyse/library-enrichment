# `sqlparser::ast::DenyStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DenyStatement.json).

<a id="op-7cf6dfc757ff581b0d32da85"></a>
## DenyStatement

`struct` · `sqlparser::ast::DenyStatement` · sqlparser 0.62.0

```rust
struct DenyStatement
```

Source: `src/ast/mod.rs:7777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `DENY` statement

[MsSql](https://learn.microsoft.com/en-us/sql/t-sql/statements/deny-transact-sql)

<a id="op-fc1086d92b1144550f2c8d78"></a>
## cascade

`struct_field` · `sqlparser::ast::DenyStatement::cascade` · sqlparser 0.62.0

```rust
cascade: Option<CascadeOption>
```

Source: `src/ast/mod.rs:7787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional cascade option controlling dependent objects.

<a id="op-c202ff36c805284c213a85df"></a>
## clone

`function` · `sqlparser::ast::DenyStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DenyStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7774, 17], "end": [7774, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3ea440039243cf1056c8d25"></a>
## cmp

`function` · `sqlparser::ast::DenyStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DenyStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7774, 51], "end": [7774, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4c9dd8319185d51b5df44c7"></a>
## deserialize

`function` · `sqlparser::ast::DenyStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7775, 49], "end": [7775, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7775`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d43ee8453b0db61f70e270d3"></a>
## eq

`function` · `sqlparser::ast::DenyStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DenyStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7774, 24], "end": [7774, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-349e25dd0375bc0e0809a014"></a>
## fmt

`function` · `sqlparser::ast::DenyStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7774, 10], "end": [7774, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3935538921f11059a99a762"></a>
## fmt

`function` · `sqlparser::ast::DenyStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7790, 1], "end": [7805, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7791`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c170348b7fe2955736a79acb"></a>
## granted_by

`struct_field` · `sqlparser::ast::DenyStatement::granted_by` · sqlparser 0.62.0

```rust
granted_by: Option<Ident>
```

Source: `src/ast/mod.rs:7785`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional identifier of the principal that performed the grant.

<a id="op-7a7c7637fe22738af9ec80b0"></a>
## grantees

`struct_field` · `sqlparser::ast::DenyStatement::grantees` · sqlparser 0.62.0

```rust
grantees: Vec<Grantee>
```

Source: `src/ast/mod.rs:7783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The grantees (users/roles) to whom the denial applies.

<a id="op-842898bc03f3d6c0dd298a49"></a>
## hash

`function` · `sqlparser::ast::DenyStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7774, 56], "end": [7774, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faa6a2f20de811aa453191e1"></a>
## objects

`struct_field` · `sqlparser::ast::DenyStatement::objects` · sqlparser 0.62.0

```rust
objects: GrantObjects
```

Source: `src/ast/mod.rs:7781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The objects the privileges apply to.

<a id="op-37e6928c4ed8d1afa6dbc772"></a>
## partial_cmp

`function` · `sqlparser::ast::DenyStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DenyStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7774, 35], "end": [7774, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-346f5547cd64c32c73aeb5e3"></a>
## privileges

`struct_field` · `sqlparser::ast::DenyStatement::privileges` · sqlparser 0.62.0

```rust
privileges: Privileges
```

Source: `src/ast/mod.rs:7779`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The privileges to deny.

<a id="op-5d255833ca439afdb5e68a1d"></a>
## serialize

`function` · `sqlparser::ast::DenyStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7775, 38], "end": [7775, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7775`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-488d185db357249276d4348f"></a>
## visit

`function` · `sqlparser::ast::DenyStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7776, 40], "end": [7776, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83459b8640831a576a35466a"></a>
## visit

`function` · `sqlparser::ast::DenyStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7776, 47], "end": [7776, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
