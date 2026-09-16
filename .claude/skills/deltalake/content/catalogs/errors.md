# Errors

17 error enums, 183 variants. Matching on a variant is the only way to
tell a retryable failure from a permanent one -- a stringified error cannot be matched,
and the retry decision is the whole reason the variants exist.

| Error | Variants | Defined in |
|---|---:|---|
| `Error` | 49 | `buoyant_kernel` |
| `DeltaTableError` | 35 | `deltalake-core` |
| `Error` | 19 | `deltalake-core` |
| `UnityCatalogError` | 15 | `deltalake-catalog-unity` |
| `CommitConflictError` | 11 | `deltalake-core` |
| `TransactionError` | 9 | `deltalake-core` |
| `LakeFSOperationError` | 7 | `deltalake-lakefs` |
| `LocalFileSystemError` | 6 | `deltalake-mount` |
| `LogHistoryError` | 6 | `buoyant_kernel` |
| `Error` | 5 | `buoyant_kernel` |
| `ParseIntervalError` | 5 | `buoyant_kernel` |
| `DataCatalogError` | 4 | `deltalake-core` |
| `Error` | 4 | `deltalake-mount` |
| `LakeFSConfigError` | 3 | `deltalake-lakefs` |
| `GlueError` | 2 | `deltalake-catalog-glue` |
| `OnError` | 2 | `buoyant_kernel_engine` |
| `DeltaConfigError` | 1 | `deltalake-core` |

## `buoyant_kernel::error::Error`

`Arrow`, `Backtraced`, `ChangeDataFeedIncompatibleSchema`, `ChangeDataFeedUnsupported`, `CheckpointWrite`, `ChecksumWriteUnsupported`, `DeletionVector`, `EngineDataType`, `Extract`, `FileAlreadyExists`, `FileNotFound`, `Generic`, `GenericError`, `IOError`, `InternalError`, `InvalidCheckpoint`, `InvalidColumnMappingMode`, `InvalidDecimal`, `InvalidExpressionEvaluation`, `InvalidLogPath`, `InvalidPartitionValues`, `InvalidProtocol`, `InvalidSelectionVector`, `InvalidStructData`, `InvalidTableLocation`, `InvalidTransactionState`, `InvalidUrl`, `JoinFailure`, `LiteralExpressionTransformError`, `LogHistory`, `MalformedJson`, `MissingColumn`, `MissingData`, `MissingMetadata`, `MissingMetadataAndProtocol`, `MissingProtocol`, `MissingVersion`, `ObjectStore`, `ObjectStorePath`, `Parquet`, `ParseError`, `ParseIntError`, `ParseIntervalError`, `Reqwest`, `Schema`, `StatsValidation`, `UnexpectedColumnType`, `Unsupported`, `Utf8Error`

## `deltalake_core::errors::DeltaTableError`

`Arrow`, `ChangeDataInvalidVersionRange`, `ChangeDataNotEnabled`, `ChangeDataNotRecorded`, `ChangeDataTimestampGreaterThanCommit`, `CommitValidation`, `Generic`, `GenericError`, `InvalidData`, `InvalidDateTimeString`, `InvalidJsonLog`, `InvalidPartitionFilter`, `InvalidStatsJson`, `InvalidTableLocation`, `InvalidVersion`, `Io`, `Kernel`, `KernelError`, `MetadataError`, `MissingFeature`, `NoSchema`, `NoStartingVersionOrTimestamp`, `NotATable`, `NotInitialized`, `NotInitializedWithFiles`, `ObjectStore`, `Parquet`, `PartitionError`, `SchemaMismatch`, `SerializeLogJson`, `Transaction`, `UnsupportedColumnMapping`, `VersionAlreadyExists`, `VersionDowngrade`, `VersionMismatch`

## `deltalake_core::kernel::error::Error`

`Arrow`, `DeletionVector`, `FileNotFound`, `Generic`, `GenericError`, `InvalidGenerationExpressionJson`, `InvalidInvariantJson`, `InvalidUrl`, `MalformedJson`, `MetadataError`, `MissingColumn`, `MissingData`, `MissingMetadata`, `MissingVersion`, `ObjectStore`, `Parquet`, `Parse`, `Schema`, `UnexpectedColumnType`

## `deltalake_catalog_unity::UnityCatalogError`

`AzureCli`, `DatafusionError`, `FederatedTokenFile`, `Generic`, `InitializationError`, `InvalidCredentials`, `InvalidHeader`, `InvalidTable`, `InvalidTableURI`, `MissingConfiguration`, `MissingCredential`, `NotATable`, `RequestError`, `RequestMiddlewareError`, `TemporaryCredentialsFetchFailure`

## `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError`

`ConcurrentAppend`, `ConcurrentDeleteDelete`, `ConcurrentDeleteRead`, `ConcurrentTransaction`, `CorruptedState`, `MetadataChanged`, `NoMetadata`, `Predicate`, `ProtocolChanged`, `UnsupportedReaderVersion`, `UnsupportedWriterVersion`

## `deltalake_core::kernel::transaction::TransactionError`

`CommitConflict`, `DeltaTableAppendOnly`, `LogStoreError`, `MaxCommitAttempts`, `ObjectStore`, `SerializeLogJson`, `TableFeaturesRequired`, `UnsupportedTableFeatures`, `VersionAlreadyExists`

