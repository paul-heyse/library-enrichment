# `sqlparser::ast::AssignmentTarget`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AssignmentTarget.json).

<a id="op-e81226c0d36691f933fb6f79"></a>
## AssignmentTarget

`enum` · `sqlparser::ast::AssignmentTarget` · sqlparser 0.62.0

```rust
enum AssignmentTarget
```

Source: `src/ast/mod.rs:7830`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left-hand side of an assignment in an UPDATE statement,
e.g. `foo` in `foo = 5` (ColumnName assignment) or
`(a, b)` in `(a, b) = (1, 2)` (Tuple assignment).

<a id="op-c1b2d0657a7cdb77f2f5a6df"></a>
## ColumnName

`variant` · `sqlparser::ast::AssignmentTarget::ColumnName` · sqlparser 0.62.0

```rust
ColumnName
```

Source: `src/ast/mod.rs:7832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single column

<a id="op-f37ad1410b510b0ae45e91a9"></a>
## Tuple

`variant` · `sqlparser::ast::AssignmentTarget::Tuple` · sqlparser 0.62.0

```rust
Tuple
```

Source: `src/ast/mod.rs:7834`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A tuple of columns

<a id="op-62a86c5feed4486a5dc9dd8d"></a>
## clone

`function` · `sqlparser::ast::AssignmentTarget::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AssignmentTarget
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7827, 17], "end": [7827, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c859d08fc4bb088b46708e1"></a>
## cmp

`function` · `sqlparser::ast::AssignmentTarget::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AssignmentTarget) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7827, 51], "end": [7827, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-901ddec99e0486a5c5270a7a"></a>
## deserialize

`function` · `sqlparser::ast::AssignmentTarget::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7828, 49], "end": [7828, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7828`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d86991064b476b5c5a66dd6b"></a>
## eq

`function` · `sqlparser::ast::AssignmentTarget::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AssignmentTarget) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7827, 24], "end": [7827, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d0115368d43ab8db5d2cf32"></a>
## fmt

`function` · `sqlparser::ast::AssignmentTarget::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7827, 10], "end": [7827, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdbb248ccf0f4e35edff1ff5"></a>
## fmt

`function` · `sqlparser::ast::AssignmentTarget::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7837, 1], "end": [7844, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7838`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b35259259544e355f71110ab"></a>
## hash

`function` · `sqlparser::ast::AssignmentTarget::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7827, 56], "end": [7827, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f0df5280afc139ed94f2705"></a>
## partial_cmp

`function` · `sqlparser::ast::AssignmentTarget::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AssignmentTarget) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7827, 35], "end": [7827, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faf092ad9618a9992d961b2f"></a>
## serialize

`function` · `sqlparser::ast::AssignmentTarget::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7828, 38], "end": [7828, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7828`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46a98b8dd235f6eff097c614"></a>
## span

`function` · `sqlparser::ast::AssignmentTarget::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "super::AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1441, 1], "end": [1448, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71d083ac9500aba184c8b05e"></a>
## visit

`function` · `sqlparser::ast::AssignmentTarget::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7829, 47], "end": [7829, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7829`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a58e1175d78a6ac9b1aa8317"></a>
## visit

`function` · `sqlparser::ast::AssignmentTarget::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AssignmentTarget", "path": "AssignmentTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7829, 40], "end": [7829, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7829`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
