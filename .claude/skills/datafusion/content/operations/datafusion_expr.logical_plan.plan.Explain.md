# `datafusion_expr::logical_plan::plan::Explain`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Explain.json).

<a id="op-7a2cab6d7cc1e2bcbec63373"></a>
## Explain

`struct` · `datafusion_expr::logical_plan::plan::Explain` · datafusion-expr 55.1.0

```rust
struct Explain
```

Source: `src/logical_plan/plan.rs:3551`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Produces a relation with string representations of
various parts of the plan

See [the documentation] for more information

[the documentation]: https://datafusion.apache.org/user-guide/sql/explain.html

<a id="op-af18133c3f5e0b4851c2ff14"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Explain::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Explain
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Explain", "path": "Explain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3550, 17], "end": [3550, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3550`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c31e1002ef7a4b3754882ef"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Explain::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Explain) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Explain", "path": "Explain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3550, 24], "end": [3550, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3550`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e17b537a1cc6fe8e6336f33"></a>
## explain_format

`struct_field` · `datafusion_expr::logical_plan::plan::Explain::explain_format` · datafusion-expr 55.1.0

```rust
explain_format: datafusion_common::format::ExplainFormat
```

Source: `src/logical_plan/plan.rs:3556`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Output format for explain, if specified.
If none, defaults to `text`

<a id="op-1d8d26cd4a5ac05a55420b50"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Explain::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Explain", "path": "Explain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3550, 10], "end": [3550, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3550`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fb99e4290930fa00192167d"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Explain::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Explain", "path": "Explain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3550, 39], "end": [3550, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3550`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25cbb9ab704b8aad3df0aa1c"></a>
## logical_optimization_succeeded

`struct_field` · `datafusion_expr::logical_plan::plan::Explain::logical_optimization_succeeded` · datafusion-expr 55.1.0

```rust
logical_optimization_succeeded: bool
```

Source: `src/logical_plan/plan.rs:3564`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Used by physical planner to check if should proceed with planning

<a id="op-01987eb24de47e73a25fff3a"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Explain::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Explain", "path": "Explain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3571, 1], "end": [3605, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:3572`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7c42f8294e4dc40706bb3fe"></a>
## plan

`struct_field` · `datafusion_expr::logical_plan::plan::Explain::plan` · datafusion-expr 55.1.0

```rust
plan: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:3558`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The logical plan that is being EXPLAIN'd

<a id="op-2c0c95dd6f89d57870a735cf"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::Explain::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:3562`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The output schema of the explain (2 columns of text)

<a id="op-f17a2432a5be35c7e2733b2a"></a>
## show_statistics

`struct_field` · `datafusion_expr::logical_plan::plan::Explain::show_statistics` · datafusion-expr 55.1.0

```rust
show_statistics: Option<bool>
```

Source: `src/logical_plan/plan.rs:3567`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Statement-level override for `datafusion.explain.show_statistics`.
When `None`, the session-config value is used.

<a id="op-f69e33c39b8cdc43dc1a7875"></a>
## stringified_plans

`struct_field` · `datafusion_expr::logical_plan::plan::Explain::stringified_plans` · datafusion-expr 55.1.0

```rust
stringified_plans: Vec<StringifiedPlan>
```

Source: `src/logical_plan/plan.rs:3560`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Represent the various stages plans have gone through

<a id="op-4b1b71c4e5db3aa25b297d1e"></a>
## verbose

`struct_field` · `datafusion_expr::logical_plan::plan::Explain::verbose` · datafusion-expr 55.1.0

```rust
verbose: bool
```

Source: `src/logical_plan/plan.rs:3553`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Should extra (detailed, intermediate plans) be included?
