# DataFusion 55 / Arrow 59 integration seam

This skill contains the exact preserved trait/member records for Session, TableProvider,
ExecutionPlan and RecordBatch in [datafusion-arrow-contracts.json](datafusion-arrow-contracts.json).
They retain raw type trees, full documentation, artifact identity and source spans from the
DataFusion 55.1.0 / Arrow 59.3.0 capture. Their documentation references may name external APIs;
those names are not promises that every external contract is included here.

A Delta provider supplies a schema, filter-pushdown decisions and a scan plan. DataFusion's
Session supplies registries, runtime and planning context. ExecutionPlan execution produces
partitioned RecordBatch streams; consuming a stream can still fail. RecordBatch schema and
array types are the boundary to inspect before casting/downcasting. Delta planning additionally
requires its extension planner and stores; an arbitrary DataFusion session is not equivalent.

Use the Delta task routes for selection. No installed sibling skill is required to read these
contracts. A broader DataFusion/Arrow reference is optional for work beyond this seam.
