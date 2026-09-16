# Errors and observability

Errors are enums for a reason: the retry decision depends on which variant it is, and a stringified error cannot be matched. A commit conflict is retryable, a protocol mismatch never is, and an object-store failure depends. Operations return metrics alongside the new table, which is the only record of what the operation actually did.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_core::errors::DeltaTableError` | enum | 24 | [prose](../api/deltalake_core.errors.md#deltatableerror) | [records](../model/deltalake_core.errors.json) |
| `buoyant_kernel::error::Error` | enum | 42 | [prose](../api/buoyant_kernel.error.md#error) | [records](../model/buoyant_kernel.error.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `deltalake_core::operations::CustomExecuteHandler` | 4 | 0 | 1 | [CustomExecuteHandler](../traits/CustomExecuteHandler.md) |
| `buoyant_kernel::metrics::reporter::MetricsReporter` | 1 | 0 | 1 | [MetricsReporter](../traits/MetricsReporter.md) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Match on the variant to decide retryability; never on the message.
- Record operation metrics: they are how you find out an optimize compacted nothing.

## Anti-patterns

- Collapsing every failure into one error type and retrying all of them.
- Discarding the metrics an operation returns and inferring success from the absence of an error.

## Agent checklist

- Does the retry policy branch on variants?
- Are operation metrics recorded somewhere durable?
