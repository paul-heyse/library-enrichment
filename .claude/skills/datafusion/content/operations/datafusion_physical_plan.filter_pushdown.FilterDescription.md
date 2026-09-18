# `datafusion_physical_plan::filter_pushdown::FilterDescription`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter_pushdown.FilterDescription.json).

<a id="op-0116d642527aa63591adbd0f"></a>
## FilterDescription

`struct` · `datafusion_physical_plan::filter_pushdown::FilterDescription` · datafusion-physical-plan 55.1.0

```rust
struct FilterDescription
```

Source: `src/filter_pushdown.rs:483`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Describes how filters should be pushed down to children.

This structure contains filter descriptions for each child node, specifying:
- Which parent filters can be pushed down to each child
- Which self-generated filters should be pushed down to each child

The filter routing is determined by column analysis - filters can only be pushed
to children whose schemas contain all the referenced columns.

<a id="op-ca1ed5a6e8859e4effd84f53"></a>
## all_unsupported

`function` · `datafusion_physical_plan::filter_pushdown::FilterDescription::all_unsupported` · datafusion-physical-plan 55.1.0

```rust
fn all_unsupported(parent_filters: &[Arc<dyn PhysicalExpr>], children: &[&Arc<dyn ExecutionPlan>]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterDescription", "path": "FilterDescription"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [558, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Mark all parent filters as unsupported for all children.

<a id="op-4fd26fa9acba7ffd42d88c7b"></a>
## clone

`function` · `datafusion_physical_plan::filter_pushdown::FilterDescription::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> FilterDescription
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterDescription", "path": "FilterDescription"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [482, 17], "end": [482, 22], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/filter_pushdown.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93a48497c3e9f3e0c9082812"></a>
## default

`function` · `datafusion_physical_plan::filter_pushdown::FilterDescription::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterDescription", "path": "FilterDescription"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 1], "end": [494, 2], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/filter_pushdown.rs:491`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e36ca2d556d0b5245aa9e398"></a>
## fmt

`function` · `datafusion_physical_plan::filter_pushdown::FilterDescription::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterDescription", "path": "FilterDescription"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [482, 10], "end": [482, 15], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter_pushdown.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-410e6e81986f32ee73492403"></a>
## from_children

`function` · `datafusion_physical_plan::filter_pushdown::FilterDescription::from_children` · datafusion-physical-plan 55.1.0

```rust
fn from_children(parent_filters: Vec<Arc<dyn PhysicalExpr>>, children: &[&Arc<dyn ExecutionPlan>]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterDescription", "path": "FilterDescription"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [558, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:515`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Build a filter description by analyzing which parent filters can be pushed to each child.
This method automatically determines filter routing based on column analysis:
- If all columns referenced by a filter exist in a child's schema, it can be pushed down
- Otherwise, it cannot be pushed down to that child

<a id="op-c9bc860821c57623e5f3ebbf"></a>
## new

`function` · `datafusion_physical_plan::filter_pushdown::FilterDescription::new` · datafusion-physical-plan 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterDescription", "path": "FilterDescription"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [558, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:498`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new empty FilterDescription

<a id="op-968a03fa568e47d55970ff6f"></a>
## parent_filters

`function` · `datafusion_physical_plan::filter_pushdown::FilterDescription::parent_filters` · datafusion-physical-plan 55.1.0

```rust
fn parent_filters(&self) -> Vec<Vec<PushedDownPredicate>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterDescription", "path": "FilterDescription"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [558, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-858c69cec8c5165e474f0271"></a>
## self_filters

`function` · `datafusion_physical_plan::filter_pushdown::FilterDescription::self_filters` · datafusion-physical-plan 55.1.0

```rust
fn self_filters(&self) -> Vec<Vec<Arc<dyn PhysicalExpr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterDescription", "path": "FilterDescription"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [558, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:551`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90927258df83b1e6226402aa"></a>
## with_child

`function` · `datafusion_physical_plan::filter_pushdown::FilterDescription::with_child` · datafusion-physical-plan 55.1.0

```rust
fn with_child(self, child: ChildFilterDescription) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterDescription", "path": "FilterDescription"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [558, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Add a child filter description
