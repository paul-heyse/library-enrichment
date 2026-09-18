# `sqlparser::ast::query::SelectInto`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SelectInto.json).

<a id="op-7056ad72bc1a38d69f4ef7e8"></a>
## SelectInto

`struct` · `sqlparser::ast::query::SelectInto` · sqlparser 0.62.0

```rust
struct SelectInto
```

Source: `src/ast/query.rs:3686`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SELECT INTO` clause options.

<a id="op-4bcfb47ab1cfabec1978de4e"></a>
## clone

`function` · `sqlparser::ast::query::SelectInto::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SelectInto
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3682, 17], "end": [3682, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e51bac1f28f26beb8ac3b587"></a>
## cmp

`function` · `sqlparser::ast::query::SelectInto::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SelectInto) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3682, 51], "end": [3682, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dd05b60525b4cfabfe03d66"></a>
## deserialize

`function` · `sqlparser::ast::query::SelectInto::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3683, 49], "end": [3683, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3683`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8714209be8a2d4b84357f33b"></a>
## eq

`function` · `sqlparser::ast::query::SelectInto::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SelectInto) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3682, 24], "end": [3682, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ca568d81182d047637ae70e"></a>
## fmt

`function` · `sqlparser::ast::query::SelectInto::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3697, 1], "end": [3705, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3698`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b3d079f91e7660ebab31ea6"></a>
## fmt

`function` · `sqlparser::ast::query::SelectInto::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3682, 10], "end": [3682, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd9f64d50593f2523b1b0da4"></a>
## hash

`function` · `sqlparser::ast::query::SelectInto::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3682, 56], "end": [3682, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fe811fa06fcc4c72dbbb9e7"></a>
## name

`struct_field` · `sqlparser::ast::query::SelectInto::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/query.rs:3694`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the target table.

<a id="op-6ad796c278dba7fa2b78a0c3"></a>
## partial_cmp

`function` · `sqlparser::ast::query::SelectInto::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SelectInto) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3682, 35], "end": [3682, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f1138998df77fb73037f8e8"></a>
## serialize

`function` · `sqlparser::ast::query::SelectInto::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3683, 38], "end": [3683, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3683`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d5d6c4885fc724fb7184427"></a>
## span

`function` · `sqlparser::ast::query::SelectInto::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "super::SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2382, 1], "end": [2393, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2383`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0253229573f6c33c44573d78"></a>
## table

`struct_field` · `sqlparser::ast::query::SelectInto::table` · sqlparser 0.62.0

```rust
table: bool
```

Source: `src/ast/query.rs:3692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TABLE` keyword present.

<a id="op-3fedbf2638127e2f5db7e5d8"></a>
## temporary

`struct_field` · `sqlparser::ast::query::SelectInto::temporary` · sqlparser 0.62.0

```rust
temporary: bool
```

Source: `src/ast/query.rs:3688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TEMPORARY` modifier.

<a id="op-0f0d91b2871384c3556d16e8"></a>
## unlogged

`struct_field` · `sqlparser::ast::query::SelectInto::unlogged` · sqlparser 0.62.0

```rust
unlogged: bool
```

Source: `src/ast/query.rs:3690`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`UNLOGGED` modifier.

<a id="op-0684ae9dbe9f93164fe88ddb"></a>
## visit

`function` · `sqlparser::ast::query::SelectInto::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3684, 40], "end": [3684, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3684`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-381f0794f890b870c859a788"></a>
## visit

`function` · `sqlparser::ast::query::SelectInto::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectInto", "path": "SelectInto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3684, 47], "end": [3684, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3684`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
