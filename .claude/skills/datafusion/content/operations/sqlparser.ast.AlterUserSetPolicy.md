# `sqlparser::ast::AlterUserSetPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AlterUserSetPolicy.json).

<a id="op-7c8717cc10e6b0b225295694"></a>
## AlterUserSetPolicy

`struct` · `sqlparser::ast::AlterUserSetPolicy` · sqlparser 0.62.0

```rust
struct AlterUserSetPolicy
```

Source: `src/ast/mod.rs:11646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER USER [ IF EXISTS ] [ <name> ] SET { AUTHENTICATION | PASSWORD | SESSION } POLICY <policy_name>
```

<a id="op-4aaee8dff3355a3160767d1c"></a>
## clone

`function` · `sqlparser::ast::AlterUserSetPolicy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterUserSetPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserSetPolicy", "path": "AlterUserSetPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11643, 17], "end": [11643, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-684091f5856b0f1681647513"></a>
## cmp

`function` · `sqlparser::ast::AlterUserSetPolicy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterUserSetPolicy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserSetPolicy", "path": "AlterUserSetPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11643, 51], "end": [11643, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a254661fc02c0c2fe5610dec"></a>
## deserialize

`function` · `sqlparser::ast::AlterUserSetPolicy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserSetPolicy", "path": "AlterUserSetPolicy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11644, 49], "end": [11644, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea35b31d513daa1d81f063e5"></a>
## eq

`function` · `sqlparser::ast::AlterUserSetPolicy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterUserSetPolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserSetPolicy", "path": "AlterUserSetPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11643, 24], "end": [11643, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72697802e2fda5f58b7e84e7"></a>
## fmt

`function` · `sqlparser::ast::AlterUserSetPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserSetPolicy", "path": "AlterUserSetPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11643, 10], "end": [11643, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e641f13b73da1437ceb3bf2"></a>
## hash

`function` · `sqlparser::ast::AlterUserSetPolicy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserSetPolicy", "path": "AlterUserSetPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11643, 56], "end": [11643, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e429dfc4ec3453d3ef7f3a25"></a>
## partial_cmp

`function` · `sqlparser::ast::AlterUserSetPolicy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterUserSetPolicy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserSetPolicy", "path": "AlterUserSetPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11643, 35], "end": [11643, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27dec940aa3d1bb46f6d08b7"></a>
## policy

`struct_field` · `sqlparser::ast::AlterUserSetPolicy::policy` · sqlparser 0.62.0

```rust
policy: Ident
```

Source: `src/ast/mod.rs:11650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The identifier of the policy to apply.

<a id="op-e5870c0eb0718a5a1c92c2ec"></a>
## policy_kind

`struct_field` · `sqlparser::ast::AlterUserSetPolicy::policy_kind` · sqlparser 0.62.0

```rust
policy_kind: UserPolicyKind
```

Source: `src/ast/mod.rs:11648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The kind of user policy being set (authentication/password/session).

<a id="op-99c018a695130b95bba5181d"></a>
## serialize

`function` · `sqlparser::ast::AlterUserSetPolicy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserSetPolicy", "path": "AlterUserSetPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11644, 38], "end": [11644, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04cce0d5a604e8695b56c65c"></a>
## visit

`function` · `sqlparser::ast::AlterUserSetPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserSetPolicy", "path": "AlterUserSetPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11645, 40], "end": [11645, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc267139231ba92500a35217"></a>
## visit

`function` · `sqlparser::ast::AlterUserSetPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserSetPolicy", "path": "AlterUserSetPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11645, 47], "end": [11645, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
