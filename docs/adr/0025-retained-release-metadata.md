---
id: ADR-0025
title: Retain release metadata as qualified Arrow observations
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-12, DM-24]
design: [§3.3, §6.3]
review: "not-required: Internal projection within the accepted typed evidence and publication boundaries of ADR-0022 and ADR-0023."
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A retained metadata value requires an independently paged relation rather than the bounded nested projection.
verification: release_metadata_roundtrip; snapshot_admission; evidence_retention
---

# ADR-0025: Retain release metadata as qualified Arrow observations

## Context

Daemon cutover exposed a missing target consumer: offline resolution currently replays an
entire saved JSON response, including Python distribution inventory and Rust documentation
configuration. Those facts must survive exact-version reuse without repeating extraction.

## Scope

An additional `release_metadata` relation within the accepted Rust-owned format 4.0 contract.
This adds no new tool, storage engine, publication boundary or epistemic class.

## Drivers

Retained evidence, one typed authority, precise provenance and bounded offline retrieval.

## Options

- Keep serialized response documents: violates the accepted removal of JSON catalog indexes.
- Repeat archive inspection on every resolution: wastes work and reconstructs retained facts.
- Store qualified typed metadata observations: selected.

## Decision

Persist release ID, semantic metadata ID, an explicit Rust-docs/Python-distribution variant
and qualified producer source in `release_metadata`. Encode configuration, file mappings,
ordered repeated headers and documentation inventory as Arrow Struct/List values. Preserve
unknown versus explicitly empty target selections. Mutable latest-version freshness and
request/envelope fields are not metadata facts. Distinct qualified observations remain rows.

### Consequences

The snapshot has nine authoritative evidence relations. Metadata participates in semantic
identity, admission, provenance closure, exact-file publication and export. Resolution selects
bounded metadata rows from its pinned snapshot; it does not replay stored response JSON.

### Compensating controls

Apply the existing record/batch/file budgets; reject invalid tags, unsafe archive paths,
duplicate headers, inconsistent IDs and missing input references. Large inventories fail a
declared budget rather than silently truncating metadata or becoming JSON extensions.

## Evidence

| Claim | Primary source | Retrieved | Quote |
|---|---|---|---|
| The existing response carries distribution facts | [Rust DTO](../../crates/enrichment-core/src/wire/data.rs) | 2026-09-14 | `pub python: Option<crate::producer::python::Distribution>` |
| Distribution values include ordered repeated headers | [Rust producer contract](../../crates/enrichment-core/src/producer/python.rs) | 2026-09-14 | `pub metadata: BTreeMap<String, Vec<String>>` |

## Verification

`release_metadata_roundtrip`, `snapshot_admission`, and `evidence_retention` cover the native
representation, closure and offline reuse. This record accepts the internal projection;
complete daemon integration and acceptance are still required.

## Boundaries preserved

§B1–§B13 remain unchanged. ADR-0022 owns the evidence contract and ADR-0023 owns publication.
Metadata is normalized by Rust; the Python worker and MCP adapter remain separate.

## More information

[Plan 10](../plans/10-arrow-datafusion-architecture.md), target relations and offline reuse.

## Status history

- 2026-09-14 — accepted as an internal projection within the already approved target contract.
