# `sqlparser::ast::query::Fetch`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Fetch.json).

<a id="op-6dce35e512b44f5e1d9f43eb"></a>
## Fetch

`struct` · `sqlparser::ast::query::Fetch` · sqlparser 0.62.0

```rust
struct Fetch
```

Source: `src/ast/query.rs:3487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FETCH` clause options.

<a id="op-6e7cd5f92d7392fd7cc141a3"></a>
## clone

`function` · `sqlparser::ast::query::Fetch::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Fetch
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3483, 17], "end": [3483, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d103b630594530995cad2810"></a>
## cmp

`function` · `sqlparser::ast::query::Fetch::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Fetch) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3483, 51], "end": [3483, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23a9ee70da0ec3214aae5b55"></a>
## deserialize

`function` · `sqlparser::ast::query::Fetch::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3484, 49], "end": [3484, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3484`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a35fb12ef1b91d9c3e40a1ae"></a>
## eq

`function` · `sqlparser::ast::query::Fetch::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Fetch) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3483, 24], "end": [3483, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b68cac9c00594d7987bde71"></a>
## fmt

`function` · `sqlparser::ast::query::Fetch::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3496, 1], "end": [3506, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cead08d7f76b413e08f7b093"></a>
## fmt

`function` · `sqlparser::ast::query::Fetch::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3483, 10], "end": [3483, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2eb319d7e4612f155168270"></a>
## hash

`function` · `sqlparser::ast::query::Fetch::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3483, 56], "end": [3483, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc205aa0c9675c3b775fc988"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Fetch::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Fetch) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3483, 35], "end": [3483, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c99bb9e4f2b47101d431420"></a>
## percent

`struct_field` · `sqlparser::ast::query::Fetch::percent` · sqlparser 0.62.0

```rust
percent: bool
```

Source: `src/ast/query.rs:3491`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PERCENT` modifier is present.

<a id="op-1e2e8791035379b97bc92150"></a>
## quantity

`struct_field` · `sqlparser::ast::query::Fetch::quantity` · sqlparser 0.62.0

```rust
quantity: Option<Expr>
```

Source: `src/ast/query.rs:3493`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional quantity expression (e.g. `FETCH FIRST 10 ROWS`).

<a id="op-d86b234ccb5e857585e67778"></a>
## serialize

`function` · `sqlparser::ast::query::Fetch::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3484, 38], "end": [3484, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3484`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c40766e07fb89c69060202af"></a>
## span

`function` · `sqlparser::ast::query::Fetch::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "super::Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [181, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b17f3a2b05a76758c35c43e5"></a>
## visit

`function` · `sqlparser::ast::query::Fetch::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3485, 40], "end": [3485, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3485`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea77ff17e0d4bbf871d5f6d0"></a>
## visit

`function` · `sqlparser::ast::query::Fetch::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Fetch", "path": "Fetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3485, 47], "end": [3485, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3485`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8daef3d630158a3a64019c09"></a>
## with_ties

`struct_field` · `sqlparser::ast::query::Fetch::with_ties` · sqlparser 0.62.0

```rust
with_ties: bool
```

Source: `src/ast/query.rs:3489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WITH TIES` option is present.
