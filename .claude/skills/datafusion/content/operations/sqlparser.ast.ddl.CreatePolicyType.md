# `sqlparser::ast::ddl::CreatePolicyType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreatePolicyType.json).

<a id="op-b834329e6377691cc06d1004"></a>
## CreatePolicyType

`enum` · `sqlparser::ast::ddl::CreatePolicyType` · sqlparser 0.62.0

```rust
enum CreatePolicyType
```

Source: `src/ast/ddl.rs:5629`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Policy type for a `CREATE POLICY` statement.
```sql
AS [ PERMISSIVE | RESTRICTIVE ]
```
[PostgreSQL](https://www.postgresql.org/docs/current/sql-createpolicy.html)

<a id="op-a62f0904892cd8ad631e1cfd"></a>
## Permissive

`variant` · `sqlparser::ast::ddl::CreatePolicyType::Permissive` · sqlparser 0.62.0

```rust
Permissive
```

Source: `src/ast/ddl.rs:5631`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Policy allows operations unless explicitly denied.

<a id="op-2fd54eaaa2a2055ff6570cea"></a>
## Restrictive

`variant` · `sqlparser::ast::ddl::CreatePolicyType::Restrictive` · sqlparser 0.62.0

```rust
Restrictive
```

Source: `src/ast/ddl.rs:5633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Policy denies operations unless explicitly allowed.

<a id="op-99a8ed2bf8d09de1b337cc47"></a>
## clone

`function` · `sqlparser::ast::ddl::CreatePolicyType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreatePolicyType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5626, 17], "end": [5626, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c448be047344ae2a25d11cb"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreatePolicyType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreatePolicyType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5626, 57], "end": [5626, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c527435c1362d838239684fb"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreatePolicyType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5627, 49], "end": [5627, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cc892127043b95c12ff3326"></a>
## eq

`function` · `sqlparser::ast::ddl::CreatePolicyType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreatePolicyType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5626, 30], "end": [5626, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-653a478073bd0420b6faee3e"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreatePolicyType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5626, 10], "end": [5626, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c2674781b59d50e92da5013"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreatePolicyType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5636, 1], "end": [5643, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c352f728d5e014be48a46a6"></a>
## hash

`function` · `sqlparser::ast::ddl::CreatePolicyType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5626, 62], "end": [5626, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-894cfb21a1f3e0fc3075a5d2"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreatePolicyType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreatePolicyType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5626, 41], "end": [5626, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa27f0bd3a22faaed375b083"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreatePolicyType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5627, 38], "end": [5627, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35c3192aea6a0a6b9d491944"></a>
## visit

`function` · `sqlparser::ast::ddl::CreatePolicyType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5628, 40], "end": [5628, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c46d3627feccbfb921284fa"></a>
## visit

`function` · `sqlparser::ast::ddl::CreatePolicyType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyType", "path": "CreatePolicyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5628, 47], "end": [5628, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
