# `datafusion_proto_models::generated::datafusion::logical_plan_node`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.logical_plan_node.json`](../model/datafusion_proto_models.generated.datafusion.logical_plan_node.json)

## LogicalPlanType

`enum` · `datafusion_proto_models::generated::datafusion::logical_plan_node::LogicalPlanType`

```rust
enum LogicalPlanType
```

**Variants**: `ListingScan`, `Projection`, `Selection`, `Limit`, `Aggregate`, `Join`, `Sort`, `Repartition`, `EmptyRelation`, `CreateExternalTable`, `Explain`, `Window`, `Analyze`, `CrossJoin`, `Values`, `Extension`, `CreateCatalogSchema`, `Union`, `CreateCatalog`, `SubqueryAlias`, `CreateView`, `Distinct`, `ViewScan`, `CustomScan`, `Prepare`, `DropView`, `DistinctOn`, `CopyTo`, `Unnest`, `RecursiveQuery`, `CteWorkTableScan`, `Dml`, `EmptyTableScan`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<LogicalPlanType>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.logical_plan_node.LogicalPlanType.md).


---
