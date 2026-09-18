# `sqlparser::ast::AlterUserAddRoleDelegation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AlterUserAddRoleDelegation.json).

<a id="op-4121c2e9fd08d9aa4223c7ff"></a>
## AlterUserAddRoleDelegation

`struct` · `sqlparser::ast::AlterUserAddRoleDelegation` · sqlparser 0.62.0

```rust
struct AlterUserAddRoleDelegation
```

Source: `src/ast/mod.rs:11573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER USER [ IF EXISTS ] [ <name> ] ADD DELEGATED AUTHORIZATION OF ROLE <role_name> TO SECURITY INTEGRATION <integration_name>
```

<a id="op-30103aa383c48a4891d67667"></a>
## clone

`function` · `sqlparser::ast::AlterUserAddRoleDelegation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterUserAddRoleDelegation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddRoleDelegation", "path": "AlterUserAddRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11570, 17], "end": [11570, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11570`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d24618d3cb538fd4ca26abd6"></a>
## cmp

`function` · `sqlparser::ast::AlterUserAddRoleDelegation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterUserAddRoleDelegation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddRoleDelegation", "path": "AlterUserAddRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11570, 51], "end": [11570, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11570`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84f78597ca31eed4dbbaad15"></a>
## deserialize

`function` · `sqlparser::ast::AlterUserAddRoleDelegation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddRoleDelegation", "path": "AlterUserAddRoleDelegation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11571, 49], "end": [11571, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c385c62bd0b75ac71c5a3b00"></a>
## eq

`function` · `sqlparser::ast::AlterUserAddRoleDelegation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterUserAddRoleDelegation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddRoleDelegation", "path": "AlterUserAddRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11570, 24], "end": [11570, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11570`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c25ea3de15c641adc54b5a4"></a>
## fmt

`function` · `sqlparser::ast::AlterUserAddRoleDelegation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddRoleDelegation", "path": "AlterUserAddRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11570, 10], "end": [11570, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11570`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20fe8f949732509ac0976070"></a>
## hash

`function` · `sqlparser::ast::AlterUserAddRoleDelegation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddRoleDelegation", "path": "AlterUserAddRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11570, 56], "end": [11570, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11570`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39308ec5b6688cb81496fb60"></a>
## integration

`struct_field` · `sqlparser::ast::AlterUserAddRoleDelegation::integration` · sqlparser 0.62.0

```rust
integration: Ident
```

Source: `src/ast/mod.rs:11577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Security integration receiving the delegation.

<a id="op-b8526a5da10017a13353cb66"></a>
## partial_cmp

`function` · `sqlparser::ast::AlterUserAddRoleDelegation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterUserAddRoleDelegation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddRoleDelegation", "path": "AlterUserAddRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11570, 35], "end": [11570, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11570`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f866b216105457e8ffe7330"></a>
## role

`struct_field` · `sqlparser::ast::AlterUserAddRoleDelegation::role` · sqlparser 0.62.0

```rust
role: Ident
```

Source: `src/ast/mod.rs:11575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Role name to delegate.

<a id="op-9b207b36dd90c0e8abeb60f8"></a>
## serialize

`function` · `sqlparser::ast::AlterUserAddRoleDelegation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddRoleDelegation", "path": "AlterUserAddRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11571, 38], "end": [11571, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-794ec262a18c5983440d0e7a"></a>
## visit

`function` · `sqlparser::ast::AlterUserAddRoleDelegation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddRoleDelegation", "path": "AlterUserAddRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11572, 47], "end": [11572, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c38140cdf00d12df9d91c4d4"></a>
## visit

`function` · `sqlparser::ast::AlterUserAddRoleDelegation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddRoleDelegation", "path": "AlterUserAddRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11572, 40], "end": [11572, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
