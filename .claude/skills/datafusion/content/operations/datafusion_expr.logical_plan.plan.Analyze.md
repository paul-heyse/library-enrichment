# `datafusion_expr::logical_plan::plan::Analyze`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Analyze.json).

<a id="op-4b441ef286ba93f390e5475b"></a>
## Analyze

`struct` · `datafusion_expr::logical_plan::plan::Analyze` · datafusion-expr 55.1.0

```rust
struct Analyze
```

Source: `src/logical_plan/plan.rs:3610`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Runs the actual plan, and then prints the physical plan with
with execution metrics.

<a id="op-e8f10e3358c8babff61367c5"></a>
## analyze_categories

`struct_field` · `datafusion_expr::logical_plan::plan::Analyze::analyze_categories` · datafusion-expr 55.1.0

```rust
analyze_categories: Option<datafusion_common::format::ExplainAnalyzeCategories>
```

Source: `src/logical_plan/plan.rs:3624`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Statement-level override for `datafusion.explain.analyze_categories`.
When `None`, the session-config value is used.

<a id="op-ca9d910d89297c731d22f8ef"></a>
## analyze_level

`struct_field` · `datafusion_expr::logical_plan::plan::Analyze::analyze_level` · datafusion-expr 55.1.0

```rust
analyze_level: Option<datafusion_common::format::MetricType>
```

Source: `src/logical_plan/plan.rs:3621`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Statement-level override for `datafusion.explain.analyze_level`.
When `None`, the session-config value is used.

<a id="op-fdf6c4bcae5e20ecdf5d334a"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Analyze::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Analyze
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3609, 17], "end": [3609, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3609`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e485d20b086125c3abb7bbf"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Analyze::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Analyze) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3609, 24], "end": [3609, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3609`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-746c3dfdc481c713326a719d"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Analyze::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3609, 10], "end": [3609, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3609`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adca3c9ef1ea7c0491c13e32"></a>
## format

`struct_field` · `datafusion_expr::logical_plan::plan::Analyze::format` · datafusion-expr 55.1.0

```rust
format: datafusion_common::format::ExplainFormat
```

Source: `src/logical_plan/plan.rs:3614`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Output syntax/format for the rendered physical plan + metrics.

<a id="op-f6f4c995df88afa77d39dc17"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Analyze::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3609, 39], "end": [3609, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3609`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58ae03e8e4a0fa2e7434ce41"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::Analyze::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:3616`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The logical plan that is being EXPLAIN ANALYZE'd

<a id="op-6a1c22432cf2d41e0ce4f384"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Analyze::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3631, 1], "end": [3640, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:3632`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-235f06aed323d71d021593db"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::Analyze::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:3618`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The output schema of the explain (2 columns of text)

<a id="op-8f4e8369118b0c1fb7efc867"></a>
## verbose

`struct_field` · `datafusion_expr::logical_plan::plan::Analyze::verbose` · datafusion-expr 55.1.0

```rust
verbose: bool
```

Source: `src/logical_plan/plan.rs:3612`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Should extra detail be included?
