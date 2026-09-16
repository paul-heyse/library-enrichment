#!/usr/bin/env bash
# Reproducible index lookups behind the Interface-checked claims in
# design_review_schema-engineering-typed-values_2026-09-16.md. Run from the repository root.
set -euo pipefail
DF=.claude/skills/datafusion/content
DL=.claude/skills/deltalake/content
q() { echo; echo '```'; echo "\$ $*"; eval "$*" 2>&1 | head -40; echo '```'; }
echo "# Index lookups — schema engineering review (2026-09-16)"
echo
echo "Pins: DataFusion 55.1.0, Arrow/Parquet 59.3.0, delta-rs 58f07cd6, kernel 8ba063f8f84fec222000f66d40d70911d7c79675 (skill PROVENANCE)."
echo "Each block is the exact query run against the prebuilt skill indexes and the first 40 lines it returned."
echo
echo "## DataFusion"
q "rg -P '^datafusion_expr::udf::ScalarUDFImpl\t(return_field_from_args|return_type|struct_field_mapping|simplify|coerce_types|is_nullable|output_ordering|preserves_lex_ordering|short_circuits)\t' $DF/index/methods.tsv | cut -f2,4"
q "rg -n 'struct ReturnFieldArgs|struct ScalarFunctionArgs|struct StructFieldMapping' -A3 $DF/api/datafusion_expr.udf.md | rg 'struct|Fields'"
q "rg -P '^datafusion_expr::expr_schema::ExprSchemable\t' $DF/index/methods.tsv | cut -f2,4 | sort -u"
q "rg -P '^datafusion_common::metadata::FieldMetadata\t' $DF/index/methods.tsv | cut -f2,4 | sort -u"
q "rg -P '^datafusion_expr::expr::Alias\twith_metadata|lit_with_metadata|datafusion_functions::core::expr_fn::with_metadata|datafusion_expr::expr::Cast\tnew_from_field' $DF/index/methods.tsv $DF/index/symbols.tsv | cut -f1,2 | sort -u"
q "rg -P '^arrow_schema::extension::' $DF/index/symbols.tsv | cut -f1,2"
q "rg -P '^arrow_schema::field::Field\t(with_extension_type|try_with_extension_type|try_extension_type|extension_type_name|extension_type_metadata|try_canonical_extension_type)\t' $DF/index/methods.tsv | cut -f2,4"
q "rg -n 'trait DFExtensionType|^- .datafusion_common::types::canonical_extensions' $DF/api/datafusion_common.types.extension.md"
q "rg -P '^datafusion_expr::registry::(ExtensionTypeRegistry|MemoryExtensionTypeRegistry|ExtensionTypeRegistration)\t' $DF/index/methods.tsv | cut -f1,2 | sort -u"
q "rg -P '^datafusion::execution::session_state::SessionStateBuilder\t(with_extension_type_registry|with_analyzer_rule|with_analyzer_rules|with_expr_planners|with_type_planner|with_query_planner)\t' $DF/index/methods.tsv | cut -f2,4"
q "rg -n 'name-based|positional|explicit .CAST' $DF/corpus/guides/user-guide/sql/struct_coercion.md | head -12"
q "rg -P '^arrow_schema::datatype::DataType\tequals_datatype\t|^datafusion_common::dfschema::DFSchema\t(has_equivalent_names_and_types|logically_equivalent_names_and_types)\t' $DF/index/methods.tsv | cut -f1,2,4"
q "rg -n 'Are struct columns accessed|Direct references to whole struct' $DF/api/datafusion_datasource_parquet.row_filter.md"
q "rg -P '^parquet::arrow::ProjectionMask\t(leaves|roots|columns)\t' $DF/index/methods.tsv | cut -f2,4"
q "rg -P '^datafusion::dataframe::DataFrame\tunnest_columns|^datafusion_common::unnest::UnnestOptions\t' $DF/index/methods.tsv | cut -f2,4"
q "rg -P '^datafusion_functions::core::expr_fn::(union_extract|union_tag)\t' $DF/index/symbols.tsv | cut -f1,2"
q "rg -P 'pushdown_filters|schema_force_view_types|bloom_filter_on_read|parquet.pruning' $DF/catalogs/config-options.md | cut -c1-120"
q "rg -P '^datafusion\tnested_expressions\t' $DF/index/features.tsv"
echo
echo "## delta-rs and kernel"
q "rg -P '^buoyant_kernel::schema::StructField\t(not_null|nullable|is_nullable|with_metadata|add_metadata|get_config_value|metadata_with_string_values|column_mapping_id|physical_name|can_read_as|try_from_arrow)\t' $DL/index/methods.tsv | cut -f2,4"
q "rg -n 'Variants' $DL/api/buoyant_kernel.schema.md | rg 'ColumnMappingId'"
q "rg -n 'Variants' $DL/api/buoyant_kernel.schema.md | rg 'Struct|Array|Map|Primitive' | head -3"
q "cat $DL/traits/DataCheck.md | rg 'fn |^- '"
q "cat $DL/traits/StructTypeExt.md | rg 'fn '"
q "rg -n 'DataSkippingNumIndexedCols|DataSkippingStatsColumns|CheckpointWriteStatsAsStruct|ColumnMappingMode' $DL/api/deltalake_core.table.config.md | cut -c1-160 | head -4"
q "rg -n 'Fields|fn ' $DL/api/deltalake_core.operations.write.configs.md"
q "rg -n 'fn |Verifies|nullCount' $DL/api/buoyant_kernel.transaction.stats_verifier.md"
q "rg -P '^buoyant_kernel::expressions::column_names::ColumnName\t(new|from_naive_str_split|parent|join|parse_column_name_list)\t' $DL/index/methods.tsv | cut -f2,4"
q "rg -n 'fn parse_sql_predicate_to_kernel|Only constructs the kernel' $DL/api/deltalake_core.delta_datafusion.expr.md"
q "rg -P '^deltalake_core::delta_datafusion::table_provider::DeltaScanConfig\t(with_schema|with_parquet_pushdown|with_file_column_name|with_wrap_partition_values)\t' $DL/index/methods.tsv | cut -f2,4"
q "rg -n 'fn scan\(|fn supports_filters_pushdown' $DL/api/deltalake_core.delta_datafusion.table_provider.next.md"
q "rg -n 'partitionColumns\|.Array\[String\]|clusteringColumns. is the name path|delta.columnMapping.nested.ids' $DL/corpus/protocol/PROTOCOL.md | cut -c1-140 | head -5"
q "rg -c 'stats_from_parquet_metadata|fn check_batch' $DL/corpus -g '*.rs' || echo 'index-silent: delta-rs crates/core source is not in the corpus'"
q "ls $DL/corpus"

echo
echo "## Follow-up lookups after the capability-inventory cross-check"
q "rg -P '^datafusion_common::nested_struct::' $DF/index/symbols.tsv | cut -f1,2"
q "rg -n 'by name|at least one common|only struct columns can be cast|name-based nested struct casting' $DF/api/datafusion_common.nested_struct.md"
q "rg -n -A3 'Unsupported Casts' $DF/api/arrow_cast.cast.md"
q "rg -n 'nested_struct|cast_column' $DF/api/datafusion_physical_expr.expressions.cast.md || echo 'index-silent: CastExpr prose does not say which cast it selects'"
q "rg -P 'skip_metadata|skip_arrow_metadata' $DF/catalogs/config-options.md | cut -c1-120"
q "rg -P '^arrow-schema\tcanonical_extension_types' $DF/index/features.tsv"
q "rg -P '^datafusion_physical_expr_adapter::schema_rewriter::(PhysicalExprAdapterFactory|BatchAdapterFactory)\t' $DF/index/symbols.tsv | cut -f1,2; rg -n 'Nested struct fields are recursively adapted|Deprecated' $DF/api/datafusion_physical_expr_adapter.schema_rewriter.md $DF/api/datafusion_datasource.schema_adapter.md | head -4"
q "rg -n -B6 'delta-kernel-rs/issues/1075' $DL/corpus/tests/it_datafusion/integration_datafusion.rs | head -12"
q "rg -n 'async fn test_zorder_nested_columns|async fn test_zorder_rejects_invalid_nested_path|ZOrder\(vec!' $DL/corpus/tests/it_datafusion/command_optimize.rs | head -6"
q "rg -n 'async fn test_schema_merge_append_missing_non_nullable_column|Invalid data found' $DL/corpus/tests/it_datafusion/integration_datafusion.rs | head -4"
q "rg -n 'NonNullFieldChecker|InvariantChecker' $DL/api/buoyant_kernel.transforms.schema.md"
q "rg -P '^deltalake_core::operations::write::WriteBuilder\t(with_cast_safety|with_schema_mode|with_partition_columns)\t' $DL/index/methods.tsv | cut -f2,4"
q "rg -P '^buoyant_kernel::schema::DataType\t(unshredded_variant|variant_type)\t' $DL/index/methods.tsv | cut -f2,4"
