# `sqlparser::ast::AlterUserRemoveRoleDelegation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AlterUserRemoveRoleDelegation.json).

<a id="op-33bb75e382dd684c50b50747"></a>
## AlterUserRemoveRoleDelegation

`struct` · `sqlparser::ast::AlterUserRemoveRoleDelegation` · sqlparser 0.62.0

```rust
struct AlterUserRemoveRoleDelegation
```

Source: `src/ast/mod.rs:11586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER USER [ IF EXISTS ] [ <name> ] REMOVE DELEGATED { AUTHORIZATION OF ROLE <role_name> | AUTHORIZATIONS } FROM SECURITY INTEGRATION <integration_name>
```

<a id="op-c351f66bdddc7ab9136ae77e"></a>
## clone

`function` · `sqlparser::ast::AlterUserRemoveRoleDelegation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterUserRemoveRoleDelegation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserRemoveRoleDelegation", "path": "AlterUserRemoveRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11583, 17], "end": [11583, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-460b0f89f9fb74915aac9568"></a>
## cmp

`function` · `sqlparser::ast::AlterUserRemoveRoleDelegation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterUserRemoveRoleDelegation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserRemoveRoleDelegation", "path": "AlterUserRemoveRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11583, 51], "end": [11583, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e024726e32233788b5864f49"></a>
## deserialize

`function` · `sqlparser::ast::AlterUserRemoveRoleDelegation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserRemoveRoleDelegation", "path": "AlterUserRemoveRoleDelegation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11584, 49], "end": [11584, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-804974e200e416efc3174659"></a>
## eq

`function` · `sqlparser::ast::AlterUserRemoveRoleDelegation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterUserRemoveRoleDelegation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserRemoveRoleDelegation", "path": "AlterUserRemoveRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11583, 24], "end": [11583, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09ea2e92cd419eb556af93fd"></a>
## fmt

`function` · `sqlparser::ast::AlterUserRemoveRoleDelegation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserRemoveRoleDelegation", "path": "AlterUserRemoveRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11583, 10], "end": [11583, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d11690736a26055c99937ccd"></a>
## hash

`function` · `sqlparser::ast::AlterUserRemoveRoleDelegation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserRemoveRoleDelegation", "path": "AlterUserRemoveRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11583, 56], "end": [11583, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9d20abccb43fadf614deba9"></a>
## integration

`struct_field` · `sqlparser::ast::AlterUserRemoveRoleDelegation::integration` · sqlparser 0.62.0

```rust
integration: Ident
```

Source: `src/ast/mod.rs:11590`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Security integration from which to remove delegation.

<a id="op-7fd751775dc7906eb5116fb7"></a>
## partial_cmp

`function` · `sqlparser::ast::AlterUserRemoveRoleDelegation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterUserRemoveRoleDelegation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserRemoveRoleDelegation", "path": "AlterUserRemoveRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11583, 35], "end": [11583, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4642275dd91a6748d4362b9a"></a>
## role

`struct_field` · `sqlparser::ast::AlterUserRemoveRoleDelegation::role` · sqlparser 0.62.0

```rust
role: Option<Ident>
```

Source: `src/ast/mod.rs:11588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional role name to remove delegation for.

<a id="op-293e10e5b9887d88571d8c11"></a>
## serialize

`function` · `sqlparser::ast::AlterUserRemoveRoleDelegation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserRemoveRoleDelegation", "path": "AlterUserRemoveRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11584, 38], "end": [11584, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60cc0a1b2e24898f9429525c"></a>
## visit

`function` · `sqlparser::ast::AlterUserRemoveRoleDelegation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserRemoveRoleDelegation", "path": "AlterUserRemoveRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11585, 40], "end": [11585, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faa010fbb76156f03e0b1b05"></a>
## visit

`function` · `sqlparser::ast::AlterUserRemoveRoleDelegation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserRemoveRoleDelegation", "path": "AlterUserRemoveRoleDelegation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11585, 47], "end": [11585, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
