# Crate map

60 crates hold the pinned surface. The `datafusion` facade documents 1,376
items of its own and re-exports the rest, so the crate an item is *reached* through is
usually not the crate that defines it. Resolve access paths through
[`../index/aliases.tsv`](../index/aliases.tsv) before attributing an item to a crate.

| Crate | Items | Traits | Notable extension points |
|---|---:|---:|---|
| `arrow` | 82 | 1 | `RandomTemporalValue` |
| `arrow-arith` | 67 | 0 | — |
| `arrow-array` | 338 | 32 | `Array`, `ArrayBuilder`, `ArrowPrimitiveType`, `ArrowTemporalType` |
| `arrow-avro` | 42 | 3 | `AsyncFileReader`, `AvroFormat` |
| `arrow-buffer` | 52 | 6 | `ArrowNativeType`, `MemoryPool`, `MemoryReservation` |
| `arrow-cast` | 45 | 4 | `ArrayFormatterFactory`, `DecimalCast`, `Parser` |
| `arrow-csv` | 9 | 0 | — |
| `arrow-data` | 30 | 0 | — |
| `arrow-flight` | 126 | 4 | `ProstMessageExt` |
| `arrow-ipc` | 299 | 0 | — |
| `arrow-json` | 27 | 4 | `Encoder`, `JsonFormat`, `JsonSerializable` |
| `arrow-ord` | 24 | 0 | — |
| `arrow-pyarrow` | 7 | 3 | `FromPyArrow`, `IntoPyArrow`, `ToPyArrow` |
| `arrow-row` | 9 | 0 | — |
| `arrow-schema` | 41 | 1 | `ExtensionType` |
| `arrow-select` | 26 | 1 | `MergeIndex` |
| `arrow-string` | 23 | 0 | — |
| `datafusion` | 61 | 5 | `DataFilePaths`, `ListingTableConfigExt`, `ReadOptions` |
| `datafusion-catalog` | 29 | 5 | `StreamProvider`, `UrlTableFactory` |
| `datafusion-catalog-listing` | 13 | 0 | — |
| `datafusion-common` | 378 | 41 | `ConfigField`, `DFExtensionType`, `DFHeapSize`, `TreeNodeContainer` |
| `datafusion-common-runtime` | 7 | 1 | — |
| `datafusion-datasource` | 74 | 17 | `FileFormat`, `FileFormatFactory`, `FileOpener`, `FileSource` |
| `datafusion-datasource-arrow` | 4 | 0 | — |
| `datafusion-datasource-avro` | 4 | 0 | — |
| `datafusion-datasource-csv` | 9 | 0 | — |
| `datafusion-datasource-json` | 10 | 0 | — |
| `datafusion-datasource-parquet` | 32 | 1 | `ParquetFileReaderFactory` |
| `datafusion-doc` | 25 | 0 | — |
| `datafusion-execution` | 62 | 13 | `CacheValue`, `FileMetadata`, `MemoryPool`, `RecordBatchStream` |
| `datafusion-expr` | 374 | 24 | `AggregateUDFImpl`, `ExprPlanner`, `Literal`, `ScalarUDFImpl` |
| `datafusion-expr-common` | 60 | 4 | `Accumulator`, `GroupsAccumulator` |
| `datafusion-ffi` | 118 | 1 | `ExtensionOptionsFFIProvider` |
| `datafusion-functions` | 443 | 1 | `FieldAccessor` |
| `datafusion-functions-aggregate` | 141 | 0 | — |
| `datafusion-functions-aggregate-common` | 48 | 1 | `VecAllocExt` |
| `datafusion-functions-nested` | 184 | 2 | `IndexAccessor`, `SliceAccessor` |
| `datafusion-functions-table` | 12 | 1 | `SeriesValue` |
| `datafusion-functions-window` | 39 | 0 | — |
| `datafusion-functions-window-common` | 3 | 0 | — |
| `datafusion-macros` | 1 | 0 | — |
| `datafusion-optimizer` | 53 | 3 | `AnalyzerRule`, `OptimizerConfig`, `OptimizerRule` |
| `datafusion-physical-expr` | 129 | 2 | `StandardWindowFunctionExpr`, `WindowExpr` |
| `datafusion-physical-expr-adapter` | 11 | 2 | `PhysicalExprAdapter`, `PhysicalExprAdapterFactory` |
| `datafusion-physical-expr-common` | 67 | 6 | `PhysicalExpr`, `PhysicalExprDecode`, `PhysicalExprEncode`, `RecordOutput` |
| `datafusion-physical-optimizer` | 59 | 0 | — |
| `datafusion-physical-plan` | 269 | 15 | `DisplayAs`, `ExecutionPlan`, `GroupColumn`, `StatisticsProvider` |
| `datafusion-proto` | 71 | 7 | `LogicalExtensionCodec`, `PhysicalExtensionCodec`, `PhysicalPlanNodeExt`, `PhysicalProtoConverterExtension` |
| `datafusion-proto-common` | 107 | 1 | `FromOptionalField` |
| `datafusion-proto-models` | 337 | 0 | — |
| `datafusion-pruning` | 8 | 1 | — |
| `datafusion-session` | 19 | 12 | `CatalogProviderList`, `PhysicalOptimizerRule`, `SchemaProvider`, `TableProvider` |
| `datafusion-spark` | 337 | 1 | `SessionStateBuilderSpark` |
| `datafusion-sql` | 50 | 2 | `Dialect` |
| `datafusion-substrait` | 126 | 2 | `SubstraitConsumer`, `SubstraitProducer` |
| `object_store` | 144 | 10 | `MultipartStore`, `ObjectStore`, `PaginatedListStore`, `Signer` |
| `parquet` | 258 | 34 | `AsBytes`, `AsyncFileReader`, `ColumnIndexIterators`, `DataType` |
| `parquet-variant` | 26 | 4 | `BuilderSpecificState`, `MetadataBuilder`, `VariantBuilderExt`, `VariantDecimalType` |
| `parquet-variant-compute` | 21 | 1 | `IntoShreddingField` |
| `sqlparser` | 1664 | 6 | `Dialect`, `Spanned`, `Visit`, `VisitMut` |
