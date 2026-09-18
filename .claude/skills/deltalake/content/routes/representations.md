# Representation routes

| Representation | Reviewed decisions |
|---|---|
| (DeltaTable, DeleteMetrics) | [Update or delete through built-in Delta operations](../capabilities/delta.dml.md) |
| (DeltaTable, MergeMetrics) | [Merge with explicit key, null, clause and duplicate semantics](../capabilities/delta.merge.md) |
| (DeltaTable, Metrics) | [Compact or Z-order while preserving logical data](../capabilities/delta.optimize.md) |
| (DeltaTable, UpdateMetrics) | [Update or delete through built-in Delta operations](../capabilities/delta.dml.md) |
| (DeltaTable, VacuumMetrics) | [Preview and execute vacuum with explicit historical-read consequences](../capabilities/delta.retention.md) |
| Add actions | [Write batches or a logical plan; distinguish staging from publication](../capabilities/delta.write.md) |
| Arc<dyn ExecutionPlan> | [Read change data with explicit bounds, images and residual filtering](../capabilities/delta.cdf.md) |
| Arc<dyn Session> | [Preserve session capabilities and configure Delta planning](../capabilities/delta.session.md) |
| Arc<dyn TableProvider> | [Choose a Delta-aware scan or table provider](../capabilities/delta.read.md) |
| Arrow RecordBatch | [Choose a Delta-aware scan or table provider](../capabilities/delta.read.md) |
| Arrow Schema | [Preserve schema meaning across Arrow, Delta and schema evolution](../capabilities/delta.schema.md) |
| Cargo feature profile | [Separate feature vocabulary, protocol admission and operation support](../capabilities/delta.features.md) |
| CommitProperties | [Reason about optimistic commits and errors after publication](../capabilities/delta.commit.md), [Use transaction markers without assuming replay suppression](../capabilities/delta.replay.md) |
| DataFrame source | [Merge with explicit key, null, clause and duplicate semantics](../capabilities/delta.merge.md) |
| DataFusion catalog/provider | [Resolve table names separately from storage and provider freshness](../capabilities/delta.catalog.md) |
| DataFusion filters | [Choose a Delta-aware scan or table provider](../capabilities/delta.read.md) |
| DeltaCdfTableProvider | [Read change data with explicit bounds, images and residual filtering](../capabilities/delta.cdf.md) |
| DeltaSessionContext | [Preserve session capabilities and configure Delta planning](../capabilities/delta.session.md) |
| DeltaTable | [Update or delete through built-in Delta operations](../capabilities/delta.dml.md), [Open, load and select a table version](../capabilities/delta.open.md), [Compact or Z-order while preserving logical data](../capabilities/delta.optimize.md), [Preview and execute vacuum with explicit historical-read consequences](../capabilities/delta.retention.md), [Write batches or a logical plan; distinguish staging from publication](../capabilities/delta.write.md) |
| DeltaTable snapshot | [Choose a Delta-aware scan or table provider](../capabilities/delta.read.md) |
| Engine | [Use kernel snapshots, scans and engine interfaces at the correct layer](../capabilities/delta.kernel.md) |
| EngineData | [Use kernel snapshots, scans and engine interfaces at the correct layer](../capabilities/delta.kernel.md) |
| LogStore | [Choose object storage and log publication adapters](../capabilities/delta.storage.md) |
| LogicalPlan | [Write batches or a logical plan; distinguish staging from publication](../capabilities/delta.write.md) |
| ObjectStore | [Choose object storage and log publication adapters](../capabilities/delta.storage.md) |
| OptimizeType | [Compact or Z-order while preserving logical data](../capabilities/delta.optimize.md) |
| RecordBatchWriter | [Write batches or a logical plan; distinguish staging from publication](../capabilities/delta.write.md) |
| RuntimeEnv | [Preserve session capabilities and configure Delta planning](../capabilities/delta.session.md) |
| SchemaMode | [Preserve schema meaning across Arrow, Delta and schema evolution](../capabilities/delta.schema.md) |
| SendableRecordBatchStream | [Choose a Delta-aware scan or table provider](../capabilities/delta.read.md) |
| SessionState | [Preserve session capabilities and configure Delta planning](../capabilities/delta.session.md) |
| StorageOptions | [Choose object storage and log publication adapters](../capabilities/delta.storage.md) |
| UDF registry | [Preserve session capabilities and configure Delta planning](../capabilities/delta.session.md) |
| URL scheme | [Choose object storage and log publication adapters](../capabilities/delta.storage.md) |
| VacuumMode | [Preview and execute vacuum with explicit historical-read consequences](../capabilities/delta.retention.md) |
| Vec<RecordBatch> | [Write batches or a logical plan; distinguish staging from publication](../capabilities/delta.write.md) |
| actions | [Reason about optimistic commits and errors after publication](../capabilities/delta.commit.md) |
| admission result | [Separate feature vocabulary, protocol admission and operation support](../capabilities/delta.features.md) |
| application ID and version | [Use transaction markers without assuming replay suppression](../capabilities/delta.replay.md) |
| backend publication behavior | [Choose object storage and log publication adapters](../capabilities/delta.storage.md) |
| catalog client options | [Resolve table names separately from storage and provider freshness](../capabilities/delta.catalog.md) |
| catalog/schema/table identity | [Resolve table names separately from storage and provider freshness](../capabilities/delta.catalog.md) |
| change rows and metadata columns | [Read change data with explicit bounds, images and residual filtering](../capabilities/delta.cdf.md) |
| checkpoint artifacts | [Choose time travel, restore, checkpoints or log maintenance](../capabilities/delta.history.md) |
| column expressions | [Update or delete through built-in Delta operations](../capabilities/delta.dml.md) |
| committed version | [Reason about optimistic commits and errors after publication](../capabilities/delta.commit.md), [Write batches or a logical plan; distinguish staging from publication](../capabilities/delta.write.md) |
| configured table storage | [Choose object storage and log publication adapters](../capabilities/delta.storage.md) |
| deleted or candidate file paths | [Preview and execute vacuum with explicit historical-read consequences](../capabilities/delta.retention.md) |
| engine I/O results | [Use kernel snapshots, scans and engine interfaces at the correct layer](../capabilities/delta.kernel.md) |
| evolved table metadata | [Preserve schema meaning across Arrow, Delta and schema evolution](../capabilities/delta.schema.md) |
| execution hooks | [Reason about optimistic commits and errors after publication](../capabilities/delta.commit.md) |
| filtered engine data | [Use kernel snapshots, scans and engine interfaces at the correct layer](../capabilities/delta.kernel.md) |
| historical snapshot | [Choose time travel, restore, checkpoints or log maintenance](../capabilities/delta.history.md) |
| join predicate | [Merge with explicit key, null, clause and duplicate semantics](../capabilities/delta.merge.md) |
| kept versions | [Preview and execute vacuum with explicit historical-read consequences](../capabilities/delta.retention.md) |
| kernel StructType | [Preserve schema meaning across Arrow, Delta and schema evolution](../capabilities/delta.schema.md) |
| kernel schema/snapshot | [Use kernel snapshots, scans and engine interfaces at the correct layer](../capabilities/delta.kernel.md) |
| loaded snapshot | [Open, load and select a table version](../capabilities/delta.open.md) |
| loaded table | [Read change data with explicit bounds, images and residual filtering](../capabilities/delta.cdf.md), [Choose time travel, restore, checkpoints or log maintenance](../capabilities/delta.history.md), [Choose full overwrite, predicate replacement or merge](../capabilities/delta.replace.md) |
| new DeltaTable version | [Choose full overwrite, predicate replacement or merge](../capabilities/delta.replace.md) |
| new restored version | [Choose time travel, restore, checkpoints or log maintenance](../capabilities/delta.history.md) |
| normalized Delta schema | [Preserve schema meaning across Arrow, Delta and schema evolution](../capabilities/delta.schema.md) |
| nullable/nested arrays | [Preserve schema meaning across Arrow, Delta and schema evolution](../capabilities/delta.schema.md) |
| operation | [Separate feature vocabulary, protocol admission and operation support](../capabilities/delta.features.md) |
| operation-specific decision | [Separate feature vocabulary, protocol admission and operation support](../capabilities/delta.features.md) |
| optional ending bound | [Read change data with explicit bounds, images and residual filtering](../capabilities/delta.cdf.md) |
| ordered clauses | [Merge with explicit key, null, clause and duplicate semantics](../capabilities/delta.merge.md) |
| partition selection | [Compact or Z-order while preserving logical data](../capabilities/delta.optimize.md) |
| post-commit result | [Reason about optimistic commits and errors after publication](../capabilities/delta.commit.md) |
| predicate | [Update or delete through built-in Delta operations](../capabilities/delta.dml.md) |
| predicate Expr or SQL | [Choose full overwrite, predicate replacement or merge](../capabilities/delta.replace.md) |
| preserved rows outside predicate | [Choose full overwrite, predicate replacement or merge](../capabilities/delta.replace.md) |
| projection | [Choose a Delta-aware scan or table provider](../capabilities/delta.read.md) |
| replacement batches | [Choose full overwrite, predicate replacement or merge](../capabilities/delta.replace.md) |
| resolved operation session | [Preserve session capabilities and configure Delta planning](../capabilities/delta.session.md) |
| restore target | [Choose time travel, restore, checkpoints or log maintenance](../capabilities/delta.history.md) |
| retention interval | [Preview and execute vacuum with explicit historical-read consequences](../capabilities/delta.retention.md) |
| rewritten files | [Compact or Z-order while preserving logical data](../capabilities/delta.optimize.md) |
| scan metadata | [Use kernel snapshots, scans and engine interfaces at the correct layer](../capabilities/delta.kernel.md) |
| snapshot transaction state | [Use transaction markers without assuming replay suppression](../capabilities/delta.replay.md) |
| snapshot/read set | [Reason about optimistic commits and errors after publication](../capabilities/delta.commit.md) |
| starting version or timestamp | [Read change data with explicit bounds, images and residual filtering](../capabilities/delta.cdf.md) |
| storage options | [Open, load and select a table version](../capabilities/delta.open.md) |
| table URL | [Open, load and select a table version](../capabilities/delta.open.md) |
| table protocol | [Separate feature vocabulary, protocol admission and operation support](../capabilities/delta.features.md) |
| table storage location | [Resolve table names separately from storage and provider freshness](../capabilities/delta.catalog.md) |
| table version/history | [Choose time travel, restore, checkpoints or log maintenance](../capabilities/delta.history.md) |
| target size | [Compact or Z-order while preserving logical data](../capabilities/delta.optimize.md) |
| transaction_version lookup | [Use transaction markers without assuming replay suppression](../capabilities/delta.replay.md) |
| txn action | [Use transaction markers without assuming replay suppression](../capabilities/delta.replay.md) |
| updated snapshot | [Reason about optimistic commits and errors after publication](../capabilities/delta.commit.md) |
| version or timestamp | [Open, load and select a table version](../capabilities/delta.open.md) |
