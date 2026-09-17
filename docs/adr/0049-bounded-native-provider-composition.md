---
id: ADR-0049
title: Compose native providers through bounded discovery and owned Delta opening
status: accepted
date: 2026-09-16
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-16, DM-42]
design: [§17]
review: not-required: Narrow upstream injection and formatter corrections within the accepted Plan 17 provider contract.
evidence: Interface-checked
supersedes: []
superseded-by: null
revisit: The exact upstream versions expose bounded atomic listing and controlled Delta opening with the complete scan contract.
verification: native_provider_discovery tests, native Delta mutation tests and Plan 17 Q02/Q10.
---

# ADR-0049: Compose native providers through bounded discovery and owned Delta opening

## Context

Plan 17 FP05 requires the actual native listing and Delta factory routes. The pinned listing
materializes the whole store and silently skips name collisions. The pinned Delta factory opens
through independent defaults, without an exact capture, supplied store or semantic scan contract.
The upstream-verifier inspected the exact pins and confirmed the missing composition seams.

## Scope

Patch DataFusion catalog 55.1.0 and delta-rs core at
`58f07cd62bfbce3649a7e1c87c696288068ae184`. Keep the existing Arrow/DataFusion/kernel universe.
No change to publication, retention, mutation admission or provider trust boundaries.

## Drivers

Bound inventory before accumulation. Retain one owned store/session and exact snapshot. Preserve
full semantic fields without duplicating native providers. Refuse incomplete discovery atomically.

## Options

- Use defaults: fails boundedness, deterministic discovery and owned-runtime requirements.
- Implement replacement providers: duplicates the library mechanism this architecture targets.
- Add narrow upstream seams and a formatter correction: selected; qualify actual consumers.

## Decision

Vendor the two exact source packages with a provenance manifest. Listing consumes the stream under
object/candidate bounds, rejects normalized collisions, prepares privately and atomically publishes
one complete inventory. DeltaTableFactory accepts an opening hook returning the admitted DeltaTable
and its explicit DeltaScanConfig. Its provider builder retains that table's snapshot and log store;
the owned route requires the supplied SessionState. No second open can infer a factory result's identity.

All application snapshot preparation uses this factory. Service recovery first validates bounded
storage discovery against the captured semantic registry. Discovery never chooses a published head;
committed exact vectors remain authority. Captures retain the existing retention lease. Correct
nested IN formatting recursively through SqlFormat so persisted CHECK SQL remains parseable.

### Consequences

The application retains native providers and field semantics. Two narrowly patched dependencies
require source provenance and route tests until a qualified upstream release supplies these seams.
All vendored source participates in the native worker identity. No kernel patch is required.

### Compensating controls

Actual counting-stream inventory tests, atomic failure/collision tests, shared-store/exact-version
capture, schema metadata and native CHECK round trips. Revisit register R-47 owns patch retirement.

## Evidence

Retrieved 2026-09-16 from pinned source, independently verified in
[provider-composition.md](../design_review/reviews/evidence/combined-schema-runtime-plan-2026-09-16/provider-composition.md).

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Listing accumulates the complete stream | [DataFusion listing](https://github.com/apache/datafusion/blob/55.1.0/datafusion/catalog/src/listing_schema.rs) | 2026-09-16 | `try_collect().await?` |
| Factory owns provider construction | [Delta factory](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs) | 2026-09-16 | `table.table_provider()` |
| IN left operand uses ordinary Display | [Delta formatter](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/expr.rs#L632) | 2026-09-16 | `{expr} IN` |

## Verification

`native_provider_discovery` exercises actual native listing/factory and error atomicity.
Native Delta mutation tests exercise nested positive/negative IN after reload, valid and invalid rows.
Plan 17 Q02/Q10 remain not_run until their complete obligations execute; source inspection is not a pass.

## Boundaries preserved

All §B1–§B13 remain binding. Rust owns admission; Python remains a thin adapter. The same native
provider and mutation machinery applies, exact publication and evidence distinctions remain intact,
and no repository-under-study writes, unrestricted SQL, migration reader or compatibility service is added.

## More information

[Plan 17](../plans/17-schema-governed-unified-runtime-hard-pivot.md),
[compatibility matrix](../architecture/compatibility-matrix.md), R-47 in the ADR register.

## Status history

- 2026-09-16 — accepted within the authorized hard pivot; interface-checked, route qualification pending.
