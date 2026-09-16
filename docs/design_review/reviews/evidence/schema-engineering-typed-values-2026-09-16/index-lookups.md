# Index lookups — schema engineering review (2026-09-16)

Pins: DataFusion 55.1.0, Arrow/Parquet 59.3.0, delta-rs 58f07cd6, kernel 8ba063f8f84fec222000f66d40d70911d7c79675 (skill PROVENANCE).
Each block is the exact query run against the prebuilt skill indexes and the first 40 lines it returned.

## DataFusion

```
$ rg -P '^datafusion_expr::udf::ScalarUDFImpl\t(return_field_from_args|return_type|struct_field_mapping|simplify|coerce_types|is_nullable|output_ordering|preserves_lex_ordering|short_circuits)\t' .claude/skills/datafusion/content/index/methods.tsv | cut -f2,4
coerce_types	fn coerce_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
is_nullable	fn is_nullable(&self, _args: &[Expr], _schema: &dyn ExprSchema) -> bool
output_ordering	fn output_ordering(&self, inputs: &[ExprProperties]) -> Result<SortProperties>
preserves_lex_ordering	fn preserves_lex_ordering(&self, _inputs: &[ExprProperties]) -> Result<bool>
return_field_from_args	fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
return_type	fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
short_circuits	fn short_circuits(&self) -> bool
simplify	fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
struct_field_mapping	fn struct_field_mapping(&self, _literal_args: &[Option<ScalarValue>]) -> Option<StructFieldMapping>
```

```
$ rg -n 'struct ReturnFieldArgs|struct ScalarFunctionArgs|struct StructFieldMapping' -A3 .claude/skills/datafusion/content/api/datafusion_expr.udf.md | rg 'struct|Fields'
12:struct ReturnFieldArgs<'a>
15-**Fields**: `arg_fields`, `scalar_arguments`
38:struct ScalarFunctionArgs
41-**Fields**: `args`, `arg_fields`, `number_rows`, `return_field`, `config_options`
150:struct StructFieldMapping
153-**Fields**: `field_accessor`, `fields`
```

```
$ rg -P '^datafusion_expr::expr_schema::ExprSchemable\t' .claude/skills/datafusion/content/index/methods.tsv | cut -f2,4 | sort -u
cast_to	fn cast_to(self, cast_to_type: &DataType, schema: &dyn ExprSchema) -> Result<Expr>
data_type_and_nullable	fn data_type_and_nullable(&self, schema: &dyn ExprSchema) -> Result<(DataType, bool)>
get_type	fn get_type(&self, schema: &dyn ExprSchema) -> Result<DataType>
metadata	fn metadata(&self, schema: &dyn ExprSchema) -> Result<FieldMetadata>
nullable	fn nullable(&self, input_schema: &dyn ExprSchema) -> Result<bool>
to_field	fn to_field(&self, input_schema: &dyn ExprSchema) -> Result<(Option<TableReference>, Arc<Field>)>
```

```
$ rg -P '^datafusion_common::metadata::FieldMetadata\t' .claude/skills/datafusion/content/index/methods.tsv | cut -f2,4 | sort -u
add_to_field	fn add_to_field(&self, field: Field) -> Field
add_to_field_ref	fn add_to_field_ref(&self, field_ref: FieldRef) -> FieldRef
extend	fn extend(&mut self, other: Self)
from	fn from(field: &Field) -> Self
from	fn from(inner: BTreeMap<String, String>) -> Self
from	fn from(map: &HashMap<String, String>) -> Self
from	fn from(map: HashMap<String, String>) -> Self
from	fn from(map: &std::collections::HashMap<String, String>) -> Self
from	fn from(map: std::collections::HashMap<String, String>) -> Self
inner	fn inner(&self) -> &BTreeMap<String, String>
into_inner	fn into_inner(self) -> Arc<BTreeMap<String, String>>
is_empty	fn is_empty(&self) -> bool
len	fn len(&self) -> usize
merge_options	fn merge_options(m: Option<&FieldMetadata>, n: Option<&FieldMetadata>) -> Option<FieldMetadata>
new_empty	fn new_empty() -> Self
new	fn new(inner: BTreeMap<String, String>) -> Self
new_from_field	fn new_from_field(field: &Field) -> Self
to_hashmap	fn to_hashmap(&self) -> std::collections::HashMap<String, String>
```

```
$ rg -P '^datafusion_expr::expr::Alias\twith_metadata|lit_with_metadata|datafusion_functions::core::expr_fn::with_metadata|datafusion_expr::expr::Cast\tnew_from_field' .claude/skills/datafusion/content/index/methods.tsv .claude/skills/datafusion/content/index/symbols.tsv | cut -f1,2 | sort -u
.claude/skills/datafusion/content/index/methods.tsv:datafusion_expr::expr::Alias	with_metadata
.claude/skills/datafusion/content/index/methods.tsv:datafusion_expr::expr::Cast	new_from_field
.claude/skills/datafusion/content/index/symbols.tsv:datafusion_expr::literal::lit_with_metadata	function
.claude/skills/datafusion/content/index/symbols.tsv:datafusion_functions::core::expr_fn::with_metadata	function
```

```
$ rg -P '^arrow_schema::extension::' .claude/skills/datafusion/content/index/symbols.tsv | cut -f1,2
arrow_schema::extension::EXTENSION_TYPE_METADATA_KEY	constant
arrow_schema::extension::EXTENSION_TYPE_NAME_KEY	constant
arrow_schema::extension::ExtensionType	trait
arrow_schema::extension::canonical::CanonicalExtensionType	enum
arrow_schema::extension::canonical::bool8::Bool8	struct
arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor	struct
arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata	struct
arrow_schema::extension::canonical::json::Json	struct
arrow_schema::extension::canonical::json::JsonMetadata	struct
arrow_schema::extension::canonical::opaque::Opaque	struct
arrow_schema::extension::canonical::opaque::OpaqueMetadata	struct
arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset	struct
arrow_schema::extension::canonical::uuid::Uuid	struct
arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor	struct
arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata	struct
```

```
$ rg -P '^arrow_schema::field::Field\t(with_extension_type|try_with_extension_type|try_extension_type|extension_type_name|extension_type_metadata|try_canonical_extension_type)\t' .claude/skills/datafusion/content/index/methods.tsv | cut -f2,4
extension_type_metadata	fn extension_type_metadata(&self) -> Option<&str>
extension_type_name	fn extension_type_name(&self) -> Option<&str>
try_canonical_extension_type	fn try_canonical_extension_type(&self) -> Result<CanonicalExtensionType, ArrowError>
try_extension_type	fn try_extension_type<E: ExtensionType>(&self) -> Result<E, ArrowError>
try_with_extension_type	fn try_with_extension_type<E: ExtensionType>(&mut self, extension_type: E) -> Result<(), ArrowError>
with_extension_type	fn with_extension_type<E: ExtensionType>(self, extension_type: E) -> Self
```

```
$ rg -n 'trait DFExtensionType|^- .datafusion_common::types::canonical_extensions' .claude/skills/datafusion/content/api/datafusion_common.types.extension.md
10:trait DFExtensionType: Debug + Send + Sync
15:- `datafusion_common::types::canonical_extensions::bool8::DFBool8`
16:- `datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor`
17:- `datafusion_common::types::canonical_extensions::json::DFJson`
18:- `datafusion_common::types::canonical_extensions::opaque::DFOpaque`
19:- `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset`
20:- `datafusion_common::types::canonical_extensions::uuid::DFUuid`
21:- `datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor`
```

```
$ rg -P '^datafusion_expr::registry::(ExtensionTypeRegistry|MemoryExtensionTypeRegistry|ExtensionTypeRegistration)\t' .claude/skills/datafusion/content/index/methods.tsv | cut -f1,2 | sort -u
datafusion_expr::registry::ExtensionTypeRegistration	create_df_extension_type
datafusion_expr::registry::ExtensionTypeRegistration	new_arc
datafusion_expr::registry::ExtensionTypeRegistration	type_name
datafusion_expr::registry::ExtensionTypeRegistry	add_extension_type_registration
datafusion_expr::registry::ExtensionTypeRegistry	create_extension_type_for_field
datafusion_expr::registry::ExtensionTypeRegistry	extend
datafusion_expr::registry::ExtensionTypeRegistry	extension_type_registration
datafusion_expr::registry::ExtensionTypeRegistry	extension_type_registrations
datafusion_expr::registry::ExtensionTypeRegistry	remove_extension_type_registration
datafusion_expr::registry::MemoryExtensionTypeRegistry	add_extension_type_registration
datafusion_expr::registry::MemoryExtensionTypeRegistry	all_extension_types
datafusion_expr::registry::MemoryExtensionTypeRegistry	extension_type_registration
datafusion_expr::registry::MemoryExtensionTypeRegistry	extension_type_registrations
datafusion_expr::registry::MemoryExtensionTypeRegistry	from
datafusion_expr::registry::MemoryExtensionTypeRegistry	new_empty
datafusion_expr::registry::MemoryExtensionTypeRegistry	new_with_canonical_extension_types
datafusion_expr::registry::MemoryExtensionTypeRegistry	new_with_types
datafusion_expr::registry::MemoryExtensionTypeRegistry	remove_extension_type_registration
```

```
$ rg -P '^datafusion::execution::session_state::SessionStateBuilder\t(with_extension_type_registry|with_analyzer_rule|with_analyzer_rules|with_expr_planners|with_type_planner|with_query_planner)\t' .claude/skills/datafusion/content/index/methods.tsv | cut -f2,4
with_analyzer_rule	fn with_analyzer_rule(self, analyzer_rule: Arc<dyn AnalyzerRule + Send + Sync>) -> Self
with_analyzer_rules	fn with_analyzer_rules(self, rules: Vec<Arc<dyn AnalyzerRule + Send + Sync>>) -> Self
with_expr_planners	fn with_expr_planners(self, expr_planners: Vec<Arc<dyn ExprPlanner>>) -> Self
with_extension_type_registry	fn with_extension_type_registry(self, registry: ExtensionTypeRegistryRef) -> Self
with_query_planner	fn with_query_planner(self, query_planner: Arc<dyn QueryPlanner + Send + Sync>) -> Self
with_type_planner	fn with_type_planner(self, type_planner: Arc<dyn TypePlanner>) -> Self
```

```
$ rg -n 'name-based|positional|explicit .CAST' .claude/skills/datafusion/content/corpus/guides/user-guide/sql/struct_coercion.md | head -12
22:DataFusion uses **name-based field mapping** when coercing struct types across different operations. This document explains how struct coercion works, when it applies, and how to handle NULL fields.
26:When combining structs from different sources (e.g., in UNION, array construction, or JOINs), DataFusion matches struct fields by **name** rather than by **position**. This provides more robust and predictable behavior compared to positional matching.
40:The following query operations use name-based field mapping for struct coercion:
233:If you have existing code that relied on **positional** struct field matching, you may need to update it.
237:**Old behavior (positional):**
240:-- These would have been positionally mapped (left-to-right)
242:-- Old result (positional): [{"x": 1, "y": 2}, {"y": 3, "x": 4}]
245:**New behavior (name-based):**
248:-- Now uses name-based matching
262:If you need precise control over struct field order and types, use explicit `CAST`:
```

```
$ rg -P '^arrow_schema::datatype::DataType\tequals_datatype\t|^datafusion_common::dfschema::DFSchema\t(has_equivalent_names_and_types|logically_equivalent_names_and_types)\t' .claude/skills/datafusion/content/index/methods.tsv | cut -f1,2,4
arrow_schema::datatype::DataType	equals_datatype	fn equals_datatype(&self, other: &DataType) -> bool
datafusion_common::dfschema::DFSchema	has_equivalent_names_and_types	fn has_equivalent_names_and_types(&self, other: &Self) -> Result<()>
datafusion_common::dfschema::DFSchema	logically_equivalent_names_and_types	fn logically_equivalent_names_and_types(&self, other: &Self) -> bool
```

```
$ rg -n 'Are struct columns accessed|Direct references to whole struct' .claude/skills/datafusion/content/api/datafusion_datasource_parquet.row_filter.md
55:- Are struct columns accessed via `get_field` where the leaf type is primitive
56:- Direct references to whole struct columns will prevent pushdown
```

```
$ rg -P '^parquet::arrow::ProjectionMask\t(leaves|roots|columns)\t' .claude/skills/datafusion/content/index/methods.tsv | cut -f2,4
columns	fn columns<'a>(schema: &SchemaDescriptor, names: impl IntoIterator<Item = &'a str>) -> Self
leaves	fn leaves(schema: &SchemaDescriptor, indices: impl IntoIterator<Item = usize>) -> Self
roots	fn roots(schema: &SchemaDescriptor, indices: impl IntoIterator<Item = usize>) -> Self
```

```
$ rg -P '^datafusion::dataframe::DataFrame\tunnest_columns|^datafusion_common::unnest::UnnestOptions\t' .claude/skills/datafusion/content/index/methods.tsv | cut -f2,4
unnest_columns	fn unnest_columns(self, columns: &[&str]) -> Result<DataFrame>
unnest_columns_with_options	fn unnest_columns_with_options(self, columns: &[&str], options: UnnestOptions) -> Result<DataFrame>
expand_empty_as_null	fn expand_empty_as_null(&self) -> bool
new	fn new() -> Self
preserve_nulls	fn preserve_nulls(&self) -> bool
with_null_handling	fn with_null_handling(self, null_handling: NullHandling) -> Self
with_preserve_nulls	fn with_preserve_nulls(self, preserve_nulls: bool) -> Self
with_recursions	fn with_recursions(self, recursion: RecursionUnnestOption) -> Self
```

```
$ rg -P '^datafusion_functions::core::expr_fn::(union_extract|union_tag)\t' .claude/skills/datafusion/content/index/symbols.tsv | cut -f1,2
datafusion_functions::core::expr_fn::union_extract	function
datafusion_functions::core::expr_fn::union_tag	function
```

```
$ rg -P 'pushdown_filters|schema_force_view_types|bloom_filter_on_read|parquet.pruning' .claude/skills/datafusion/content/catalogs/config-options.md | cut -c1-120
| `datafusion.execution.parquet.bloom_filter_on_read` | true | — | (reading) Use any available bloom filters when read
| `datafusion.execution.parquet.force_filter_selections` | false | — | (reading) Force the use of RowSelections for fi
| `datafusion.execution.parquet.max_predicate_cache_size` | NULL | — | (reading) The maximum predicate cache size, in 
| `datafusion.execution.parquet.pruning` | true | — | (reading) If true, the parquet reader attempts to skip entire ro
| `datafusion.execution.parquet.pushdown_filters` | false | — | (reading) If true, filter expressions are be applied d
| `datafusion.execution.parquet.schema_force_view_types` | true | — | (reading) If true, parquet reader will read colu
```

```
$ rg -P '^datafusion\tnested_expressions\t' .claude/skills/datafusion/content/index/features.tsv
datafusion	nested_expressions	datafusion-functions-nested	default
```

## delta-rs and kernel

```
$ rg -P '^buoyant_kernel::schema::StructField\t(not_null|nullable|is_nullable|with_metadata|add_metadata|get_config_value|metadata_with_string_values|column_mapping_id|physical_name|can_read_as|try_from_arrow)\t' .claude/skills/deltalake/content/index/methods.tsv | cut -f2,4
add_metadata	fn add_metadata(self, metadata: impl IntoIterator<Item = (impl Into<String>, impl Into<MetadataValue>)>) -> Self
can_read_as	fn can_read_as(&self, read_field: &Self) -> Result<(), Error>
column_mapping_id	fn column_mapping_id(&self) -> Option<i64>
get_config_value	fn get_config_value(&self, key: &ColumnMetadataKey) -> Option<&MetadataValue>
is_nullable	fn is_nullable(&self) -> bool
metadata_with_string_values	fn metadata_with_string_values(&self) -> HashMap<String, String>
not_null	fn not_null(name: impl Into<String>, data_type: impl Into<DataType>) -> Self
nullable	fn nullable(name: impl Into<String>, data_type: impl Into<DataType>) -> Self
physical_name	fn physical_name(&self, column_mapping_mode: ColumnMappingMode) -> &str
try_from_arrow	fn try_from_arrow(arrow_field: &ArrowField) -> Result<Self, ArrowError>
with_metadata	fn with_metadata(self, metadata: impl IntoIterator<Item = (impl Into<String>, impl Into<MetadataValue>)>) -> Self
```

```
$ rg -n 'Variants' .claude/skills/deltalake/content/api/buoyant_kernel.schema.md | rg 'ColumnMappingId'
15:**Variants**: `ColumnMappingId`, `ColumnMappingPhysicalName`, `ColumnMappingNestedIds`, `ParquetFieldId`, `ParquetFieldNestedIds`, `GenerationExpression`, `CurrentDefault`, `IdentityStart`, `IdentityStep`, `IdentityHighWaterMark`, `IdentityAllowExplicitInsert`, `InternalColumn`, `Invariants`, `MetadataSpec`
```

```
$ rg -n 'Variants' .claude/skills/deltalake/content/api/buoyant_kernel.schema.md | rg 'Struct|Array|Map|Primitive' | head -3
15:**Variants**: `ColumnMappingId`, `ColumnMappingPhysicalName`, `ColumnMappingNestedIds`, `ParquetFieldId`, `ParquetFieldNestedIds`, `GenerationExpression`, `CurrentDefault`, `IdentityStart`, `IdentityStep`, `IdentityHighWaterMark`, `IdentityAllowExplicitInsert`, `InternalColumn`, `Invariants`, `MetadataSpec`
39:**Variants**: `Primitive`, `Array`, `Struct`, `Map`, `Variant`
```

```
$ cat .claude/skills/deltalake/content/traits/DataCheck.md | rg 'fn |^- '
fn as_any(&self) -> &dyn Any
fn get_expression(&self) -> &str
fn get_name(&self) -> &str
- `deltalake_core::kernel::schema::schema::Invariant`
- `deltalake_core::table::columns::Constraint`
- `deltalake_core::table::columns::GeneratedColumn`
```

```
$ cat .claude/skills/deltalake/content/traits/StructTypeExt.md | rg 'fn '
fn get_generated_columns(&self) -> Result<Vec<GeneratedColumn>, Error>
fn get_invariants(&self) -> Result<Vec<Invariant>, Error>
```

```
$ rg -n 'DataSkippingNumIndexedCols|DataSkippingStatsColumns|CheckpointWriteStatsAsStruct|ColumnMappingMode' .claude/skills/deltalake/content/api/deltalake_core.table.config.md | cut -c1-160 | head -4
69:**Variants**: `AppendOnly`, `AutoOptimizeAutoCompact`, `AutoOptimizeOptimizeWrite`, `CheckpointInterval`, `CheckpointWriteStatsAsJson`, `CheckpointWriteStats
119:fn num_indexed_cols(&self) -> DataSkippingNumIndexedCols
```

```
$ rg -n 'Fields|fn ' .claude/skills/deltalake/content/api/deltalake_core.operations.write.configs.md
15:**Fields**: `num_indexed_cols`, `stats_columns`
22:fn from_config(config: &TableConfiguration) -> Self
23:fn new(num_indexed_cols: DataSkippingNumIndexedCols, stats_columns: Option<Vec<String>>) -> Self
```

```
$ rg -n 'fn |Verifies|nullCount' .claude/skills/deltalake/content/api/buoyant_kernel.transaction.stats_verifier.md
12:fn verify_num_records_present(add_files: &[Box<dyn EngineData>]) -> DeltaResult<()>
33:fn new(required_columns: Vec<(ColumnName, DataType)>) -> Self
34:fn verify(&self, add_files: &[Box<dyn EngineData>]) -> DeltaResult<()>
37:Verifies that add file statistics contain required columns.
39:For each required column, validates that `nullCount` is present (non-null) and that
41:(`nullCount == numRecords`).
```

```
$ rg -P '^buoyant_kernel::expressions::column_names::ColumnName\t(new|from_naive_str_split|parent|join|parse_column_name_list)\t' .claude/skills/deltalake/content/index/methods.tsv | cut -f2,4
from_naive_str_split	fn from_naive_str_split(name: impl AsRef<str>) -> Self
join	fn join(&self, right: &ColumnName) -> ColumnName
new	fn new(iter: impl CollectInto<Self>) -> Self
parent	fn parent(&self) -> Option<ColumnName>
parse_column_name_list	fn parse_column_name_list(names: impl AsRef<str>) -> DeltaResult<Vec<ColumnName>>
```

```
$ rg -n 'fn parse_sql_predicate_to_kernel|Only constructs the kernel' .claude/skills/deltalake/content/api/deltalake_core.delta_datafusion.expr.md
54:fn parse_sql_predicate_to_kernel(predicate: impl AsRef<str>, table_schema: &delta_kernel::schema::StructType, session: &dyn Session) -> DeltaResult<delta_kernel::expressions::Predicate>
62:(delete, update, merge) are resolved. Only constructs the kernel can
```

```
$ rg -P '^deltalake_core::delta_datafusion::table_provider::DeltaScanConfig\t(with_schema|with_parquet_pushdown|with_file_column_name|with_wrap_partition_values)\t' .claude/skills/deltalake/content/index/methods.tsv | cut -f2,4
with_file_column_name	fn with_file_column_name<S: ToString>(self, name: S) -> Self
with_parquet_pushdown	fn with_parquet_pushdown(self, pushdown: bool) -> Self
with_schema	fn with_schema(self, schema: SchemaRef) -> Self
with_wrap_partition_values	fn with_wrap_partition_values(self, wrap: bool) -> Self
```

```
$ rg -n 'fn scan\(|fn supports_filters_pushdown' .claude/skills/deltalake/content/api/deltalake_core.delta_datafusion.table_provider.next.md
123:async fn scan(&self, session: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
125:fn supports_filters_pushdown(&self, filter: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
```

```
$ rg -n 'partitionColumns\|.Array\[String\]|clusteringColumns. is the name path|delta.columnMapping.nested.ids' .claude/skills/deltalake/content/corpus/protocol/PROTOCOL.md | cut -c1-140 | head -5
511:partitionColumns|`Array[String]`| An array containing the names of columns by which the data should be partitioned | required
1524:- Require that the nested `element` field of ArrayTypes and the nested `key` and `value` fields of MapTypes be assigned 32 bit integer 
1559:      "delta.columnMapping.nested.ids": {
1578:      "delta.columnMapping.nested.ids": {
1602:              "delta.columnMapping.nested.ids": {
```

```
$ rg -c 'stats_from_parquet_metadata|fn check_batch' .claude/skills/deltalake/content/corpus -g '*.rs' || echo 'index-silent: delta-rs crates/core source is not in the corpus'
index-silent: delta-rs crates/core source is not in the corpus
```

```
$ ls .claude/skills/deltalake/content/corpus
examples
guides
protocol
tests
```

## Follow-up lookups after the capability-inventory cross-check

```
$ rg -P '^datafusion_common::nested_struct::' .claude/skills/datafusion/content/index/symbols.tsv | cut -f1,2
datafusion_common::nested_struct::adapt_batch_to_schema	function
datafusion_common::nested_struct::cast_column	function
datafusion_common::nested_struct::has_one_of_more_common_fields	function
datafusion_common::nested_struct::requires_nested_struct_cast	function
datafusion_common::nested_struct::validate_data_type_compatibility	function
datafusion_common::nested_struct::validate_struct_compatibility	function
```

```
$ rg -n 'by name|at least one common|only struct columns can be cast|name-based nested struct casting' .claude/skills/datafusion/content/api/datafusion_common.nested_struct.md
36:types, it enforces that **only struct columns can be cast to struct types**.
98:Check if two field lists have at least one common field by name.
114:name-based nested struct casting logic, rather than Arrow's standard cast.
156:- **Field Matching**: Fields are matched by name (case-sensitive)
```

```
$ rg -n -A3 'Unsupported Casts' .claude/skills/datafusion/content/api/arrow_cast.cast.md
87:Unsupported Casts (check with `can_cast_types` before calling):
88-* To or from `StructArray`
89-* `List` to `Primitive`
90-* `Interval` and `Duration`
```

```
$ rg -n 'nested_struct|cast_column' .claude/skills/datafusion/content/api/datafusion_physical_expr.expressions.cast.md || echo 'index-silent: CastExpr prose does not say which cast it selects'
index-silent: CastExpr prose does not say which cast it selects
```

```
$ rg -P 'skip_metadata|skip_arrow_metadata' .claude/skills/datafusion/content/catalogs/config-options.md | cut -c1-120
| `datafusion.execution.parquet.skip_arrow_metadata` | false | — | (writing) Skip encoding the embedded arrow metadata
| `datafusion.execution.parquet.skip_metadata` | true | — | (reading) If true, the parquet reader skip the optional em
```

```
$ rg -P '^arrow-schema\tcanonical_extension_types' .claude/skills/datafusion/content/index/features.tsv
arrow-schema	canonical_extension_types	dep:serde_core,dep:serde_json	-
```

```
$ rg -P '^datafusion_physical_expr_adapter::schema_rewriter::(PhysicalExprAdapterFactory|BatchAdapterFactory)\t' .claude/skills/datafusion/content/index/symbols.tsv | cut -f1,2; rg -n 'Nested struct fields are recursively adapted|Deprecated' .claude/skills/datafusion/content/api/datafusion_physical_expr_adapter.schema_rewriter.md .claude/skills/datafusion/content/api/datafusion_datasource.schema_adapter.md | head -4
datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory	struct
datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapterFactory	trait
.claude/skills/datafusion/content/api/datafusion_physical_expr_adapter.schema_rewriter.md:94:- **Struct field adaptation**: Nested struct fields are recursively adapted
.claude/skills/datafusion/content/api/datafusion_datasource.schema_adapter.md:9:> **Deprecated** — since 52.0.0: DefaultSchemaAdapterFactory has been removed. Use PhysicalExprAdapterFactory instead. See upgrading.md for more details.
.claude/skills/datafusion/content/api/datafusion_datasource.schema_adapter.md:33:Deprecated: Default [`SchemaAdapterFactory`] for mapping schemas.
.claude/skills/datafusion/content/api/datafusion_datasource.schema_adapter.md:56:> **Deprecated** — since 52.0.0: SchemaMapping has been removed. Use PhysicalExprAdapterFactory instead. See upgrading.md for more details.
```

```
$ rg -n -B6 'delta-kernel-rs/issues/1075' .claude/skills/deltalake/content/corpus/tests/it_datafusion/integration_datafusion.rs | head -12
763-        // Validate a table that contains nested structures.
764-
765-        // This table is interesting since it goes through schema evolution.
766-        // In particular 'new_column' contains statistics for when it
767-        // is introduced (10) but the commit following (11) does not contain
768-        // statistics for this column.
769:        // TODO: re-enable tests once https://github.com/delta-io/delta-kernel-rs/issues/1075
```

```
$ rg -n 'async fn test_zorder_nested_columns|async fn test_zorder_rejects_invalid_nested_path|ZOrder\(vec!' .claude/skills/deltalake/content/corpus/tests/it_datafusion/command_optimize.rs | head -6
766:        .with_type(OptimizeType::ZOrder(vec!["val".to_string()]))
985:    assert_optimize_preserves_live_rows_with_deletion_vectors(OptimizeType::ZOrder(vec![
1794:        .with_type(OptimizeType::ZOrder(vec!["x".to_string()]))
1813:    let result = dt.optimize().with_type(OptimizeType::ZOrder(vec![])).await;
1832:        .with_type(OptimizeType::ZOrder(vec!["non-existent".to_string()]))
1859:        .with_type(OptimizeType::ZOrder(vec!["date".to_string()]))
```

```
$ rg -n 'async fn test_schema_merge_append_missing_non_nullable_column|Invalid data found' .claude/skills/deltalake/content/corpus/tests/it_datafusion/integration_datafusion.rs | head -4
2018:async fn test_schema_merge_append_missing_non_nullable_column_with_generated_columns_fails() {
2038:        err.contains("Invalid data found"),
```

```
$ rg -n 'NonNullFieldChecker|InvariantChecker' .claude/skills/deltalake/content/api/buoyant_kernel.transforms.schema.md
59:- `buoyant_kernel::schema::InvariantChecker`
61:- `buoyant_kernel::schema::NonNullFieldChecker`
```

```
$ rg -P '^deltalake_core::operations::write::WriteBuilder\t(with_cast_safety|with_schema_mode|with_partition_columns)\t' .claude/skills/deltalake/content/index/methods.tsv | cut -f2,4
with_cast_safety	fn with_cast_safety(self, safe: bool) -> Self
with_partition_columns	fn with_partition_columns(self, partition_columns: impl IntoIterator<Item = impl Into<String>>) -> Self
with_schema_mode	fn with_schema_mode(self, schema_mode: SchemaMode) -> Self
```

```
$ rg -P '^buoyant_kernel::schema::DataType\t(unshredded_variant|variant_type)\t' .claude/skills/deltalake/content/index/methods.tsv | cut -f2,4
unshredded_variant	fn unshredded_variant() -> Self
variant_type	fn variant_type(fields: impl IntoIterator<Item = StructField>) -> DeltaResult<Self>
```
