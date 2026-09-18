# `datafusion_sql::unparser::ast::UnnestRelationBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.ast.UnnestRelationBuilder.json).

<a id="op-752b28211a9b78f03c1cf45b"></a>
## UnnestRelationBuilder

`struct` · `datafusion_sql::unparser::ast::UnnestRelationBuilder` · datafusion-sql 55.1.0

```rust
struct UnnestRelationBuilder
```

Source: `src/unparser/ast.rs:710`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77f6c1b3a709e531224a7be8"></a>
## alias

`struct_field` · `datafusion_sql::unparser::ast::UnnestRelationBuilder::alias` · datafusion-sql 55.1.0

```rust
alias: Option<ast::TableAlias>
```

Source: `src/unparser/ast.rs:711`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0644be7aa47814a56694746"></a>
## alias

`function` · `datafusion_sql::unparser::ast::UnnestRelationBuilder::alias` · datafusion-sql 55.1.0

```rust
fn alias(&mut self, value: Option<ast::TableAlias>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UnnestRelationBuilder", "path": "UnnestRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 1], "end": [762, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:719`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50fe48a34518339d055401eb"></a>
## array_exprs

`function` · `datafusion_sql::unparser::ast::UnnestRelationBuilder::array_exprs` · datafusion-sql 55.1.0

```rust
fn array_exprs(&mut self, value: Vec<ast::Expr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UnnestRelationBuilder", "path": "UnnestRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 1], "end": [762, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e62cc879943fa794d74f768"></a>
## array_exprs

`struct_field` · `datafusion_sql::unparser::ast::UnnestRelationBuilder::array_exprs` · datafusion-sql 55.1.0

```rust
array_exprs: Vec<ast::Expr>
```

Source: `src/unparser/ast.rs:712`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17b4f1d792e8bab463790be7"></a>
## build

`function` · `datafusion_sql::unparser::ast::UnnestRelationBuilder::build` · datafusion-sql 55.1.0

```rust
fn build(&self) -> Result<ast::TableFactor, BuilderError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UnnestRelationBuilder", "path": "UnnestRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 1], "end": [762, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:743`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed6968cf7472cd6857b00ed6"></a>
## clone

`function` · `datafusion_sql::unparser::ast::UnnestRelationBuilder::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> UnnestRelationBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UnnestRelationBuilder", "path": "UnnestRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 10], "end": [709, 15], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/ast.rs:709`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2281ed06aba2e264d293fc6"></a>
## default

`function` · `datafusion_sql::unparser::ast::UnnestRelationBuilder::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UnnestRelationBuilder", "path": "UnnestRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [764, 1], "end": [768, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/ast.rs:765`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7984a52c1fde9a41b39b0b5"></a>
## with_offset

`function` · `datafusion_sql::unparser::ast::UnnestRelationBuilder::with_offset` · datafusion-sql 55.1.0

```rust
fn with_offset(&mut self, value: bool) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UnnestRelationBuilder", "path": "UnnestRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 1], "end": [762, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:728`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2aeee2595872233a94f1a848"></a>
## with_offset_alias

`function` · `datafusion_sql::unparser::ast::UnnestRelationBuilder::with_offset_alias` · datafusion-sql 55.1.0

```rust
fn with_offset_alias(&mut self, value: Option<ast::Ident>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UnnestRelationBuilder", "path": "UnnestRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 1], "end": [762, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:733`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd1bb768d0adad82d2545cdc"></a>
## with_ordinality

`function` · `datafusion_sql::unparser::ast::UnnestRelationBuilder::with_ordinality` · datafusion-sql 55.1.0

```rust
fn with_ordinality(&mut self, value: bool) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UnnestRelationBuilder", "path": "UnnestRelationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [718, 1], "end": [762, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:738`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
