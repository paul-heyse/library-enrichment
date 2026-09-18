# `sqlparser::ast::SecretOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SecretOption.json).

<a id="op-73d49f55bcbf0f3f2e79dd5b"></a>
## SecretOption

`struct` · `sqlparser::ast::SecretOption` · sqlparser 0.62.0

```rust
struct SecretOption
```

Source: `src/ast/mod.rs:8928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A key/value identifier pair used for secret or key-based options.

<a id="op-b1db918b94fb72f203bac9c7"></a>
## clone

`function` · `sqlparser::ast::SecretOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SecretOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8924, 17], "end": [8924, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8924`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74d3cc7497f2df15b33fe93e"></a>
## cmp

`function` · `sqlparser::ast::SecretOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SecretOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8924, 51], "end": [8924, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8924`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3066830c5d43074186e7a77c"></a>
## deserialize

`function` · `sqlparser::ast::SecretOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8925, 49], "end": [8925, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a805011d183fd860174fea9"></a>
## eq

`function` · `sqlparser::ast::SecretOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SecretOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8924, 24], "end": [8924, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8924`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16dfbdb25ca79e29f9752d11"></a>
## fmt

`function` · `sqlparser::ast::SecretOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8935, 1], "end": [8939, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4306881313d885aeb137e6e9"></a>
## fmt

`function` · `sqlparser::ast::SecretOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8924, 10], "end": [8924, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8924`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05572e888a876bbfc5b7819b"></a>
## hash

`function` · `sqlparser::ast::SecretOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8924, 56], "end": [8924, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8924`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b18456c590ef53eb5b80d06e"></a>
## key

`struct_field` · `sqlparser::ast::SecretOption::key` · sqlparser 0.62.0

```rust
key: Ident
```

Source: `src/ast/mod.rs:8930`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The option key identifier.

<a id="op-2baea36827cbd7ee951dc4be"></a>
## partial_cmp

`function` · `sqlparser::ast::SecretOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SecretOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8924, 35], "end": [8924, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8924`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eefb4a203830a6670e96409f"></a>
## serialize

`function` · `sqlparser::ast::SecretOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8925, 38], "end": [8925, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c18e42725ce9fc068930d085"></a>
## value

`struct_field` · `sqlparser::ast::SecretOption::value` · sqlparser 0.62.0

```rust
value: Ident
```

Source: `src/ast/mod.rs:8932`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The option value identifier.

<a id="op-2e2c4b035bfc5ed0ad9554cb"></a>
## visit

`function` · `sqlparser::ast::SecretOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8926, 47], "end": [8926, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c0df02d8243b616eaaffe12"></a>
## visit

`function` · `sqlparser::ast::SecretOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SecretOption", "path": "SecretOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8926, 40], "end": [8926, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
