# `datafusion_sql::unparser::ast::QueryBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.ast.QueryBuilder.json).

<a id="op-cee9a5eaaf0c47ffe7eddec0"></a>
## QueryBuilder

`struct` · `datafusion_sql::unparser::ast::QueryBuilder` · datafusion-sql 55.1.0

```rust
struct QueryBuilder
```

Source: `src/unparser/ast.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c683526a8ee207df1cefd471"></a>
## body

`function` · `datafusion_sql::unparser::ast::QueryBuilder::body` · datafusion-sql 55.1.0

```rust
fn body(&mut self, value: Box<ast::SetExpr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8d44d9202971dedc5b7f3b5"></a>
## build

`function` · `datafusion_sql::unparser::ast::QueryBuilder::build` · datafusion-sql 55.1.0

```rust
fn build(&self) -> Result<ast::Query, BuilderError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e237446f341987bcaea7dc0c"></a>
## clone

`function` · `datafusion_sql::unparser::ast::QueryBuilder::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> QueryBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/ast.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3aa269c2fc5434756fe4302"></a>
## default

`function` · `datafusion_sql::unparser::ast::QueryBuilder::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [136, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/ast.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b3a868ff465fbe19603e967"></a>
## distinct_union

`function` · `datafusion_sql::unparser::ast::QueryBuilder::distinct_union` · datafusion-sql 55.1.0

```rust
fn distinct_union(&mut self) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e57a652431dd80fe0228744"></a>
## fetch

`function` · `datafusion_sql::unparser::ast::QueryBuilder::fetch` · datafusion-sql 55.1.0

```rust
fn fetch(&mut self, value: Option<ast::Fetch>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd7dfd4bf7d779bd4b461ce7"></a>
## for_clause

`function` · `datafusion_sql::unparser::ast::QueryBuilder::for_clause` · datafusion-sql 55.1.0

```rust
fn for_clause(&mut self, value: Option<ast::ForClause>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79adea97f4e9547bb6bef1d7"></a>
## is_distinct_union

`function` · `datafusion_sql::unparser::ast::QueryBuilder::is_distinct_union` · datafusion-sql 55.1.0

```rust
fn is_distinct_union(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37616b096a368928a429818b"></a>
## limit

`function` · `datafusion_sql::unparser::ast::QueryBuilder::limit` · datafusion-sql 55.1.0

```rust
fn limit(&mut self, value: Option<ast::Expr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e24ea64c03fb2f8ecd406029"></a>
## limit_by

`function` · `datafusion_sql::unparser::ast::QueryBuilder::limit_by` · datafusion-sql 55.1.0

```rust
fn limit_by(&mut self, value: Vec<ast::Expr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76a0297aa8dda4622b8f5165"></a>
## locks

`function` · `datafusion_sql::unparser::ast::QueryBuilder::locks` · datafusion-sql 55.1.0

```rust
fn locks(&mut self, value: Vec<ast::LockClause>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85282e14b13bc7927950be65"></a>
## offset

`function` · `datafusion_sql::unparser::ast::QueryBuilder::offset` · datafusion-sql 55.1.0

```rust
fn offset(&mut self, value: Option<ast::Offset>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd18f795da1199320249e7a5"></a>
## order_by

`function` · `datafusion_sql::unparser::ast::QueryBuilder::order_by` · datafusion-sql 55.1.0

```rust
fn order_by(&mut self, value: OrderByKind) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49f77a59b8eae2b80d5cd7ae"></a>
## take_body

`function` · `datafusion_sql::unparser::ast::QueryBuilder::take_body` · datafusion-sql 55.1.0

```rust
fn take_body(&mut self) -> Option<Box<ast::SetExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eca45af9363ed1f6da8174e8"></a>
## with

`function` · `datafusion_sql::unparser::ast::QueryBuilder::with` · datafusion-sql 55.1.0

```rust
fn with(&mut self, value: Option<ast::With>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::QueryBuilder", "path": "QueryBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [131, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
