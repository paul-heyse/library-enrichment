# `sqlparser::ast::SetAssignment`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SetAssignment.json).

<a id="op-711d107966820211f98ba694"></a>
## SetAssignment

`struct` · `sqlparser::ast::SetAssignment` · sqlparser 0.62.0

```rust
struct SetAssignment
```

Source: `src/ast/mod.rs:6462`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Assignment for a `SET` statement (name [=|TO] value)

<a id="op-a01db40a66912fcaf6f48c13"></a>
## clone

`function` · `sqlparser::ast::SetAssignment::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetAssignment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6459, 17], "end": [6459, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f494220643c4dc0392383346"></a>
## cmp

`function` · `sqlparser::ast::SetAssignment::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetAssignment) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6459, 51], "end": [6459, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b24f7ec8c6f6af594e97b8e"></a>
## deserialize

`function` · `sqlparser::ast::SetAssignment::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6460, 49], "end": [6460, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-862623e763ab194ecf6986fa"></a>
## eq

`function` · `sqlparser::ast::SetAssignment::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetAssignment) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6459, 24], "end": [6459, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74a8857bed6ad59e6848b51e"></a>
## fmt

`function` · `sqlparser::ast::SetAssignment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6471, 1], "end": [6481, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0e386ae2b1bf80c475df29a"></a>
## fmt

`function` · `sqlparser::ast::SetAssignment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6459, 10], "end": [6459, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d304583fa8679a886647491f"></a>
## hash

`function` · `sqlparser::ast::SetAssignment::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6459, 56], "end": [6459, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e5771012860d47d4c8eeac0"></a>
## name

`struct_field` · `sqlparser::ast::SetAssignment::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:6466`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Assignment target name.

<a id="op-3cfbad82cd5a1cc02f5c4e93"></a>
## partial_cmp

`function` · `sqlparser::ast::SetAssignment::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetAssignment) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6459, 35], "end": [6459, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-972136df2fe247a10d2eb979"></a>
## scope

`struct_field` · `sqlparser::ast::SetAssignment::scope` · sqlparser 0.62.0

```rust
scope: Option<ContextModifier>
```

Source: `src/ast/mod.rs:6464`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional context scope (e.g., SESSION or LOCAL).

<a id="op-2244121d414dbec1c65c29b6"></a>
## serialize

`function` · `sqlparser::ast::SetAssignment::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6460, 38], "end": [6460, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63a3bff8e2fe837d1595fd4a"></a>
## value

`struct_field` · `sqlparser::ast::SetAssignment::value` · sqlparser 0.62.0

```rust
value: Expr
```

Source: `src/ast/mod.rs:6468`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Assigned expression value.

<a id="op-3469e399b327b3b2242df048"></a>
## visit

`function` · `sqlparser::ast::SetAssignment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6461, 40], "end": [6461, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2785d6921485ce4d6ca7f3f"></a>
## visit

`function` · `sqlparser::ast::SetAssignment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetAssignment", "path": "SetAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6461, 47], "end": [6461, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
