# `datafusion_sql::unparser::ast::FlattenRelationBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.ast.FlattenRelationBuilder.json).

<a id="op-293b17eb63c605fbb4305ef5"></a>
## FlattenRelationBuilder

`struct` · `datafusion_sql::unparser::ast::FlattenRelationBuilder` · datafusion-sql 55.1.0

```rust
struct FlattenRelationBuilder
```

Source: `src/unparser/ast.rs:773`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Builds a `LATERAL FLATTEN(INPUT => expr, OUTER => bool)` table factor
for Snowflake-style unnesting.

<a id="op-496317739ce56751898c8bf8"></a>
## alias

`function` · `datafusion_sql::unparser::ast::FlattenRelationBuilder::alias` · datafusion-sql 55.1.0

```rust
fn alias(&mut self, value: Option<ast::TableAlias>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::FlattenRelationBuilder", "path": "FlattenRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [781, 1], "end": [834, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:782`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9361bc43ecb2a0b75c423ed0"></a>
## alias

`struct_field` · `datafusion_sql::unparser::ast::FlattenRelationBuilder::alias` · datafusion-sql 55.1.0

```rust
alias: Option<ast::TableAlias>
```

Source: `src/unparser/ast.rs:774`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8cdc0dae8a374c60f06140a"></a>
## build

`function` · `datafusion_sql::unparser::ast::FlattenRelationBuilder::build` · datafusion-sql 55.1.0

```rust
fn build(&self) -> Result<ast::TableFactor, BuilderError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::FlattenRelationBuilder", "path": "FlattenRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [781, 1], "end": [834, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:797`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a42db0607323bd2be2a51910"></a>
## clone

`function` · `datafusion_sql::unparser::ast::FlattenRelationBuilder::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> FlattenRelationBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::FlattenRelationBuilder", "path": "FlattenRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 10], "end": [772, 15], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/ast.rs:772`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e1fe3b563a069a1c3005756"></a>
## default

`function` · `datafusion_sql::unparser::ast::FlattenRelationBuilder::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::FlattenRelationBuilder", "path": "FlattenRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [836, 1], "end": [840, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/ast.rs:837`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b579704ada15ea33dd7f9b73"></a>
## input_expr

`struct_field` · `datafusion_sql::unparser::ast::FlattenRelationBuilder::input_expr` · datafusion-sql 55.1.0

```rust
input_expr: Option<ast::Expr>
```

Source: `src/unparser/ast.rs:776`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The input expression to flatten (e.g. a column reference).

<a id="op-f5abdda6a813a87da397a740"></a>
## input_expr

`function` · `datafusion_sql::unparser::ast::FlattenRelationBuilder::input_expr` · datafusion-sql 55.1.0

```rust
fn input_expr(&mut self, value: ast::Expr) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::FlattenRelationBuilder", "path": "FlattenRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [781, 1], "end": [834, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:787`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20eae1c23edb0e4d87d9c8b9"></a>
## outer

`function` · `datafusion_sql::unparser::ast::FlattenRelationBuilder::outer` · datafusion-sql 55.1.0

```rust
fn outer(&mut self, value: bool) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::FlattenRelationBuilder", "path": "FlattenRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [781, 1], "end": [834, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:792`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8939845b8c62ffe8361ce5e6"></a>
## outer

`struct_field` · `datafusion_sql::unparser::ast::FlattenRelationBuilder::outer` · datafusion-sql 55.1.0

```rust
outer: bool
```

Source: `src/unparser/ast.rs:778`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether to preserve rows for NULL/empty inputs (Snowflake `OUTER` param).
