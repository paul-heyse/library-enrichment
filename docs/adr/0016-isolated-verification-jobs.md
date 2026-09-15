---
id: ADR-0016
title: Own isolated verification and durable caller interests
status: superseded
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-07, DM-12, DM-13, DM-28, DM-29, DM-32, DM-46]
design: [§8.2, §8.3, §9.1, §9.2, §9.3, §10]
review: docs/design_review/reviews/design_review_isolated-verification_2026-09-13.md
evidence: Proposed
supersedes: []
superseded-by: ADR-0017
revisit: Another execution platform or network-enabled target execution is required.
verification: execution_boundary; durable_jobs; verification_fixture; semantic_fixture; schema-conformance; gate-phase 4
---

# ADR-0016: Own isolated verification and durable caller interests

## Context and scope

Phase 4 needs executable consumer evidence, persistent jobs, reusable semantics and explicit
cancellation. Current static evidence and declared environments cannot establish these facts.
This specifies the Rust-owned wire and execution boundary without changing the frozen envelope.

## Decision

Expose typed verification and job-control requests. A request identifies an existing context and
snapshot, a bounded agent-supplied snippet, compile/typecheck/runtime mode, explicit profile and
test intent. It never supplies shell commands, container arguments or host paths. Static reads
remain independent. An omitted snapshot is pinned before queuing; subsequent pointer changes
cannot change a job's inputs. Store the exact request, snippet, source IDs, environment inputs,
image and producer identities, commands, bounded output, exit/deadline/cancellation outcome.
A successful process exit means success only for that explicit probe, not assertion completeness.

The daemon owns a durable journal with the existing seven states. Every submission gets a
separate interest token even when it shares an in-flight job. A cancellation detaches that
interest; only removal of the last interest requests process cancellation. Persist transitions
before returning them. Terminal results survive restart; interrupted work fails with a concrete
resubmission action and is never silently rerun. Deduplication includes pinned source/snapshot,
normalized options, actual image/tool identity, environment and policy. Queue and concurrency
limits apply before spawning. Poll waits are bounded; adapters disconnecting do not cancel jobs.
A pending envelope carries the frozen JobHandle and a typed data object with the interest token.

Only configured build/runtime profiles may request execution, and both require a working
isolation backend regardless of permissive legacy configuration switches. Linux rootless Podman
is the initial backend. An operator admits exact derived image identities into service-owned
storage; ordinary execution uses pull=never, an immutable image, non-root UID, read-only root,
dropped capabilities, no-new-privileges, no network, explicit CPU/memory/process/time/output
limits and service-owned capsule mounts. The broker receives only required local identity/bus
settings; the container receives no host secrets, home, toolchain or studied repository mount.
Cancellation/deadline/output overflow removes the exact owned container and descendants before
a terminal cancellation is published. Missing isolation is POLICY_DENIED, never host execution.

Separate network acquisition from target execution. Admit registry-only manifests/requirements
and exact source artifacts, discard unadmitted package-manager configuration, preserve complete
locks and hashes, then run offline in a capsule. Python uses verified wheel-only uv resolution,
Rust validates/fetches admitted wheel URLs, and offline installation uses the complete closure.
Unavailable source builds/native dependencies are explicit environment gaps. Rust uses sanitized
consumer manifests and registry-only dependency resolution; offline execution does not itself
prove containment. Resolved or verified environments create new identities/contexts, preserving
original declarations and unreproduced project inputs.

LSP sessions are keyed by producer image/server/capsule identity, bounded and reused. Rust owns
framing, initialization, negotiated positions, document versions, cancellation and shutdown.
Queries distinguish unsupported, unresolved, incomplete and empty results. Signature reads do
not start servers. ty disables external uv integration; rust-analyzer separately controls build
scripts, procedural macros and check-on-save. Hosted Rust JSON remains first; fallback requires
explicit build policy and the dated nightly, with emitted format checked before publication.

## Options and consequences

Host execution is simpler but violates the repository and environment boundary. A generic task
engine adds unnecessary authority and mechanisms. Concrete Rust producer operations and a small
journal keep the execution and publication contract inspectable. Images cost disk and setup time;
only actual qualification establishes readiness. Additional platforms remain unsupported until
qualified, rather than being reported operational from configuration alone.

## Evidence

| Claim | Source | Retrieved | Evidence |
|---|---|---|---|
| uv wheel-only mode excludes ordinary source builds but still permits editable builds | [uv 0.12.13 source](https://raw.githubusercontent.com/astral-sh/uv/0.12.13/crates/uv-cli/src/lib.rs) | 2026-09-13 | "uv may still build editable requirements"; input admission remains necessary |
| Cargo fetch is separate from compilation | [Cargo 797e8a9bc source](https://raw.githubusercontent.com/rust-lang/cargo/797e8a9bc/src/cargo/ops/cargo_fetch.rs) | 2026-09-13 | Package resolution/download operation; does not establish subprocess isolation |
| Rootless resource and identity flags compose on this host | docs/architecture/compatibility-matrix.md Phase 4 rows; .dev-state/p4p startup receipts | 2026-09-13 | Actual UID/GID 65532, cgroup limits, read-only root and network=none; stress/cancellation not yet accepted |
| ty and rust-analyzer differ in LSP capabilities | Compatibility matrix exact-release initialization probes | 2026-09-13 | Negotiated UTF-8/UTF-16 and document diagnostics; workspace diagnostics unavailable from rust-analyzer |

The upstream-verifier's dated primary-source and executable receipts support these limited claims.
They do not establish the unimplemented service's acceptance.

## Verification

Durable-journal transition/restart/two-interest tests; actual container execution with correct and
incorrect snippets, output/deadline/fork/network/filesystem cases; LSP diagnostics/Unicode/capability
cases; dated-nightly fallback; C20 content-and-path canary digest for every enabled profile.
Run generated-schema checks, Phase 4 gate and independent acceptance reruns.

## Boundaries preserved

B1–B13 remain binding: Rust authority, thin adapter/static worker, ty/rust-analyzer producers,
immutable evidence, no embedded agent, no graph or generic workflow engine, no automatic project
edits, explicit policy and hosted rustdoc first. No envelope enum or frozen contract changes.

## Status history

- 2026-09-13 — accepted at Proposed strength following scoped proposal review; implementation and acceptance obligations remain explicit.
- 2026-09-13 — superseded by ADR-0017.
