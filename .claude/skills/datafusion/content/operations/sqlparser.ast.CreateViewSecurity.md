# `sqlparser::ast::CreateViewSecurity`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateViewSecurity.json).

<a id="op-b06cf2a384fc9311784439b0"></a>
## CreateViewSecurity

`enum` · `sqlparser::ast::CreateViewSecurity` · sqlparser 0.62.0

```rust
enum CreateViewSecurity
```

Source: `src/ast/mod.rs:10497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL `CREATE VIEW` security parameter: [SQL SECURITY { DEFINER | INVOKER }]
MySQL `CREATE VIEW` SQL SECURITY options.

<a id="op-b5a001eac61e3de5d6092399"></a>
## Definer

`variant` · `sqlparser::ast::CreateViewSecurity::Definer` · sqlparser 0.62.0

```rust
Definer
```

Source: `src/ast/mod.rs:10499`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The view runs with the privileges of the definer.

<a id="op-d8956473296c02c57b5f1319"></a>
## Invoker

`variant` · `sqlparser::ast::CreateViewSecurity::Invoker` · sqlparser 0.62.0

```rust
Invoker
```

Source: `src/ast/mod.rs:10501`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The view runs with the privileges of the invoker.

<a id="op-3d9f816500db1cb189afdd05"></a>
## clone

`function` · `sqlparser::ast::CreateViewSecurity::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateViewSecurity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10493, 17], "end": [10493, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10493`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d20881325c5d4badecb5c9bc"></a>
## cmp

`function` · `sqlparser::ast::CreateViewSecurity::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateViewSecurity) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10493, 51], "end": [10493, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10493`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc31b21decda11a265538f6b"></a>
## deserialize

`function` · `sqlparser::ast::CreateViewSecurity::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10494, 49], "end": [10494, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bda2a179f802e4283151c37"></a>
## eq

`function` · `sqlparser::ast::CreateViewSecurity::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateViewSecurity) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10493, 24], "end": [10493, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10493`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a73bcd28f10ead0bc9959a0"></a>
## fmt

`function` · `sqlparser::ast::CreateViewSecurity::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10504, 1], "end": [10511, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10505`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b66821708c21500c6e51fb25"></a>
## fmt

`function` · `sqlparser::ast::CreateViewSecurity::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10493, 10], "end": [10493, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10493`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a362d4300cf73d4b7ac4292"></a>
## hash

`function` · `sqlparser::ast::CreateViewSecurity::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10493, 56], "end": [10493, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10493`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88158a5142ed3c1f016e2b65"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateViewSecurity::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateViewSecurity) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10493, 35], "end": [10493, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10493`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1312b0cbb699b5622b73998"></a>
## serialize

`function` · `sqlparser::ast::CreateViewSecurity::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10494, 38], "end": [10494, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aabee148889434111aa4c26c"></a>
## visit

`function` · `sqlparser::ast::CreateViewSecurity::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10495, 40], "end": [10495, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-deae26b79c39340399270e70"></a>
## visit

`function` · `sqlparser::ast::CreateViewSecurity::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewSecurity", "path": "CreateViewSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10495, 47], "end": [10495, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
