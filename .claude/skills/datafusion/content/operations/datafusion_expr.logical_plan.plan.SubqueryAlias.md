# `datafusion_expr::logical_plan::plan::SubqueryAlias`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.SubqueryAlias.json).

<a id="op-552fb54c96bb55e58c6409d2"></a>
## SubqueryAlias

`struct` · `datafusion_expr::logical_plan::plan::SubqueryAlias` · datafusion-expr 55.1.0

```rust
struct SubqueryAlias
```

Source: `src/logical_plan/plan.rs:2546`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Aliased subquery

<a id="op-e29abe850597961acb74a9a2"></a>
## alias

`struct_field` · `datafusion_expr::logical_plan::plan::SubqueryAlias::alias` · datafusion-expr 55.1.0

```rust
alias: datafusion_common::TableReference
```

Source: `src/logical_plan/plan.rs:2550`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The alias for the input relation

<a id="op-82150e1396c556e47b62f9a2"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::SubqueryAlias::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> SubqueryAlias
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::SubqueryAlias", "path": "SubqueryAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2543, 17], "end": [2543, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:2543`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba1cafed75f17839ce558307"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::SubqueryAlias::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &SubqueryAlias) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::SubqueryAlias", "path": "SubqueryAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2543, 24], "end": [2543, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:2543`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4593c38e97c762dd326fdbd8"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::SubqueryAlias::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::SubqueryAlias", "path": "SubqueryAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2543, 10], "end": [2543, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:2543`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11286fd8cdb35967788e1dbf"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::SubqueryAlias::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::SubqueryAlias", "path": "SubqueryAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2543, 39], "end": [2543, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:2543`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70620e87c852ca925810cf0d"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::SubqueryAlias::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:2548`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The incoming logical plan

<a id="op-e5eb6a717aa86a57aaee86db"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::SubqueryAlias::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::SubqueryAlias", "path": "SubqueryAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2613, 1], "end": [2622, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:2614`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3822e5cc27c5fa6267d00d7"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::SubqueryAlias::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:2552`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The schema with qualified field names

<a id="op-3443c222545220844c40604a"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::SubqueryAlias::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(plan: Arc<LogicalPlan>, alias: impl Into<TableReference>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::SubqueryAlias", "path": "SubqueryAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2555, 1], "end": [2610, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:2556`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
