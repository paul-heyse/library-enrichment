# `sqlparser::ast::Privileges`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Privileges.json).

<a id="op-e776efc571ca81795b8786f7"></a>
## Privileges

`enum` · `sqlparser::ast::Privileges` · sqlparser 0.62.0

```rust
enum Privileges
```

Source: `src/ast/mod.rs:6820`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Privileges granted in a GRANT statement or revoked in a REVOKE statement.

<a id="op-f9c63a7b2368b10d1fb289a7"></a>
## Actions

`variant` · `sqlparser::ast::Privileges::Actions` · sqlparser 0.62.0

```rust
Actions
```

Source: `src/ast/mod.rs:6827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specific privileges (e.g. `SELECT`, `INSERT`)

<a id="op-363d3b0f63dd680e0d824c3d"></a>
## All

`variant` · `sqlparser::ast::Privileges::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/mod.rs:6822`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

All privileges applicable to the object type

<a id="op-7cc4cf449fb45424ee9fdc72"></a>
## clone

`function` · `sqlparser::ast::Privileges::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Privileges
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6817, 17], "end": [6817, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f2e4e5e482c6a526d8b3f3b"></a>
## cmp

`function` · `sqlparser::ast::Privileges::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Privileges) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6817, 51], "end": [6817, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0a4c904f39be936bd85fab8"></a>
## deserialize

`function` · `sqlparser::ast::Privileges::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6818, 49], "end": [6818, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6818`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9c796baafacafaae12209bf"></a>
## eq

`function` · `sqlparser::ast::Privileges::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Privileges) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6817, 24], "end": [6817, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bf731bb4bb3111cd82f8fc6"></a>
## fmt

`function` · `sqlparser::ast::Privileges::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6830, 1], "end": [6851, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f8a54fdd5227be43de27055"></a>
## fmt

`function` · `sqlparser::ast::Privileges::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6817, 10], "end": [6817, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be0c30d1b96d94a89d77cd7b"></a>
## hash

`function` · `sqlparser::ast::Privileges::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6817, 56], "end": [6817, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ac633146b6b9050b18a6e48"></a>
## partial_cmp

`function` · `sqlparser::ast::Privileges::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Privileges) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6817, 35], "end": [6817, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d30c208bc891ef318201bcd3"></a>
## serialize

`function` · `sqlparser::ast::Privileges::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6818, 38], "end": [6818, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6818`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-736b9b8c85c5840a9e11ccec"></a>
## visit

`function` · `sqlparser::ast::Privileges::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6819, 40], "end": [6819, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a201c9cd9d3bd13149aae0a7"></a>
## visit

`function` · `sqlparser::ast::Privileges::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Privileges", "path": "Privileges"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6819, 47], "end": [6819, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
