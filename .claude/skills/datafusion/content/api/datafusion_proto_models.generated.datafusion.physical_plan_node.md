# `datafusion_proto_models::generated::datafusion::physical_plan_node`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.physical_plan_node.json`](../model/datafusion_proto_models.generated.datafusion.physical_plan_node.json)

## PhysicalPlanType

`enum` · `datafusion_proto_models::generated::datafusion::physical_plan_node::PhysicalPlanType`

```rust
enum PhysicalPlanType
```

**Variants**: `ParquetScan`, `CsvScan`, `Empty`, `Projection`, `GlobalLimit`, `LocalLimit`, `Aggregate`, `HashJoin`, `Sort`, `CoalesceBatches`, `Filter`, `Merge`, `Repartition`, `Window`, `CrossJoin`, `AvroScan`, `Extension`, `Union`, `Explain`, `SortPreservingMerge`, `NestedLoopJoin`, `Analyze`, `JsonSink`, `SymmetricHashJoin`, `Interleave`, `PlaceholderRow`, `CsvSink`, `ParquetSink`, `Unnest`, `JsonScan`, `Cooperative`, `GenerateSeries`, `SortMergeJoin`, `MemoryScan`, `AsyncFunc`, `Buffer`, `ArrowScan`, `ScalarSubquery`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<PhysicalPlanType>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.physical_plan_node.PhysicalPlanType.md).


---
