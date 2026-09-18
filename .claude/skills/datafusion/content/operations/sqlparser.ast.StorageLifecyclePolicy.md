# `sqlparser::ast::StorageLifecyclePolicy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.StorageLifecyclePolicy.json).

<a id="op-1522f93997cd137c10396021"></a>
## StorageLifecyclePolicy

`struct` · `sqlparser::ast::StorageLifecyclePolicy` · sqlparser 0.62.0

```rust
struct StorageLifecyclePolicy
```

Source: `src/ast/mod.rs:10605`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `[ WITH ] STORAGE LIFECYCLE POLICY <policy_name> ON ( <col_name> [ , ... ] )`

<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-ad26ab54d9feddbea0e3fde7"></a>
## clone

`function` · `sqlparser::ast::StorageLifecyclePolicy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> StorageLifecyclePolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10602, 17], "end": [10602, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0054d743a1d7ecb5ab8f77f"></a>
## cmp

`function` · `sqlparser::ast::StorageLifecyclePolicy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &StorageLifecyclePolicy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10602, 51], "end": [10602, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fe64ddbb3dc0ddcd400ed3a"></a>
## deserialize

`function` · `sqlparser::ast::StorageLifecyclePolicy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10603, 49], "end": [10603, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10603`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be3cace6047a57f402baa1df"></a>
## eq

`function` · `sqlparser::ast::StorageLifecyclePolicy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &StorageLifecyclePolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10602, 24], "end": [10602, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36a4f15cd02000b9583e52ae"></a>
## fmt

`function` · `sqlparser::ast::StorageLifecyclePolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10612, 1], "end": [10621, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10613`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-743208b134ec431cb23ed337"></a>
## fmt

`function` · `sqlparser::ast::StorageLifecyclePolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10602, 10], "end": [10602, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fcddde8b4708ee1bbc1c02a"></a>
## hash

`function` · `sqlparser::ast::StorageLifecyclePolicy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10602, 56], "end": [10602, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bef4aaac5e8c2e163c088bb4"></a>
## on

`struct_field` · `sqlparser::ast::StorageLifecyclePolicy::on` · sqlparser 0.62.0

```rust
on: Vec<Ident>
```

Source: `src/ast/mod.rs:10609`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column names the policy applies to.

<a id="op-6549703cf58fda443e95fbe0"></a>
## partial_cmp

`function` · `sqlparser::ast::StorageLifecyclePolicy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &StorageLifecyclePolicy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10602, 35], "end": [10602, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad2eee116fc344a8759beaac"></a>
## policy

`struct_field` · `sqlparser::ast::StorageLifecyclePolicy::policy` · sqlparser 0.62.0

```rust
policy: ObjectName
```

Source: `src/ast/mod.rs:10607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The fully-qualified policy object name.

<a id="op-ef94973385a71f3dcc5c303d"></a>
## serialize

`function` · `sqlparser::ast::StorageLifecyclePolicy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10603, 38], "end": [10603, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10603`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f7b4701db77bd919c56448e"></a>
## visit

`function` · `sqlparser::ast::StorageLifecyclePolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10604, 47], "end": [10604, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b91ecc0a02e1d24d7afbe085"></a>
## visit

`function` · `sqlparser::ast::StorageLifecyclePolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageLifecyclePolicy", "path": "StorageLifecyclePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10604, 40], "end": [10604, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
