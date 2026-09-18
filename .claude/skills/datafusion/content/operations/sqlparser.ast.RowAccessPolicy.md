# `sqlparser::ast::RowAccessPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.RowAccessPolicy.json).

<a id="op-d14d19cde72fb791df2ffa38"></a>
## RowAccessPolicy

`struct` · `sqlparser::ast::RowAccessPolicy` · sqlparser 0.62.0

```rust
struct RowAccessPolicy
```

Source: `src/ast/mod.rs:10574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `WITH ROW ACCESS POLICY policy_name ON (identifier, ...)`

<https://docs.snowflake.com/en/sql-reference/sql/create-table>
<https://docs.snowflake.com/en/user-guide/security-row-intro>

<a id="op-14134b4820beb73fefc5e8a9"></a>
## clone

`function` · `sqlparser::ast::RowAccessPolicy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RowAccessPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10571, 17], "end": [10571, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62a249bf364a35dc8cf60763"></a>
## cmp

`function` · `sqlparser::ast::RowAccessPolicy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RowAccessPolicy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10571, 51], "end": [10571, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-124a836118fe5cf385bbee99"></a>
## deserialize

`function` · `sqlparser::ast::RowAccessPolicy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10572, 49], "end": [10572, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fb6e2a80a906a7c1a6b6a2a"></a>
## eq

`function` · `sqlparser::ast::RowAccessPolicy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RowAccessPolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10571, 24], "end": [10571, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71247121e49524397a869b65"></a>
## fmt

`function` · `sqlparser::ast::RowAccessPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10588, 1], "end": [10597, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10589`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0c5ffbe6e6e79d3bde1f75d"></a>
## fmt

`function` · `sqlparser::ast::RowAccessPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10571, 10], "end": [10571, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad0c175e1adf6ca6ffd9b860"></a>
## hash

`function` · `sqlparser::ast::RowAccessPolicy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10571, 56], "end": [10571, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f2ec3699a74f7461ec91f9d"></a>
## new

`function` · `sqlparser::ast::RowAccessPolicy::new` · sqlparser 0.62.0

```rust
fn new(policy: ObjectName, on: Vec<Ident>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10581, 1], "end": [10586, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:10583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new `RowAccessPolicy` for the given `policy` and `on` identifiers.

<a id="op-1d6c6e77c4620041ee2fc2cf"></a>
## on

`struct_field` · `sqlparser::ast::RowAccessPolicy::on` · sqlparser 0.62.0

```rust
on: Vec<Ident>
```

Source: `src/ast/mod.rs:10578`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Identifiers for the columns or objects the policy applies to.

<a id="op-104811db7a3889fe4d3c3c68"></a>
## partial_cmp

`function` · `sqlparser::ast::RowAccessPolicy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RowAccessPolicy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10571, 35], "end": [10571, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10571`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-529a05ee5c7b1c8234b94689"></a>
## policy

`struct_field` · `sqlparser::ast::RowAccessPolicy::policy` · sqlparser 0.62.0

```rust
policy: ObjectName
```

Source: `src/ast/mod.rs:10576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The fully-qualified policy object name.

<a id="op-16de641861b00e2778aa3e44"></a>
## serialize

`function` · `sqlparser::ast::RowAccessPolicy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10572, 38], "end": [10572, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d8dfacdb93b60ea2204a724"></a>
## visit

`function` · `sqlparser::ast::RowAccessPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10573, 47], "end": [10573, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eae3f1804cf7c254cc5497c8"></a>
## visit

`function` · `sqlparser::ast::RowAccessPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RowAccessPolicy", "path": "RowAccessPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10573, 40], "end": [10573, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
