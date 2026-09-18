# `sqlparser::ast::ShowStatementFilter`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ShowStatementFilter.json).

<a id="op-936f17aa3e6b05ff7deff53a"></a>
## ShowStatementFilter

`enum` · `sqlparser::ast::ShowStatementFilter` · sqlparser 0.62.0

```rust
enum ShowStatementFilter
```

Source: `src/ast/mod.rs:9145`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Filter forms usable in SHOW statements.

<a id="op-44645c1890572633c0a3e680"></a>
## ILike

`variant` · `sqlparser::ast::ShowStatementFilter::ILike` · sqlparser 0.62.0

```rust
ILike
```

Source: `src/ast/mod.rs:9149`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Filter using ILIKE pattern.

<a id="op-675388ed83fbfed66154f5ee"></a>
## Like

`variant` · `sqlparser::ast::ShowStatementFilter::Like` · sqlparser 0.62.0

```rust
Like
```

Source: `src/ast/mod.rs:9147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Filter using LIKE pattern.

<a id="op-ea42bd2bc5a3e5949fa32e9e"></a>
## NoKeyword

`variant` · `sqlparser::ast::ShowStatementFilter::NoKeyword` · sqlparser 0.62.0

```rust
NoKeyword
```

Source: `src/ast/mod.rs:9153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Filter provided without a keyword (raw string).

<a id="op-760142eaa31615738a2bbf55"></a>
## Where

`variant` · `sqlparser::ast::ShowStatementFilter::Where` · sqlparser 0.62.0

```rust
Where
```

Source: `src/ast/mod.rs:9151`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Filter using a WHERE expression.

<a id="op-025ae7e0b3e9e45142d8e524"></a>
## clone

`function` · `sqlparser::ast::ShowStatementFilter::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ShowStatementFilter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9141, 17], "end": [9141, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20c32f994710c97c99873822"></a>
## cmp

`function` · `sqlparser::ast::ShowStatementFilter::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ShowStatementFilter) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9141, 51], "end": [9141, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb7f5e22679388da3861a127"></a>
## deserialize

`function` · `sqlparser::ast::ShowStatementFilter::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9142, 49], "end": [9142, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c07cfafee8b2b2a7c17e782"></a>
## eq

`function` · `sqlparser::ast::ShowStatementFilter::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ShowStatementFilter) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9141, 24], "end": [9141, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3abbe75e8a48abe9fe56f16c"></a>
## fmt

`function` · `sqlparser::ast::ShowStatementFilter::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9156, 1], "end": [9166, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4382f679e5d95db6efdb7b2b"></a>
## fmt

`function` · `sqlparser::ast::ShowStatementFilter::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9141, 10], "end": [9141, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b110cf4846bab41491bba296"></a>
## hash

`function` · `sqlparser::ast::ShowStatementFilter::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9141, 56], "end": [9141, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d9f8e112a46ac7b4746dfb7"></a>
## partial_cmp

`function` · `sqlparser::ast::ShowStatementFilter::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ShowStatementFilter) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9141, 35], "end": [9141, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0381720599c6b2879a42a540"></a>
## serialize

`function` · `sqlparser::ast::ShowStatementFilter::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9142, 38], "end": [9142, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7e4d5cd6c46d65f0ec9b81c"></a>
## visit

`function` · `sqlparser::ast::ShowStatementFilter::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9143, 47], "end": [9143, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9143`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce69b203d8d7132a2ac71a51"></a>
## visit

`function` · `sqlparser::ast::ShowStatementFilter::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementFilter", "path": "ShowStatementFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9143, 40], "end": [9143, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9143`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
