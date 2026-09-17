# The DataFusion boundary

This repository indexes the instrumentation, not the thing instrumented. DataFusion, Arrow, Parquet and object_store are deliberately absent, and their access paths land in `index/unresolved.tsv`. That is a boundary, not a gap -- say which you mean when you report silence.

## What this seam cannot tell you

- Anything about `ExecutionPlan`, `SessionStateBuilder`, `PhysicalOptimizerRule`, `MetricsSet` or `ObjectStore` beyond how this library attaches to them.
- Tell you whether a DataFusion version composes with a datafusion-tracing version beyond the rows in `catalogs/compatibility.md`.
- A DataFusion optimizer rule's behaviour, even though `Rule` spans name them.

## Before you call it done

- Is the question about the instrumentation, or about the query engine?
