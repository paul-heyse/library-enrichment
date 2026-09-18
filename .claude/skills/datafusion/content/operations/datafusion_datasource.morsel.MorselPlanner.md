# `datafusion_datasource::morsel::MorselPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.morsel.MorselPlanner.json).

<a id="op-75e3192888d61e52d84762e9"></a>
## MorselPlanner

`trait` · `datafusion_datasource::morsel::MorselPlanner` · datafusion-datasource 55.1.0

```rust
trait MorselPlanner: Send + Debug
```

Source: `src/morsel/mod.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A Morsel Planner is responsible for creating morsels for a given scan.

The [`MorselPlanner`](../operations/datafusion_datasource.morsel.MorselPlanner.md#op-75e3192888d61e52d84762e9) is the unit of I/O. There is only ever a single I/O
outstanding for a specific planner. DataFusion may run
multiple planners in parallel, which corresponds to multiple parallel
I/O requests.

It is not a Rust `Stream` so that it can explicitly separate CPU bound
work from I/O work.

The design is similar to `ParquetPushDecoder`: when `plan` is called, it
should do CPU work to produce the next morsels or discover the next I/O
phase.

Best practice is to spawn I/O in a Tokio task on a separate runtime to
ensure that CPU work doesn't block or slow down I/O work, but this is not
strictly required by the API.

<a id="op-14db0580d35832f565a1984f"></a>
## plan

`function` · `datafusion_datasource::morsel::MorselPlanner::plan` · datafusion-datasource 55.1.0

```rust
fn plan(Box<self>) -> Result<Option<MorselPlan>>
```

Source: `src/morsel/mod.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Attempt to plan morsels. This may involve CPU work, such as parsing
parquet metadata and evaluating pruning predicates.

It should NOT do any I/O work, such as reading from the file. If I/O is
required, the returned [`MorselPlan`](../operations/datafusion_datasource.morsel.MorselPlan.md#op-5accb7e9b2b1602fe2da379e) should contain a pending planner
future that the caller polls to drive the I/O work to completion. Once
that future resolves, it yields a planner ready for work.

Note this function is **not async** to make it explicitly clear that if
I/O is required, it should be done in the returned `io_future`.

Returns `None` if the planner has no more work to do.

# Empty Morsel Plans

It may return `None`, which means no batches will be read from the file
(e.g. due to late-pruning based on statistics).

# Output Ordering

See the comments on [`MorselPlan`](../operations/datafusion_datasource.morsel.MorselPlan.md#op-5accb7e9b2b1602fe2da379e) for the logical output order.
