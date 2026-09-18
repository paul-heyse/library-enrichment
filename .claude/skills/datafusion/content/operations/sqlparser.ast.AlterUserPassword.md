# `sqlparser::ast::AlterUserPassword`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AlterUserPassword.json).

<a id="op-de68a411552364df5c9f86f4"></a>
## AlterUserPassword

`struct` · `sqlparser::ast::AlterUserPassword` · sqlparser 0.62.0

```rust
struct AlterUserPassword
```

Source: `src/ast/mod.rs:11769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER USER <role_specification> [ WITH ] PASSWORD { 'password' | NULL }``
```

<a id="op-2fa1298aea766bb92c114275"></a>
## clone

`function` · `sqlparser::ast::AlterUserPassword::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterUserPassword
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11766, 17], "end": [11766, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78a89978c52728de8ac19551"></a>
## cmp

`function` · `sqlparser::ast::AlterUserPassword::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterUserPassword) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11766, 51], "end": [11766, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf7847899113b86908607dcf"></a>
## deserialize

`function` · `sqlparser::ast::AlterUserPassword::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11767, 49], "end": [11767, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11767`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1fa6c43a9c0b242688a1751"></a>
## encrypted

`struct_field` · `sqlparser::ast::AlterUserPassword::encrypted` · sqlparser 0.62.0

```rust
encrypted: bool
```

Source: `src/ast/mod.rs:11771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the password is encrypted.

<a id="op-e22c5c9e9371feda9b897f30"></a>
## eq

`function` · `sqlparser::ast::AlterUserPassword::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterUserPassword) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11766, 24], "end": [11766, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d839233d622afccb6f0ce9f4"></a>
## fmt

`function` · `sqlparser::ast::AlterUserPassword::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11776, 1], "end": [11788, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4fe550723a9905ad740d491"></a>
## fmt

`function` · `sqlparser::ast::AlterUserPassword::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11766, 10], "end": [11766, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb790f12ebf7575aefceb33f"></a>
## hash

`function` · `sqlparser::ast::AlterUserPassword::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11766, 56], "end": [11766, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14882c77f41e374c12eaef25"></a>
## partial_cmp

`function` · `sqlparser::ast::AlterUserPassword::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterUserPassword) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11766, 35], "end": [11766, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-415e00599df764944972cebf"></a>
## password

`struct_field` · `sqlparser::ast::AlterUserPassword::password` · sqlparser 0.62.0

```rust
password: Option<String>
```

Source: `src/ast/mod.rs:11773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The password string, or `None` for `NULL`.

<a id="op-57c1c4a26896b5cb79135c2e"></a>
## serialize

`function` · `sqlparser::ast::AlterUserPassword::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11767, 38], "end": [11767, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11767`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ef8371c45ff4e8e068b0c9e"></a>
## visit

`function` · `sqlparser::ast::AlterUserPassword::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11768, 47], "end": [11768, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7e15c617169d00987bf35b2"></a>
## visit

`function` · `sqlparser::ast::AlterUserPassword::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUserPassword", "path": "AlterUserPassword"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11768, 40], "end": [11768, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
