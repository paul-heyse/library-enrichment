# Match the boundary to data, plan or ABI interchange

IPC/Flight move data, proto/Substrait encode plans, and FFI crosses an ABI boundary. Each needs its own compatibility, registry and ownership contract.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| Arrow IPC / Flight | Move batches/schema | No query-plan semantics follow from serializing data. |
| DataFusion proto | Serialize supported DataFusion plans | Logical and physical extensions have different codecs; restore required registries/providers. |
| Substrait | Exchange supported relational plans across engines | Extension coverage and function mappings determine interoperability. |
| datafusion-ffi | Separately compiled in-process providers/functions | Follow explicit ABI/version and release-callback ownership rules. |

## Contract

**codecs.** LogicalExtensionCodec and PhysicalExtensionCodec serve different plan layers. Custom nodes and functions may need codec/registry support; successful encoding does not prove equivalent execution elsewhere.
Claim `df.interchange.codecs`; upstream_contract_interpretation; evidence: upstream.

**ownership.** FFI and stream boundaries need resource/release lifetime review. Stable ABI does not imply arbitrary DataFusion-version compatibility or durable serialization.
Claim `df.interchange.ownership`; upstream_contract_interpretation; evidence: upstream.

**validation.** Verify a round trip including schema, required registration, custom nodes and execution results under the exact sender/receiver profiles.
Claim `df.interchange.validation`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Identify whether the payload is data, a plan, or in-process handles.
- List extension codecs and registry/store dependencies.
- Test restoration and execution in the receiver environment.

## Limits and unknowns

- This card is source-backed routing. Cross-engine round trips and FFI lifetimes are not executed in this probe suite.

## Exact contracts

- [`datafusion_proto::logical_plan::LogicalExtensionCodec`](../operations/datafusion_proto.logical_plan.LogicalExtensionCodec.md#op-4284dcb76d9545ce9708c7f6) — `trait LogicalExtensionCodec: Debug + Send + Sync + std::any::Any`
- [`datafusion_proto::physical_plan::PhysicalExtensionCodec`](../operations/datafusion_proto.physical_plan.PhysicalExtensionCodec.md#op-850a12f382d8c2d858dedae2) — `trait PhysicalExtensionCodec: Debug + Send + Sync + Any`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
