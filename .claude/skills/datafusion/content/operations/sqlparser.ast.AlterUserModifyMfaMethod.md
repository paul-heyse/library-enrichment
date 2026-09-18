# `sqlparser::ast::AlterUserModifyMfaMethod`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AlterUserModifyMfaMethod.json).

<a id="op-83dcde00510f262f32fb05be"></a>
## AlterUserModifyMfaMethod

`struct` · `sqlparser::ast::AlterUserModifyMfaMethod` · sqlparser 0.62.0

```rust
struct AlterUserModifyMfaMethod
```

Source: `src/ast/mod.rs:11610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER USER [ IF EXISTS ] [ <name> ] MODIFY MFA METHOD <mfa_method> SET COMMENT = '<string>'
```

<a id="op-f22a9afb69028e7b375fbe8f"></a>
## clone

`function` · `sqlparser::ast::AlterUserModifyMfaMethod::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterUserModifyMfaMethod
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserModifyMfaMethod", "path": "AlterUserModifyMfaMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11607, 17], "end": [11607, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0df97e895efd2f74836e32c6"></a>
## cmp

`function` · `sqlparser::ast::AlterUserModifyMfaMethod::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterUserModifyMfaMethod) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserModifyMfaMethod", "path": "AlterUserModifyMfaMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11607, 51], "end": [11607, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fd1fc520b580826b10ab8cf"></a>
## comment

`struct_field` · `sqlparser::ast::AlterUserModifyMfaMethod::comment` · sqlparser 0.62.0

```rust
comment: String
```

Source: `src/ast/mod.rs:11614`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The new comment for the MFA method.

<a id="op-a6e5f6d347cd4dec37b2f6b2"></a>
## deserialize

`function` · `sqlparser::ast::AlterUserModifyMfaMethod::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserModifyMfaMethod", "path": "AlterUserModifyMfaMethod"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11608, 49], "end": [11608, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fc33e35f1a21f2cd62b3657"></a>
## eq

`function` · `sqlparser::ast::AlterUserModifyMfaMethod::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterUserModifyMfaMethod) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserModifyMfaMethod", "path": "AlterUserModifyMfaMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11607, 24], "end": [11607, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-700c486d866da6c43c726bc5"></a>
## fmt

`function` · `sqlparser::ast::AlterUserModifyMfaMethod::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserModifyMfaMethod", "path": "AlterUserModifyMfaMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11607, 10], "end": [11607, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-134759cb911fd5920c9ce44d"></a>
## hash

`function` · `sqlparser::ast::AlterUserModifyMfaMethod::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserModifyMfaMethod", "path": "AlterUserModifyMfaMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11607, 56], "end": [11607, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6269a2365c850d555f782776"></a>
## method

`struct_field` · `sqlparser::ast::AlterUserModifyMfaMethod::method` · sqlparser 0.62.0

```rust
method: MfaMethodKind
```

Source: `src/ast/mod.rs:11612`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The MFA method being modified.

<a id="op-93f25e7c98e33c3764c7ed45"></a>
## partial_cmp

`function` · `sqlparser::ast::AlterUserModifyMfaMethod::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterUserModifyMfaMethod) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserModifyMfaMethod", "path": "AlterUserModifyMfaMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11607, 35], "end": [11607, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a1bf736c4c08862a3c77a74"></a>
## serialize

`function` · `sqlparser::ast::AlterUserModifyMfaMethod::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserModifyMfaMethod", "path": "AlterUserModifyMfaMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11608, 38], "end": [11608, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d66e38f3aaf982367b038ce"></a>
## visit

`function` · `sqlparser::ast::AlterUserModifyMfaMethod::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserModifyMfaMethod", "path": "AlterUserModifyMfaMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11609, 40], "end": [11609, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11609`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8256d2df0a4811f5989051e"></a>
## visit

`function` · `sqlparser::ast::AlterUserModifyMfaMethod::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserModifyMfaMethod", "path": "AlterUserModifyMfaMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11609, 47], "end": [11609, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11609`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
