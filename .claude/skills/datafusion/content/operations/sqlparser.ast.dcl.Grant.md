# `sqlparser::ast::dcl::Grant`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.Grant.json).

<a id="op-576a2a31577e6f2d7ef194e6"></a>
## Grant

`struct` · `sqlparser::ast::dcl::Grant` · sqlparser 0.62.0

```rust
struct Grant
```

Source: `src/ast/dcl.rs:438`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

GRANT privileges ON objects TO grantees

<a id="op-603b16cb63099d34e84e4284"></a>
## as_grantor

`struct_field` · `sqlparser::ast::dcl::Grant::as_grantor` · sqlparser 0.62.0

```rust
as_grantor: Option<super::Ident>
```

Source: `src/ast/dcl.rs:448`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `AS GRANTOR` identifier.

<a id="op-683813b5c7d739abef2ae15f"></a>
## clone

`function` · `sqlparser::ast::dcl::Grant::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Grant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 17], "end": [435, 22], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dcl.rs:435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5d734d620bdefca06a66dd1"></a>
## cmp

`function` · `sqlparser::ast::dcl::Grant::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Grant) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 51], "end": [435, 54], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dcl.rs:435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fec091d9f87d8c898e2f6e4"></a>
## current_grants

`struct_field` · `sqlparser::ast::dcl::Grant::current_grants` · sqlparser 0.62.0

```rust
current_grants: Option<ast::CurrentGrantsKind>
```

Source: `src/ast/dcl.rs:456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `CURRENT GRANTS` modifier.

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/grant-privilege)

<a id="op-e3cba548839af4cc6711bd83"></a>
## deserialize

`function` · `sqlparser::ast::dcl::Grant::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [436, 49], "end": [436, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dcl.rs:436`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa5e08f2b09d06689e2a008e"></a>
## eq

`function` · `sqlparser::ast::dcl::Grant::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Grant) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 24], "end": [435, 33], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dcl.rs:435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9dd6c5cf8b070476bf0dbdc"></a>
## fmt

`function` · `sqlparser::ast::dcl::Grant::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 10], "end": [435, 15], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dcl.rs:435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea6f2110eb493c22cecf69da"></a>
## fmt

`function` · `sqlparser::ast::dcl::Grant::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 1], "end": [480, 2], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dcl.rs:460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f04ab0c72ecfaf34af876638"></a>
## granted_by

`struct_field` · `sqlparser::ast::dcl::Grant::granted_by` · sqlparser 0.62.0

```rust
granted_by: Option<super::Ident>
```

Source: `src/ast/dcl.rs:452`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `GRANTED BY` identifier.

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dcl-statements)

<a id="op-2159559835de2cc0f988a6d2"></a>
## grantees

`struct_field` · `sqlparser::ast::dcl::Grant::grantees` · sqlparser 0.62.0

```rust
grantees: Vec<ast::Grantee>
```

Source: `src/ast/dcl.rs:444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of grantees receiving the privileges.

<a id="op-8c3c3230d555cb62c1b57be3"></a>
## hash

`function` · `sqlparser::ast::dcl::Grant::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 56], "end": [435, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dcl.rs:435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5351423d88404932dceaed92"></a>
## objects

`struct_field` · `sqlparser::ast::dcl::Grant::objects` · sqlparser 0.62.0

```rust
objects: Option<ast::GrantObjects>
```

Source: `src/ast/dcl.rs:442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional objects the privileges apply to.

<a id="op-6d31bdda0f6ba2c41cf851e2"></a>
## partial_cmp

`function` · `sqlparser::ast::dcl::Grant::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Grant) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 35], "end": [435, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dcl.rs:435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bdb7e0babe09357c4dc3a3d"></a>
## privileges

`struct_field` · `sqlparser::ast::dcl::Grant::privileges` · sqlparser 0.62.0

```rust
privileges: ast::Privileges
```

Source: `src/ast/dcl.rs:440`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Privileges being granted.

<a id="op-22997f8ec43ae68489db2b6f"></a>
## serialize

`function` · `sqlparser::ast::dcl::Grant::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [436, 38], "end": [436, 47], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dcl.rs:436`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42428fee7d86c36aa891b477"></a>
## visit

`function` · `sqlparser::ast::dcl::Grant::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [437, 47], "end": [437, 55], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dcl.rs:437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5bbac19a8bed5b4cb6d1827"></a>
## visit

`function` · `sqlparser::ast::dcl::Grant::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [437, 40], "end": [437, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dcl.rs:437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af93857d4d11167d366b64d2"></a>
## with_grant_option

`struct_field` · `sqlparser::ast::dcl::Grant::with_grant_option` · sqlparser 0.62.0

```rust
with_grant_option: bool
```

Source: `src/ast/dcl.rs:446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `WITH GRANT OPTION` is present.
