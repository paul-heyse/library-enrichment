# `sqlparser::ast::ddl::ColumnPolicyProperty`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ColumnPolicyProperty.json).

<a id="op-155b8a72dc98907d18a2b368"></a>
## ColumnPolicyProperty

`struct` · `sqlparser::ast::ddl::ColumnPolicyProperty` · sqlparser 0.62.0

```rust
struct ColumnPolicyProperty
```

Source: `src/ast/ddl.rs:1844`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Properties describing a column policy (masking or projection).

<a id="op-b0403bde92931155e160ffd1"></a>
## clone

`function` · `sqlparser::ast::ddl::ColumnPolicyProperty::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ColumnPolicyProperty
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicyProperty", "path": "ColumnPolicyProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1840, 17], "end": [1840, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-829207bd6f1233bae8c5c287"></a>
## cmp

`function` · `sqlparser::ast::ddl::ColumnPolicyProperty::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ColumnPolicyProperty) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicyProperty", "path": "ColumnPolicyProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1840, 51], "end": [1840, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf990be930e4fc5a2ae2f5f0"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ColumnPolicyProperty::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicyProperty", "path": "ColumnPolicyProperty"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1841, 49], "end": [1841, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1841`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ca03d1a9979448f425b52a8"></a>
## eq

`function` · `sqlparser::ast::ddl::ColumnPolicyProperty::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ColumnPolicyProperty) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicyProperty", "path": "ColumnPolicyProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1840, 24], "end": [1840, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acff2c38c7168eb361d738cc"></a>
## fmt

`function` · `sqlparser::ast::ddl::ColumnPolicyProperty::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicyProperty", "path": "ColumnPolicyProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1840, 10], "end": [1840, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cabb9c823a0d28e760c71c0"></a>
## hash

`function` · `sqlparser::ast::ddl::ColumnPolicyProperty::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicyProperty", "path": "ColumnPolicyProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1840, 56], "end": [1840, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0f073be7bbc28416f853c45"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ColumnPolicyProperty::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ColumnPolicyProperty) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicyProperty", "path": "ColumnPolicyProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1840, 35], "end": [1840, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2abda35f42f9f80f2252c748"></a>
## policy_name

`struct_field` · `sqlparser::ast::ddl::ColumnPolicyProperty::policy_name` · sqlparser 0.62.0

```rust
policy_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:1853`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the policy to apply to the column.

<a id="op-f429148c8fe691b11ec870f2"></a>
## serialize

`function` · `sqlparser::ast::ddl::ColumnPolicyProperty::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicyProperty", "path": "ColumnPolicyProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1841, 38], "end": [1841, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1841`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ae983a95a91d9ff0caec8a2"></a>
## using_columns

`struct_field` · `sqlparser::ast::ddl::ColumnPolicyProperty::using_columns` · sqlparser 0.62.0

```rust
using_columns: Option<Vec<ast::Ident>>
```

Source: `src/ast/ddl.rs:1855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of column identifiers referenced by the policy.

<a id="op-187fdb190afa7c4860173219"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnPolicyProperty::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicyProperty", "path": "ColumnPolicyProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1842, 47], "end": [1842, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1842`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2e0d0d2f89502b69e4bece2"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnPolicyProperty::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicyProperty", "path": "ColumnPolicyProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1842, 40], "end": [1842, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1842`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45dd6ba44488b183ca29ee3d"></a>
## with

`struct_field` · `sqlparser::ast::ddl::ColumnPolicyProperty::with` · sqlparser 0.62.0

```rust
with: bool
```

Source: `src/ast/ddl.rs:1851`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This flag indicates that the column policy option is declared using the `WITH` prefix.
Example
```sql
WITH PROJECTION POLICY sample_policy
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table
