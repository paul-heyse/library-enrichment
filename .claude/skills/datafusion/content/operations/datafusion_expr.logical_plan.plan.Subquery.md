# `datafusion_expr::logical_plan::plan::Subquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Subquery.json).

<a id="op-86bb296d75e0cfbc352cad3b"></a>
## Subquery

`struct` · `datafusion_expr::logical_plan::plan::Subquery` · datafusion-expr 55.1.0

```rust
struct Subquery
```

Source: `src/logical_plan/plan.rs:4410`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Subquery

<a id="op-fb61446841536f5422017396"></a>
## can_normalize

`function` · `datafusion_expr::logical_plan::plan::Subquery::can_normalize` · datafusion-expr 55.1.0

```rust
fn can_normalize(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Subquery", "path": "Subquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4419, 1], "end": [4423, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "datafusion_common::cse::Normalizeable", "path": "Normalizeable"}, "trait_path": "datafusion_common::cse::Normalizeable"}`

Source: `src/logical_plan/plan.rs:4420`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3832f68c0ba4861a993dae8"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Subquery::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Subquery
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Subquery", "path": "Subquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4409, 10], "end": [4409, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:4409`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bb1be62981fff3087d337b5"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Subquery::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Subquery) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Subquery", "path": "Subquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4409, 17], "end": [4409, 26], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:4409`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eda29673672f6b01663c50d9"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Subquery::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Subquery", "path": "Subquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4456, 1], "end": [4460, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:4457`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-130e7152710a8e52c866828e"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Subquery::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Subquery", "path": "Subquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4409, 44], "end": [4409, 48], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:4409`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-193e88a018a3b54edd043a80"></a>
## normalize_eq

`function` · `datafusion_expr::logical_plan::plan::Subquery::normalize_eq` · datafusion-expr 55.1.0

```rust
fn normalize_eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Subquery", "path": "Subquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4425, 1], "end": [4436, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "datafusion_common::cse::NormalizeEq", "path": "NormalizeEq"}, "trait_path": "datafusion_common::cse::NormalizeEq"}`

Source: `src/logical_plan/plan.rs:4426`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f00d1d1ce4bf760409352111"></a>
## outer_ref_columns

`struct_field` · `datafusion_expr::logical_plan::plan::Subquery::outer_ref_columns` · datafusion-expr 55.1.0

```rust
outer_ref_columns: Vec<Expr>
```

Source: `src/logical_plan/plan.rs:4414`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The outer references used in the subquery

<a id="op-3667e4455cd55372893a3736"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Subquery::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Subquery) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Subquery", "path": "Subquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4409, 32], "end": [4409, 42], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:4409`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d1447014149c25ad497416b"></a>
## spans

`struct_field` · `datafusion_expr::logical_plan::plan::Subquery::spans` · datafusion-expr 55.1.0

```rust
spans: datafusion_common::Spans
```

Source: `src/logical_plan/plan.rs:4416`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Span information for subquery projection columns

<a id="op-b5775427c7ed6e3eb1e8b886"></a>
## subquery

`struct_field` · `datafusion_expr::logical_plan::plan::Subquery::subquery` · datafusion-expr 55.1.0

```rust
subquery: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:4412`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The subquery

<a id="op-a0b15a9c329abc59849f186e"></a>
## try_from_expr

`function` · `datafusion_expr::logical_plan::plan::Subquery::try_from_expr` · datafusion-expr 55.1.0

```rust
fn try_from_expr(plan: &Expr) -> Result<&Subquery>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Subquery", "path": "Subquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4438, 1], "end": [4454, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4439`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e39c74209176e0dca3d6fca3"></a>
## with_plan

`function` · `datafusion_expr::logical_plan::plan::Subquery::with_plan` · datafusion-expr 55.1.0

```rust
fn with_plan(&self, plan: Arc<LogicalPlan>) -> Subquery
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Subquery", "path": "Subquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4438, 1], "end": [4454, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4447`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
