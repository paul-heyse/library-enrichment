# `sqlparser::ast::ContactEntry`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ContactEntry.json).

<a id="op-7df27cb4ea6ba981884e314d"></a>
## ContactEntry

`struct` · `sqlparser::ast::ContactEntry` · sqlparser 0.62.0

```rust
struct ContactEntry
```

Source: `src/ast/mod.rs:10655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `WITH CONTACT ( purpose = contact [ , purpose = contact ...] )`

<https://docs.snowflake.com/en/sql-reference/sql/create-database>

<a id="op-b8e79c7119ef2f6f53dc6501"></a>
## clone

`function` · `sqlparser::ast::ContactEntry::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ContactEntry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10652, 17], "end": [10652, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2342e2bd1532c574c749f78"></a>
## cmp

`function` · `sqlparser::ast::ContactEntry::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ContactEntry) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10652, 51], "end": [10652, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-970ded340ff2eb78222e02d3"></a>
## contact

`struct_field` · `sqlparser::ast::ContactEntry::contact` · sqlparser 0.62.0

```rust
contact: String
```

Source: `src/ast/mod.rs:10659`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The contact information associated with the purpose.

<a id="op-aaba0401bcd4192b5062b91b"></a>
## deserialize

`function` · `sqlparser::ast::ContactEntry::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10653, 49], "end": [10653, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10653`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-504c476d437d655414a0b7bf"></a>
## eq

`function` · `sqlparser::ast::ContactEntry::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ContactEntry) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10652, 24], "end": [10652, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0be0fc86a5daa465a97974ce"></a>
## fmt

`function` · `sqlparser::ast::ContactEntry::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10652, 10], "end": [10652, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfda375fbf1038641d475c5f"></a>
## fmt

`function` · `sqlparser::ast::ContactEntry::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10662, 1], "end": [10666, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1fe8729212f0f7f3ad6e14e"></a>
## hash

`function` · `sqlparser::ast::ContactEntry::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10652, 56], "end": [10652, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30effdcfabf6b9f3a78c21da"></a>
## partial_cmp

`function` · `sqlparser::ast::ContactEntry::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ContactEntry) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10652, 35], "end": [10652, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b409cfdeb20d8bc3b6f70952"></a>
## purpose

`struct_field` · `sqlparser::ast::ContactEntry::purpose` · sqlparser 0.62.0

```rust
purpose: String
```

Source: `src/ast/mod.rs:10657`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The purpose label for the contact entry.

<a id="op-cd247ffc870d3581fe2617fe"></a>
## serialize

`function` · `sqlparser::ast::ContactEntry::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10653, 38], "end": [10653, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10653`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63e9be0ff259f3387ea159c8"></a>
## visit

`function` · `sqlparser::ast::ContactEntry::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10654, 40], "end": [10654, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf947f7714d866f05edab0be"></a>
## visit

`function` · `sqlparser::ast::ContactEntry::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContactEntry", "path": "ContactEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10654, 47], "end": [10654, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
