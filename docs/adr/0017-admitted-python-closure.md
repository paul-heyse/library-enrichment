---
id: ADR-0017
title: Admit dependency metadata before following registry edges
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-07, DM-20, DM-28, DM-43, DM-46]
design: [§8.2, §8.3, §9.1, §9.2, §9.3, §10]
review: docs/design_review/reviews/design_review_admitted-python-closure_2026-09-13.md
evidence: Implemented
supersedes: [ADR-0016]
superseded-by: null
revisit: A required package needs unsupported markers, native wheel selection, or dependency backtracking.
verification: requirements::tests; verification_fixture_admits_transitive_metadata; execution_boundary; gate-phase 4
---

# ADR-0017: Admit dependency metadata before following registry edges

## Context

The Phase 4 implementation review found that a networked uv resolver can follow a direct URL,
Git or filesystem dependency in transitive metadata before Rust inspects its eventual lock.
Post-resolution validation is too late to enforce source admission. The proposed pep508_rs0.9.2
alternative introduces unadmitted duplicate thiserror1 dependencies and differs from current
marker semantics; no pin, duplicate exception or vendored fork is added.

## Decision

Retain every decision in ADR-0016 except its Python networked uv-resolution mechanism. This
record supersedes that proposal as a whole, adopting its unchanged request, durable-job,
container, cancellation, LSP and probe contracts by reference.

Rust resolves a bounded registry wheel closure itself. Parse every Requires-Dist in an acquired
wheel before following any dependency named by that metadata, including inactive requirements.
Reject direct URL, Git, filesystem/editable and malformed forms before dependency fetching.
Evaluate admitted markers against the explicit Linux x86_64 CPython3.14.7 capsule and requested
extras. Unknown/unreproduced marker fields fail with ENVIRONMENT_UNRESOLVED. This is an explicit
supported grammar/environment boundary, not a claim of universal packaging-language support.

Apply the existing PEP440 version implementation to constraints. Select an eligible pure wheel
through the existing registry selector, enforce HTTP policy and digest bounds, safely extract
and corroborate actual internal wheel metadata, then recursively admit its requirements. Keep
at most256 packages,1024 expansion steps,512MiB dependency bytes and an overall acquisition
deadline. Every HTTP wait races caller cancellation and that deadline. Cycles retain selected
identities; incompatible later constraints fail explicitly rather than publishing an invalid
solution. Backtracking and native/source build resolution remain explicit unsupported scopes.

The retained deterministic lock contains all selected packages, versions, hashes, URLs,
requirements and activated extras plus resolver/environment identity. Only the complete admitted
closure is sent to uv for offline, no-index, hash-required, binary-only installation. uv never
receives permission to discover dependency sources over the network. The original selected root
wheel is reused by digest, never silently replaced with a newer artifact.

## Options and consequences

Networked uv is convenient but cannot enforce this admission boundary from its output alone.
Adding pep508_rs would violate current dependency policy and still require semantic qualification.
A conservative Rust parser and bounded resolver provide executable scope with explicit gaps.
They require packaging-grammar regression tests and deliberately refuse unsupported environments
or conflicting greedy choices instead of claiming a general SAT solver or full Python installer.

## Evidence

| Claim | Source | Retrieved | Evidence |
|---|---|---|---|
| Requirements may name direct sources and conditional dependencies | [PyPA dependency specifiers](https://packaging.python.org/en/latest/specifications/dependency-specifiers/) | 2026-09-13 | Grammar includes URL references, extras and environment markers; admission precedes effects |
| Wheel-only uv resolution is insufficient source admission | [uv0.12.13 source](https://raw.githubusercontent.com/astral-sh/uv/0.12.13/crates/uv-cli/src/lib.rs) | 2026-09-13 | "uv may still build editable requirements"; verified matrix and review F1 |
| Candidate parser cannot be pinned under current policy unchanged | Upstream-verifier exact pep508_rs0.9.2 archive/Cargo metadata and API probe | 2026-09-13 | thiserror1/thiserror-impl1 duplicate existing2; current-marker differences remain |
| Admitted closure and hostile transitive metadata behavior run end to end | .dev-state/phase4-verification.log | 2026-09-14 UTC | Five focused real MCP fixtures passed; not a complete Phase4 gate |

## Verification

Core parser tests reject source references and unknown marker environments. Real HTTP/MCP fixture
installs and imports an admitted dependency; hostile transitive URL metadata fails before a source
request or resolver invocation. Add cancellation-during-transfer and conflicting-constraint cases.
Full Phase4 gate and independent re-review remain required; the prior checkpoint review is Revise.

## Boundaries preserved

B1–B13 remain unchanged. Rust owns resolution, fetching, policy and publication; the Python adapter
remains thin and ty remains the semantic engine. No generic resolver service, embedded agent,
duplicate dependency exception or project mutation is introduced.

## Status history

- 2026-09-13 — accepted revised contract following scoped review; implementation present, acceptance open.
