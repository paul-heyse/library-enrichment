# `sqlparser::ast::query::TableSampleBucket`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableSampleBucket.json).

<a id="op-e73f8cdf4621684306e9e919"></a>
## TableSampleBucket

`struct` · `sqlparser::ast::query::TableSampleBucket` · sqlparser 0.62.0

```rust
struct TableSampleBucket
```

Source: `src/ast/query.rs:1907`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bucket-based sampling clause: `BUCKET <bucket> OUT OF <total> [ON <expr>]`.

<a id="op-a7d4ed2338e3c38adc545762"></a>
## bucket

`struct_field` · `sqlparser::ast::query::TableSampleBucket::bucket` · sqlparser 0.62.0

```rust
bucket: ValueWithSpan
```

Source: `src/ast/query.rs:1909`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The bucket index expression.

<a id="op-751bbd786b67eb661e2c392e"></a>
## clone

`function` · `sqlparser::ast::query::TableSampleBucket::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableSampleBucket
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1903, 17], "end": [1903, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed02f012b7ddc6593ed9c8b6"></a>
## cmp

`function` · `sqlparser::ast::query::TableSampleBucket::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableSampleBucket) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1903, 51], "end": [1903, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edd2d946bf291dfd67828a16"></a>
## deserialize

`function` · `sqlparser::ast::query::TableSampleBucket::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1904, 49], "end": [1904, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1904`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f949845f99735ce424c1f5b3"></a>
## eq

`function` · `sqlparser::ast::query::TableSampleBucket::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableSampleBucket) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1903, 24], "end": [1903, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06dce504a4fd52f8ca1d51fc"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleBucket::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1916, 1], "end": [1924, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1917`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bf51c1635fbe7a1b18e759b"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleBucket::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1903, 10], "end": [1903, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d39d94e490d6d0c77b4517dd"></a>
## hash

`function` · `sqlparser::ast::query::TableSampleBucket::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1903, 56], "end": [1903, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e3d4578d4a20de173c69a66"></a>
## on

`struct_field` · `sqlparser::ast::query::TableSampleBucket::on` · sqlparser 0.62.0

```rust
on: Option<Expr>
```

Source: `src/ast/query.rs:1913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `ON <expr>` specification.

<a id="op-7b4f102e08e2932e0aad49f8"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableSampleBucket::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableSampleBucket) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1903, 35], "end": [1903, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fda59496e44bbc707222997c"></a>
## serialize

`function` · `sqlparser::ast::query::TableSampleBucket::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1904, 38], "end": [1904, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1904`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-324b3207232435014280029d"></a>
## total

`struct_field` · `sqlparser::ast::query::TableSampleBucket::total` · sqlparser 0.62.0

```rust
total: ValueWithSpan
```

Source: `src/ast/query.rs:1911`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The total number of buckets expression.

<a id="op-1b03699f4b34647ceecbdb9e"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleBucket::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1905, 40], "end": [1905, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d25ac6a64a28a1d88c8af9a6"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleBucket::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleBucket", "path": "TableSampleBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1905, 47], "end": [1905, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
