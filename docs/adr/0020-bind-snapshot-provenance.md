---
id: ADR-0020
title: Bind fragment provenance and preserve historical snapshot readers
status: superseded
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-11, DM-12, DM-15, DM-46]
design: [§3.1, §3.3, §6.3, §8.2]
review: docs/design_review/reviews/design_review_phase456-completion_2026-09-14.md
evidence: Tested
supersedes: []
superseded-by: ADR-0022
revisit: Another snapshot projection or semantic identity input changes.
verification: historical_snapshots; revision_revalidation; release_scoped_citations; schema-conformance
---

# ADR-0020: Bind fragment provenance and preserve historical snapshot readers

## Context

R-17 records a citation using a blob's first acquisition locator for later releases.
R-18 records revision receipt timestamps changing semantic snapshot identity. Both
violate the intended distinction between retained provenance and semantic identity.
Source inspection also found storage 2.0 for Python/revision snapshots despite status
advertising only the wire version 1.0. The planned storage 1.1 was based on that stale claim.

## Scope

Fragment provenance, semantic snapshot inputs and backward-readable storage. The frozen
wire envelope and existing release/environment/context identities are unchanged.

## Drivers

Exact citations, stable semantic replay, immutable history and honest compatibility claims.

## Options

- Keep first-blob provenance: rejected because equal bytes do not imply equal source locators.
- Rewrite old snapshots: rejected because pinned evidence is immutable.
- Store acquisition locators in fragments and add versioned readers: selected.

## Decision

Every newly published fragment records its acquisition-specific source URI when known.
The qualified fragment content is an input digest, so source/version-match changes change
the semantic snapshot. Raw operational revision receipts remain content-addressed artifacts
with append-only context acquisition records; their timestamps are excluded from semantic
input hashes. The snapshot retains the first producer provenance for identical semantics.
The producer projection includes name/version, normalized configuration digest, semantic
input digests, execution profile, outcome and declared gaps; it excludes start/end times,
diagnostic logs and receipt payloads. Repeated operational acquisitions link their retained
receipt artifact to the exact context outside the snapshot, under `acquisitions/<context>/`.
An append-only acquisition event records an acquisition ID, receipt artifact ID/digest, context,
and an explicit acquired-but-unpublished outcome. Publication appends a second event naming
the selected snapshot or a failed/unpublished outcome; absence of that event indicates an
interrupted or incomplete publication, never success. A revalidation may have a new receipt
and event without changing semantic identity.

The snapshot key binds declared release/environment through its context identity, storage and
normalizer versions, source input digests, qualified fragments and producer projection. Its
meaning-changing observed configuration and indexed/missing coverage are also input digests.
ADR-0015's immutable acquisition, archive and identity contracts remain; its allowance
for different *receipt artifacts* does not require different semantic snapshots.

Validated evidence for an exact release and environment has no age-based expiry. `cache_ok`
and `offline` reuse the retained resolution regardless of registry or mutable-document TTLs.
Those TTLs govern mutable lookup metadata only. Explicit `revalidate` may acquire additional
evidence without deleting the old snapshot. New versions receive separate evidence; neither
their discovery nor storage pressure deletes prior evidence. Only operator-initiated evidence
cleanup may remove retained facts. Temporary process/session cleanup never removes evidence.
Runtime and semantic observations remain valid within their recorded environment and input
scope; neither an empty result nor transient acquisition failure is a permanent absence claim.

One new storage writer emits 3.0. Readers accept actual historical 1.0 and 2.0 as well
as 3.0, reject unknown formats, and never rewrite old files. The wire envelope remains
1.0. Legacy fragments without an acquisition locator cite the immutable artifact handle
with unknown version matching rather than borrowing unrelated first-retrieval metadata.
Readers validate version-specific columns/types/nullability, required tags and locator JSON,
and the agreement of table metadata with manifest storage version, snapshot and context.
Historical 1.0 permits absent Python observation columns; historical 1.0/2.0 lack acquisition locators.
Those absent optional historical fields become explicit unknowns; corruption and unsupported
schemas are errors rather than empty successful reads or valid retained evidence.

### Consequences

New snapshots have new format-derived identities once. Subsequent unchanged revalidation
is stable. Readers carry two historical projections and unknown provenance remains visible.

### Compensating controls

Capture genuine 1.0/2.0 fixture bytes before changing the writer. Test both reader paths,
same-content cross-release citations, revalidation and unsupported-format rejection.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Semantic content excludes timestamps | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §6.3 | 2026-09-14 | "Exclude wall-clock timestamps and temporary paths from semantic content hashes" |
| Python already writes a second format | `crates/enrichment-core/src/producer/python.rs`, prechange source archive | 2026-09-14 | `pub const TABLE_VERSION: &str = "2.0";` |
| Historical bytes were genuinely produced | `tests/fixtures/historical-snapshots/*/capture.json` | 2026-09-14 | Captured from two successful real MCP fixture tests before writer changes |

## Verification

`historical_snapshots` reads the captured Parquet files through DataFusion. The MCP
revision-revalidation and release-scoped-citation fixtures exercise source-to-result behavior.
`just schema-conformance` preserves the frozen envelope; no gate is retired or weakened.

## Boundaries preserved

§B1–§B13 remain: Rust owns identities, storage and publication; Python DTOs are generated;
the repository under study is never modified; no new database or service is introduced.

## More information

Plan 09 W1; deferred rows R-17, R-18, R-19 and R-22.

## Status history

- 2026-09-14 — proposed; implementation and executable acceptance in progress.
- 2026-09-14 — accepted after the completion review's Accept-scoped W1 verdict; genuine historical readers, corruption rejection, retention, revision revalidation and release-scoped citations passed focused tests. This does not accept the remaining phase scope.
- 2026-09-14 — superseded by ADR-0022.
