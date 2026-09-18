# `datafusion_expr::logical_plan::plan::EmptyRelation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.EmptyRelation.json).

<a id="op-8e0891ba15042deac865bbd5"></a>
## EmptyRelation

`struct` · `datafusion_expr::logical_plan::plan::EmptyRelation` · datafusion-expr 55.1.0

```rust
struct EmptyRelation
```

Source: `src/logical_plan/plan.rs:2284`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Relationship produces 0 or 1 placeholder rows with specified output schema
In most cases the output schema for `EmptyRelation` would be empty,
however, it can be non-empty typically for optimizer rules

<a id="op-202a55b37edbc5c985b3a078"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::EmptyRelation::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> EmptyRelation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::EmptyRelation", "path": "EmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2283, 17], "end": [2283, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:2283`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6bd50eb937666aba694d764"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::EmptyRelation::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &EmptyRelation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::EmptyRelation", "path": "EmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2283, 24], "end": [2283, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:2283`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9541e686ab62acfd6e2d68a"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::EmptyRelation::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::EmptyRelation", "path": "EmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2283, 10], "end": [2283, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:2283`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c39baae87dff1f1917bb8cdb"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::EmptyRelation::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::EmptyRelation", "path": "EmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2283, 39], "end": [2283, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:2283`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea94c79cb072dfb740840ed7"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::EmptyRelation::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::EmptyRelation", "path": "EmptyRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2292, 1], "end": [2299, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:2293`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4032192a1730a3480897840"></a>
## produce_one_row

`struct_field` · `datafusion_expr::logical_plan::plan::EmptyRelation::produce_one_row` · datafusion-expr 55.1.0

```rust
produce_one_row: bool
```

Source: `src/logical_plan/plan.rs:2286`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether to produce a placeholder row

<a id="op-8c1fca070cf0a8c894184c97"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::EmptyRelation::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:2288`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The schema description of the output
