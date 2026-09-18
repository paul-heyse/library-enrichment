# `datafusion_proto::physical_plan::PhysicalPlanDecodeContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.PhysicalPlanDecodeContext.json).

<a id="op-dae200c85d1c8f72f5a4c75b"></a>
## PhysicalPlanDecodeContext

`struct` · `datafusion_proto::physical_plan::PhysicalPlanDecodeContext` · datafusion-proto 55.1.0

```rust
struct PhysicalPlanDecodeContext<'a>
```

Source: `src/physical_plan/mod.rs:935`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Context threaded through physical-plan deserialization.

This bundles the stable per-call inputs for deserialization and the
per-scope `ScalarSubqueryResults` handle needed while reconstructing
`ScalarSubqueryExpr` nodes inside a `ScalarSubqueryExec` input plan.

<a id="op-be6b65e97291d06428020f0f"></a>
## clone

`function` · `datafusion_proto::physical_plan::PhysicalPlanDecodeContext::clone` · datafusion-proto 55.1.0

```rust
fn clone(&self) -> PhysicalPlanDecodeContext<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_proto::physical_plan::PhysicalPlanDecodeContext", "path": "PhysicalPlanDecodeContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [934, 10], "end": [934, 15], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/physical_plan/mod.rs:934`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2652744733c63834e5ec11df"></a>
## codec

`function` · `datafusion_proto::physical_plan::PhysicalPlanDecodeContext::codec` · datafusion-proto 55.1.0

```rust
fn codec(&self) -> &'a dyn PhysicalExtensionCodec
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_proto::physical_plan::PhysicalPlanDecodeContext", "path": "PhysicalPlanDecodeContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [979, 2], "filename": "src/physical_plan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_plan/mod.rs:957`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Returns the physical extension codec used for deserialization.

<a id="op-6a991de3cab2982c8a12acb9"></a>
## new

`function` · `datafusion_proto::physical_plan::PhysicalPlanDecodeContext::new` · datafusion-proto 55.1.0

```rust
fn new(task_ctx: &'a TaskContext, codec: &'a dyn PhysicalExtensionCodec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_proto::physical_plan::PhysicalPlanDecodeContext", "path": "PhysicalPlanDecodeContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [979, 2], "filename": "src/physical_plan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_plan/mod.rs:943`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Creates a new root decode context.

<a id="op-342ff797c6b29c2622b50cb2"></a>
## scalar_subquery_results

`function` · `datafusion_proto::physical_plan::PhysicalPlanDecodeContext::scalar_subquery_results` · datafusion-proto 55.1.0

```rust
fn scalar_subquery_results(&self) -> Option<&ScalarSubqueryResults>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_proto::physical_plan::PhysicalPlanDecodeContext", "path": "PhysicalPlanDecodeContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [979, 2], "filename": "src/physical_plan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_plan/mod.rs:963`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Returns the scalar subquery results container for the current scope, if
one is active.

<a id="op-1406966b5f60df0514a13aab"></a>
## task_ctx

`function` · `datafusion_proto::physical_plan::PhysicalPlanDecodeContext::task_ctx` · datafusion-proto 55.1.0

```rust
fn task_ctx(&self) -> &'a TaskContext
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_proto::physical_plan::PhysicalPlanDecodeContext", "path": "PhysicalPlanDecodeContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [979, 2], "filename": "src/physical_plan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_plan/mod.rs:952`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Returns the task context used for deserialization.

<a id="op-c085d7f2ae832ec84d524760"></a>
## with_scalar_subquery_results

`function` · `datafusion_proto::physical_plan::PhysicalPlanDecodeContext::with_scalar_subquery_results` · datafusion-proto 55.1.0

```rust
fn with_scalar_subquery_results(&self, scalar_subquery_results: ScalarSubqueryResults) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_proto::physical_plan::PhysicalPlanDecodeContext", "path": "PhysicalPlanDecodeContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [979, 2], "filename": "src/physical_plan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_plan/mod.rs:969`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Returns a child context with a different scalar subquery results
container.
