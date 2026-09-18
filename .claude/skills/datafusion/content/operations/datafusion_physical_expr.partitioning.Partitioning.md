# `datafusion_physical_expr::partitioning::Partitioning`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.partitioning.Partitioning.json).

<a id="op-b631e1d1890e8ea81608b4a4"></a>
## Partitioning

`enum` · `datafusion_physical_expr::partitioning::Partitioning` · datafusion-physical-expr 55.1.0

```rust
enum Partitioning
```

Source: `src/partitioning.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

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

<a id="op-2e61f3626627abaac1462fec"></a>
## Hash

`variant` · `datafusion_physical_expr::partitioning::Partitioning::Hash` · datafusion-physical-expr 55.1.0

```rust
Hash
```

Source: `src/partitioning.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Allocate rows based on a hash of one of more expressions and the specified number of
partitions

<a id="op-416521ebc0d82658601f1876"></a>
## Range

`variant` · `datafusion_physical_expr::partitioning::Partitioning::Range` · datafusion-physical-expr 55.1.0

```rust
Range
```

Source: `src/partitioning.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Partition rows by source-declared ranges

<a id="op-c43db61e90295e278e145c28"></a>
## RoundRobinBatch

`variant` · `datafusion_physical_expr::partitioning::Partitioning::RoundRobinBatch` · datafusion-physical-expr 55.1.0

```rust
RoundRobinBatch
```

Source: `src/partitioning.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Allocate batches using a round-robin algorithm and the specified number of partitions

<a id="op-744bc6600f997e21b89b9460"></a>
## UnknownPartitioning

`variant` · `datafusion_physical_expr::partitioning::Partitioning::UnknownPartitioning` · datafusion-physical-expr 55.1.0

```rust
UnknownPartitioning
```

Source: `src/partitioning.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Unknown partitioning scheme with a known number of partitions

<a id="op-f9deb76c0d855776e37bb371"></a>
## clone

`function` · `datafusion_physical_expr::partitioning::Partitioning::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> Partitioning
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 17], "end": [120, 22], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/partitioning.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bbf8bfc3daef13a28936d78"></a>
## eq

`function` · `datafusion_physical_expr::partitioning::Partitioning::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Partitioning) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [688, 2], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/partitioning.rs:673`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1750e9b66266f3dcfd93d784"></a>
## fmt

`function` · `datafusion_physical_expr::partitioning::Partitioning::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 10], "end": [120, 15], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/partitioning.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf73350d6cd4c9ad0a8b6150"></a>
## fmt

`function` · `datafusion_physical_expr::partitioning::Partitioning::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [151, 2], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/partitioning.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5595e9c77293dc1be328b2f8"></a>
## partition_count

`function` · `datafusion_physical_expr::partitioning::Partitioning::partition_count` · datafusion-physical-expr 55.1.0

```rust
fn partition_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [520, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the number of partitions in this partitioning scheme

<a id="op-e4c83fb0ae46c77931da0381"></a>
## project

`function` · `datafusion_physical_expr::partitioning::Partitioning::project` · datafusion-physical-expr 55.1.0

```rust
fn project(&self, mapping: &ProjectionMapping, input_eq_properties: &EquivalenceProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [520, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:490`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Calculate the output partitioning after applying the given projection.

<a id="op-6969e0e55418a29d57c89477"></a>
## satisfaction

`function` · `datafusion_physical_expr::partitioning::Partitioning::satisfaction` · datafusion-physical-expr 55.1.0

```rust
fn satisfaction(&self, required: &Distribution, eq_properties: &EquivalenceProperties, allow_subset: bool) -> PartitioningSatisfaction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [520, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:402`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns how this [`Partitioning`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-b631e1d1890e8ea81608b4a4) satisfies the partitioning scheme mandated
by the `required` [`Distribution`](../operations/datafusion_physical_expr.partitioning.Distribution.md#op-1570134c2bb176dfcdc9c836).

<a id="op-1fe9af867ab18c752360709a"></a>
## satisfy

`function` · `datafusion_physical_expr::partitioning::Partitioning::satisfy` · datafusion-physical-expr 55.1.0

```rust
fn satisfy(&self, required: &Distribution, eq_properties: &EquivalenceProperties) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [520, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3ff452970d24fbacd73216d"></a>
## try_from_proto

`function` · `datafusion_physical_expr::partitioning::Partitioning::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::Partitioning, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Option<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [533, 1], "end": [647, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:592`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`Partitioning`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-b631e1d1890e8ea81608b4a4) from its protobuf representation.

Returns `Ok(None)` when the message carries no `partition_method`, which
the wire format uses to mean "no output partitioning declared"; callers
for which it is required should turn that into their own error.

<a id="op-016fcced53cecab826c263f8"></a>
## try_to_proto

`function` · `datafusion_physical_expr::partitioning::Partitioning::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<datafusion_proto_models::protobuf::Partitioning>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [533, 1], "end": [647, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:535`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Serialize this partitioning into its protobuf representation.
