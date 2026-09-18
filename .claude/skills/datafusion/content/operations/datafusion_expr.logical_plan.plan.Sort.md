# `datafusion_expr::logical_plan::plan::Sort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Sort.json).

<a id="op-42634263aeb74254b1671bfa"></a>
## Sort

`struct` · `datafusion_expr::logical_plan::plan::Sort` · datafusion-expr 55.1.0

```rust
struct Sort
```

Source: `src/logical_plan/plan.rs:4220`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Sorts its input according to a list of sort expressions.

<a id="op-c3eb8c4d6932539c512ca608"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Sort::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Sort
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4219, 17], "end": [4219, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:4219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89644398740802299233cdb8"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Sort::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Sort) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4219, 24], "end": [4219, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:4219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-621138c096cc8d3556bc0dfd"></a>
## expr

`struct_field` · `datafusion_expr::logical_plan::plan::Sort::expr` · datafusion-expr 55.1.0

```rust
expr: Vec<expr::Sort>
```

Source: `src/logical_plan/plan.rs:4222`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The sort expressions

<a id="op-4e0509a2b862811da128b393"></a>
## fetch

`struct_field` · `datafusion_expr::logical_plan::plan::Sort::fetch` · datafusion-expr 55.1.0

```rust
fetch: Option<usize>
```

Source: `src/logical_plan/plan.rs:4226`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional fetch limit

<a id="op-5b91ffc7a811795486466d55"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Sort::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4219, 10], "end": [4219, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:4219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50e09fcc648cdea6941f907c"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Sort::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4219, 51], "end": [4219, 55], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:4219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7a4aff7daaf43a0409ba3cf"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::Sort::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:4224`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The incoming logical plan

<a id="op-908478ddb3c3a60b6df7da51"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Sort::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Sort) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4219, 39], "end": [4219, 49], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:4219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
