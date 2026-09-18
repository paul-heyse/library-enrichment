# `datafusion_proto_models::generated::datafusion::plan_type`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.plan_type.json`](../model/datafusion_proto_models.generated.datafusion.plan_type.json)

## PlanTypeEnum

`enum` · `datafusion_proto_models::generated::datafusion::plan_type::PlanTypeEnum`

```rust
enum PlanTypeEnum
```

**Variants**: `InitialLogicalPlan`, `AnalyzedLogicalPlan`, `FinalAnalyzedLogicalPlan`, `OptimizedLogicalPlan`, `FinalLogicalPlan`, `InitialPhysicalPlan`, `InitialPhysicalPlanWithStats`, `InitialPhysicalPlanWithSchema`, `OptimizedPhysicalPlan`, `FinalPhysicalPlan`, `FinalPhysicalPlanWithStats`, `FinalPhysicalPlanWithSchema`, `PhysicalPlanError`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<PlanTypeEnum>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.plan_type.PlanTypeEnum.md).


---
