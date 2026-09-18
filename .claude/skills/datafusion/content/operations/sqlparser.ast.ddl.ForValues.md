# `sqlparser::ast::ddl::ForValues`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ForValues.json).

<a id="op-ba244136cfe6b6a6f474a9a2"></a>
## ForValues

`enum` · `sqlparser::ast::ddl::ForValues` · sqlparser 0.62.0

```rust
enum ForValues
```

Source: `src/ast/ddl.rs:3394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL partition bound specification for `PARTITION OF`.

Specifies partition bounds for a child partition table.

See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createtable.html)

<a id="op-254b6193018059422f87aa47"></a>
## Default

`variant` · `sqlparser::ast::ddl::ForValues::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/ddl.rs:3412`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DEFAULT`

<a id="op-100700b4cc7dbc8e2e8e7f45"></a>
## From

`variant` · `sqlparser::ast::ddl::ForValues::From` · sqlparser 0.62.0

```rust
From
```

Source: `src/ast/ddl.rs:3398`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR VALUES FROM (expr|MINVALUE|MAXVALUE, ...) TO (expr|MINVALUE|MAXVALUE, ...)`

<a id="op-f82cd28a98e17a152004d20f"></a>
## In

`variant` · `sqlparser::ast::ddl::ForValues::In` · sqlparser 0.62.0

```rust
In
```

Source: `src/ast/ddl.rs:3396`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR VALUES IN (expr, ...)`

<a id="op-41d5f477c1d29fc285c7ccd0"></a>
## With

`variant` · `sqlparser::ast::ddl::ForValues::With` · sqlparser 0.62.0

```rust
With
```

Source: `src/ast/ddl.rs:3405`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR VALUES WITH (MODULUS n, REMAINDER r)`

<a id="op-762460fa5db932bb692de74b"></a>
## clone

`function` · `sqlparser::ast::ddl::ForValues::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ForValues
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3391, 17], "end": [3391, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-266a6344c191c93c0bf741c9"></a>
## cmp

`function` · `sqlparser::ast::ddl::ForValues::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ForValues) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3391, 51], "end": [3391, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce13a759b7c6388967bc77d1"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ForValues::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3392, 49], "end": [3392, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a646e18f152a0d6e943e1d4"></a>
## eq

`function` · `sqlparser::ast::ddl::ForValues::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ForValues) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3391, 24], "end": [3391, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-840228e5303a61c806d43d16"></a>
## fmt

`function` · `sqlparser::ast::ddl::ForValues::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3415, 1], "end": [3438, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92cb38d4cd6082fb68a998d2"></a>
## fmt

`function` · `sqlparser::ast::ddl::ForValues::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3391, 10], "end": [3391, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f45bcef9ee2385cdb9b3b5c5"></a>
## hash

`function` · `sqlparser::ast::ddl::ForValues::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3391, 56], "end": [3391, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83df7f6c6cf8657772ba8f9c"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ForValues::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ForValues) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3391, 35], "end": [3391, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee9c531d05c8d983fb14d1e0"></a>
## serialize

`function` · `sqlparser::ast::ddl::ForValues::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3392, 38], "end": [3392, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa0a2e1d445f7e1b0630623b"></a>
## span

`function` · `sqlparser::ast::ddl::ForValues::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "super::ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [668, 1], "end": [682, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:669`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a01ceda8652f4638ebff216"></a>
## visit

`function` · `sqlparser::ast::ddl::ForValues::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3393, 47], "end": [3393, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3393`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8ab90f8ee3c387e154187a7"></a>
## visit

`function` · `sqlparser::ast::ddl::ForValues::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ForValues", "path": "ForValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3393, 40], "end": [3393, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3393`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
