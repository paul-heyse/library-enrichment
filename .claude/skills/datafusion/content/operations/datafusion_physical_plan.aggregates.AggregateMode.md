# `datafusion_physical_plan::aggregates::AggregateMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.AggregateMode.json).

<a id="op-cf03c9522154a9c007266794"></a>
## AggregateMode

`enum` · `datafusion_physical_plan::aggregates::AggregateMode` · datafusion-physical-plan 55.1.0

```rust
enum AggregateMode
```

Source: `src/aggregates/mod.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Aggregation modes

See [`Accumulator::state`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-e13afcecd1da09f0b485e6dc) for background information on multi-phase
aggregation and how these modes are used.

# Variants and their input/output modes

Each variant can be characterized by its [`AggregateInputMode`](../operations/datafusion_physical_plan.aggregates.AggregateInputMode.md#op-f7dada427bf42eb004cdb459) and
[`AggregateOutputMode`](../operations/datafusion_physical_plan.aggregates.AggregateOutputMode.md#op-6147b1a705ac6e29745affac):

```text
                      | Input: Raw data           | Input: Partial state
Output: Final values  | Single, SinglePartitioned | Final, FinalPartitioned
Output: Partial state | Partial                   | PartialReduce
```

Use [`AggregateMode::input_mode`](../operations/datafusion_physical_plan.aggregates.AggregateMode.md#op-7cff56bb237b4b0e694981df) and [`AggregateMode::output_mode`](../operations/datafusion_physical_plan.aggregates.AggregateMode.md#op-35faa34494009ff33a5783fc)
to query these properties.

<a id="op-ed5740f1aa2f02b0d39278b7"></a>
## Final

`variant` · `datafusion_physical_plan::aggregates::AggregateMode::Final` · datafusion-physical-plan 55.1.0

```rust
Final
```

Source: `src/aggregates/mod.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

*Final* of multiple layers of aggregation, in exactly one partition

Final aggregate that produces a single partition of output by combining
the output of multiple partial aggregates.

This is the second phase of a multi-phase aggregation.

This mode requires that the input is a single partition

Note: Adjacent `Partial` and `Final` mode aggregation is equivalent to a `Single`
mode aggregation node. The `Final` mode is required since this is used in an
intermediate step. The [`CombinePartialFinalAggregate`] physical optimizer rule
will replace this combination with `Single` mode for more efficient execution.

[`CombinePartialFinalAggregate`]: https://docs.rs/datafusion/latest/datafusion/physical_optimizer/combine_partial_final_agg/struct.CombinePartialFinalAggregate.html

<a id="op-272118d05401d355de466003"></a>
## FinalPartitioned

`variant` · `datafusion_physical_plan::aggregates::AggregateMode::FinalPartitioned` · datafusion-physical-plan 55.1.0

```rust
FinalPartitioned
```

Source: `src/aggregates/mod.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

*Final* of multiple layers of aggregation, input is *Partitioned*

Final aggregate that works on pre-partitioned data.

This mode requires that all rows with a particular grouping key are in
the same partitions, such as is the case with Hash repartitioning on the
group keys. If a group key is duplicated, duplicate groups would be
produced

<a id="op-dbd2e47964788e31d4fdea12"></a>
## Partial

`variant` · `datafusion_physical_plan::aggregates::AggregateMode::Partial` · datafusion-physical-plan 55.1.0

```rust
Partial
```

Source: `src/aggregates/mod.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

One of multiple layers of aggregation, any input partitioning

Partial aggregate that can be applied in parallel across input
partitions.

This is the first phase of a multi-phase aggregation.

<a id="op-3a75f37b0b6a63121cd3935a"></a>
## PartialReduce

`variant` · `datafusion_physical_plan::aggregates::AggregateMode::PartialReduce` · datafusion-physical-plan 55.1.0

```rust
PartialReduce
```

Source: `src/aggregates/mod.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Combine multiple partial aggregations to produce a new partial
aggregation.

Input is intermediate accumulator state (like Final), but output is
also intermediate accumulator state (like Partial). This enables
tree-reduce aggregation strategies where partial results from
multiple workers are combined in multiple stages before a final
evaluation.

```text
              Final
           /        \
    PartialReduce   PartialReduce
    /         \      /         \
 Partial   Partial  Partial   Partial
```

# Motivation

This reduces shuffling traffic in a distributed setting. See
<https://github.com/datafusion-contrib/datafusion-distributed/issues/360>
for details.

<a id="op-a1592a327e99972c0d0119ae"></a>
## Single

`variant` · `datafusion_physical_plan::aggregates::AggregateMode::Single` · datafusion-physical-plan 55.1.0

```rust
Single
```

Source: `src/aggregates/mod.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

*Single* layer of Aggregation, input is exactly one partition

Applies the entire logical aggregation operation in a single operator,
as opposed to Partial / Final modes which apply the logical aggregation using
two operators.

This mode requires that the input is a single partition (like Final)

<a id="op-c1be66429e879d3c5d4c4837"></a>
## SinglePartitioned

`variant` · `datafusion_physical_plan::aggregates::AggregateMode::SinglePartitioned` · datafusion-physical-plan 55.1.0

```rust
SinglePartitioned
```

Source: `src/aggregates/mod.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

*Single* layer of Aggregation, input is *Partitioned*

Applies the entire logical aggregation operation in a single operator,
as opposed to Partial / Final modes which apply the logical aggregation
using two operators.

This mode requires that the input has more than one partition, and is
partitioned by group key (like FinalPartitioned).

<a id="op-f7dc22d774bdbdc0cecc8d51"></a>
## clone

`function` · `datafusion_physical_plan::aggregates::AggregateMode::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> AggregateMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateMode", "path": "AggregateMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 23], "end": [285, 28], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aggregates/mod.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-127bc8aad430f502d82de38e"></a>
## eq

`function` · `datafusion_physical_plan::aggregates::AggregateMode::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &AggregateMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateMode", "path": "AggregateMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 30], "end": [285, 39], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aggregates/mod.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0518b9c2b2821595d078ca10"></a>
## fmt

`function` · `datafusion_physical_plan::aggregates::AggregateMode::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateMode", "path": "AggregateMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 10], "end": [285, 15], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregates/mod.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f61607f1e2af3c61ee3164d9"></a>
## hash

`function` · `datafusion_physical_plan::aggregates::AggregateMode::hash` · datafusion-physical-plan 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateMode", "path": "AggregateMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 45], "end": [285, 49], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/aggregates/mod.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cff56bb237b4b0e694981df"></a>
## input_mode

`function` · `datafusion_physical_plan::aggregates::AggregateMode::input_mode` · datafusion-physical-plan 55.1.0

```rust
fn input_mode(&self) -> AggregateInputMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateMode", "path": "AggregateMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [361, 1], "end": [394, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the [`AggregateInputMode`](../operations/datafusion_physical_plan.aggregates.AggregateInputMode.md#op-f7dada427bf42eb004cdb459) for this mode: whether this
stage consumes raw input data or intermediate accumulator state.

See the [table above](AggregateMode#variants-and-their-inputoutput-modes)
for details.

<a id="op-35faa34494009ff33a5783fc"></a>
## output_mode

`function` · `datafusion_physical_plan::aggregates::AggregateMode::output_mode` · datafusion-physical-plan 55.1.0

```rust
fn output_mode(&self) -> AggregateOutputMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateMode", "path": "AggregateMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [361, 1], "end": [394, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the [`AggregateOutputMode`](../operations/datafusion_physical_plan.aggregates.AggregateOutputMode.md#op-6147b1a705ac6e29745affac) for this mode: whether this
stage produces intermediate accumulator state or final output values.

See the [table above](AggregateMode#variants-and-their-inputoutput-modes)
for details.
