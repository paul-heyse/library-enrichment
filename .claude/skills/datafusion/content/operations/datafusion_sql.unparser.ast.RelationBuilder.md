# `datafusion_sql::unparser::ast::RelationBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.ast.RelationBuilder.json).

<a id="op-db81c919d030c0f6ec1bee03"></a>
## RelationBuilder

`struct` · `datafusion_sql::unparser::ast::RelationBuilder` · datafusion-sql 55.1.0

```rust
struct RelationBuilder
```

Source: `src/unparser/ast.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab0bbdccadf6942963331737"></a>
## alias

`function` · `datafusion_sql::unparser::ast::RelationBuilder::alias` · datafusion-sql 55.1.0

```rust
fn alias(&mut self, value: Option<ast::TableAlias>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [575, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92c7b66ef8ae701788750ed4"></a>
## build

`function` · `datafusion_sql::unparser::ast::RelationBuilder::build` · datafusion-sql 55.1.0

```rust
fn build(&self) -> Result<Option<ast::TableFactor>, BuilderError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [575, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:554`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ee944e54052625a6d0830c4"></a>
## clone

`function` · `datafusion_sql::unparser::ast::RelationBuilder::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> RelationBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 10], "end": [477, 15], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/ast.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e89a4c52f24c8aa3035a3ba"></a>
## default

`function` · `datafusion_sql::unparser::ast::RelationBuilder::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [576, 1], "end": [580, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/ast.rs:577`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70cf89129d83cab1f614fedd"></a>
## derived

`function` · `datafusion_sql::unparser::ast::RelationBuilder::derived` · datafusion-sql 55.1.0

```rust
fn derived(&mut self, value: DerivedRelationBuilder) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [575, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:503`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-275c6276c7c347272a504cbc"></a>
## empty

`function` · `datafusion_sql::unparser::ast::RelationBuilder::empty` · datafusion-sql 55.1.0

```rust
fn empty(&mut self) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [575, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:527`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d62b28f569e1c54f294918a7"></a>
## flatten

`function` · `datafusion_sql::unparser::ast::RelationBuilder::flatten` · datafusion-sql 55.1.0

```rust
fn flatten(&mut self, value: FlattenRelationBuilder) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [575, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:522`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b86b6b4394f34504eca7fe2"></a>
## has_relation

`function` · `datafusion_sql::unparser::ast::RelationBuilder::has_relation` · datafusion-sql 55.1.0

```rust
fn has_relation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [575, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:496`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-956e556c0427e3f11f8f559d"></a>
## nested_join

`function` · `datafusion_sql::unparser::ast::RelationBuilder::nested_join` · datafusion-sql 55.1.0

```rust
fn nested_join(&mut self, value: ast::TableWithJoins, alias: Option<ast::TableAlias>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [575, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:508`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-912c858d04c51034048bc48b"></a>
## table

`function` · `datafusion_sql::unparser::ast::RelationBuilder::table` · datafusion-sql 55.1.0

```rust
fn table(&mut self, value: TableRelationBuilder) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [575, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:499`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2ff9c0c68f266b0210fa7da"></a>
## unnest

`function` · `datafusion_sql::unparser::ast::RelationBuilder::unnest` · datafusion-sql 55.1.0

```rust
fn unnest(&mut self, value: UnnestRelationBuilder) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::RelationBuilder", "path": "RelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [575, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:517`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
