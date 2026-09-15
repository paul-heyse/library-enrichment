# Isolated verification proposal review

## 1. Scope and method

Codex, 2026-09-13. Reviewed ADR-0016, design §§8–10, the Phase 4 plan, core request/job/policy
contracts, daemon ownership and upstream qualification matrix against the charter and addendum.
This is a proposal review. No complete runtime, dependency closure, LSP lifecycle or C20 acceptance
is inferred from image startup. Phase 5 client installation and Phase 6 operations are separate.

## 2. Authority

Rust owns request validation, durable state, actual execution policy and result publication.
Python projects the Rust request/result schemas; it cannot enable execution or own job state.

## 3. Semantic contracts

Requested versus observed environment, source versus executable artifact, submission versus shared
job, process success versus compatibility, and cancellation request versus completed cleanup are
separate facts. Optional values do not stand for successful resolution or unsupported operations.

## 4. Execution and publication

The proposed runner lowers concrete producer intent into bounded isolated processes. Inputs pin
source/snapshot/image identities before scheduling. Atomic durable state precedes responses and
only completed validated evidence is published. Restart does not repeat runtime side effects.

## 5. Adversarial journeys

Two callers share a job; one cancels without killing the other's interest. Last cancellation
waits for actual container cleanup. A daemon restart preserves terminal evidence and interrupts
active work honestly. A configuration enabling runtime cannot compensate for missing isolation.
A successful typecheck followed by failing runtime assertion retains both observations. A mutable
context pointer cannot change a queued job's source. A canary repo remains byte-for-byte unchanged.

## 6. Gate assessment

| Gate | Proposal assessment | Required implementation evidence |
|---|---|---|
| G1 | pass | Rust-generated requests, durable job/result identities |
| G2 | pass | Seven states, independent interests, explicit evidence classes |
| G3 | pass | Admission before process creation, source/image/config validation |
| G4 | pass | Actual network/filesystem/resource/cleanup boundary tests |
| G5 | pass | Atomic journal/publication and interrupted restart tests |
| G6 | pass | Complete dedup key and immutable pinned inputs |
| G7 | pass | Scoped probe claims and explicit unsupported/incomplete results |

These are proposal assessments, not executed acceptance-gate results.

## 7. Findings and applicability

| Risk | Principles | Correction | Oracle |
|---|---|---|---|
| Killing shared work on one cancellation | DM-28, DM-35 | Independent durable caller interests | durable_jobs |
| Configuration mistaken for operational sandbox | DM-07, DM-43 | Actual qualification and per-run constraints | execution_boundary |
| Host environment or mutable inputs alter results | DM-12, DM-32, DM-46 | Explicit image/capsule/pinned-input identities | verification_fixture |
| Exit zero reported as universal correctness | DM-08, DM-13 | Explicit probe scope and retained raw outcome | verification_fixture |

Authority, identity, state/effects, concurrency, boundaries and provenance apply. Numerical-model
principles and generic query optimization do not bear on this concrete execution addition.

## 8. Alternatives

Host subprocesses violate the required boundary. A generic scheduler increases authority without
improving these three producer operations. A small journal plus specific producers is proportionate.

## 9. Verification obligations

All oracles named in ADR-0016, generated-schema conformance, Phase 4 acceptance and independent
rerun. Missing images, tools or supported platforms remain blocked; no simulated client pass.

## 10. Exceptions

No binding exception. Linux rootless Podman is the initial qualified backend. Other execution
platforms and target network access require their own explicit evidence and decision.

## 11. Decision

Accept the proposed contract at **Proposed** strength. Implementation remains unaccepted until the
named behavioral, containment, durability and integration checks execute successfully.
