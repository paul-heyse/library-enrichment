# `sqlparser::ast::ddl::Msck`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.Msck.json).

<a id="op-3d91468db61492db8a97ae80"></a>
## Msck

`struct` · `sqlparser::ast::ddl::Msck` · sqlparser 0.62.0

```rust
struct Msck
```

Source: `src/ast/ddl.rs:4259`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `MSCK` statement.

```sql
MSCK [REPAIR] TABLE table_name [ADD|DROP|SYNC PARTITIONS]
```
MSCK (Hive) - MetaStore Check command

<a id="op-d28982d5c44c5a4c95eb9ae1"></a>
## clone

`function` · `sqlparser::ast::ddl::Msck::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Msck
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4256, 17], "end": [4256, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d032d3ff54f09ebe82926b13"></a>
## cmp

`function` · `sqlparser::ast::ddl::Msck::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Msck) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4256, 51], "end": [4256, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a466039097e97b255e487d3e"></a>
## deserialize

`function` · `sqlparser::ast::ddl::Msck::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4257, 49], "end": [4257, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4257`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fed64256e9f34f22e2d2bf6a"></a>
## eq

`function` · `sqlparser::ast::ddl::Msck::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Msck) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4256, 24], "end": [4256, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68812e0020bd488de44b65a7"></a>
## fmt

`function` · `sqlparser::ast::ddl::Msck::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4269, 1], "end": [4282, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea88f0013cd7cbb36c3827e3"></a>
## fmt

`function` · `sqlparser::ast::ddl::Msck::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4256, 10], "end": [4256, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c93cdaec753691f9b43b7a47"></a>
## hash

`function` · `sqlparser::ast::ddl::Msck::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4256, 56], "end": [4256, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55ce71467036ace84d18edbe"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::Msck::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Msck) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4256, 35], "end": [4256, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d03e5ffcad639dbd1c1ddfc"></a>
## partition_action

`struct_field` · `sqlparser::ast::ddl::Msck::partition_action` · sqlparser 0.62.0

```rust
partition_action: Option<super::AddDropSync>
```

Source: `src/ast/ddl.rs:4266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Partition action (ADD, DROP, or SYNC)

<a id="op-53de4c5e2dde96b796a85166"></a>
## repair

`struct_field` · `sqlparser::ast::ddl::Msck::repair` · sqlparser 0.62.0

```rust
repair: bool
```

Source: `src/ast/ddl.rs:4264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to repair the table

<a id="op-b6c824e928947e50ca9b9e1f"></a>
## serialize

`function` · `sqlparser::ast::ddl::Msck::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4257, 38], "end": [4257, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4257`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be8723bd81de8437e86455df"></a>
## span

`function` · `sqlparser::ast::ddl::Msck::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4284, 1], "end": [4288, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:4285`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-380dfc40b34b48fee512d3df"></a>
## table_name

`struct_field` · `sqlparser::ast::ddl::Msck::table_name` · sqlparser 0.62.0

```rust
table_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table name to check

<a id="op-4c56b460deeeccfecb14d034"></a>
## visit

`function` · `sqlparser::ast::ddl::Msck::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4258, 47], "end": [4258, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4258`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5789e7631b0f209d6426eda"></a>
## visit

`function` · `sqlparser::ast::ddl::Msck::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4258, 40], "end": [4258, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4258`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
