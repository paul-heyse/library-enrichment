# `sqlparser::ast::ActionApplyType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ActionApplyType.json).

<a id="op-8c84cc5606414e085f064609"></a>
## ActionApplyType

`enum` · `sqlparser::ast::ActionApplyType` · sqlparser 0.62.0

```rust
enum ActionApplyType
```

Source: `src/ast/mod.rs:7250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/sql/grant-privilege>
under `globalPrivileges` in the `APPLY` privilege.

<a id="op-743dc9bb45f01576ec5696af"></a>
## AggregationPolicy

`variant` · `sqlparser::ast::ActionApplyType::AggregationPolicy` · sqlparser 0.62.0

```rust
AggregationPolicy
```

Source: `src/ast/mod.rs:7252`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply an aggregation policy.

<a id="op-1b64f8f31afb6b373099ab46"></a>
## AuthenticationPolicy

`variant` · `sqlparser::ast::ActionApplyType::AuthenticationPolicy` · sqlparser 0.62.0

```rust
AuthenticationPolicy
```

Source: `src/ast/mod.rs:7254`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply an authentication policy.

<a id="op-f31f8abba6f7d52fed05a57e"></a>
## JoinPolicy

`variant` · `sqlparser::ast::ActionApplyType::JoinPolicy` · sqlparser 0.62.0

```rust
JoinPolicy
```

Source: `src/ast/mod.rs:7256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply a join policy.

<a id="op-c1c45942d8559d7ace5d5570"></a>
## MaskingPolicy

`variant` · `sqlparser::ast::ActionApplyType::MaskingPolicy` · sqlparser 0.62.0

```rust
MaskingPolicy
```

Source: `src/ast/mod.rs:7258`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply a masking policy.

<a id="op-827a749862058ec261291ad6"></a>
## PackagesPolicy

`variant` · `sqlparser::ast::ActionApplyType::PackagesPolicy` · sqlparser 0.62.0

```rust
PackagesPolicy
```

Source: `src/ast/mod.rs:7260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply a packages policy.

<a id="op-ffd0ae0b28a5f8561febb1df"></a>
## PasswordPolicy

`variant` · `sqlparser::ast::ActionApplyType::PasswordPolicy` · sqlparser 0.62.0

```rust
PasswordPolicy
```

Source: `src/ast/mod.rs:7262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply a password policy.

<a id="op-8aa657d7a32a14644993e1e0"></a>
## ProjectionPolicy

`variant` · `sqlparser::ast::ActionApplyType::ProjectionPolicy` · sqlparser 0.62.0

```rust
ProjectionPolicy
```

Source: `src/ast/mod.rs:7264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply a projection policy.

<a id="op-a0ad68a9a41fc1b71555eab9"></a>
## RowAccessPolicy

`variant` · `sqlparser::ast::ActionApplyType::RowAccessPolicy` · sqlparser 0.62.0

```rust
RowAccessPolicy
```

Source: `src/ast/mod.rs:7266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply a row access policy.

<a id="op-85394221f49acc50ba1420ca"></a>
## SessionPolicy

`variant` · `sqlparser::ast::ActionApplyType::SessionPolicy` · sqlparser 0.62.0

```rust
SessionPolicy
```

Source: `src/ast/mod.rs:7268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply a session policy.

<a id="op-ce25072e44bb898db01db3c0"></a>
## Tag

`variant` · `sqlparser::ast::ActionApplyType::Tag` · sqlparser 0.62.0

```rust
Tag
```

Source: `src/ast/mod.rs:7270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply a tag.

<a id="op-745a01294a86a7fb734ac527"></a>
## clone

`function` · `sqlparser::ast::ActionApplyType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ActionApplyType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7245, 17], "end": [7245, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57762e49fb5f4dd24dab5a51"></a>
## cmp

`function` · `sqlparser::ast::ActionApplyType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ActionApplyType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7245, 51], "end": [7245, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcc9c5fea3eed6821810634f"></a>
## deserialize

`function` · `sqlparser::ast::ActionApplyType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7246, 49], "end": [7246, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7246`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ae11a6c6fd080c91d9fcd3"></a>
## eq

`function` · `sqlparser::ast::ActionApplyType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ActionApplyType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7245, 24], "end": [7245, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d97f8e8bb8eaa7b8b9a85d3"></a>
## fmt

`function` · `sqlparser::ast::ActionApplyType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7245, 10], "end": [7245, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0ca1487daf0f0bbcb9dda4a"></a>
## fmt

`function` · `sqlparser::ast::ActionApplyType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7273, 1], "end": [7288, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7274`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9981b2ba75be410333f47dd"></a>
## hash

`function` · `sqlparser::ast::ActionApplyType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7245, 56], "end": [7245, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-978922c821511672b46afd27"></a>
## partial_cmp

`function` · `sqlparser::ast::ActionApplyType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ActionApplyType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7245, 35], "end": [7245, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12d93a358334551c58e36652"></a>
## serialize

`function` · `sqlparser::ast::ActionApplyType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7246, 38], "end": [7246, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7246`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00cc0a42a381693295ff65de"></a>
## visit

`function` · `sqlparser::ast::ActionApplyType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7247, 40], "end": [7247, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7247`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24113bd798d597f0cf91ca6c"></a>
## visit

`function` · `sqlparser::ast::ActionApplyType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionApplyType", "path": "ActionApplyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7247, 47], "end": [7247, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7247`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
