# `datafusion_sql::unparser::ast::TableRelationBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.ast.TableRelationBuilder.json).

<a id="op-7d582c9fcc50fe55adb58e7e"></a>
## TableRelationBuilder

`struct` · `datafusion_sql::unparser::ast::TableRelationBuilder` · datafusion-sql 55.1.0

```rust
struct TableRelationBuilder
```

Source: `src/unparser/ast.rs:583`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ca2632c16912520208268b1"></a>
## alias

`function` · `datafusion_sql::unparser::ast::TableRelationBuilder::alias` · datafusion-sql 55.1.0

```rust
fn alias(&mut self, value: Option<ast::TableAlias>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableRelationBuilder", "path": "TableRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 1], "end": [653, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:598`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21e7ade39c799de649519b22"></a>
## args

`function` · `datafusion_sql::unparser::ast::TableRelationBuilder::args` · datafusion-sql 55.1.0

```rust
fn args(&mut self, value: Option<Vec<ast::FunctionArg>>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableRelationBuilder", "path": "TableRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 1], "end": [653, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:602`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eee0b5b2df6134771aee8b52"></a>
## build

`function` · `datafusion_sql::unparser::ast::TableRelationBuilder::build` · datafusion-sql 55.1.0

```rust
fn build(&self) -> Result<ast::TableFactor, BuilderError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableRelationBuilder", "path": "TableRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 1], "end": [653, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:622`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a461276776b144e22bee73ce"></a>
## clone

`function` · `datafusion_sql::unparser::ast::TableRelationBuilder::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> TableRelationBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableRelationBuilder", "path": "TableRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [582, 10], "end": [582, 15], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/ast.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4376b5a24d9a352f3cb3225e"></a>
## default

`function` · `datafusion_sql::unparser::ast::TableRelationBuilder::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableRelationBuilder", "path": "TableRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [658, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/ast.rs:655`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-279d79698b755abb8cb39132"></a>
## index_hints

`function` · `datafusion_sql::unparser::ast::TableRelationBuilder::index_hints` · datafusion-sql 55.1.0

```rust
fn index_hints(&mut self, value: Vec<ast::TableIndexHints>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableRelationBuilder", "path": "TableRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 1], "end": [653, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:618`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1d70c1abff99f72125376e0"></a>
## name

`function` · `datafusion_sql::unparser::ast::TableRelationBuilder::name` · datafusion-sql 55.1.0

```rust
fn name(&mut self, value: ast::ObjectName) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableRelationBuilder", "path": "TableRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 1], "end": [653, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:594`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-245186594afe591d45b25608"></a>
## partitions

`function` · `datafusion_sql::unparser::ast::TableRelationBuilder::partitions` · datafusion-sql 55.1.0

```rust
fn partitions(&mut self, value: Vec<ast::Ident>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableRelationBuilder", "path": "TableRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 1], "end": [653, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:614`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afbcaf57957a95490e2e9c09"></a>
## version

`function` · `datafusion_sql::unparser::ast::TableRelationBuilder::version` · datafusion-sql 55.1.0

```rust
fn version(&mut self, value: Option<ast::TableVersion>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableRelationBuilder", "path": "TableRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 1], "end": [653, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:610`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcae7a5db0144bc505028629"></a>
## with_hints

`function` · `datafusion_sql::unparser::ast::TableRelationBuilder::with_hints` · datafusion-sql 55.1.0

```rust
fn with_hints(&mut self, value: Vec<ast::Expr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::TableRelationBuilder", "path": "TableRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 1], "end": [653, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:606`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
