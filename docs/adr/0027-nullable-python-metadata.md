---
id: ADR-0027
title: Preserve absent Python declaration metadata as null
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-12, DM-24]
design: [§3.3, §6.3]
review: docs/design_review/reviews/design_review_nullable-python-metadata_2026-09-14.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: Source declarations and built distribution metadata need independent provenance beyond the current archive and release source.
verification: release_metadata_roundtrip; revision_fixture; archive_metadata_validation
---

# ADR-0027: Preserve absent Python declaration metadata as null

## Context and scope

Real revision acquisition exposes source files without built METADATA/PKG-INFO. The current
producer defaults missing declaration name/version to empty strings. Native metadata admission
then rejects otherwise useful source evidence because it requires both strings to be nonempty.
A commit is an exact source revision, not an invented distribution metadata version.

This changes the Rust `Distribution.name` and `Distribution.version` wire fields to nullable
strings and their native Arrow children to nullable strings. It amends the projection in
ADR-0025 without replacing its authority or publication boundary. There is one target reader
and writer, with no preservation or migration of development snapshots.

## Decision

Keep exact selected release/revision identity in `Release` and exact acquired bytes in the
artifact digest. Keep `Distribution.name` and `Distribution.version` as the optional values
actually extracted from built METADATA/PKG-INFO. Missing is null. Derive each scalar mechanically from its selected lower-case header key:
absent key requires null; exactly one nonempty parsed header value requires that exact
`Some(value)`. Reject duplicate scalar headers (including equal duplicates), empty or
whitespace-only values, and scalar/header mismatches at native admission. Preserve valid
repeated nonscalar headers unchanged. Exact preservation means the parsed header value;
the retained artifact preserves the raw bytes. Do not infer these values from a source commit, archive filename, source directory,
or a backend invocation. Literal pyproject declarations already recorded in revision receipts
remain separately qualified; they do not become built distribution headers.

Ordinary PyPI acquisition continues to validate required Name/Version headers and their
agreement with the selected distribution before publication. Null therefore permits honest
unbuilt-source inventory; it does not waive a required PyPI package validation. Source-revision
coverage explicitly says it is unbuilt and lacks complete distribution/generated contents.

Preserve null through Arrow, Parquet, DataFusion, offline resolution, and export. Preserve
non-null name/version values exactly. Update generated DTOs from Rust and delete empty-string
fallbacks. Tests must independently distinguish absent declarations, empty malformed headers,
and validated present values; source identity remains exact in every case.

## Alternatives and consequences

Inventing a version from the commit would conflate identity domains. Rejecting all unbuilt
source would discard supported revision evidence. Keeping empty strings would hide an unknown
value behind a value-shaped sentinel. Nullable declarations preserve the actual observed scope
with two narrow schema changes and no second metadata authority.

## Boundaries preserved

Rust owns identities, validation, Arrow schemas and publication. Static Griffe never imports
packages. No build backend is invoked to obtain a missing field. The Python adapter is generated
from Rust types. Exact artifact/revision identity and the six epistemic classes remain distinct.

## Evidence

| Claim | Primary source | Retrieved | Observation |
|---|---|---|---|
| Source inventory currently loses absence to an empty value | `crates/enrichment-core/src/producer/python/archive.rs::inventory` | 2026-09-14 | Optional metadata header lookup ends in `unwrap_or_default`. |
| Ordinary distributions already have a strict validation boundary | `crates/enrichment-core/src/producer/python/archive.rs::validate_metadata` | 2026-09-14 | Required scalar Name/Version headers are checked against selected artifact identity. |
| Current metadata admission rejects revision inventory | `.dev-state/plan10-resolution-e2e.log` | 2026-09-14 | Three real revision fixtures failed with incomplete metadata identity or an independent coverage conflict. |
| Built Core Metadata requires Name and Version | [PyPA Core Metadata](https://packaging.python.org/en/latest/specifications/core-metadata/) | 2026-09-14 | Required fields remain validated for completed distributions; Core Metadata Dynamic cannot name these fields. |
| Source version can depend on backend computation | [PyPA pyproject specification](https://packaging.python.org/en/latest/specifications/pyproject-toml/#dynamic) | 2026-09-14 | Project version can be explicitly dynamic; project name must remain static. |

Primary-source details and the distinction between project dynamic version and forbidden Core
Metadata Dynamic fields are recorded in the compatibility matrix, verified 2026-09-14.

## Verification and delivery

`release_metadata_roundtrip`, `archive_metadata_validation`, real `test_revision_fixture.py`,
`just schemas-generate`, `just schema-conformance`, affected Clippy and Python contract checks.
Acceptance is at Proposed contract strength, not implementation or phase qualification.

## Status history

- 2026-09-14: accepted after the scoped design review resolved N1 and primary-source verification completed.
