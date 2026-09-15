---
id: ADR-0033
title: Keep raw producer allocation inside the native worker
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-07, DM-27, DM-35, DM-56]
design: [§4.2, §6.3, §10]
review: docs/design_review/reviews/design_review_native-architecture-completion_2026-09-14.md
evidence: Implemented
supersedes: []
superseded-by: null
revisit: The pinned rustdoc and public-api libraries expose enforceable streaming allocation boundaries.
verification: native_rustdoc_streams_reject_corruption_and_recover_after_bad_input; every_tool_answers_offline_from_the_snapshot_published_cold; hostile_footer_allocation_is_confined_to_native_worker
---

# ADR-0033: Keep raw producer allocation inside the native worker

## Context

Plan 12 AR2 identifies raw rustdoc/public-api allocation before application limits. Both libraries
materialize whole input representations, and public-api disables the JSON parser's depth limit.
Keeping those allocations in the daemon undermines otherwise bounded native ingestion.

## Scope

Extend ADR-0031's private native worker to perform Rustdoc extraction. The executable is now
`library-enrichment-native-worker`; no old executable alias or in-process fallback remains in
the supported launch path. This is static extraction, without executing studied library code.

## Drivers

Confine allocator aborts and stack exhaustion, retain exact provenance, reuse the native process
supervisor, and feed the single Arrow/DataFusion ingestion path without retaining a daemon AST.

## Options

- Post-deserialization limits: insufficient; the allocator can fail before those checks.
- Another rustdoc parser or a public-api fork: duplicates upstream semantics and maintenance.
- Extend the existing bounded worker: selected; one supervisor handles native allocation risks.

## Decision

A closed request identifies verified input bytes, extraction options and a private output root.
The worker installs the existing Linux address-space/CPU/core limits before parsing. It verifies
the raw digest, runs the pinned normalizer and public-api, and emits three finite producer
record streams plus a bounded receipt. Each stream has explicit byte, row and per-record limits.
Missing tools, malformed input, rendering errors and process failures are explicit failures.

The daemon retains only the header and temporary stream descriptors, verifies complete stream
hashes, and decodes at most one bounded producer record at a time. These temporary JSON records
are an extraction transport, analogous to the Python worker boundary; they are neither queried
nor published as evidence. Typed Arrow staging, native reference joins/deduplication and canonical
Parquet publication remain the only evidence path. The daemon never deserializes a raw rustdoc
Crate. The upstream raw AST and signature representation exist only in the bounded worker.

The existing Python worker also installs Rust-supplied address-space and CPU limits from its
generated request before parsing studied source; it preserves stricter inherited limits and
checks the installed values. Its bounded request is decoded before this installation. Native
imports and interpreter startup precede it. Complete source/stub declarations are streamed from
Griffe traversal into a bounded worker response. Documentation is not silently truncated: an
oversized declaration creates a file-scoped gap, with the exact source artifact still retained.
Rust bounds the received transport and performs response decoding and preparation off its async
executor. The extraction version and actual worker interpreter enter normalization provenance.

### Consequences

Cold extraction adds process and finite sequential I/O costs. Warm retained evidence still
requires no producer run. Worker output is disposable when ingestion finishes or fails. Builds
and deployment must include the renamed native executable. The limit is per native process,
not a claim about aggregate service RSS or DataFusion allocations.

### Compensating controls

Reuse request-bound receipts, exact capture/digest checks, bounded output draining, cancellation,
wall deadlines, kill/reap supervision and retention leases. No user-selectable executable, shell,
working-repository access, migration reader or alternative normalization engine is introduced.

## Evidence

Verified 2026-09-14 through Context7 discovery, the required upstream-verifier and direct inspection
of the pinned installed sources.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| public-api materializes JSON and disables depth protection | [public-api 0.52.2](https://raw.githubusercontent.com/cargo-public-api/cargo-public-api/public-api-v0.52.2/public-api/src/lib.rs) | 2026-09-14 | `std::fs::read_to_string(self.rustdoc_json)?`; `deserializer.disable_recursion_limit()` |
| rustdoc-types retains whole-crate maps | [rustdoc-types 0.59.0](https://raw.githubusercontent.com/aDotInTheVoid/rustdoc-types/v0.59.0/src/lib.rs) | 2026-09-14 | `pub index: HashMap<Id, Item>`; `pub paths: HashMap<Id, ItemSummary>` |

| Python process limits are explicit pairs | [Python resource](https://docs.python.org/3.14/library/resource.html) | 2026-09-14 | `Sets new limits of consumption of resource.`; `a tuple (soft, hard)` |

ADR-0031 records the existing native allocation-limit and supervision evidence. This record
extends its consumers; it does not replace those controls with post-allocation estimates.

## Verification

`native_rustdoc_streams_reject_corruption_and_recover_after_bad_input` exercises the actual
worker, bounded record consumption, malformed-input refusal, changed output and retention.
`every_tool_answers_offline_from_the_snapshot_published_cold` exercises actual acquisition,
native extraction, relational publication and offline research. Existing hostile-allocation and
cancellation tests continue to exercise the shared supervisor. Plan 12's ledger records executed
commands; this proposal does not claim full product acceptance.

## Boundaries preserved

§B1–§B13 remain intact. Rust owns producer identities, normalization, querying and publication;
Python remains the adapter and separate static/runtime producer. The worker publishes nothing.
Build/runtime policy, exact-context evidence retention and repository isolation are unchanged.

## More information

[Plan 12](../plans/12-architecture-first-completion.md),
[ADR-0031](0031-bounded-native-parquet-admission.md).

## Status history

- 2026-09-14 — proposed; implemented with scoped review and remaining integrated validation open.

- 2026-09-14 — accepted after scoped native architecture review; remaining actual producer,
  client and final performance qualification stays in Plan 12.
