# Table features

Protocol features are the compatibility contract: a reader that does not support a
table's required feature must refuse the table rather than read it wrongly. Which is
why this is a catalog and not a footnote -- the set below is what this pin can honour.

Add one with `DeltaTable::add_feature`. Adding a feature raises the protocol
version and can make the table unreadable by older clients; that is the point of it.

## `deltalake_core::kernel::models::actions::TableFeatures`

18 variants.

`AppendOnly`, `ChangeDataFeed`, `CheckConstraints`, `ColumnMapping`, `DeletionVectors`, `DomainMetadata`, `GeneratedColumns`, `IcebergCompatV1`, `IdentityColumns`, `Invariants`, `MaterializePartitionColumns`, `RowTracking`, `TimestampNanos`, `TimestampWithoutTimezone`, `V2Checkpoint`, `VariantShreddingPreview`, `VariantType`, `VariantTypePreview`

## `buoyant_kernel::table_features::TableFeature`

30 variants.

`AllowColumnDefaults`, `AppendOnly`, `CatalogManaged`, `CatalogOwnedPreview`, `ChangeDataFeed`, `CheckConstraints`, `ClusteredTable`, `ColumnMapping`, `DeletionVectors`, `DomainMetadata`, `GeneratedColumns`, `IcebergCompatV1`, `IcebergCompatV2`, `IcebergCompatV3`, `IdentityColumns`, `InCommitTimestamp`, `Invariants`, `MaterializePartitionColumns`, `RowTracking`, `TimestampNanos`, `TimestampWithoutTimezone`, `TypeWidening`, `TypeWideningPreview`, `Unknown`, `V2Checkpoint`, `VacuumProtocolCheck`, `VariantShredding`, `VariantShreddingPreview`, `VariantType`, `VariantTypePreview`

