# `datafusion_physical_plan::execution_plan::PlanProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.PlanProperties.json).

<a id="op-5e820cca2acc2c7b69a4a0b3"></a>
## PlanProperties

`struct` · `datafusion_physical_plan::execution_plan::PlanProperties` · datafusion-physical-plan 55.1.0

```rust
struct PlanProperties
```

Source: `src/execution_plan.rs:1486`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Stores plan properties used in query optimization.

Serves as a cache for these properties, which are often
expensive to compute.

<a id="op-1acd2a9b0a76d42e31f39492"></a>
## boundedness

`struct_field` · `datafusion_physical_plan::execution_plan::PlanProperties::boundedness` · datafusion-physical-plan 55.1.0

```rust
boundedness: Boundedness
```

Source: `src/execution_plan.rs:1494`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

See [ExecutionPlanProperties::boundedness](../operations/datafusion_physical_plan.execution_plan.ExecutionPlanProperties.md#op-ce73570a083ab9f454993449)

<a id="op-4e6cb88f8f81bcaebfd97944"></a>
## clone

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> PlanProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1485, 17], "end": [1485, 22], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution_plan.rs:1485`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07d05f549ac9b127817f86dd"></a>
## emission_type

`struct_field` · `datafusion_physical_plan::execution_plan::PlanProperties::emission_type` · datafusion-physical-plan 55.1.0

```rust
emission_type: EmissionType
```

Source: `src/execution_plan.rs:1492`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

See [ExecutionPlanProperties::pipeline_behavior](../operations/datafusion_physical_plan.execution_plan.ExecutionPlanProperties.md#op-d4adffad96065419c6819b12)

<a id="op-a68b7fe8560186b7beec2cf0"></a>
## eq_properties

`struct_field` · `datafusion_physical_plan::execution_plan::PlanProperties::eq_properties` · datafusion-physical-plan 55.1.0

```rust
eq_properties: datafusion_physical_expr::EquivalenceProperties
```

Source: `src/execution_plan.rs:1488`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

See [ExecutionPlanProperties::equivalence_properties](../operations/datafusion_physical_plan.execution_plan.ExecutionPlanProperties.md#op-4d2f1108b5c44db4c4ceff72)

<a id="op-30b4011480a3628db37c4cda"></a>
## equivalence_properties

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::equivalence_properties` · datafusion-physical-plan 55.1.0

```rust
fn equivalence_properties(&self) -> &EquivalenceProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1581`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e26b5b6d4148cc513a9b4a53"></a>
## evaluation_type

`struct_field` · `datafusion_physical_plan::execution_plan::PlanProperties::evaluation_type` · datafusion-physical-plan 55.1.0

```rust
evaluation_type: EvaluationType
```

Source: `src/execution_plan.rs:1495`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-982823c7d3e87b0b6364a6ec"></a>
## fmt

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1485, 10], "end": [1485, 15], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution_plan.rs:1485`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6797ddc8fd0fdca04801c9e3"></a>
## new

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::new` · datafusion-physical-plan 55.1.0

```rust
fn new(eq_properties: EquivalenceProperties, partitioning: Partitioning, emission_type: EmissionType, boundedness: Boundedness) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1503`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Construct a new `PlanPropertiesCache` from the

<a id="op-0a9f2f42ec7f59633e4de427"></a>
## output_ordering

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::output_ordering` · datafusion-physical-plan 55.1.0

```rust
fn output_ordering(&self) -> Option<&LexOrdering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1589`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a3171a95296e509663d48d2"></a>
## output_partitioning

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::output_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn output_partitioning(&self) -> &Partitioning
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1585`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4ca07f4b79bb7c6abcf019c"></a>
## partitioning

`struct_field` · `datafusion_physical_plan::execution_plan::PlanProperties::partitioning` · datafusion-physical-plan 55.1.0

```rust
partitioning: Partitioning
```

Source: `src/execution_plan.rs:1490`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

See [ExecutionPlanProperties::output_partitioning](../operations/datafusion_physical_plan.execution_plan.ExecutionPlanProperties.md#op-4ab6b4cf5473b37decc7b47c)

<a id="op-a9ac1e87738ad7a1bf0e6ada"></a>
## scheduling_type

`struct_field` · `datafusion_physical_plan::execution_plan::PlanProperties::scheduling_type` · datafusion-physical-plan 55.1.0

```rust
scheduling_type: SchedulingType
```

Source: `src/execution_plan.rs:1496`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-552f9b92d2dbe7a152b871d5"></a>
## set_constraints

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::set_constraints` · datafusion-physical-plan 55.1.0

```rust
fn set_constraints(&mut self, constraints: Constraints)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1571`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set constraints having mut reference.

<a id="op-92cbd9334013822706a1e80b"></a>
## set_eq_properties

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::set_eq_properties` · datafusion-physical-plan 55.1.0

```rust
fn set_eq_properties(&mut self, eq_properties: EquivalenceProperties)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1529`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set equivalence properties having mut reference.

<a id="op-865e0e05fb46f8a1eee5df4a"></a>
## with_boundedness

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::with_boundedness` · datafusion-physical-plan 55.1.0

```rust
fn with_boundedness(self, boundedness: Boundedness) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1543`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Overwrite boundedness with its new value.

<a id="op-d964895c4f68742e583087d5"></a>
## with_constraints

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::with_constraints` · datafusion-physical-plan 55.1.0

```rust
fn with_constraints(self, constraints: Constraints) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1576`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Overwrite constraints with its new value.

<a id="op-6f5032a2173f20cd6f9f2d64"></a>
## with_emission_type

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::with_emission_type` · datafusion-physical-plan 55.1.0

```rust
fn with_emission_type(self, emission_type: EmissionType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1549`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Overwrite emission type with its new value.

<a id="op-f630227d68d6a82c851ed7cf"></a>
## with_eq_properties

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::with_eq_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_eq_properties(self, eq_properties: EquivalenceProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1537`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Overwrite equivalence properties with its new value.

<a id="op-33bac7dec97037157eaae980"></a>
## with_evaluation_type

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::with_evaluation_type` · datafusion-physical-plan 55.1.0

```rust
fn with_evaluation_type(self, drive_type: EvaluationType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1565`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the [`EvaluationType`](../operations/datafusion_physical_plan.execution_plan.EvaluationType.md#op-a4cd8eb4587cae8903087cf9).

Defaults to [`EvaluationType::Lazy`](../operations/datafusion_physical_plan.execution_plan.EvaluationType.md#op-0109c19e28976f5c8a763716)

<a id="op-a003bda207be7e6c214f13e7"></a>
## with_partitioning

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::with_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn with_partitioning(self, partitioning: Partitioning) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1523`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Overwrite output partitioning with its new value.

<a id="op-563a4220b1f7239fcbc96905"></a>
## with_scheduling_type

`function` · `datafusion_physical_plan::execution_plan::PlanProperties::with_scheduling_type` · datafusion-physical-plan 55.1.0

```rust
fn with_scheduling_type(self, scheduling_type: SchedulingType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1501, 1], "end": [1597, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1557`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the [`SchedulingType`](../operations/datafusion_physical_plan.execution_plan.SchedulingType.md#op-ec54a4a7e216224a2876595f).

Defaults to [`SchedulingType::NonCooperative`](../operations/datafusion_physical_plan.execution_plan.SchedulingType.md#op-87aaf2d02bb118a5b3d7c4aa)
