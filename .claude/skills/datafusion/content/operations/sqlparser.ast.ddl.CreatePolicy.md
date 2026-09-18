# `sqlparser::ast::ddl::CreatePolicy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreatePolicy.json).

<a id="op-d5d5e6f8a96304ba77809f1e"></a>
## CreatePolicy

`struct` · `sqlparser::ast::ddl::CreatePolicy` · sqlparser 0.62.0

```rust
struct CreatePolicy
```

Source: `src/ast/ddl.rs:5576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE POLICY statement.

See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createpolicy.html)

<a id="op-0b56bde2afb06833afe5c5ea"></a>
## clone

`function` · `sqlparser::ast::ddl::CreatePolicy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreatePolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5573, 17], "end": [5573, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9180ba7e39d62ef821d6ffe"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreatePolicy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreatePolicy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5573, 51], "end": [5573, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f23307559ad68936ce728fe"></a>
## command

`struct_field` · `sqlparser::ast::ddl::CreatePolicy::command` · sqlparser 0.62.0

```rust
command: Option<CreatePolicyCommand>
```

Source: `src/ast/ddl.rs:5585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional command the policy applies to (e.g., `SELECT`).

<a id="op-9c02363b020ca92dffe0db99"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreatePolicy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5574, 49], "end": [5574, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7002aa25b09164244de23e73"></a>
## eq

`function` · `sqlparser::ast::ddl::CreatePolicy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreatePolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5573, 24], "end": [5573, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c8f01d05e9d31cbb5fac95d"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreatePolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5594, 1], "end": [5619, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5595`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b35dbeca86d9159f43095664"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreatePolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5573, 10], "end": [5573, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b66cc42d9b5b40ea87956251"></a>
## hash

`function` · `sqlparser::ast::ddl::CreatePolicy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5573, 56], "end": [5573, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01f5b30b6adbf5a1acf147bf"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreatePolicy::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:5578`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the policy.

<a id="op-6b2524488c201cf5ff31c8eb"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreatePolicy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreatePolicy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5573, 35], "end": [5573, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e4f9528aa64d455a1615405"></a>
## policy_type

`struct_field` · `sqlparser::ast::ddl::CreatePolicy::policy_type` · sqlparser 0.62.0

```rust
policy_type: Option<CreatePolicyType>
```

Source: `src/ast/ddl.rs:5583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional policy type (e.g., `PERMISSIVE` / `RESTRICTIVE`).

<a id="op-67980c233faa086e9754940a"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreatePolicy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5574, 38], "end": [5574, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-861c37487c82f2c793139f6c"></a>
## table_name

`struct_field` · `sqlparser::ast::ddl::CreatePolicy::table_name` · sqlparser 0.62.0

```rust
table_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:5581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table the policy is defined on.

<a id="op-2f1490db24438f9fdc4eba81"></a>
## to

`struct_field` · `sqlparser::ast::ddl::CreatePolicy::to` · sqlparser 0.62.0

```rust
to: Option<Vec<Owner>>
```

Source: `src/ast/ddl.rs:5587`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of grantee owners.

<a id="op-c35168a5098b0db423e0b060"></a>
## using

`struct_field` · `sqlparser::ast::ddl::CreatePolicy::using` · sqlparser 0.62.0

```rust
using: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:5589`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional expression for the `USING` clause.

<a id="op-2092533b7c1b69cd916e6d51"></a>
## visit

`function` · `sqlparser::ast::ddl::CreatePolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5575, 40], "end": [5575, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7a59e33bf60d87e5d2c5aaa"></a>
## visit

`function` · `sqlparser::ast::ddl::CreatePolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5575, 47], "end": [5575, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb1c6008ca606ebf5eb955e2"></a>
## with_check

`struct_field` · `sqlparser::ast::ddl::CreatePolicy::with_check` · sqlparser 0.62.0

```rust
with_check: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:5591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional expression for the `WITH CHECK` clause.
