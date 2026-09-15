---
id: ADR-0013
title: Preserve Python observations in the shared evidence model
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-08, DM-14, DM-20, DM-23, DM-32, DM-51]
design: [§3.1, §5.1, §5.2, §5.3, §6.3]
review: docs/design_review/reviews/design_review_static-python-contract_2026-09-13.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A producer cannot express its observations without replacing another producer's evidence.
verification: P01; P02; P03; P04; P05; P07; schema-conformance; legacy snapshot round trip
---

# ADR-0013: Preserve Python observations in the shared evidence model

## Context

The current model and request parser assume Rust. Griffe normally merges source and stubs,
which would discard disagreements required by blueprint §5.3. Python identity must not use
SemVer or infer an analyzed interpreter from the service worker.

## Decision

Add explicit Python interpreter/platform/extras fields and typed distribution metadata. Rust
selects exact PEP 440 releases and suitable artifacts, verifies hashes and safely extracts them.
Pin verified pep440_rs 0.7.3 and zip 8.6.0 (deflate-flate2 only), subject to compile and dependency
policy admission. Do not add pep508_rs's conflicting thiserror major; retain Requires-Dist
verbatim for the later environment producer to resolve.

Preserve separate source/stub observations, publicness signals and extraction gaps. Griffe 2.3.0
runs statically outside studied repositories, with explicit sanitized roots and no inspection or
extensions. Rust owns the worker DTO, normalization and publication; raw output is an artifact.
Add typed Python symbol kinds/inheritance. Canonical table version 2 extends the shared tables
with optional Python metadata; legacy column sets decode with absent observations. Existing
Rust names/IDs remain readable as historical compatibility fields. Root envelope remains 1.0;
generate worker/request/public data schemas from Rust.

## Evidence

| Claim | Primary source | Retrieved | Evidence |
|---|---|---|---|
| Source and stubs are separate evidence | docs/blueprint/IMPLEMENTATION_BLUEPRINT.md §5.3 | 2026-09-13 | "store separate observations and report conflicts" |
| Static loading and merge behavior verified | https://raw.githubusercontent.com/mkdocstrings/griffe/2.3.0/packages/griffelib/src/griffe/_internal/loader.py | 2026-09-13 | Verified upstream source in compatibility matrix; inspection must be explicitly disabled |
| Version selection API | https://docs.rs/pep440_rs/0.7.3/pep440_rs/ | 2026-09-13 | VersionSpecifiers::contains verified by upstream-verifier |
| Bounded archive reader API | https://docs.rs/zip/8.6.0/zip/read/struct.ZipFile.html | 2026-09-13 | enclosed_name, size, encrypted, unix_mode and is_symlink verified |

## Consequences

The public projection remains compact but retains distinct observations when they disagree.
Static inspection never claims target execution, complete dynamic API, or dependency resolution.
Universal artifacts can yield evidence with unknown environment; platform claims require declared
compatibility. Existing immutable snapshots remain authoritative and are never rewritten.

## Verification

P01–P05/P07, worker no-import/side-effect tests, hostile archives, selector compatibility cases,
Arrow semantic round trips, legacy snapshot read, schema conformance, real MCP cold/offline Python.
Dependency admission runs cargo-deny without expanding skips.

## Boundaries preserved

§B1–§B13: Rust core/publication, thin Python, separate extraction, static-first, no repository
mutation, immutable snapshots, six epistemic classes, separate Context7 and no embedded LLM.

## Status history

- 2026-09-13 — accepted at Proposed strength after scoped contract review; implementation pending.
