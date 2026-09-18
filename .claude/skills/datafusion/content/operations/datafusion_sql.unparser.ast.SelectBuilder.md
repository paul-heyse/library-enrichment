# `datafusion_sql::unparser::ast::SelectBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.ast.SelectBuilder.json).

<a id="op-c700cde82f70eb9033ef7aeb"></a>
## SelectBuilder

`struct` · `datafusion_sql::unparser::ast::SelectBuilder` · datafusion-sql 55.1.0

```rust
struct SelectBuilder
```

Source: `src/unparser/ast.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-998d71fb7aaab48becead3ed"></a>
## add_flatten_table_alias

`function` · `datafusion_sql::unparser::ast::SelectBuilder::add_flatten_table_alias` · datafusion-sql 55.1.0

```rust
fn add_flatten_table_alias(&mut self, alias: String)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Register a table alias as pointing to a LATERAL FLATTEN relation.

<a id="op-18c15a7190621242166ed527"></a>
## already_projected

`function` · `datafusion_sql::unparser::ast::SelectBuilder::already_projected` · datafusion-sql 55.1.0

```rust
fn already_projected(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Returns true if a projection has been explicitly set via `projection()`.

This method is used to determine whether the SELECT clause has already been
defined, which helps avoid creating duplicate projection nodes during query
unparsing. It returns `true` for both empty and non-empty projections.

# Returns

- `true` if `projection()` has been called (regardless of whether it was empty or not)
- `false` if no projection has been set yet

# Example

```ignore
let mut builder = SelectBuilder::default();
assert!(!builder.already_projected());

builder.projection(vec![]);
assert!(builder.already_projected()); // true even for empty projection

builder.projection(vec![SelectItem::Wildcard(...)]);
assert!(builder.already_projected()); // true for non-empty projection
```

<a id="op-2f7eb105023caab8a0b0a65a"></a>
## build

`function` · `datafusion_sql::unparser::ast::SelectBuilder::build` · datafusion-sql 55.1.0

```rust
fn build(&self) -> Result<ast::Select, BuilderError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acd60c4c5457adac0efa953d"></a>
## clone

`function` · `datafusion_sql::unparser::ast::SelectBuilder::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> SelectBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [138, 10], "end": [138, 15], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/ast.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82c2df43316df97e5667103a"></a>
## cluster_by

`function` · `datafusion_sql::unparser::ast::SelectBuilder::cluster_by` · datafusion-sql 55.1.0

```rust
fn cluster_by(&mut self, value: Vec<ast::Expr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6af8d25473fd64cedd7db008"></a>
## current_flatten_alias

`function` · `datafusion_sql::unparser::ast::SelectBuilder::current_flatten_alias` · datafusion-sql 55.1.0

```rust
fn current_flatten_alias(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Returns the most recently generated flatten alias, or `None` if
`next_flatten_alias` has not been called yet.

<a id="op-539832daea29baed3badf723"></a>
## default

`function` · `datafusion_sql::unparser::ast::SelectBuilder::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 1], "end": [429, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/ast.rs:426`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97b2280894cdee489d31df1b"></a>
## distinct

`function` · `datafusion_sql::unparser::ast::SelectBuilder::distinct` · datafusion-sql 55.1.0

```rust
fn distinct(&mut self, value: Option<ast::Distinct>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c11a299578c6712b9bfd38a"></a>
## distribute_by

`function` · `datafusion_sql::unparser::ast::SelectBuilder::distribute_by` · datafusion-sql 55.1.0

```rust
fn distribute_by(&mut self, value: Vec<ast::Expr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bf1d492d5a6f85942dae91b"></a>
## flatten_table_aliases_empty

`function` · `datafusion_sql::unparser::ast::SelectBuilder::flatten_table_aliases_empty` · datafusion-sql 55.1.0

```rust
fn flatten_table_aliases_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Returns true if no FLATTEN table aliases have been registered.

<a id="op-e40b2a67e10b3b3acb12f2c0"></a>
## from

`function` · `datafusion_sql::unparser::ast::SelectBuilder::from` · datafusion-sql 55.1.0

```rust
fn from(&mut self, value: Vec<TableWithJoinsBuilder>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9039e2fe93924cd63d15bb7"></a>
## group_by

`function` · `datafusion_sql::unparser::ast::SelectBuilder::group_by` · datafusion-sql 55.1.0

```rust
fn group_by(&mut self, value: ast::GroupByExpr) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22264abafc317681894d2418"></a>
## has_selection

`function` · `datafusion_sql::unparser::ast::SelectBuilder::has_selection` · datafusion-sql 55.1.0

```rust
fn has_selection(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a520731d10960a2accb427b"></a>
## having

`function` · `datafusion_sql::unparser::ast::SelectBuilder::having` · datafusion-sql 55.1.0

```rust
fn having(&mut self, value: Option<ast::Expr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcc9f8cc90367f1d27095a08"></a>
## into

`function` · `datafusion_sql::unparser::ast::SelectBuilder::into` · datafusion-sql 55.1.0

```rust
fn into(&mut self, value: Option<ast::SelectInto>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fe28c6469700be845af577a"></a>
## is_flatten_table_alias

`function` · `datafusion_sql::unparser::ast::SelectBuilder::is_flatten_table_alias` · datafusion-sql 55.1.0

```rust
fn is_flatten_table_alias(&self, alias: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Returns true if the given table alias refers to a FLATTEN relation.

<a id="op-806cd7f0c8ff18d9464d40f4"></a>
## lateral_views

`function` · `datafusion_sql::unparser::ast::SelectBuilder::lateral_views` · datafusion-sql 55.1.0

```rust
fn lateral_views(&mut self, value: Vec<ast::LateralView>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-036c64aa2a4491f2c90c03a2"></a>
## named_window

`function` · `datafusion_sql::unparser::ast::SelectBuilder::named_window` · datafusion-sql 55.1.0

```rust
fn named_window(&mut self, value: Vec<ast::NamedWindowDefinition>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59f9c889ecf9c008ba0c5a3f"></a>
## next_flatten_alias

`function` · `datafusion_sql::unparser::ast::SelectBuilder::next_flatten_alias` · datafusion-sql 55.1.0

```rust
fn next_flatten_alias(&mut self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Generate a unique alias for a LATERAL FLATTEN relation
(`_unnest_1`, `_unnest_2`, …). Each call returns a fresh name.

<a id="op-67d6c3018d2acaf2d0de2e5c"></a>
## pop_from

`function` · `datafusion_sql::unparser::ast::SelectBuilder::pop_from` · datafusion-sql 55.1.0

```rust
fn pop_from(&mut self) -> Option<TableWithJoinsBuilder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68793dd30f5de01c103c7006"></a>
## pop_projections

`function` · `datafusion_sql::unparser::ast::SelectBuilder::pop_projections` · datafusion-sql 55.1.0

```rust
fn pop_projections(&mut self) -> Vec<ast::SelectItem>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32a01d9f3ff66c0303f19768"></a>
## projection

`function` · `datafusion_sql::unparser::ast::SelectBuilder::projection` · datafusion-sql 55.1.0

```rust
fn projection(&mut self, value: Vec<ast::SelectItem>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fc02ea194ca4cb78249b2e4"></a>
## push_from

`function` · `datafusion_sql::unparser::ast::SelectBuilder::push_from` · datafusion-sql 55.1.0

```rust
fn push_from(&mut self, value: TableWithJoinsBuilder) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-998b8d6c2f014d3a7cc91a0c"></a>
## qualify

`function` · `datafusion_sql::unparser::ast::SelectBuilder::qualify` · datafusion-sql 55.1.0

```rust
fn qualify(&mut self, value: Option<ast::Expr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebe95e9980a061a2dc5df0fe"></a>
## replace_mark

`function` · `datafusion_sql::unparser::ast::SelectBuilder::replace_mark` · datafusion-sql 55.1.0

```rust
fn replace_mark(&mut self, existing_expr: &ast::Expr, value: &ast::Expr) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Replaces the selection with a new value.

This function is used to replace a specific expression within the selection.
Unlike the `selection` method which combines existing and new selections with AND,
this method searches for and replaces occurrences of a specific expression.

This method is primarily used to modify LEFT MARK JOIN expressions.
When processing a LEFT MARK JOIN, we need to replace the placeholder expression
with the actual join condition in the selection clause.

# Arguments

* `existing_expr` - The expression to replace
* `value` - The new expression to set as the selection

<a id="op-671faab62e44da5f55be970b"></a>
## selection

`function` · `datafusion_sql::unparser::ast::SelectBuilder::selection` · datafusion-sql 55.1.0

```rust
fn selection(&mut self, value: Option<ast::Expr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad988c5c5bbfd10c1a1dcf40"></a>
## sort_by

`function` · `datafusion_sql::unparser::ast::SelectBuilder::sort_by` · datafusion-sql 55.1.0

```rust
fn sort_by(&mut self, value: Vec<ast::OrderByExpr>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0ae7065e6d492377ded78f7"></a>
## top

`function` · `datafusion_sql::unparser::ast::SelectBuilder::top` · datafusion-sql 55.1.0

```rust
fn top(&mut self, value: Option<ast::Top>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-439e9276e286375c8bcaf462"></a>
## value_table_mode

`function` · `datafusion_sql::unparser::ast::SelectBuilder::value_table_mode` · datafusion-sql 55.1.0

```rust
fn value_table_mode(&mut self, value: Option<ast::ValueTableMode>) -> &mut Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::SelectBuilder", "path": "SelectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [424, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
