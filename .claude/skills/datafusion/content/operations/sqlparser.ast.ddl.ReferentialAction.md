# `sqlparser::ast::ddl::ReferentialAction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ReferentialAction.json).

<a id="op-f87077d6815e26dc925cd078"></a>
## ReferentialAction

`enum` · `sqlparser::ast::ddl::ReferentialAction` · sqlparser 0.62.0

```rust
enum ReferentialAction
```

Source: `src/ast/ddl.rs:2313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<referential_action> =
{ RESTRICT | CASCADE | SET NULL | NO ACTION | SET DEFAULT }`

Used in foreign key constraints in `ON UPDATE` and `ON DELETE` options.

<a id="op-420911625cfa4ad0c8f4cd6a"></a>
## Cascade

`variant` · `sqlparser::ast::ddl::ReferentialAction::Cascade` · sqlparser 0.62.0

```rust
Cascade
```

Source: `src/ast/ddl.rs:2317`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CASCADE` - propagate the action to referencing rows.

<a id="op-b23fd5a5e060ed37b549196e"></a>
## NoAction

`variant` · `sqlparser::ast::ddl::ReferentialAction::NoAction` · sqlparser 0.62.0

```rust
NoAction
```

Source: `src/ast/ddl.rs:2321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NO ACTION` - no action at the time; may be deferred.

<a id="op-34a317308a7a6fe86f24135f"></a>
## Restrict

`variant` · `sqlparser::ast::ddl::ReferentialAction::Restrict` · sqlparser 0.62.0

```rust
Restrict
```

Source: `src/ast/ddl.rs:2315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RESTRICT` - disallow action if it would break referential integrity.

<a id="op-296e10001b481b053ef6623e"></a>
## SetDefault

`variant` · `sqlparser::ast::ddl::ReferentialAction::SetDefault` · sqlparser 0.62.0

```rust
SetDefault
```

Source: `src/ast/ddl.rs:2323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET DEFAULT` - set referencing columns to their default values.

<a id="op-b7d101e30ca59fc34fce7e5b"></a>
## SetNull

`variant` · `sqlparser::ast::ddl::ReferentialAction::SetNull` · sqlparser 0.62.0

```rust
SetNull
```

Source: `src/ast/ddl.rs:2319`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET NULL` - set referencing columns to NULL.

<a id="op-70f7ccb428c1ac98e38eb586"></a>
## clone

`function` · `sqlparser::ast::ddl::ReferentialAction::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ReferentialAction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2310, 23], "end": [2310, 28], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0598a35f605ae38d13e660c4"></a>
## cmp

`function` · `sqlparser::ast::ddl::ReferentialAction::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ReferentialAction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2310, 57], "end": [2310, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-384227d1742da7fb29f86898"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ReferentialAction::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2311, 49], "end": [2311, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2311`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ffcdc0097da971758283d14"></a>
## eq

`function` · `sqlparser::ast::ddl::ReferentialAction::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ReferentialAction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2310, 30], "end": [2310, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b1696d94d322255a2cdb462"></a>
## fmt

`function` · `sqlparser::ast::ddl::ReferentialAction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2310, 10], "end": [2310, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5b0fdae714c33f38c09f6f8"></a>
## fmt

`function` · `sqlparser::ast::ddl::ReferentialAction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2326, 1], "end": [2336, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2327`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7098dbe45646052236dcd856"></a>
## hash

`function` · `sqlparser::ast::ddl::ReferentialAction::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2310, 62], "end": [2310, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06c62283aef19fcc1c398579"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ReferentialAction::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ReferentialAction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2310, 41], "end": [2310, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e62d21725b5612fab182a81f"></a>
## serialize

`function` · `sqlparser::ast::ddl::ReferentialAction::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2311, 38], "end": [2311, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2311`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a8e7ca200687aec1a234014"></a>
## span

`function` · `sqlparser::ast::ddl::ReferentialAction::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "super::ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [848, 1], "end": [852, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:849`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8d5b5bb6f3288129c889193"></a>
## visit

`function` · `sqlparser::ast::ddl::ReferentialAction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2312, 47], "end": [2312, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2312`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea4342ecd039c31e25a4de97"></a>
## visit

`function` · `sqlparser::ast::ddl::ReferentialAction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReferentialAction", "path": "ReferentialAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2312, 40], "end": [2312, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2312`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
