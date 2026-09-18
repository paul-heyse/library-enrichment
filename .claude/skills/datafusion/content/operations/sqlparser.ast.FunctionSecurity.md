# `sqlparser::ast::FunctionSecurity`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionSecurity.json).

<a id="op-7b14ab69e6f23586deb94890"></a>
## FunctionSecurity

`enum` · `sqlparser::ast::FunctionSecurity` · sqlparser 0.62.0

```rust
enum FunctionSecurity
```

Source: `src/ast/mod.rs:9979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Security attribute for functions: SECURITY DEFINER or SECURITY INVOKER.

[PostgreSQL](https://www.postgresql.org/docs/current/sql-createfunction.html)

<a id="op-02f9cf5364cf93c0bc1340e3"></a>
## Definer

`variant` · `sqlparser::ast::FunctionSecurity::Definer` · sqlparser 0.62.0

```rust
Definer
```

Source: `src/ast/mod.rs:9981`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Execute the function with the privileges of the user who defined it.

<a id="op-93fe5810716ac2b11f98f47b"></a>
## Invoker

`variant` · `sqlparser::ast::FunctionSecurity::Invoker` · sqlparser 0.62.0

```rust
Invoker
```

Source: `src/ast/mod.rs:9983`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Execute the function with the privileges of the user who invokes it.

<a id="op-cdc9c50f79957fc1fac5e467"></a>
## clone

`function` · `sqlparser::ast::FunctionSecurity::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionSecurity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9976, 17], "end": [9976, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9976`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80980a1df72773169c5edebd"></a>
## cmp

`function` · `sqlparser::ast::FunctionSecurity::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionSecurity) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9976, 51], "end": [9976, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9976`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b362070486505de8c8e4048c"></a>
## deserialize

`function` · `sqlparser::ast::FunctionSecurity::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9977, 49], "end": [9977, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9977`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7796e0aa5109bcc50e74deb5"></a>
## eq

`function` · `sqlparser::ast::FunctionSecurity::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionSecurity) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9976, 24], "end": [9976, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9976`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47a0bfd3b1b13a5b15889958"></a>
## fmt

`function` · `sqlparser::ast::FunctionSecurity::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9976, 10], "end": [9976, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9976`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a358a8a3d2ced2ac6bf5d6a0"></a>
## fmt

`function` · `sqlparser::ast::FunctionSecurity::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9986, 1], "end": [9993, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae2a87828187cdd58b8100e8"></a>
## hash

`function` · `sqlparser::ast::FunctionSecurity::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9976, 56], "end": [9976, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9976`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2b4509b5da1539100d3b777"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionSecurity::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionSecurity) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9976, 35], "end": [9976, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9976`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d914550fb584f5e2b362acdd"></a>
## serialize

`function` · `sqlparser::ast::FunctionSecurity::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9977, 38], "end": [9977, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9977`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61725a035ba6d30307f4cc65"></a>
## visit

`function` · `sqlparser::ast::FunctionSecurity::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9978, 47], "end": [9978, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9978`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfec88c90238f9d2f2f8a51e"></a>
## visit

`function` · `sqlparser::ast::FunctionSecurity::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionSecurity", "path": "FunctionSecurity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9978, 40], "end": [9978, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9978`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
