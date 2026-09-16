# Foreign trait implementations

47 traits defined elsewhere are implemented here (plus 34 std/core
traits, omitted as noise). This is the whole integration seam, generated -- not a
curated highlight of the famous ones.

Two columns, because the distinction is load-bearing. **Public** implementors are types
you can name, store in a struct field and write in a signature. **Internal** ones live in
private modules: the impl is real and runs, and it is why the integration works, but you
cannot name the type -- you reach the behaviour through whatever public function returns
it. Do not read an internal implementor as an extension point you can substitute.

The trait is not in this index; only the implementations are. To read a foreign trait's
own contract, resolve its canonical path in that library's own reference.

Each row names at most four implementors. `content/index/foreign-impls.tsv` is the
complete table -- one row per implementation, with the implementor's full canonical path
and its `nameable` flag -- and is what to grep when a row here ends in `+n`.

| Foreign trait | Public | Internal | Public implementors | Internal implementors |
|---|---:|---:|---|---|
| `serde_core::de::Deserialize` | 120 | 20 | `Action`, `Add`, `AddCDCFile`, `ArrayData` +110 | `actions::ProtocolInner`, `client::LakeFSErrorResponse`, `clustering::ClusteringDomainMetadata`, `crc::CrcRaw` +16 |
| `serde_core::ser::Serialize` | 102 | 15 | `Action`, `Add`, `AddCDCFile`, `ArrayData` +94 | `actions::ProtocolInner`, `clustering::ClusteringDomainMetadata`, `crc::CrcRaw`, `log_replay::InternalScanState` +11 |
| `futures_core::stream::Stream` | 1 | 9 | `FileStream` | `barrier::MergeBarrierStream`, `column_mapping::ColumnMappingStream`, `data_validation::DataValidationStream`, `exec::DeltaScanStream` +5 |
| `datafusion_physical_plan::display::DisplayAs` | 1 | 8 | `DeltaScanExec` | `barrier::MergeBarrierExec`, `column_mapping::ColumnMappingExec`, `data_sink::DeltaDataSink`, `data_validation::DataValidationExec` +4 |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | 1 | 7 | `DeltaScanExec` | `barrier::MergeBarrierExec`, `column_mapping::ColumnMappingExec`, `data_validation::DataValidationExec`, `exec_meta::DeltaScanMetaExec` +3 |
| `datafusion_execution::stream::RecordBatchStream` | 0 | 7 | -- | `barrier::MergeBarrierStream`, `column_mapping::ColumnMappingStream`, `data_validation::DataValidationStream`, `exec::DeltaScanStream` +3 |
| `datafusion_session::planner::ExtensionPlanner` | 1 | 5 | `DeltaExtensionPlanner` | `data_validation::DataValidationExtensionPlanner`, `delete::DeleteMetricExtensionPlanner`, `merge::MergeMetricExtensionPlanner`, `metrics::WriteMetricExtensionPlanner` +1 |
| `object_store::ObjectStore` | 3 | 3 | `ConditionalPutShim`, `DeltaIOStorageBackend`, `S3StorageBackend` | `file::MountFileStorageBackend`, `sorted::SortedListStore`, `storage::GcsStorageBackend` |
| `tracing_core::field::Visit` | 0 | 5 | -- | `events::FileReadAttrs`, `events::ScanMetadataCompletedAttrs`, `events::StorageAttrs`, `events::TransactionCommitAttrs` +1 |
| `buoyant_kernel::schema::compare::SchemaComparison` | 3 | 1 | `DataType`, `StructField`, `StructType` | `compare::Nullable` |
| `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore` | 0 | 4 | -- | `barrier::MergeBarrier`, `data_validation::DataValidation`, `logical::MetricObserver`, `validation::MergeValidation` |
| `datafusion_common::pruning::PruningStatistics` | 2 | 1 | `EagerSnapshot`, `LogDataHandler` | `state::AddContainer` |
| `datafusion_expr::udf::ScalarUDFImpl` | 0 | 3 | -- | `datafusion::ZOrderUDF`, `expr::MakeParquetArray`, `to_json::ToJson` |
| `buoyant_kernel::action_reconciliation::RetentionCalculator` | 2 | 0 | `CheckpointWriter`, `LogCompactionWriter` | -- |
| `buoyant_kernel::kernel_predicates::parquet_stats_skipping::ParquetStatsProvider` | 0 | 2 | -- | `parquet_row_group_skipping::CheckpointRowGroupFilter`, `parquet_row_group_skipping::RowGroupFilter` |
| `buoyant_kernel::log_replay::deduplicator::Deduplicator` | 0 | 2 | -- | `deduplicator::CheckpointDeduplicator`, `log_replay::FileActionDeduplicator` |
| `datafusion_common::tree_node::TreeNodeVisitor` | 0 | 2 | -- | `data_validation::NotNullExtractor`, `find_files::FindFilesExprProperties` |
| `datafusion_session::schema::SchemaProvider` | 2 | 0 | `ListingSchemaProvider`, `UnitySchemaProvider` | -- |
| `datafusion_session::table::TableProvider` | 2 | 0 | `DeltaCdfTableProvider`, `DeltaScan` | -- |
| `serde_core::de::Visitor` | 0 | 2 | -- | `serde::EagerSnapshotVisitor`, `serde::SnapshotVisitor` |
| `arrow_json::writer::encoder::Encoder` | 0 | 1 | -- | `arrow_utils::NullValueMapEncoder` |
| `arrow_json::writer::encoder::EncoderFactory` | 0 | 1 | -- | `arrow_utils::NullValueMapEncoderFactory` |
| `aws_credential_types::provider::credentials::ProvideCredentials` | 0 | 1 | -- | `credentials::OptionsCredentialsProvider` |
| `buoyant_kernel::kernel_predicates::ResolveColumnAsScalar` | 0 | 1 | -- | `kernel_predicates::EmptyColumnResolver` |
| `buoyant_kernel::scan::field_classifiers::TransformFieldClassifier` | 0 | 1 | -- | `field_classifiers::CdfTransformFieldClassifier` |
| `buoyant_kernel::struct_patch::ExpressionItem` | 1 | 0 | `ExpressionRef` | -- |
| `buoyant_kernel::struct_patch::SchemaPatchItem` | 1 | 0 | `StructField` | -- |
| `datafusion_datasource::sink::DataSink` | 0 | 1 | -- | `data_sink::DeltaDataSink` |
| `datafusion_expr::planner::ContextProvider` | 0 | 1 | -- | `expr::DeltaContextProvider` |
| `datafusion_expr::planner::ExprPlanner` | 0 | 1 | -- | `expr::CustomNestedFunctionPlanner` |
| `datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapterFactory` | 0 | 1 | -- | `expr_adapter::DeltaPhysicalExprAdapterFactory` |
| `datafusion_proto::logical_plan::LogicalExtensionCodec` | 1 | 0 | `DeltaLogicalCodec` | -- |
| `datafusion_proto::physical_plan::PhysicalExtensionCodec` | 1 | 0 | `DeltaPhysicalCodec` | -- |
| `datafusion_session::catalog::CatalogProvider` | 1 | 0 | `UnityCatalogProvider` | -- |
| `datafusion_session::catalog::CatalogProviderList` | 1 | 0 | `UnityCatalogList` | -- |
| `datafusion_session::planner::QueryPlanner` | 1 | 0 | `DeltaPlanner` | -- |
| `datafusion_session::table::TableProviderFactory` | 1 | 0 | `DeltaTableFactory` | -- |
| `moka::policy::Expiry` | 0 | 1 | -- | `datafusion::TokenExpiry` |
| `object_store::client::CredentialProvider` | 0 | 1 | -- | `credentials::AWSForObjectStore` |
| `parquet::arrow::async_reader::AsyncFileReader` | 1 | 0 | `ParquetObjectReader` | -- |
| `parquet::arrow::async_writer::AsyncFileWriter` | 0 | 1 | -- | `writer::ParquetObjectWriter` |
| `sqlparser::ast::visitor::VisitorMut` | 0 | 1 | -- | `expr::SparkGeneratedColumnExprRewrite` |
| `strum::EnumCount` | 1 | 0 | `TableFeature` | -- |
| `strum::IntoEnumIterator` | 1 | 0 | `TableFeature` | -- |
| `tracing_subscriber::layer::Layer` | 1 | 0 | `ReportGeneratorLayer` | -- |
| `validator::traits::Validate` | 1 | 0 | `TableMetadataUpdate` | -- |
| `validator::traits::ValidateArgs` | 1 | 0 | `TableMetadataUpdate` | -- |

