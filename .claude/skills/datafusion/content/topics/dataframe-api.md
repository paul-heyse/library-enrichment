# DataFrame API

A `DataFrame` is a `LogicalPlan` plus the `SessionState` that will execute it. Every transformation returns a new `DataFrame`; nothing runs until a terminal method is called. `collect()` materializes everything, `execute_stream()` yields batches as they are produced. The 63 methods include a great deal that is not obvious from the common examples — `join_on`, `unnest_columns`, `repartition`, `cache`, `describe`, `write_table`.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion::dataframe::DataFrame` | struct | 63 | [prose](../api/datafusion.dataframe.md#dataframe) | [records](../model/datafusion.dataframe.json) |
| `datafusion::dataframe::DataFrameWriteOptions` | struct | 6 | [prose](../api/datafusion.dataframe.md#dataframewriteoptions) | [records](../model/datafusion.dataframe.json) |
| `datafusion::execution::context::SessionContext` | struct | 92 | [prose](../api/datafusion.execution.context.md#sessioncontext) | [records](../model/datafusion.execution.context.json) |

## Runnable examples (4)

- [`corpus/examples/dataframe/cache_factory.rs`](../corpus/examples/dataframe/cache_factory.rs)
- [`corpus/examples/dataframe/dataframe.rs`](../corpus/examples/dataframe/dataframe.rs)
- [`corpus/examples/dataframe/deserialize_to_struct.rs`](../corpus/examples/dataframe/deserialize_to_struct.rs)
- [`corpus/examples/dataframe/main.rs`](../corpus/examples/dataframe/main.rs)

## Upstream guides

- [`corpus/guides/user-guide/dataframe.md`](../corpus/guides/user-guide/dataframe.md)
- [`corpus/guides/library-user-guide/using-the-dataframe-api.md`](../corpus/guides/library-user-guide/using-the-dataframe-api.md)

## Decision rules

- Prefer `execute_stream()` over `collect()` whenever the result is not known to be small.
- `into_parts()` drops to the `LogicalPlan` and `SessionState` when you need to run the analyzer, optimizer and physical planner yourself.
- `cache()` materializes an intermediate result when a plan reuses it several times.

## Anti-patterns

- `collect()` followed by iteration that could have been a streaming consumer.
- Rebuilding a plan per row group instead of expressing the whole thing once and letting the optimizer partition it.

## Agent checklist

- Is the terminal method streaming or materializing, and is that deliberate?
- Has `EXPLAIN` been read for a plan that will run more than once?
