# Delete, update and merge

Delta has no in-place update. Every DML operation rewrites the files it touches and commits `Remove` plus `Add`, so the cost is set by how many files the predicate reaches, not by how many rows change. A predicate that prunes to one file is cheap; the same predicate over an unpartitioned column is a table rewrite.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_core::operations::delete::DeleteBuilder` | struct | 11 | [prose](../api/deltalake_core.operations.delete.md#deletebuilder) | [records](../model/deltalake_core.operations.delete.json) |
| `deltalake_core::operations::update::UpdateBuilder` | struct | 11 | [prose](../api/deltalake_core.operations.update.md#updatebuilder) | [records](../model/deltalake_core.operations.update.json) |
| `deltalake_core::operations::merge::MergeBuilder` | struct | 19 | [prose](../api/deltalake_core.operations.merge.md#mergebuilder) | [records](../model/deltalake_core.operations.merge.json) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Write the predicate against a partition or statistics column so file skipping can do the work.
- Merge needs source and target aliases to disambiguate; set them rather than relying on defaults.
- Deletion vectors turn a delete into a metadata operation instead of a rewrite -- but only if the table feature is enabled.

## Anti-patterns

- A merge predicate that matches more than one source row per target row. The result is not an error, it is wrong data -- deduplicate the source first.
- Ordering merge clauses generic-before-specific. Clauses are evaluated in order and the first match wins, so a broad update clause placed first silently shadows the delete or the narrower update you wrote after it.
- Unqualified column references in a merge predicate, which resolve ambiguously wherever the two schemas share a name -- the normal case for a join key.
- Omitting the predicate on a delete or update. Both are legal without one and both then rewrite the entire table.
- Using delete-then-append to emulate an update, which doubles the rewrite and breaks atomicity.

## Agent checklist

- Does the predicate prune files, or scan everything?
- Are merge source and target aliases set?
- Would deletion vectors avoid the rewrite entirely?
