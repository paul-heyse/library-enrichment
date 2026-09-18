# `datafusion_physical_expr::partitioning`

Crate `datafusion-physical-expr` · 4 public items · structured records in [`model/datafusion_physical_expr.partitioning.json`](../model/datafusion_physical_expr.partitioning.json)

## Distribution

`enum` · `datafusion_physical_expr::partitioning::Distribution`

Also reachable as `datafusion::physical_expr::Distribution`, `datafusion::physical_plan::Distribution`, `datafusion_physical_expr::Distribution`, `datafusion_physical_plan::Distribution`, `datafusion_physical_plan::execution_plan::Distribution`

```rust
enum Distribution
```

**Variants**: `UnspecifiedDistribution`, `SinglePartition`, `HashPartitioned`, `KeyPartitioned`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn create_partitioning(self, partition_count: usize) -> Partitioning
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.partitioning.Distribution.md).


How data is distributed amongst partitions. See [`Partitioning`] for more
details.

---

## Partitioning

`enum` · `datafusion_physical_expr::partitioning::Partitioning`

Also reachable as `datafusion::physical_expr::Partitioning`, `datafusion::physical_plan::Partitioning`, `datafusion_physical_expr::Partitioning`, `datafusion_physical_plan::Partitioning`, `datafusion_physical_plan::execution_plan::Partitioning`

```rust
enum Partitioning
```

**Variants**: `RoundRobinBatch`, `Hash`, `Range`, `UnknownPartitioning`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq

**Methods** (6)

```rust
fn partition_count(&self) -> usize
fn project(&self, mapping: &ProjectionMapping, input_eq_properties: &EquivalenceProperties) -> Self
fn satisfaction(&self, required: &Distribution, eq_properties: &EquivalenceProperties, allow_subset: bool) -> PartitioningSatisfaction
fn satisfy(&self, required: &Distribution, eq_properties: &EquivalenceProperties) -> bool
fn try_from_proto(node: &datafusion_proto_models::protobuf::Partitioning, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Option<Self>>
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<datafusion_proto_models::protobuf::Partitioning>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.partitioning.Partitioning.md).


Output partitioning supported by [`ExecutionPlan`]s.

Calling [`ExecutionPlan::execute`] produce one or more independent streams of
[`RecordBatch`]es in parallel, referred to as partitions. The streams are Rust
`async` [`Stream`]s (a special kind of future). The number of output
partitions varies based on the input and the operation performed.

For example, an `ExecutionPlan` that has output partitioning of 3 will
produce 3 distinct output streams as the result of calling
`ExecutionPlan::execute(0)`, `ExecutionPlan::execute(1)`, and
`ExecutionPlan::execute(2)`, as shown below:

```text
                                                  ...         ...        ...
              ...                                  ▲           ▲           ▲
                                                   │           │           │
               ▲                                   │           │           │
               │                                   │           │           │
               │                               ┌───┴────┐  ┌───┴────┐  ┌───┴────┐
    ┌────────────────────┐                     │ Stream │  │ Stream │  │ Stream │
    │   ExecutionPlan    │                     │  (0)   │  │  (1)   │  │  (2)   │
    └────────────────────┘                     └────────┘  └────────┘  └────────┘
               ▲                                   ▲           ▲           ▲
               │                                   │           │           │
    ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─                          │           │           │
            Input        │                         │           │           │
    └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─                          │           │           │
               ▲                               ┌ ─ ─ ─ ─   ┌ ─ ─ ─ ─   ┌ ─ ─ ─ ─
               │                                 Input  │    Input  │    Input  │
               │                               │ Stream    │ Stream    │ Stream
                                                  (0)   │     (1)   │     (2)   │
              ...                              └ ─ ▲ ─ ─   └ ─ ▲ ─ ─   └ ─ ▲ ─ ─
                                                   │           │           │
                                                   │           │           │
                                                   │           │           │

ExecutionPlan with 1 input                      3 (async) streams, one for each
that has 3 partitions, which itself             output partition
has 3 output partitions
```

It is common (but not required) that an `ExecutionPlan` has the same number
of input partitions as output partitions. However, some plans have different
numbers such as the `RepartitionExec` that redistributes batches from some
number of inputs to some number of outputs

```text
              ...                                     ...         ...        ...

                                                       ▲           ▲           ▲
               ▲                                       │           │           │
               │                                       │           │           │
      ┌────────┴───────────┐                           │           │           │
      │  RepartitionExec   │                      ┌────┴───┐  ┌────┴───┐  ┌────┴───┐
      └────────────────────┘                      │ Stream │  │ Stream │  │ Stream │
               ▲                                  │  (0)   │  │  (1)   │  │  (2)   │
               │                                  └────────┘  └────────┘  └────────┘
               │                                       ▲           ▲           ▲
               ...                                     │           │           │
                                                       └──────────┐│┌──────────┘
                                                                  │││
                                                                  │││
RepartitionExec with 1 input
partition and 3 output partitions                 3 (async) streams, that internally
                                                   pull from the same input stream
                                                                 ...
```

# Additional Examples

A simple `FileScanExec` might produce one output stream (partition) for each
file (note the actual DataFusion file scanners can read individual files in
parallel, potentially producing multiple partitions per file)

Plans such as `SortPreservingMerge` produce a single output stream
(1 output partition) by combining some number of input streams (input partitions)

Plans such as `FilterExec` produce the same number of output streams
(partitions) as input streams (partitions).

[`RecordBatch`]: arrow::record_batch::RecordBatch
[`ExecutionPlan::execute`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute
[`ExecutionPlan`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html
[`Stream`]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html

---

## PartitioningSatisfaction

`enum` · `datafusion_physical_expr::partitioning::PartitioningSatisfaction`

Also reachable as `datafusion::physical_expr::PartitioningSatisfaction`, `datafusion_physical_expr::PartitioningSatisfaction`

```rust
enum PartitioningSatisfaction
```

**Variants**: `NotSatisfied`, `Exact`, `Subset`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn is_satisfied(&self) -> bool
fn is_subset(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.partitioning.PartitioningSatisfaction.md).


Represents how a [`Partitioning`] satisfies a [`Distribution`] requirement.

---

## RangePartitioning

`struct` · `datafusion_physical_expr::partitioning::RangePartitioning`

Also reachable as `datafusion::physical_expr::RangePartitioning`, `datafusion::physical_plan::RangePartitioning`, `datafusion_physical_expr::RangePartitioning`, `datafusion_physical_plan::RangePartitioning`

```rust
struct RangePartitioning
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn new(ordering: LexOrdering, split_points: Vec<SplitPoint>) -> Self
fn ordering(&self) -> &LexOrdering
fn partition_count(&self) -> usize
fn split_points(&self) -> &[SplitPoint]
fn try_new(ordering: LexOrdering, split_points: Vec<SplitPoint>) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.partitioning.RangePartitioning.md).


Physical range partitioning.

[`RangePartitioning`] describes an ordered key space with split points.

- `ordering` defines the partitioning key and ordering.
- `split_points` define the boundaries between adjacent partitions.

Comparisons use the lexicographic order defined by `ordering`, including
`ASC`/`DESC` and null ordering. Split points must be strictly ordered
according to that ordering, and each split point must have one value per
ordering expression. See [`SplitPoint`] for the shared boundary convention.

Like other user-specified data properties such as sortedness, if a source
declares range partitioning, it is responsible for placing each row in the
partition described by the split points. DataFusion will not validate this is
upheld.

For a single range key:

```text
ordering = [date ASC NULLS LAST]
split_points = [
  (2022-01-01),
  (2023-01-01),
]

partition 0: date before 2022-01-01
partition 1: date between 2022-01-01 (inclusive) and 2023-01-01 (exclusive)
partition 2: date at/after 2023-01-01
```

The same model extends to compound keys.
For `ordering = [time ASC, city ASC]`, split points are ordered
lexicographically by `(time, city)`:

```text
ordering = [time ASC NULLS LAST, city ASC NULLS LAST]
split_points = [
  (2022, Allston),
  (2023, Allston),
]

partition 0: keys before  (2022, Allston)
partition 1: keys between (2022, Allston) and (2023, Allston)
partition 2: keys at/after (2023, Allston)
```

NOTE: Optimizer and execution behavior for this partitioning is intentionally
not implemented and will be introduced incrementally. See
<https://github.com/apache/datafusion/issues/22395>.

---
