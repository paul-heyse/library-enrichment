# Operation construction and results

Awaitable builders are one subset. Explicit build/execute and writer flush/commit paths
are included below. Constructor edges use nominal structured return types; clause
closures do not construct standalone delete/update operations. Defaults and effects
are characterized in the [task routes](../routes/tasks.md).

| Owner | Access | Constructed by | Awaited result / explicit methods |
|---|---|---|---|
| `buoyant_kernel::commit_range::builder::CommitRangeBuilder` | public | buoyant_kernel::commit_range::CommitRange::builder_for; buoyant_kernel::commit_range::CommitRange::builder_from | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::engine::arrow_utils::RowIndexBuilder` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::incremental_scan::IncrementalScanBuilder` | public | buoyant_kernel::snapshot::Snapshot::incremental_scan_builder | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::scan::Scan` | public | buoyant_kernel::scan::ScanBuilder::build | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::scan::ScanBuilder` | public | buoyant_kernel::snapshot::Snapshot::scan_builder | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::schema::StructTypeBuilder` | public | buoyant_kernel::schema::StructType::builder | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::snapshot::builder::SnapshotBuilder` | public | buoyant_kernel::snapshot::Snapshot::builder_for; buoyant_kernel::snapshot::Snapshot::builder_from | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::struct_patch::StructPatchBuilder` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::table_changes::scan::TableChangesScan` | public | buoyant_kernel::table_changes::scan::TableChangesScanBuilder::build | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::table_changes::scan::TableChangesScanBuilder` | public | buoyant_kernel::table_changes::TableChanges::into_scan_builder; buoyant_kernel::table_changes::TableChanges::scan_builder | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder` | public | buoyant_kernel::snapshot::Snapshot::alter_table | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder` | public | buoyant_kernel::transaction::create_table::create_table | Explicit build/execute/flush; see full contracts |
| `buoyant_kernel_engine::DefaultEngineBuilder` | public | buoyant_kernel_engine::DefaultEngine::builder | Explicit build/execute/flush; see full contracts |
| `deltalake_aws::storage::S3StorageOptionsBuilder` | public | deltalake_aws::storage::S3StorageOptions::builder | Explicit build/execute/flush; see full contracts |
| `deltalake_catalog_unity::UnityCatalogBuilder` | public | deltalake_catalog_unity::UnityCatalogBuilderBuilder::build | Explicit build/execute/flush; see full contracts |
| `deltalake_catalog_unity::UnityCatalogBuilderBuilder` | public | deltalake_catalog_unity::UnityCatalogBuilder::builder | Explicit build/execute/flush; see full contracts |
| `deltalake_catalog_unity::client::ClientOptionsBuilder` | public | deltalake_catalog_unity::client::ClientOptions::builder | Explicit build/execute/flush; see full contracts |
| `deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder` | public | deltalake_core::delta_datafusion::table_provider::next::DeltaScan::builder; deltalake_core::table::DeltaTable::table_provider | [`type Output = Result<Arc<dyn TableProvider>, DataFusionError>`](../operations/deltalake_core.delta_datafusion.table_provider.TableProviderBuilder.md#op-0c281d39cb0a8cd338ebf0ea) |
| `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `deltalake_core::kernel::snapshot::scan::ScanBuilder` | public | deltalake_core::kernel::snapshot::Snapshot::into_scan_builder; deltalake_core::kernel::snapshot::Snapshot::scan_builder | Explicit build/execute/flush; see full contracts |
| `deltalake_core::kernel::transaction::CommitBuilder` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `deltalake_core::kernel::transaction::PostCommit` | public | Inspect full contract | [`type Output = Result<FinalizedCommit, DeltaTableError>`](../operations/deltalake_core.kernel.transaction.PostCommit.md#op-ac053af56f7b4e9c6fe51d14) |
| `deltalake_core::kernel::transaction::PreCommit` | public | deltalake_core::kernel::transaction::CommitBuilder::build | [`type Output = Result<FinalizedCommit, DeltaTableError>`](../operations/deltalake_core.kernel.transaction.PreCommit.md#op-0cd645fcfb1b90c75917013c) |
| `deltalake_core::kernel::transaction::PreparedCommit` | public | deltalake_core::kernel::transaction::PreCommit::into_prepared_commit_future | [`type Output = Result<PostCommit, DeltaTableError>`](../operations/deltalake_core.kernel.transaction.PreparedCommit.md#op-e0895b2b6ea14709609bf3db) |
| `deltalake_core::operations::add_column::AddColumnBuilder` | public | deltalake_core::table::DeltaTable::add_columns | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.add_column.AddColumnBuilder.md#op-7bf27a1c1116b23c09ab7711) |
| `deltalake_core::operations::add_feature::AddTableFeatureBuilder` | public | deltalake_core::table::DeltaTable::add_feature | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.add_feature.AddTableFeatureBuilder.md#op-ab0b9e6f2956284f8ea5dabd) |
| `deltalake_core::operations::constraints::ConstraintBuilder` | public | deltalake_core::table::DeltaTable::add_constraint | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.constraints.ConstraintBuilder.md#op-19fc6dbb2244e5637abd5844) |
| `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder` | public | Inspect full contract | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.convert_to_delta.ConvertToDeltaBuilder.md#op-7f7837bf5255218791d79d9f) |
| `deltalake_core::operations::create::CreateBuilder` | public | deltalake_core::table::DeltaTable::create | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.create.CreateBuilder.md#op-1a0707e153b235b5dd8a02c0) |
| `deltalake_core::operations::delete::DeleteBuilder` | public | deltalake_core::table::DeltaTable::delete | [`type Output = Result<(DeltaTable, DeleteMetrics), DeltaTableError>`](../operations/deltalake_core.operations.delete.DeleteBuilder.md#op-e74c7e65dd2b30f69a94a0bb) |
| `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder` | public | deltalake_core::table::DeltaTable::drop_column_not_null | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.drop_column_not_null.DropColumnNotNullBuilder.md#op-ba1104da68a37c21c75e6331) |
| `deltalake_core::operations::drop_constraints::DropConstraintBuilder` | public | deltalake_core::table::DeltaTable::drop_constraints | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.drop_constraints.DropConstraintBuilder.md#op-99ddcc79315dd2eb57dfe025) |
| `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder` | public | deltalake_core::table::DeltaTable::filesystem_check | [`type Output = Result<(DeltaTable, FileSystemCheckMetrics), DeltaTableError>`](../operations/deltalake_core.operations.filesystem_check.FileSystemCheckBuilder.md#op-45e4fee27d345f6eb907d916) |
| `deltalake_core::operations::generate::GenerateBuilder` | public | deltalake_core::table::DeltaTable::generate | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.generate.GenerateBuilder.md#op-af8f7de3469e11ef6c1bf3a1) |
| `deltalake_core::operations::load::LoadBuilder` | returned_inferred | deltalake_core::table::DeltaTable::scan_table | [`type Output = Result<(DeltaTable, Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>), DeltaTableError>`](../operations/deltalake_core.operations.load.LoadBuilder.md#op-10d61fc5ea060fbd1511386d) |
| `deltalake_core::operations::load_cdf::CdfLoadBuilder` | public | deltalake_core::table::DeltaTable::scan_cdf | Explicit build/execute/flush; see full contracts |
| `deltalake_core::operations::merge::MergeBuilder` | public | deltalake_core::table::DeltaTable::merge | [`type Output = Result<(DeltaTable, MergeMetrics), DeltaTableError>`](../operations/deltalake_core.operations.merge.MergeBuilder.md#op-246dbe6c66042a837a13c116) |
| `deltalake_core::operations::optimize::MergePlan` | public | deltalake_core::operations::optimize::create_merge_plan | Explicit build/execute/flush; see full contracts |
| `deltalake_core::operations::optimize::OptimizeBuilder` | public | deltalake_core::table::DeltaTable::optimize | [`type Output = Result<(DeltaTable, Metrics), DeltaTableError>`](../operations/deltalake_core.operations.optimize.OptimizeBuilder.md#op-bff0e9f1314e94985cdd4cdc) |
| `deltalake_core::operations::restore::RestoreBuilder` | public | deltalake_core::table::DeltaTable::restore | [`type Output = Result<(DeltaTable, RestoreMetrics), DeltaTableError>`](../operations/deltalake_core.operations.restore.RestoreBuilder.md#op-ad7a1ac890da137f644102e9) |
| `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder` | public | deltalake_core::table::DeltaTable::set_tbl_properties | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.set_tbl_properties.SetTablePropertiesBuilder.md#op-61c8cc899ecd1ff8064668b1) |
| `deltalake_core::operations::update::UpdateBuilder` | public | deltalake_core::table::DeltaTable::update | [`type Output = Result<(DeltaTable, UpdateMetrics), DeltaTableError>`](../operations/deltalake_core.operations.update.UpdateBuilder.md#op-92fb88dd0cb8c29f60b9d8d9) |
| `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder` | public | deltalake_core::table::DeltaTable::update_field_metadata | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.update_field_metadata.UpdateFieldMetadataBuilder.md#op-27c672b97f12a0c812fd954b) |
| `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder` | public | deltalake_core::table::DeltaTable::update_table_metadata | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.update_table_metadata.UpdateTableMetadataBuilder.md#op-354fb5541f3974c38a3acd0a) |
| `deltalake_core::operations::vacuum::VacuumBuilder` | public | deltalake_core::table::DeltaTable::vacuum | [`type Output = Result<(DeltaTable, VacuumMetrics), DeltaTableError>`](../operations/deltalake_core.operations.vacuum.VacuumBuilder.md#op-9efeffefd9cbc54190d5ac4a) |
| `deltalake_core::operations::write::WriteBuilder` | public | deltalake_core::table::DeltaTable::write | [`type Output = Result<DeltaTable, DeltaTableError>`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-3b3adb069c90d8711cc194f5) |
| `deltalake_core::table::builder::DeltaTableBuilder` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `deltalake_core::writer::DeltaWriter` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `deltalake_core::writer::json::JsonWriter` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `deltalake_core::writer::record_batch::RecordBatchWriter` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |
| `deltalake_core::writer::utils::ShareableBuffer` | public | Inspect full contract | Explicit build/execute/flush; see full contracts |

[Structured map with configuration and method contracts](operation-map.json).
Returned-inferred types are callable through public return values; their private module
canonical paths are not import paths. Internal trait methods are excluded from caller routes.
