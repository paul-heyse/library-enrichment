# Protocol recognition and admission

The enum vocabulary below is not a list of supported operations. These source-derived sets
are the default checker's explicit feature admission; legacy protocol-version handling and
operation-specific restrictions still apply. Feature gates depend on the consumer profile.

| Feature | Reader insertion | Writer insertion | Operation restriction |
|---|---|---|---|
| AllowColumnDefaults | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| AppendOnly | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': True, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| CatalogManaged | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| CatalogOwnedPreview | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| ChangeDataFeed | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': True, 'requires_feature': 'datafusion'} | Consult operation contract; uncharacterized cells remain unknown |
| CheckConstraints | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': True, 'requires_feature': 'datafusion'} | Consult operation contract; uncharacterized cells remain unknown |
| ClusteredTable | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| ColumnMapping | {'present_in_source_set': True, 'requires_feature': 'datafusion'} | {'present_in_source_set': True, 'requires_feature': 'datafusion'} | CDF build rejects non-None column mapping |
| DeletionVectors | {'present_in_source_set': True, 'requires_feature': None} | {'present_in_source_set': True, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| DomainMetadata | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| GeneratedColumns | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': True, 'requires_feature': 'datafusion'} | Consult operation contract; uncharacterized cells remain unknown |
| IcebergCompatV1 | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| IcebergCompatV2 | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| IcebergCompatV3 | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| IdentityColumns | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| InCommitTimestamp | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| Invariants | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': True, 'requires_feature': 'datafusion'} | Consult operation contract; uncharacterized cells remain unknown |
| MaterializePartitionColumns | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| RowTracking | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| TimestampNanos | {'present_in_source_set': True, 'requires_feature': 'nanosecond-timestamps'} | {'present_in_source_set': True, 'requires_feature': 'nanosecond-timestamps'} | Consult operation contract; uncharacterized cells remain unknown |
| TimestampWithoutTimezone | {'present_in_source_set': True, 'requires_feature': None} | {'present_in_source_set': True, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| TypeWidening | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| TypeWideningPreview | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| Unknown | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| V2Checkpoint | {'present_in_source_set': True, 'requires_feature': None} | {'present_in_source_set': True, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| VacuumProtocolCheck | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| VariantShredding | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| VariantShreddingPreview | {'present_in_source_set': False, 'requires_feature': None} | {'present_in_source_set': False, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| VariantType | {'present_in_source_set': True, 'requires_feature': None} | {'present_in_source_set': True, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |
| VariantTypePreview | {'present_in_source_set': True, 'requires_feature': None} | {'present_in_source_set': True, 'requires_feature': None} | Consult operation contract; uncharacterized cells remain unknown |

[Structured matrix](protocol-matrix.json). [Reviewed feature decisions](../capabilities/delta.features.md).
