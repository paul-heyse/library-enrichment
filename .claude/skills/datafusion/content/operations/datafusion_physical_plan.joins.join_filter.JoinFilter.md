# `datafusion_physical_plan::joins::join_filter::JoinFilter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.join_filter.JoinFilter.json).

<a id="op-2c36e92fdae023ebe60d1dc7"></a>
## JoinFilter

`struct` · `datafusion_physical_plan::joins::join_filter::JoinFilter` · datafusion-physical-plan 55.1.0

```rust
struct JoinFilter
```

Source: `src/joins/join_filter.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Filter applied before join output. Fields are crate-public to allow
downstream implementations to experiment with custom joins.

<a id="op-62acb4c0f49d485dcc5ff2b1"></a>
## build_column_indices

`function` · `datafusion_physical_plan::joins::join_filter::JoinFilter::build_column_indices` · datafusion-physical-plan 55.1.0

```rust
fn build_column_indices(left_indices: Vec<usize>, right_indices: Vec<usize>) -> Vec<ColumnIndex>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_filter::JoinFilter", "path": "JoinFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [108, 2], "filename": "src/joins/join_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/join_filter.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Helper for building ColumnIndex vector from left and right indices

<a id="op-47f1c06b19f1b8665949bb4c"></a>
## clone

`function` · `datafusion_physical_plan::joins::join_filter::JoinFilter::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> JoinFilter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_filter::JoinFilter", "path": "JoinFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 17], "end": [26, 22], "filename": "src/joins/join_filter.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/joins/join_filter.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16c1d388faa1c05504bd410a"></a>
## column_indices

`function` · `datafusion_physical_plan::joins::join_filter::JoinFilter::column_indices` · datafusion-physical-plan 55.1.0

```rust
fn column_indices(&self) -> &[ColumnIndex]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_filter::JoinFilter", "path": "JoinFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [108, 2], "filename": "src/joins/join_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/join_filter.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Column indices for intermediate batch creation

<a id="op-0b8c263ac55d9683ea3c3c53"></a>
## expression

`function` · `datafusion_physical_plan::joins::join_filter::JoinFilter::expression` · datafusion-physical-plan 55.1.0

```rust
fn expression(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_filter::JoinFilter", "path": "JoinFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [108, 2], "filename": "src/joins/join_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/join_filter.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Filter expression

<a id="op-983adc45a9f9194d22e24d47"></a>
## fmt

`function` · `datafusion_physical_plan::joins::join_filter::JoinFilter::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_filter::JoinFilter", "path": "JoinFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/joins/join_filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/join_filter.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99305d9ce9ac23bf616f61cb"></a>
## fmt

`function` · `datafusion_physical_plan::joins::join_filter::JoinFilter::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_filter::JoinFilter", "path": "JoinFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "src/joins/join_filter.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/joins/join_filter.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cb0eac10092d968678268f9"></a>
## new

`function` · `datafusion_physical_plan::joins::join_filter::JoinFilter::new` · datafusion-physical-plan 55.1.0

```rust
fn new(expression: Arc<dyn PhysicalExpr>, column_indices: Vec<ColumnIndex>, schema: SchemaRef) -> JoinFilter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_filter::JoinFilter", "path": "JoinFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [108, 2], "filename": "src/joins/join_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/join_filter.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates new JoinFilter

<a id="op-6fe7830ed5e7054bf46d2318"></a>
## schema

`function` · `datafusion_physical_plan::joins::join_filter::JoinFilter::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_filter::JoinFilter", "path": "JoinFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [108, 2], "filename": "src/joins/join_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/join_filter.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Intermediate batch schema

<a id="op-07d2bfec24b3937d7b327db0"></a>
## swap

`function` · `datafusion_physical_plan::joins::join_filter::JoinFilter::swap` · datafusion-physical-plan 55.1.0

```rust
fn swap(&self) -> JoinFilter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_filter::JoinFilter", "path": "JoinFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [108, 2], "filename": "src/joins/join_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/join_filter.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Rewrites the join filter if the inputs to the join are rewritten
