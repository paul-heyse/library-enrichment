# `datafusion_sql::unparser::ast::TableWithJoinsBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.ast.TableWithJoinsBuilder.json).

<a id="op-1d75508da1df13380fa04a20"></a>
## TableWithJoinsBuilder

`struct` · `datafusion_sql::unparser::ast::TableWithJoinsBuilder` · datafusion-sql 55.1.0

```rust
struct TableWithJoinsBuilder
```

Source: `src/unparser/ast.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98ebdb97efc712b0e8b096db"></a>
## build

`function` · `datafusion_sql::unparser::ast::TableWithJoinsBuilder::build` · datafusion-sql 55.1.0

```rust
fn build(&self) -> Result<Option<ast::TableWithJoins>, BuilderError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableWithJoinsBuilder", "path": "TableWithJoinsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [437, 1], "end": [470, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b98a68322f6a0b1756359c26"></a>
## clone

`function` · `datafusion_sql::unparser::ast::TableWithJoinsBuilder::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> TableWithJoinsBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableWithJoinsBuilder", "path": "TableWithJoinsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [431, 10], "end": [431, 15], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/ast.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44a7a7b0ba67ede36b486ca9"></a>
## default

`function` · `datafusion_sql::unparser::ast::TableWithJoinsBuilder::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableWithJoinsBuilder", "path": "TableWithJoinsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [475, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/ast.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fef069894711b5d9fe06055c"></a>
## joins

`function` · `datafusion_sql::unparser::ast::TableWithJoinsBuilder::joins` · datafusion-sql 55.1.0

```rust
fn joins(&mut self, value: Vec<ast::Join>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableWithJoinsBuilder", "path": "TableWithJoinsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [437, 1], "end": [470, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:443`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed2589f70e1e11d211dd49af"></a>
## push_join

`function` · `datafusion_sql::unparser::ast::TableWithJoinsBuilder::push_join` · datafusion-sql 55.1.0

```rust
fn push_join(&mut self, value: ast::Join) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableWithJoinsBuilder", "path": "TableWithJoinsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [437, 1], "end": [470, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f96abe86a8bbc1613b84d3e0"></a>
## relation

`function` · `datafusion_sql::unparser::ast::TableWithJoinsBuilder::relation` · datafusion-sql 55.1.0

```rust
fn relation(&mut self, value: RelationBuilder) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableWithJoinsBuilder", "path": "TableWithJoinsBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [437, 1], "end": [470, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:438`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
