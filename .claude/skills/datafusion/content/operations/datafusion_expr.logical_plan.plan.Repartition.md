# `datafusion_expr::logical_plan::plan::Repartition`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Repartition.json).

<a id="op-9ea8288318eb88a0be1d174f"></a>
## Repartition

`struct` · `datafusion_expr::logical_plan::plan::Repartition` · datafusion-expr 55.1.0

```rust
struct Repartition
```

Source: `src/logical_plan/plan.rs:3167`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d99a43a13046364a69f6d294"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Repartition::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Repartition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Repartition", "path": "Repartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3166, 17], "end": [3166, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3166`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac27d23e4034d256bf6d12b0"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Repartition::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Repartition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Repartition", "path": "Repartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3166, 24], "end": [3166, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3166`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95018aecd9965f332d9eb87d"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Repartition::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Repartition", "path": "Repartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3166, 10], "end": [3166, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3166`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7eff3496a986be53cdbe3546"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Repartition::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Repartition", "path": "Repartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3166, 51], "end": [3166, 55], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3166`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77f6d18305f6eec16fa596ab"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::Repartition::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:3169`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The incoming logical plan

<a id="op-bb99106f1ed26eabb6839a92"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Repartition::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Repartition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Repartition", "path": "Repartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3166, 39], "end": [3166, 49], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:3166`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c80937314fb2e8d2905c92c3"></a>
## partitioning_scheme

`struct_field` · `datafusion_expr::logical_plan::plan::Repartition::partitioning_scheme` · datafusion-expr 55.1.0

```rust
partitioning_scheme: Partitioning
```

Source: `src/logical_plan/plan.rs:3171`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The partitioning scheme
