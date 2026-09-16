# Capability map

Fifteen capability axes. Each page maps a capability to the types that provide it, the traits that extend it, and the upstream code that demonstrates it. Everything below the mental model is generated from the pinned model, so the mapping cannot drift from the API.

| Topic | Covers |
|---|---|
| [Opening tables](table-identity.md) | URLs, storage options, time travel, loading, and the cheap stats-free path |
| [Snapshots and log replay](snapshot-and-log.md) | Log segments, checkpoints, eager and lazy snapshots, file listings |
| [Schema and types](schema-and-types.md) | The kernel schema model, Arrow conversion, nested types, and schema evolution |
| [Writing data](writing.md) | Save modes, partitioning, predicate overwrite, writer properties, target file size |
| [Delete, update and merge](dml.md) | Predicate DML, merge clause families, and what each rewrites |
| [Change data feed](change-data-feed.md) | Reading row-level changes between versions |
| [Maintenance](maintenance.md) | Compaction, Z-order, vacuum, restore, filesystem check, manifest generation |
| [Protocol, features and constraints](protocol-and-constraints.md) | Reader and writer versions, table features, check constraints, column nullability |
| [Transactions and commits](transactions.md) | Commit properties, application transaction ids, conflict handling, isolation |
| [Storage and log stores](storage.md) | Object stores, log store factories, backend registration, S3 and the conditional-put problem |
| [The DataFusion seam](datafusion.md) | Table providers, scans, pushdown, expressions, and session wiring |
| [Catalogs](catalogs.md) | Unity Catalog, Glue, and the catalog trait |
| [The kernel boundary](kernel.md) | What delta-rs delegates to the Delta kernel, and the engine traits |
| [Expressions and predicates](expressions.md) | Kernel expressions, predicates, data skipping, and column references |
| [Errors and observability](errors.md) | The error taxonomy, operation metrics, and execution hooks |
