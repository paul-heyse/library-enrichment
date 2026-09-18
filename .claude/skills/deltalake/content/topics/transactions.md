# Transactions and commits

Log publication is delegated to the selected LogStore. Optimistic conflict checking and retries operate against intervening commits. Application transaction markers do not alone provide replay suppression at this pin, and errors can occur after a log entry is visible. Inspect the reviewed commit and replay contracts before designing recovery.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_core::kernel::transaction::CommitBuilder` | struct | 9 | [prose](../api/deltalake_core.kernel.transaction.md#commitbuilder) | [records](../model/deltalake_core.kernel.transaction.json) |
| `deltalake_core::kernel::transaction::CommitProperties` | struct | 9 | [prose](../api/deltalake_core.kernel.transaction.md#commitproperties) | [records](../model/deltalake_core.kernel.transaction.json) |
| `buoyant_kernel::committer::commit_types::CommitMetadata` | struct | 14 | [prose](../api/buoyant_kernel.committer.commit_types.md#commitmetadata) | [records](../model/buoyant_kernel.committer.commit_types.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `deltalake_core::operations::CustomExecuteHandler` | 4 | 0 | 1 | [CustomExecuteHandler](../traits/CustomExecuteHandler.md) |
| `buoyant_kernel::committer::Committer` | 3 | 0 | 1 | [Committer](../traits/Committer.md) |

## Configuration methods

Chainable `with_*` builders. These are invisible to anyone reading only the
constructor, which is why they are the most consistently missed part of the API.

**`CommitBuilder`** — 6 builder methods

`with_actions`, `with_app_metadata`, `with_max_retries`, `with_operation_id`, `with_post_commit_hook`, `with_post_commit_hook_handler`

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Set an app transaction id on any write that a scheduler or queue might retry.
- Put operation metadata in commit properties: it is the only durable record of why a version exists.

## Anti-patterns

- Retrying a failed write without an app transaction id, which silently duplicates data.
- Treating a commit failure as fatal when it is a conflict that a reload and retry resolves.

## Agent checklist

- Is an app transaction id set where retries are possible?
- Do commit properties record enough to explain the version later?
