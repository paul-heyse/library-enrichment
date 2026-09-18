# `sqlparser::ast::AlterUserAddMfaMethodOtp`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AlterUserAddMfaMethodOtp.json).

<a id="op-9e16fb7bde2ffcf5cef4ba7e"></a>
## AlterUserAddMfaMethodOtp

`struct` · `sqlparser::ast::AlterUserAddMfaMethodOtp` · sqlparser 0.62.0

```rust
struct AlterUserAddMfaMethodOtp
```

Source: `src/ast/mod.rs:11599`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ADD MFA METHOD OTP [ COUNT = number ]
```

<a id="op-ddcd5fc0327e6f483f356d17"></a>
## clone

`function` · `sqlparser::ast::AlterUserAddMfaMethodOtp::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterUserAddMfaMethodOtp
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddMfaMethodOtp", "path": "AlterUserAddMfaMethodOtp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11596, 17], "end": [11596, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-257d0d966e3cd27788f1fa9d"></a>
## cmp

`function` · `sqlparser::ast::AlterUserAddMfaMethodOtp::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterUserAddMfaMethodOtp) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddMfaMethodOtp", "path": "AlterUserAddMfaMethodOtp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11596, 51], "end": [11596, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a50ecdf464211599a6034c21"></a>
## count

`struct_field` · `sqlparser::ast::AlterUserAddMfaMethodOtp::count` · sqlparser 0.62.0

```rust
count: Option<ValueWithSpan>
```

Source: `src/ast/mod.rs:11601`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional OTP count parameter.

<a id="op-98f53010ea6233b5f7577fa3"></a>
## deserialize

`function` · `sqlparser::ast::AlterUserAddMfaMethodOtp::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddMfaMethodOtp", "path": "AlterUserAddMfaMethodOtp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11597, 49], "end": [11597, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11597`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0605c4778769268e9ca8642c"></a>
## eq

`function` · `sqlparser::ast::AlterUserAddMfaMethodOtp::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterUserAddMfaMethodOtp) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddMfaMethodOtp", "path": "AlterUserAddMfaMethodOtp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11596, 24], "end": [11596, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e06743b919ec4b19a5c4f6f"></a>
## fmt

`function` · `sqlparser::ast::AlterUserAddMfaMethodOtp::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddMfaMethodOtp", "path": "AlterUserAddMfaMethodOtp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11596, 10], "end": [11596, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea51d66587e5ab9d664c5d8c"></a>
## hash

`function` · `sqlparser::ast::AlterUserAddMfaMethodOtp::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddMfaMethodOtp", "path": "AlterUserAddMfaMethodOtp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11596, 56], "end": [11596, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c6216c291d912da2c8dc903"></a>
## partial_cmp

`function` · `sqlparser::ast::AlterUserAddMfaMethodOtp::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterUserAddMfaMethodOtp) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddMfaMethodOtp", "path": "AlterUserAddMfaMethodOtp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11596, 35], "end": [11596, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f7b567850046e6187a2b6cb"></a>
## serialize

`function` · `sqlparser::ast::AlterUserAddMfaMethodOtp::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddMfaMethodOtp", "path": "AlterUserAddMfaMethodOtp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11597, 38], "end": [11597, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11597`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ef7ced9d6c7413157be21c7"></a>
## visit

`function` · `sqlparser::ast::AlterUserAddMfaMethodOtp::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddMfaMethodOtp", "path": "AlterUserAddMfaMethodOtp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11598, 47], "end": [11598, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c93d4fd2c2a983aa55e39384"></a>
## visit

`function` · `sqlparser::ast::AlterUserAddMfaMethodOtp::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserAddMfaMethodOtp", "path": "AlterUserAddMfaMethodOtp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11598, 40], "end": [11598, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
