# Design review — the design spine and the service as it actually stands

Standard: [`DATA_MODEL_DESIGN_CHARTER.md`](../design_principles/DATA_MODEL_DESIGN_CHARTER.md),
with [`ADDENDUM.md`](../design_principles/ADDENDUM.md) for the §B-to-gate mapping.

## 1. Decision and scope

**Proposal:** `docs/design/DESIGN.md` as the authoritative design spine (ADR-0009), reviewed
against the service that exists in the working tree.
**Status:** mixed — see §6. The document is newly written; the code it describes is partly
Implemented and partly ahead of the document.
**Reviewer:** design-review skill, invoked by the maintainer.
**Affected revisions:** `DESIGN.md` revision 1; working tree at branch
`phase-0-service-implementation`, commit `0cd7330` plus uncommitted work.

**Observable outcome sought:** one readable statement of what the design is now, with every
claim carrying an evidence label, and every decision traceable to a record.

**Baseline:** the frozen blueprint plus eight decision records and `STATUS.md` prose.

**Supported scope and non-goals:** this reviews the design *document* and the Rust/Python
service. It does not review the process machinery ADR-0009 introduces (that record is its own
subject), the frozen contracts, or the shipped `library-research` skill.

**Constraints and uncertainty:** the tree contains substantial uncommitted work, so "the
implementation" is a moving target; see the coverage note.

### Method and coverage

Read in full: `docs/design/DESIGN.md`; `crates/enrichment-store/src/snapshot.rs`;
`crates/enrichment-core/src/canonical.rs`; `crates/enrichment-core/src/clock.rs`;
`crates/enrichment-daemon/src/ops/search.rs` (ranking through envelope construction);
`crates/enrichment-daemon/src/envelope.rs` (header and builders);
`python/enrichment_mcp/envelope.py` (header and exports); `python/enrichment_mcp/server.py`
(tool wiring, `_not_implemented`, `_research`, error mapping);
`crates/enrichment-core/src/identity/mod.rs` (`SnapshotInputs`, `derive`, `derive_id`);
`crates/enrichment-daemon/src/ops/resolve.rs:1015–1125` (the snapshot-key construction).
Skimmed: `crates/enrichment-daemon/src/fetch/mod.rs` (redirect and policy structure only).

**Established by running:** `cargo check --workspace --all-targets` (clean);
`git status --porcelain`; `python3 scripts/adr.py lint`; `bash scripts/test-adr-lint.sh`.

**Not inspected, and therefore not cleared:** `crates/enrichment-core/src/archive/` (path
traversal, symlink escape, decompression bombs — §10's extraction rules); `policy/mod.rs`;
`registry/`; `producer/normalize.rs`; `store/query.rs` and `tables.rs`; the whole Python worker
path; every test file. **No test suite was run**, so no claim here is labelled `Tested` on the
strength of this review; where a test is named it is named as the oracle that *would* settle a
question, not as evidence that it passed.

**Guarantees attacked:** the snapshot publication boundary (G5); the coverage/`ok` discipline on
the search path (G2); the exclusion of wall-clock time from content identity (G4); the
single-authority claim for envelope construction (G1); the completeness of the snapshot reuse key
(G6). **Guarantees asserted but not attacked:** archive-extraction safety; SSRF and size limits
beyond the redirect re-check; concurrency and single-flight; cancellation; restart recovery;
anything in the Python worker.

## 2. Authority and lifecycle map

Reconstructed from the document and the code. Cells I had to invent are marked **[inferred]** —
that list is itself the evidence for F1 and F5.

| Concept | Identity | Authority | Revision boundary | Update path | Derived representations |
|---|---|---|---|---|---|
| Envelope shape | `schema_version` | `crates/enrichment-core/src/wire/` | frozen contract + generated schema | edit Rust type, `just schemas-generate` | JSON Schema; Pydantic DTO |
| Evidence set | `snapshot_id` (content-derived) | daemon only; `snapshot::publish` | immutable directory + `current` pointer | new snapshot, then pointer swap | Parquet tables; search pages |
| Research context | `context_id` (content-derived from release+environment+mode) | `identity::Context::derive_id` | new derived context, never mutated | `derived_with` | — |
| Producer identity | name → version in `SnapshotInputs.producers` | the op that assembles the key | snapshot key | — | `ProducerRun` provenance |
| Execution policy | profile name | `config` | **[inferred]** — not in any identity | — | — |
| **Service state (which phase, which tools live)** | — | **[inferred] — contested. See F1.** | — | — | `service_status`; `_not_implemented`'s message; `STATUS.md`; `DESIGN.md` |

**Deliberately opaque:** rustdoc's own item IDs (never used as cross-release identity — correctly
stated in §3.1); upstream registry JSON, preserved as blobs so normalization can improve without
refetching.

**Identity behavior:** content identities are stable across map-ordering changes by construction —
`canonical.rs` sorts object keys recursively rather than relying on `serde_json::Map`'s type,
which feature unification swaps between `BTreeMap` and `IndexMap`. That is a real defence against
a real, already-encountered hazard.

## 3. Semantic contracts and invariants

| Invariant | Representation | Enforcement boundary | Failure behavior | Verification evidence |
|---|---|---|---|---|
| `status`/`job`/`error` are consistent | `wire::Outcome` | unrepresentable in the type; re-established in `TryFrom<RawEnvelope>`; restated as the schema's root `allOf` | rejected at every boundary | `just schema-conformance`; gate C19 |
| Wall-clock time never enters a content identity | `canonical.rs` contract; `clock.rs` is provenance-only | convention at call sites | none — **nothing rejects a timestamp passed into `from_content`** | asserted; no oracle found |
| A reader never sees a partial snapshot | staging → per-table read-back verify → single `rename` → pointer swap | `snapshot::publish` | staging directory left behind, swept on next open, never read as evidence | code path read; test not run |
| Absence is not proof of absence | `coverage.limitations`, always carrying the caveat | `ops/search.rs:230` | — | code path read |
| The snapshot key covers everything that can change the result | `SnapshotInputs` | `SnapshotId::derive` | silent stale hit | **unresolved — see F5** |

**Absence and uncertainty:** well handled on the path inspected. An empty search returns `ok` with
an explicit limitation — *"absence from these results is not evidence that the crate lacks a
capability"* (`ops/search.rs:230-232`) — and a missing evidence kind downgrades the envelope to
`partial` (`ops/search.rs:307`). This is the doctrine's hardest rule and the code follows it.

## 4. Derivation and execution design

Compressed: the acquisition pipeline is out of the inspected scope. The one stage traced end to
end is publication, and it is sound. `stage_tables` writes each Parquet table, reads it back, and
compares row counts before anything is renamed into place (`snapshot.rs:180-195`); a lost race to
an identical publication is detected after the failed `rename` and resolved by repointing rather
than erroring (`snapshot.rs:135-145`).

One ordering note, correct as written: the `current` pointer is swapped *after* the rename, so a
crash between them leaves a published-but-unreferenced snapshot and a context still pointing at
the previous complete one. That is the safe direction and the module says so.

## 5. Representative journeys

**Interruption during publication.** Crash mid-`stage_tables` → staging directory, no
`snapshots/` entry, no pointer change; swept on next open. Crash between `rename` and
`set_current_snapshot` → the new snapshot exists and is unreferenced; readers keep the old one.
Both observable states are valid. **No finding.**

**A caller asks whether `compare_releases` is available.** The adapter answers
`UNSUPPORTED_CAPABILITY` with *"This service is at phase 0; `compare_releases` lands in phase 3"*
(`python/enrichment_mcp/server.py:139`). The first clause is false: five Phase-1 tools are wired
to live daemon methods in the same file. **F1, F3.**

**A reviewer asks what the service currently does.** `DESIGN.md` §4, §5, §6.1 and §8 say
*Proposed*; `STATUS.md:5` says *"No evidence retrieval exists yet — that is Phase 1."* The tree
contains `ops/resolve.rs` (1292 lines), `producer/docsrs.rs`, `producer/normalize.rs`,
`search/`, `store/snapshot.rs`, `store/tables.rs` and two fixture test files. **F1.**

## 6. Acceptance gates

| Gate | Verdict | Evidence or scope rationale | Required action |
|---|---|---|---|
| G1 — Authority | **fail** | Four artifacts independently state the service's phase and capability set, and at least three are wrong (F1). Separately, `DESIGN.md` — the document ADR-0009 makes authoritative — does not describe the implementation that exists. | F1 |
| G2 — Semantic fidelity | **pass**, scoped | On the inspected paths absence, partiality and coverage are distinct and correctly propagated (`ops/search.rs:230-310`). Not assessed for normalization or the Python worker. | — |
| G3 — Validity | **not applicable to this review's coverage** | The extraction and registry-parsing boundaries where invalid input would arrive were not inspected. Recorded as uninspected, not as clean. | inspect in a follow-up |
| G4 — Hidden behavior | **pass**, scoped | `clock.rs` is provenance-only by contract; `canonical.rs` excludes timestamps and temporary paths from identity. No inspection path was found that mutates or fetches. | — |
| G5 — Consistency and recovery | **pass** | `snapshot::publish`: stage, verify by read-back, single rename, then pointer swap; staging swept and never read as evidence. | — |
| G6 — Transformation and reuse | **unresolved** | The execution-policy profile is not a declared input to `SnapshotId`, though blueprint §8.2 names `policy_profile` in the dedup key. It appears transitively covered by the producer map — but that argument is the reviewer's, not the design's (F5). | F5 |
| G7 — Truthful capability claims | **fail** | The service returns a statement about its own capability level that is false (`server.py:139`). A capability claim that is wrong *about the service itself* is the purest form of this gate's failure. | F1, F3 |

An unresolved gate is not a pass, and a strong G5 does not offset G1 or G7.

## 7. Principle findings

| # | Finding | Principles | Evidence or gap | Consequence | Proposed correction | Verification (oracle) |
|---|---|---|---|---|---|---|
| **F1** | The service's own state — which phase it is at and which tools are live — has four independent statements and no authority. `DESIGN.md` §4/§5/§6.1/§8 say *Proposed*; `STATUS.md:5` says *"No evidence retrieval exists yet"*; `server.py:139` says *"This service is at phase 0"*; and `server.py:208-378` wires `resolve_library`, `library_overview`, `search_evidence`, `inspect_symbol` and `read_artifact` to live `_research` calls backed by `ops/resolve.rs`, `ops/search.rs`, `ops/inspect.rs`, `store/snapshot.rs`. | DM-02, DM-59, DM-23 · G1, G7 | The four sites above, plus `git status --porcelain` showing 27 untracked source paths under `crates/`. | An agent reading `DESIGN.md` to decide what to build next reimplements a working `docsrs` producer. A caller reading the `UNSUPPORTED_CAPABILITY` message concludes no retrieval exists and does not call `search_evidence`, which works. A reviewer grades the design against a spine that describes a system two phases behind the code. | One authority for service state. The cheapest version: `service_status` derives the live-tool set from the daemon's dispatch table; `DESIGN.md` and `STATUS.md` cite it rather than restating it; `/handoff` regenerates `STATUS.md` from the tree, which it is already meant to do. | **`just` gate.** A recipe asserting that every tool the adapter routes to `_research` is absent from `_PHASE_OF`'s not-implemented set, and that the daemon dispatches each. None exists today. |
| **F2** | The entire Phase-1 implementation is untracked in git, so the checks that read git state silently cover nothing. | DM-31, DM-46, DM-48 · G1 | `git ls-files --error-unmatch crates/enrichment-daemon/src/ops` → UNTRACKED, likewise `search/`, `evidence/`, `store/tables.rs`, and 23 more. `STATUS.md:242` already records that `git diff --exit-code` over `schemas/generated/` "only sees tracked files"; the same blind spot now covers the whole slice. `adr.py lint`'s immutability check has the same shape — it diffs against `main`, and nothing here is on `main`. | `just ci` reports green over a tree git cannot describe. An acceptance report generated now names no commit (register row R-02) *and* the source it graded cannot be recovered. A `passed` gate becomes unreproducible in the strongest sense: the code is gone if the working tree is. | Commit the slice, or mark it explicitly as a work-in-progress branch whose gate results are not citable. The general fix is that a report should refuse to promote a gate when the tree has untracked files under `crates/` or `python/`. | **`just` gate.** Extend `scripts/acceptance-check.py`: refuse to promote any gate to `passed` when `git status --porcelain` shows untracked or modified source paths, and record the commit in `acceptance.json`. This also closes register row R-02. |
| **F3** | `_not_implemented` hardcodes the service's phase in caller-visible text, and `_PHASE_OF` retains entries for five tools that no longer route through it. | DM-59, DM-13 · G7 | `server.py:139` `f"This service is at phase 0; ..."`; `_PHASE_OF` (`server.py:57-66`) lists all eight, but only `compare_releases`, `verify_usage` and `job_control` reach `_not_implemented`. The other five entries are unreachable. | The message is wrong today and will be wrong again at every phase boundary, in a string a caller reads and acts on. The dead entries make `_PHASE_OF` look like a service-state registry when it is an error-message lookup. | Drop the phase clause from the message, or source it from one place. Reduce `_PHASE_OF` to the tools that actually use it, so it stops implying authority it does not have. | **`ast-grep` rule** in `rules/`: no literal `phase <n>` string in `python/enrichment_mcp/**`. Frictionless to add, with fixtures in `rule-tests/`. |
| **F4** | `crates/enrichment-daemon/src/envelope.rs:9-11` states the adapter "keeps its own builder for exactly one case -- reporting that the daemon is unreachable". There are five adapter-local construction sites. | DM-59 | `server.py` builds envelopes at lines 119 (own output failed own schema), 136 (`_not_implemented`), 521 (`_rpc_error_envelope`), 542 (daemon unavailable) and 555 (daemon returned no envelope). Every one is a legitimately adapter-local fact — the claim is not that the boundary leaks, only that the count is wrong. | A reader auditing the authority boundary trusts "exactly one" and stops looking; the next contributor adds a sixth without noticing a stated invariant was already false. Low severity: the boundary itself holds. | Restate as "cases the core cannot state: an unreachable daemon, a malformed daemon response, an unimplemented tool, and a mapped RPC error." | **None exists, and none is worth building** — this is a prose claim about a code shape with no cheap oracle. Recording it as prose is the honest tier (charter §H: do not add machinery costing more than the risk). |
| **F5** | The execution-policy profile is not a declared input to the snapshot identity, although blueprint §8.2 names `policy_profile` in the dedup key. | DM-31, DM-32 · G6 | `SnapshotInputs` (`identity/mod.rs:412-423`) carries `schema_version`, `normalizer_version`, `context_id`, `input_digests`, `producers` — no profile. `ops/resolve.rs:328` shows behaviour genuinely gated on `ExecutionProfile::Build.is_enabled(...)`. | If a profile change can alter the evidence without altering the producer map, a cache hit returns evidence gathered under a policy the caller did not select. I could not construct such a case — the producer map appears to move whenever the profile changes the outcome — which is exactly why this is *unresolved* rather than *violated*: the design never states the argument, so nobody is maintaining it. | Either add the profile to `SnapshotInputs`, or write the transitive-coverage argument into `DESIGN.md` §8.4 as a stated invariant so a future change has something to violate. | **A test.** Resolve the same context under `static` and under `build`, assert the snapshot identities relate as intended. None exists. |

**Applicability.** Groups 1–3 (authority, types, identity), 6 (effects and recovery), 7
(dependencies and reuse) and 10 (provenance) carried this review. Group 4 (declarative
composition) and group 5 (compilation and lowering) did not bear on it: the service has no
authoring language and no IR, by design (§B11), so the principles about templates, expansion and
progressive lowering have no subject here. Group 8 (execution representations and performance) is
not applicable *yet* — there is no performance claim in `DESIGN.md` to falsify, which is the
correct state for a design whose §14.3 calls metrics "engineering diagnostics, not a benchmarking
project". Group 9 (boundaries and providers) is partly applicable and largely uninspected: the
registry and extraction adapters are where DM-41–DM-45 would land, and this review did not read
them. Group 11 (verification) is F2's subject. Group 12 (leverage) is addressed in §8.

No maturity score is offered. Scoring dimensions while G1 and G7 fail would be exactly the
substitution charter §A forbids.

## 8. Alternatives and architectural leverage

| Alternative | Semantic duplication / extension locality | Risks | Cost | Performance evidence | Selected? |
|---|---|---|---|---|---|
| Current baseline (before ADR-0009): blueprint frozen, `STATUS.md` prose as the state-of-work record | Service state stated in prose in one place, and implicitly in code | Prose drifts silently; two reviews left no artifact | nil | none | No |
| Proposed: `DESIGN.md` as a spine, amended by ADR | Adds a *third* statement of service state (F1) unless it defers to the code for what is live | Spine drift — realised immediately, which is F1 | one document, maintained per ADR | none | Yes, with F1 outstanding |
| **Simpler viable alternative:** `DESIGN.md` carries only what the code *cannot* state — binding decisions, invariants, the amendment rule — and every "is this built yet" claim comes from generated output (`service_status`, `acceptance.json`) | One authority for state, one for intent; no per-phase editing of the spine at all | The spine becomes less readable as a narrative; a reader needs two artifacts | lower ongoing cost, higher one-off cost to wire the generation | none | **Not selected, but it is the stronger design on this axis** |

The third row is the finding behind F1 stated constructively. `DESIGN.md`'s per-section
*Evidence:* labels are the part that will rot, because they restate a fact the tree already knows.
The binding decisions, the invariants and the citation targets are the part that earns its keep —
those cannot be derived from code and are what ADRs need to bind to. A future revision should
consider narrowing the spine to the latter and generating the former.

**Machinery examined for over-construction:** the §B1–§B13 ID scheme adds a naming layer over a
table that already existed. It is justified: eight ADRs already cite "the §1.1 row" in prose, and
`adr.py lint` now resolves those citations mechanically, which prose cannot support. The register
adds nine rows of real deferrals; it is not speculative. No recommendation to remove either.

## 9. Verification and measurement plan

| Claim or risk | Evidence label | Check | Conditions and expected result | Current result / gap |
|---|---|---|---|---|
| Readers never observe a partial snapshot | **Implemented** (code path read; not run) | a test that kills publication between `stage_tables` and `rename`, and between `rename` and the pointer swap | both leave a valid observable state | no such test found |
| Absence is reported as absence of *evidence*, not of capability | **Implemented** | contract test asserting an empty search carries the limitation and the coverage block | present | `ops/search.rs:230` constructs it unconditionally; no test found asserting it |
| Wall-clock time never enters a content identity | **Proposed** — it is a convention at call sites, not an enforced contract | derive an identity twice across a second boundary and compare | equal | `canonical.rs` states the rule; nothing rejects a timestamp passed to `from_content` |
| The snapshot key is complete | **Unresolved** (F5) | resolve one context under two profiles; compare identities | as intended, once "as intended" is written down | neither the test nor the statement exists |
| Gate results are reproducible from a recorded tree | **Proposed** | `acceptance-check` refusing untracked source; commit recorded in the report | fails today | F2; register row R-02 |

**Cost accounting:** not material to this review. No performance claim was made or assessed.

## 10. Exceptions and unresolved decisions

**F5** is the only item that belongs here as a scoped unresolved decision.
**Principle IDs:** DM-31, DM-32. **Scope:** snapshot reuse across execution profiles.
**Reason:** the profile's effect appears to be carried transitively by the producer map, and the
simpler key was chosen. **Consequence:** an invariant nobody has written down is load-bearing;
**compensating control:** none today. **Evidence:** `identity/mod.rs:412-423` against blueprint
§8.2. **Owner:** paul-heyse. **Revisit trigger:** the first producer whose *output* depends on the
profile without its *identity* doing so.

No SHOULD-level deviation was found that needs an exception record. F1 and F3 are MUST-level gaps
on in-scope behaviour and are not exceptions — they are defects.

## 11. Decision and implementation changes

**Decision: Revise.**

**Reason.** G5 is genuinely strong, G2 and G4 hold on everything inspected, and the publication
and coverage paths are better than most code that claims this discipline. None of that offsets
G1 and G7: the service makes a false statement about its own capabilities to callers
(`server.py:139`), the authoritative design document describes a system two phases behind the
code, and the code in question is untracked. The charter is explicit that a truthful-capability
failure at the core is not traded against strength elsewhere.

This is not a verdict on the Phase-1 slice's quality — this review did not inspect enough of it
to have one, and says so in §1. It is a verdict on the *coherence between what the system says
about itself and what it is*.

| Priority | Change | Principles | Acceptance evidence | Regression protection |
|---|---|---|---|---|
| 1 — correctness/authority | F2: commit the slice, or declare its gate results non-citable | DM-31, DM-46 | `git status --porcelain` clean over `crates/` and `python/` | `acceptance-check` refuses to promote a gate with untracked source; the commit recorded in `acceptance.json` |
| 1 — correctness/authority | F1/F3: one authority for service state; remove the hardcoded phase | DM-02, DM-59 | `service_status` and the adapter agree with the dispatch table | a `just` gate over the live-tool set; an `ast-grep` rule banning a literal phase string in the adapter |
| 2 — semantic leverage | F5: state the profile invariant in `DESIGN.md` §8.4 or add it to `SnapshotInputs` | DM-31, DM-32 | the two-profile identity test | that test |
| 3 — accuracy | F4: correct the "exactly one case" claim | DM-59 | — | none; prose is the right tier |

**Final check.** The design's claims do *not* match the evidence in one specific, correctable
way, and the scope of what is supported is stated more narrowly than what is built rather than
more broadly — which is the safer direction, but is still a divergence. Later extensions have a
clear path: the spine, the citation targets and the amendment rule all work, and F1 is a
statement problem, not a structural one.

---

## Appendix — what this review would have missed without running anything

`cargo check --workspace --all-targets` and `git status --porcelain` produced F1 and F2 between
them, and both are the top two findings. Neither is visible from reading `DESIGN.md`, and neither
would have appeared in a review that trusted `STATUS.md`. Recorded here because the next reviewer
should start the same way.
