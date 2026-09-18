# `datafusion_physical_plan::filter::FilterExecBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter.FilterExecBuilder.json).

<a id="op-6b2b1331c53516375b4db90b"></a>
## FilterExecBuilder

`struct` · `datafusion_physical_plan::filter::FilterExecBuilder` · datafusion-physical-plan 55.1.0

```rust
struct FilterExecBuilder
```

Source: `src/filter.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Builder for [`FilterExec`](../operations/datafusion_physical_plan.filter.FilterExec.md#op-146286fb0235b9e8f3c7f14e) to set optional parameters

<a id="op-d94d8b291535fba4572c3206"></a>
## apply_projection

`function` · `datafusion_physical_plan::filter::FilterExecBuilder::apply_projection` · datafusion-physical-plan 55.1.0

```rust
fn apply_projection(self, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExecBuilder", "path": "FilterExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [223, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the projection, composing with any existing projection.

If a projection is already set, the new projection indices are mapped
through the existing projection. For example, if the current projection
is `[0, 2, 3]` and `apply_projection(Some(vec![0, 2]))` is called, the
resulting projection will be `[0, 3]` (indices 0 and 2 of `[0, 2, 3]`).

If no projection is currently set, the new projection is used directly.
If `None` is passed, the projection is cleared.

<a id="op-312f28848616dbbb262a11ae"></a>
## apply_projection_by_ref

`function` · `datafusion_physical_plan::filter::FilterExecBuilder::apply_projection_by_ref` · datafusion-physical-plan 55.1.0

```rust
fn apply_projection_by_ref(self, projection: Option<&ProjectionRef>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExecBuilder", "path": "FilterExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [223, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The same as [`Self::apply_projection`](../operations/datafusion_physical_plan.filter.FilterExecBuilder.md#op-d94d8b291535fba4572c3206) but takes projection shared reference.

<a id="op-4cf1f55279a128850cdef41b"></a>
## build

`function` · `datafusion_physical_plan::filter::FilterExecBuilder::build` · datafusion-physical-plan 55.1.0

```rust
fn build(self) -> Result<FilterExec>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExecBuilder", "path": "FilterExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [223, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Build the FilterExec, computing properties once with all configured parameters

<a id="op-a3d6cf5a81c01bc3ed3a53cd"></a>
## from

`function` · `datafusion_physical_plan::filter::FilterExecBuilder::from` · datafusion-physical-plan 55.1.0

```rust
fn from(exec: &FilterExec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExecBuilder", "path": "FilterExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [240, 2], "filename": "src/filter.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/filter.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3690f73b844607bbf3ec1a03"></a>
## new

`function` · `datafusion_physical_plan::filter::FilterExecBuilder::new` · datafusion-physical-plan 55.1.0

```rust
fn new(predicate: Arc<dyn PhysicalExpr>, input: Arc<dyn ExecutionPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExecBuilder", "path": "FilterExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [223, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new builder with required parameters (predicate and input)

<a id="op-d6e4f38f2dd046e9342e1a86"></a>
## with_batch_size

`function` · `datafusion_physical_plan::filter::FilterExecBuilder::with_batch_size` · datafusion-physical-plan 55.1.0

```rust
fn with_batch_size(self, batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExecBuilder", "path": "FilterExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [223, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the batch size

<a id="op-f1f9f149d023956f3fe8f8e5"></a>
## with_default_selectivity

`function` · `datafusion_physical_plan::filter::FilterExecBuilder::with_default_selectivity` · datafusion-physical-plan 55.1.0

```rust
fn with_default_selectivity(self, default_selectivity: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExecBuilder", "path": "FilterExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [223, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the default selectivity

<a id="op-02b9196c04b807cc1447b90a"></a>
## with_fetch

`function` · `datafusion_physical_plan::filter::FilterExecBuilder::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(self, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExecBuilder", "path": "FilterExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [223, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the fetch limit

<a id="op-1f4c953467e2b564b039ce09"></a>
## with_input

`function` · `datafusion_physical_plan::filter::FilterExecBuilder::with_input` · datafusion-physical-plan 55.1.0

```rust
fn with_input(self, input: Arc<dyn ExecutionPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExecBuilder", "path": "FilterExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [223, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the input execution plan

<a id="op-4887fefbefc4547e4287dfad"></a>
## with_predicate

`function` · `datafusion_physical_plan::filter::FilterExecBuilder::with_predicate` · datafusion-physical-plan 55.1.0

```rust
fn with_predicate(self, predicate: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExecBuilder", "path": "FilterExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [223, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the predicate expression
