---
id: ADR-0040
title: Bind immutable native catalogs and one effective policy
status: accepted
date: 2026-09-15
deciders: [paul-heyse]
level: decision
principles: [DM-16, DM-17, DM-23, DM-25, DM-26, DM-38, DM-50, DM-55, DM-56]
design: [§6.3, §8.4, §14.3]
review: docs/design_review/reviews/design_review_datafusion-capability-leverage_2026-09-15.md
evidence: Tested
supersedes: []
superseded-by: null
revisit: A required native consumer cannot use the immutable bound inventory or measured physical policy.
verification: Plan 14 J01-J17; native catalog conformance, scan statistics, scoring and index consumer tests; just architecture-check.
---

# ADR-0040: Bind immutable native catalogs and one effective policy

## Context

The capability review identifies independent snapshot, leased-view, durable-record and comparison
registration paths. The user selected direct replacement in Plan 14 while the project remains
in design. This amends native binding and discovery within the existing storage/query boundaries.

## Scope

All native evidence/catalog readers, candidate validation, operation indexes and their policy and
diagnostic consumers. Durable publication, canonical codecs and research/2.0 remain authoritative.
No old namespace, compatibility reader or parallel registration engine is retained.

## Drivers

One executable declaration for availability, keys, schema and metadata; exact immutable ownership;
engine-visible proven facts; bounded actual operation costs rather than speculative infrastructure.

## Options

- Keep procedural registration: perpetuates the review's repeated policy and visibility decisions.
- Share a builder over mutable containers: reduces duplication but leaves the admitted boundary mutable.
- Immutable bound CatalogProvider/SchemaProvider with a separate work schema: selected.

## Decision

Bind complete typed inventories into immutable native catalog/schema providers. Relation declarations
continue deriving schemas from codecs and generate mechanical key/reference checks. Admission
establishes facts before provider constraints and physical statistics expose them to optimization.

Use qualified source references, independent comparison pins and a separate operation work schema.
Cached definitions remain transparent native plans with request ownership attached after cache lookup.
Candidate inputs and raw catalog history have explicit scopes and do not inherit admitted key claims.

Capture one immutable effective native policy from validated Rust configuration. A read-only
ConfigExtension shares it with the actual Parquet format factory and bounded settings discovery;
this concrete consumer fires the deferred trigger in ADR-0038/Plan 13 without changing core policy
ownership. Typed metadata projects the same declarations/bindings that execute.

Retain verified counts through physical scans and completed indexes, preserve supported string
encodings, and report bounded rule transitions and accurately scoped native managed-memory peaks.
The bounded Plan 14 measurements select decoder filtering/reordering, Bloom only on the declared
API-observation ID, capped native whole-file groups, 1,024-row storage groups and independent
4,096-row catalog files. Retain unsorted IPC and the existing fair pool with bounded peak recording;
no speculative column-statistic or dictionary-scoring infrastructure is added without a consumer.

Persist the expanded bounded query diagnostics in `query-failures-v2.json`. Earlier diagnostic
history remains inactive evidence without translation. Canonical evidence/catalog/job formats and
research/2.0 already implement the target and are retained.

### Consequences

All consumers change together. Adding a relation extends the finite declaration/binding path.
Mutable source replacement and bare admitted names cease to be supported. Native library traits
remain an integration boundary; they do not replace admission or publication.

### Compensating controls

Pinned DataFusion 55.1.0 sources; immutable maps; narrow registration rules; source-witness and
lease negatives; exact/unknown statistics distinction; actual consumer and resource tests.

## Evidence

Verified by the upstream-verifier on 2026-09-15 against installed exact-release sources, with the
review's recorded probes as mechanism evidence. Context7 discovery is not release proof.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Optional catalog mutation can be refused | [55.1 CatalogProvider](https://github.com/apache/datafusion/blob/55.1.0/datafusion/session/src/catalog.rs) | 2026-09-15 | “Registering new schemas is not supported” |
| Schema classification can avoid table construction | [55.1 SchemaProvider](https://github.com/apache/datafusion/blob/55.1.0/datafusion/session/src/schema.rs) | 2026-09-15 | “Implementations for which this operation is cheap” |
| Extension cloning cannot share mutable configuration | [55.1 ExtensionOptions](https://github.com/apache/datafusion/blob/55.1.0/datafusion/common/src/config.rs) | 2026-09-15 | “It is important this does not share mutable state” |
| Format options reach the actual Parquet source | [55.1 ParquetFormat](https://github.com/apache/datafusion/blob/55.1.0/datafusion/datasource-parquet/src/file_format.rs) | 2026-09-15 | `with_table_parquet_options(self.options.clone())` |
| Provider-only statistics do not complete native optimizer integration | [55.1 TableProvider](https://github.com/apache/datafusion/blob/55.1.0/datafusion/session/src/table.rs) | 2026-09-15 | “Although not presently used in mainline DataFusion” |

## Verification

Plan 14 names J01–J17: consistent lookup/mutation refusal, exact source lifecycle, declared key
mapping, consumed policy/metadata, physical count optimization, string equivalence, index replay
elimination, bounded diagnostics, measured strategies and installed MCP journeys. Final source `5b2640ca15e4` passes 441 native regression cases, the focused native property and
rule tests, and actual installed raw-MCP, capacity recovery, Codex and Claude journeys. The
[qualification report](../reports/plan14-final-qualification-2026-09-15.md) maps each oracle and
D01–D13 to consumers and exact receipts. The nine physical decisions have separate source-bound
[measurements](../reports/plan14-physical-strategies-2026-09-15.md). Independent gate replay remains
separate from the review verdict and historical external-profile qualification.

## Boundaries preserved

All §B1–§B13 remain: outside-repository state; Rust ownership; thin Python transport; ty and
rust-analyzer semantics; immutable Arrow/Parquet evidence; separate Context7; local daemon;
agent-owned inference; real acceptance; excluded mechanisms; static Griffe; hosted rustdoc first.

## More information

[Plan 14](../plans/14-datafusion-catalog-policy-hard-pivot.md) defines replacement/deletion and
measurement obligations. Registers R-44/R-45 retain the observable deferred-consumer and workload triggers.

## Status history

- 2026-09-15 — proposed; implementation authorized, functional validation pending.

- 2026-09-15 — accepted after the Accept-scoped source review, complete target replacement, native/installed consumer qualification and single verified activation; measured strategy decisions are recorded separately.
