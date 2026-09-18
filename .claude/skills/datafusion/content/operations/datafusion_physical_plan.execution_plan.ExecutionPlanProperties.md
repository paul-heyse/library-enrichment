# `datafusion_physical_plan::execution_plan::ExecutionPlanProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.ExecutionPlanProperties.json).

<a id="op-ec582c74a9817b539270a5c9"></a>
## ExecutionPlanProperties

`trait` · `datafusion_physical_plan::execution_plan::ExecutionPlanProperties` · datafusion-physical-plan 55.1.0

```rust
trait ExecutionPlanProperties
```

Source: `src/execution_plan.rs:1201`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Extension trait provides an easy API to fetch various properties of
[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) objects based on [`ExecutionPlan::properties`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ef2d89ab2da894daea49ae8e).

<a id="op-ce73570a083ab9f454993449"></a>
## boundedness

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlanProperties::boundedness` · datafusion-physical-plan 55.1.0

```rust
fn boundedness(&self) -> Boundedness
```

Source: `src/execution_plan.rs:1217`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Boundedness information of the stream corresponding to this `ExecutionPlan`.
For more details, see [`Boundedness`](../operations/datafusion_physical_plan.execution_plan.Boundedness.md#op-0bd02bea0eb0e3d05f074321).

<a id="op-4d2f1108b5c44db4c4ceff72"></a>
## equivalence_properties

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlanProperties::equivalence_properties` · datafusion-physical-plan 55.1.0

```rust
fn equivalence_properties(&self) -> &EquivalenceProperties
```

Source: `src/execution_plan.rs:1242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the [`EquivalenceProperties`](../operations/datafusion_physical_expr.equivalence.properties.EquivalenceProperties.md#op-eeda1c3d472a48e6007359b5) within the plan.

Equivalence properties tell DataFusion what columns are known to be
equal, during various optimization passes. By default, this returns "no
known equivalences" which is always correct, but may cause DataFusion to
unnecessarily resort data.

If this ExecutionPlan makes no changes to the schema of the rows flowing
through it or how columns within each row relate to each other, it
should return the equivalence properties of its input. For
example, since [`FilterExec`] may remove rows from its input, but does not
otherwise modify them, it preserves its input equivalence properties.
However, since `ProjectionExec` may calculate derived expressions, it
needs special handling.

See also [`ExecutionPlan::maintains_input_order`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-fb678887a0ccc94b18374b08) and [`Self::output_ordering`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlanProperties.md#op-f9d08dd07e804299c5f6ae20)
for related concepts.

[`FilterExec`]: crate::filter::FilterExec

<a id="op-f9d08dd07e804299c5f6ae20"></a>
## output_ordering

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlanProperties::output_ordering` · datafusion-physical-plan 55.1.0

```rust
fn output_ordering(&self) -> Option<&LexOrdering>
```

Source: `src/execution_plan.rs:1213`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

If the output of this `ExecutionPlan` within each partition is sorted,
returns `Some(keys)` describing the ordering. A `None` return value
indicates no assumptions should be made on the output ordering.

For example, `SortExec` (obviously) produces sorted output as does
`SortPreservingMergeStream`. Less obviously, `Projection` produces sorted
output if its input is sorted as it does not reorder the input rows.

<a id="op-4ab6b4cf5473b37decc7b47c"></a>
## output_partitioning

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlanProperties::output_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn output_partitioning(&self) -> &Partitioning
```

Source: `src/execution_plan.rs:1204`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specifies how the output of this `ExecutionPlan` is split into
partitions.

<a id="op-d4adffad96065419c6819b12"></a>
## pipeline_behavior

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlanProperties::pipeline_behavior` · datafusion-physical-plan 55.1.0

```rust
fn pipeline_behavior(&self) -> EmissionType
```

Source: `src/execution_plan.rs:1221`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Indicates how the stream of this `ExecutionPlan` emits its results.
For more details, see [`EmissionType`](../operations/datafusion_physical_plan.execution_plan.EmissionType.md#op-05c5e3937550b6fc12eae8da).
