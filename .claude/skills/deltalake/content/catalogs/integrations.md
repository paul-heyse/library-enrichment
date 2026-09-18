# Integration routes

Full impl identity remains in foreign-impls.tsv. Routes below are authored entry
leads, not an inferred call graph. Nameable does not by itself establish constructibility.

| Trait | Implementor | Nameable | Public entry route | Decision |
|---|---|---|---|---|
| `datafusion_common::pruning::PruningStatistics` | `deltalake_core::kernel::snapshot::EagerSnapshot` | True | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_common::pruning::PruningStatistics` | `deltalake_core::kernel::snapshot::log_data::LogDataHandler` | True | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_common::pruning::PruningStatistics` | `deltalake_core::kernel::transaction::state::AddContainer` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_common::tree_node::TreeNodeVisitor` | `deltalake_core::delta_datafusion::data_validation::NotNullExtractor` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_common::tree_node::TreeNodeVisitor` | `deltalake_core::delta_datafusion::find_files::FindFilesExprProperties` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_datasource::sink::DataSink` | `deltalake_core::delta_datafusion::table_provider::data_sink::DeltaDataSink` | False | DeltaTable::write and validation/planning machinery | [delta.write](../capabilities/delta.write.md) |
| `datafusion_execution::stream::RecordBatchStream` | `deltalake_core::delta_datafusion::column_mapping::ColumnMappingStream` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_execution::stream::RecordBatchStream` | `deltalake_core::delta_datafusion::data_validation::DataValidationStream` | False | DeltaTable::write and validation/planning machinery | [delta.write](../capabilities/delta.write.md) |
| `datafusion_execution::stream::RecordBatchStream` | `deltalake_core::delta_datafusion::physical::MetricObserverStream` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_execution::stream::RecordBatchStream` | `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanStream` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_execution::stream::RecordBatchStream` | `deltalake_core::delta_datafusion::table_provider::next::scan::exec_meta::DeltaScanMetaStream` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_execution::stream::RecordBatchStream` | `deltalake_core::operations::merge::barrier::MergeBarrierStream` | False | DeltaTable::merge | [delta.merge](../capabilities/delta.merge.md) |
| `datafusion_execution::stream::RecordBatchStream` | `deltalake_core::operations::merge::validation::MergeValidationStream` | False | DeltaTable::merge | [delta.merge](../capabilities/delta.merge.md) |
| `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore` | `deltalake_core::delta_datafusion::data_validation::DataValidation` | False | DeltaTable::write and validation/planning machinery | [delta.write](../capabilities/delta.write.md) |
| `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore` | `deltalake_core::delta_datafusion::logical::MetricObserver` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore` | `deltalake_core::operations::merge::barrier::MergeBarrier` | False | DeltaTable::merge | [delta.merge](../capabilities/delta.merge.md) |
| `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore` | `deltalake_core::operations::merge::validation::MergeValidation` | False | DeltaTable::merge | [delta.merge](../capabilities/delta.merge.md) |
| `datafusion_expr::planner::ContextProvider` | `deltalake_core::delta_datafusion::expr::DeltaContextProvider` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_expr::planner::ExprPlanner` | `deltalake_core::delta_datafusion::expr::CustomNestedFunctionPlanner` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_expr::udf::ScalarUDFImpl` | `deltalake_core::delta_datafusion::engine::expressions::to_json::ToJson` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_expr::udf::ScalarUDFImpl` | `deltalake_core::delta_datafusion::expr::MakeParquetArray` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_expr::udf::ScalarUDFImpl` | `deltalake_core::operations::optimize::zorder::datafusion::ZOrderUDF` | False | DeltaTable::optimize with ZOrder | [delta.optimize](../capabilities/delta.optimize.md) |
| `datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapterFactory` | `deltalake_core::delta_datafusion::table_provider::next::scan::expr_adapter::DeltaPhysicalExprAdapterFactory` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::display::DisplayAs` | `deltalake_core::delta_datafusion::column_mapping::ColumnMappingExec` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::display::DisplayAs` | `deltalake_core::delta_datafusion::data_validation::DataValidationExec` | False | DeltaTable::write and validation/planning machinery | [delta.write](../capabilities/delta.write.md) |
| `datafusion_physical_plan::display::DisplayAs` | `deltalake_core::delta_datafusion::physical::MetricObserverExec` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::display::DisplayAs` | `deltalake_core::delta_datafusion::table_provider::DeltaScan` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::display::DisplayAs` | `deltalake_core::delta_datafusion::table_provider::data_sink::DeltaDataSink` | False | DeltaTable::write and validation/planning machinery | [delta.write](../capabilities/delta.write.md) |
| `datafusion_physical_plan::display::DisplayAs` | `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec` | True | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::display::DisplayAs` | `deltalake_core::delta_datafusion::table_provider::next::scan::exec_meta::DeltaScanMetaExec` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::display::DisplayAs` | `deltalake_core::operations::merge::barrier::MergeBarrierExec` | False | DeltaTable::merge | [delta.merge](../capabilities/delta.merge.md) |
| `datafusion_physical_plan::display::DisplayAs` | `deltalake_core::operations::merge::validation::MergeValidationExec` | False | DeltaTable::merge | [delta.merge](../capabilities/delta.merge.md) |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | `deltalake_core::delta_datafusion::column_mapping::ColumnMappingExec` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | `deltalake_core::delta_datafusion::data_validation::DataValidationExec` | False | DeltaTable::write and validation/planning machinery | [delta.write](../capabilities/delta.write.md) |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | `deltalake_core::delta_datafusion::physical::MetricObserverExec` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | `deltalake_core::delta_datafusion::table_provider::DeltaScan` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec` | True | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | `deltalake_core::delta_datafusion::table_provider::next::scan::exec_meta::DeltaScanMetaExec` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | `deltalake_core::operations::merge::barrier::MergeBarrierExec` | False | DeltaTable::merge | [delta.merge](../capabilities/delta.merge.md) |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | `deltalake_core::operations::merge::validation::MergeValidationExec` | False | DeltaTable::merge | [delta.merge](../capabilities/delta.merge.md) |
| `datafusion_proto::logical_plan::LogicalExtensionCodec` | `deltalake_core::delta_datafusion::DeltaLogicalCodec` | True | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_proto::physical_plan::PhysicalExtensionCodec` | `deltalake_core::delta_datafusion::DeltaPhysicalCodec` | True | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_session::catalog::CatalogProvider` | `deltalake_catalog_unity::datafusion::UnityCatalogProvider` | True | catalog/provider registration | [delta.catalog](../capabilities/delta.catalog.md) |
| `datafusion_session::catalog::CatalogProviderList` | `deltalake_catalog_unity::datafusion::UnityCatalogList` | True | catalog/provider registration | [delta.catalog](../capabilities/delta.catalog.md) |
| `datafusion_session::planner::ExtensionPlanner` | `deltalake_core::delta_datafusion::data_validation::DataValidationExtensionPlanner` | False | DeltaTable::write and validation/planning machinery | [delta.write](../capabilities/delta.write.md) |
| `datafusion_session::planner::ExtensionPlanner` | `deltalake_core::delta_datafusion::planner::DeltaExtensionPlanner` | True | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_session::planner::ExtensionPlanner` | `deltalake_core::operations::delete::DeleteMetricExtensionPlanner` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_session::planner::ExtensionPlanner` | `deltalake_core::operations::merge::MergeMetricExtensionPlanner` | False | DeltaTable::merge | [delta.merge](../capabilities/delta.merge.md) |
| `datafusion_session::planner::ExtensionPlanner` | `deltalake_core::operations::update::UpdateMetricExtensionPlanner` | False | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_session::planner::ExtensionPlanner` | `deltalake_core::operations::write::metrics::WriteMetricExtensionPlanner` | False | DeltaTable::write and validation/planning machinery | [delta.write](../capabilities/delta.write.md) |
| `datafusion_session::planner::QueryPlanner` | `deltalake_core::delta_datafusion::planner::DeltaPlanner` | True | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_session::schema::SchemaProvider` | `deltalake_catalog_unity::datafusion::UnitySchemaProvider` | True | catalog/provider registration | [delta.catalog](../capabilities/delta.catalog.md) |
| `datafusion_session::schema::SchemaProvider` | `deltalake_core::data_catalog::storage::ListingSchemaProvider` | True | catalog/provider registration | [delta.catalog](../capabilities/delta.catalog.md) |
| `datafusion_session::table::TableProvider` | `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider` | True | DeltaTable::scan_cdf → DeltaCdfTableProvider | [delta.cdf](../capabilities/delta.cdf.md) |
| `datafusion_session::table::TableProvider` | `deltalake_core::delta_datafusion::table_provider::next::DeltaScan` | True | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `datafusion_session::table::TableProviderFactory` | `deltalake_core::delta_datafusion::DeltaTableFactory` | True | DeltaTable::table_provider / scan_table | [delta.read](../capabilities/delta.read.md) |
| `object_store::ObjectStore` | `deltalake_aws::storage::S3StorageBackend` | True | logstore/object-store factories | [delta.storage](../capabilities/delta.storage.md) |
| `object_store::ObjectStore` | `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend` | True | logstore/object-store factories | [delta.storage](../capabilities/delta.storage.md) |
| `object_store::ObjectStore` | `deltalake_gcp::storage::GcsStorageBackend` | False | logstore/object-store factories | [delta.storage](../capabilities/delta.storage.md) |
| `object_store::ObjectStore` | `deltalake_mount::file::MountFileStorageBackend` | False | logstore/object-store factories | [delta.storage](../capabilities/delta.storage.md) |
| `object_store::ObjectStore` | `deltalake_opendal::shim::ConditionalPutShim` | True | logstore/object-store factories | [delta.storage](../capabilities/delta.storage.md) |
| `object_store::ObjectStore` | `deltalake_opendal::sorted::SortedListStore` | False | logstore/object-store factories | [delta.storage](../capabilities/delta.storage.md) |
| `object_store::client::CredentialProvider` | `deltalake_aws::credentials::AWSForObjectStore` | False | logstore/object-store factories | [delta.storage](../capabilities/delta.storage.md) |
