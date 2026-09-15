# Static Python evidence contract

## 1. Decision and scope

Proposal review, Codex, 2026-09-13. Scope: Phase 2 Rust-owned model and worker contract.
Read blueprint §§3,5–8,10,13; current identities, typed records, Arrow projections, snapshot
publication, retrieval and MCP validation; compatibility matrix's verified Griffe and ZIP
interfaces. Runtime/ty, durable jobs and client acceptance remain later required phases.
This review establishes decidability of the proposed change, not implemented correctness.

## 2. Authority and lifecycle map

| Meaning | Authority | Consumer |
|---|---|---|
| Distribution/release/selection | Rust PyPI reader using exact PEP 440 versions | Context and immutable inputs |
| Worker observations | Rust-generated worker DTOs; Griffe emits facts | Rust normalizer |
| Publicness and signature provenance | Separate typed Python observation per source/stub | Inspect, comparison, Arrow |
| Publication | Rust daemon after validation/Parquet round trip | Existing retrieval tools |
| Import roots/files | Rust archive inventory | Explicit worker search paths |

## 3. Semantic contracts and invariants

Add Python interpreter/platform/extras request fields; reject cross-ecosystem fields. Omitted
interpreter/platform mean unknown, not the worker's interpreter. A universal wheel can be
statically inspected without claiming target compatibility. A platform wheel requires an exact
compatible tag declaration; absent suitable wheels permits source inspection, never a build.
Selection respects requires-python, prerelease and yanked policy and verifies selected hashes.

Retain existing Rust fields/IDs as historical compatibility fields. Add optional Python
metadata and observations, typed class/attribute kinds and inheritance edges. A symbol is one
public path with separate source and stub observations. Never replace one with the other.
Unresolved aliases retain their targets and gaps. Publicness signals are separate optional
facts (__all__, naming convention, reexport, documentation), never a privacy verdict.

## 4. Derivation and execution design

Canonical tables version 2 add a nullable structured Python column. Old column sets decode
with absent Python observations; existing IDs remain untouched. Envelope stays 1.0. Generate
worker and request schemas with the same Rust schema emitter as public response data.
ZIP/tar extraction rejects traversal, links, special/encrypted entries, duplicates and resource
excess before extraction. Worker starts outside the studied repository with sanitized environment,
explicit roots, no target on sys.path, allow_inspection=False, no extensions. Each .py and .pyi
is visited independently; Rust reconciles results against the archive inventory. Missing native
or dynamic API is partial. Raw worker output is stored before normalization.

## 5. Representative journeys

A distribution with another import name resolves via archive inventory. Import-time writes never
run. A namespace package retains every discovered root. Stub-only native modules expose signatures
and source-availability gaps. Different stub/source annotations remain visible together. Inventory
entries preserve project/version and document locators without executing documentation builds.

## 6. Acceptance gates

| Gate | Verdict | Proposal evidence | Acceptance oracle |
|---|---|---|---|
| G1 | pass | Rust owns identity/model/publication | Schema conformance; worker DTO validation |
| G2 | pass | Explicit missing and separate observations | Source/stub conflict and native fixtures |
| G3 | pass | Unknown interpreter is not inferred | Exact selection and compatibility rejection |
| G4 | pass | Static worker boundary, safe extraction | P02, P10, archive adversarial cases |
| G5 | pass | Shared immutable publication path | Python MCP cold/offline and round trips |
| G6 | pass | Versioned table extension and old-column decoding | Legacy Rust snapshot read |
| G7 | pass | Typed gaps and observation limits | P01–P05/P07; no mocked acceptance |

Verdicts concern decidable contracts at Proposed strength.

## 7. Findings and applicability

| Finding | Principles | Consequence | Correction | Verification |
|---|---|---|---|---|
| Current request assumes SemVer/Rust configuration | DM-08, DM-15 | Python versions rejected or environment invented | Ecosystem-specific parsing and explicit fields | P01 and invalid cross-ecosystem requests |
| A single signature loses source/stub conflict | DM-14, DM-32 | False consensus | Separate observations and provenance | P03 plus Arrow round trip |
| Griffe's default merged loading loses evidence | DM-20, DM-23 | Stub overrides source | Independent static visits with inventory reconciliation | P02/P04/P05 |
| New columns must not invalidate historical reads | DM-51 | Pinned evidence unreadable | Optional additive projection and versioned manifest | Legacy snapshot test |

Authority, absence, provenance, boundary, migration and testability apply. Numerical precision,
remote coordination and runtime execution are outside this static slice.

## 8. Alternatives

| Alternative | Result |
|---|---|
| Python core/resolver | Reject: duplicates Rust authority |
| Merge source/stubs before normalization | Reject: destroys conflicts |
| Keep separate raw observations and normalize in Rust | Select: explicit evidence lineage |
| Build sdists during static extraction | Reject: violates execution profile |

## 9. Verification plan

Admit verified exact dependencies under cargo-deny without adding ignores. Run selector/archive/
normalizer tests; Ruff/ty and worker contracts; real stdio fixture and opt-in PyPI tests; Phase 2
gate and state isolation. No performance claim follows from this review.

## 10. Exceptions and unresolved scope

No exception to binding boundaries. Unsupported artifact tags and unavailable source are explicit
gaps, not portability claims. Execution and semantic acceptance remain required Phase 4 work.

## 11. Decision

| Priority | Decision | Strength | Condition |
|---|---|---|---|
| 1 | Accept proposed additive Python contract | Proposed | Implement named negative and legacy oracles before phase acceptance |
