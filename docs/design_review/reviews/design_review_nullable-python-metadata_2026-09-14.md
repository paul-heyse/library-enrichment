# Design review: nullable Python metadata — 2026-09-14

## 1. Decision and scope

**Final decision: Accept at Proposed contract scope.** N1 was resolved by the amended Decision
section; the original finding and initial gate judgments below remain as review history.
**Claim strength: Proposed**, with the existing interfaces inspected. Target:
[ADR-0027](../../adr/0027-nullable-python-metadata.md), amending the Python metadata projection
in [ADR-0025](../../adr/0025-retained-release-metadata.md).

The observable outcome is useful, explicitly unbuilt source-revision evidence when distribution
Name/Version headers were never produced. Exact revision and artifact identities remain present.
Existing development snapshots need no preservation, compatibility reader, or migration.

**Method and coverage:** Read the proposed ADR, ADR-0025, charter, directive, addendum and review
template; inspected the Rust distribution DTO, archive inventory/header parser and PyPI validator,
native metadata validator, Arrow projection/decoder, and the cited revision-failure log. Attacked
missing-versus-malformed headers and disagreement between header entries and scalar projections.
No code edits, builds, tests or containers. Read the separately completed packaging verification
record in `docs/architecture/compatibility-matrix.md:1136`; this review does not independently
re-fetch upstream specifications or verify the full metadata
parser, publication recovery, or all resolution paths. Existing log failures are baseline evidence,
not execution of the proposed fix.

## 2. Authority and lifecycle map

| Concept | Identity and authority | Lifecycle / derived representation |
|---|---|---|
| Selected source | Rust `Release`, exact revision identity | Remains exact when distribution headers are absent. |
| Acquired bytes | Archive digest and qualified `FactSource` | Immutable artifact; raw header formatting remains recoverable. |
| Observed distribution headers | Selected METADATA/PKG-INFO parsed header entries | Name/version scalar fields are checked projections of these entries (amended ADR-0027:36–41). |
| Source declarations | Qualified pyproject declarations in revision receipts | Remain separate; never substituted for built headers. |
| Retained metadata | `ReleaseMetadata`, semantic ID over release/details/source | Native Arrow/Parquet projection, bounded queries, offline resolution and export. |

Parsing remains ordinary Rust code. No build backend, package import, new metadata authority or
execution lifecycle is introduced.

## 3. Semantic contracts and invariants

| Invariant | Enforcement boundary | Failure / observable result | Evidence |
|---|---|---|---|
| Missing observed headers do not invent a distribution identity | Rust inventory and native metadata admission | Nullable field; exact release/artifact identity unchanged | ADR-0027 Decision; Proposed |
| Present empty values are invalid | Native metadata admission | Explicit rejection, not conversion to null | ADR-0027 Decision; Proposed |
| PyPI requires corroborated Name/Version | Existing `archive::validate_metadata` before publication | Missing, duplicate or mismatched selected identity rejected | `archive.rs:233–250`; Interface-checked |
| Nullable fields agree with retained scalar headers | Inventory and native metadata admission | Cardinality and scalar/header mismatch rejection | Amended ADR-0027:36–41; Proposed; N1 resolved |
| Null and present values survive all representations | Rust DTO, Arrow schema/decoder, offline reader and export | No empty-string or release-version fallback | ADR-0027 Decision; Proposed |

Null means that the selected metadata has no observed value for that header; it does not prove
that the package has no declared name/version. Unbuilt-source coverage does not claim complete
distribution or generated contents. Exact preservation needs to refer to the parsed value:
`metadata_headers` already trims header values and unfolds continuation lines
(`python.rs:323–347`); raw archive bytes carry byte-level fidelity.

## 4. Derivation and execution design

The existing inventory selects one metadata authority and retains parsed header entries
(`archive.rs:138–145`). Rust projects optional name/version, validates the typed metadata,
computes its qualified semantic identity, and publishes it through the existing snapshot/catalog
boundary. Arrow children become nullable and decoding must use optional accessors; generated
DTOs originate in Rust. No backend invocation fills missing values. Header changes affect the
metadata observation, while a commit is never rewritten into a metadata Version header.

## 5. Representative journeys

- **Ordinary unbuilt revision:** pyproject plus Python sources, no built metadata; both fields
  are null, revision identity exact, source coverage partial. Offline replay and export preserve it.
- **Observed headers:** one nonempty Name and Version; parsed values survive round trips exactly,
  even if their spelling differs from a normalized release selector.
- **Partial or malformed headers:** only one header present must have a deterministic projection;
  duplicate scalar headers and contradictory scalar/header pairs are rejected under the amended rule.
- **Failure:** ordinary PyPI acquisition with a missing required header still fails before
  publication. This proposal does not modify interrupted-publication semantics.

## 6. Acceptance gates

These are independent design-contract judgments, not product acceptance-test results.
The table records the **initial** judgments; the final amended-contract judgments are in §11.

| Gate | Verdict | Evidence / required action |
|---|---|---|
| G1 — Authority | Unresolved | N1 leaves header/scalar agreement unenforced in the proposed contract. |
| G2 — Semantic fidelity | Unresolved | Null is appropriate, but repeated scalar headers need a defined result; N1. |
| G3 — Validity | Unresolved | N1 must identify cardinality and consistency rejection at native admission. |
| G4 — Hidden behavior | Pass, Proposed scope | No inference, build backend or package import to obtain missing values. |
| G5 — Consistency and recovery | Pass, Proposed scope | Existing immutable publication boundary retained; no new mutable authority. Recovery not re-audited. |
| G6 — Transformation and reuse | Pass, Proposed scope | Explicit null preservation through Arrow/Parquet, offline resolution and export; executable proof remains due. |
| G7 — Truthful capability claims | Pass, Proposed scope | Unbuilt coverage and separate revision identity; strict PyPI validation retained. No implementation claim. |

## 7. Principle findings

| Finding | Principle IDs | Concrete evidence or gap | Consequence | Proposed correction | Verification |
|---|---|---|---|---|---|
| **N1 — Define scalar-header cardinality and enforce scalar/header agreement. Initially unresolved; resolved at Proposed scope in §11.** | DM-02, DM-07, DM-08 | Initial ADR-0027 Decision defined optional values and empty rejection, but did not state how repeated scalar headers or inconsistent native rows behaved. Existing `archive.rs:207–218` takes `first()`; `evidence/metadata.rs:66–99` validates independent scalar values without checking the retained header entries. | A revision containing two Name headers can silently choose one. A native row can encode null or another name alongside a present header, leaving two answers to the same question in an admitted observation. | Specify each scalar as derived from its corresponding selected header: missing key → null; exactly one nonempty parsed value → that exact value; present empty vector, empty/whitespace value, repeated scalar headers (even equal), or scalar/header mismatch → rejection at native admission. Keep legitimately repeated other headers intact. Clarify that “exact” means parsed value, with raw bytes retained in the archive. | Focused inventory and `release_metadata_roundtrip`/admission negative tests: independent absent fields; duplicate equal and unequal Name/Version; empty vector; whitespace; null/non-null and value mismatches. Recompute IDs before malformed admission tests so ID validation cannot mask the invariant. |

**Applicability:** Authority/types/identity and preservation (groups 1–3, 5, 9–11) are directly
material. DM-12 and DM-24 are satisfied at Proposed scope by separate source identity and explicit
round-trip requirements; DM-28 is satisfied by the no-build/no-import boundary. DM-02/07/08 are
satisfied at Proposed scope by the amended N1 contract. Existing lifecycle/publication is inherited, not newly verified. Scheduling,
provider selection, performance layouts and new execution mechanisms (most of groups 4, 6–8) are
outside this narrow schema decision. Proportionality (group 12) favors the small nullable change.

## 8. Alternatives and architectural leverage

| Alternative | Meaning / risk | Cost and decision |
|---|---|---|
| Current required strings with empty defaults | Conflates absence and invalid value; valid unbuilt inventory fails admission | Reject; baseline log and source explain the failure. |
| Nullable checked scalar projections | Preserves independent absence and present values without inventing identity | Select after N1; narrow Rust/schema/decoder/generated DTO changes. No performance claim. |
| Omit scalar convenience fields and read the existing header collection | Viable single authority, but consumers must repeatedly select and validate scalar headers | Simpler model, broader consumer change; unnecessary when the projections have one checked derivation. |

No new registry, metadata engine or build workflow is justified.

## 9. Verification and measurement plan

| Risk | Evidence label | Required oracle / expected result | Current result |
|---|---|---|---|
| Missing versus invalid scalar headers | Proposed | N1 cases across inventory and native admission | Not run in this review |
| Loss during interchange | Proposed | Rust/Arrow/Parquet/DTO and DataFusion selection round trips for null/null, null/value, value/null, value/value; IDs and qualified sources preserved | Not run |
| Strict PyPI boundary weakened | Interface-checked baseline; Proposed amendment | Missing, duplicate, mismatched or empty Name/Version still rejected by `archive_metadata_validation` | Not run |
| Real revision and retained reuse | Proposed | `test_revision_fixture.py`: exact commit/artifact, unbuilt coverage, no backend invocation; restart/offline/export retains nulls and original artifact closure | Baseline log shows two Python metadata failures and a separate Rust coverage failure, not a result for this fix |
| Generated contract drift | Proposed | Rust-generated schemas, schema conformance and affected Python contract checks | Not run |

No latency or resource improvement is claimed. Existing record/file budgets remain applicable.
The independent Rust coverage conflict and other logged failures are not attributed to nullability.

## 10. Exceptions and unresolved decisions

No SHOULD exception is requested. N1 was the sole identified contract blocker and is resolved.
The separately assigned primary-source verification is recorded in the compatibility matrix and
the ADR; this review read those records without independently re-fetching the sources.
Historical snapshot compatibility or a migration is explicitly unnecessary for the
authorized fresh target design.

## 11. Decision and implementation changes

**Accept at Proposed contract scope following the amended-contract recheck on 2026-09-14.**
The nullable design is proportionate and preserves the correct identity domains. ADR-0027:36–41
now explicitly defines absent-key/null equivalence, exactly-one nonempty parsed value, duplicate
scalar rejection even when equal, empty/whitespace rejection, scalar/header agreement at native
admission, preservation of valid nonscalar repetitions, and parsed-value versus raw-byte fidelity.
This resolves N1 without requiring implementation to accept a complete proposed contract.

| Gate | Final verdict | Basis |
|---|---|---|
| G1 — Authority | Pass, Proposed scope | Checked scalar derivation from one selected header authority. |
| G2 — Semantic fidelity | Pass, Proposed scope | Absent, present and malformed states have distinct specified outcomes. |
| G3 — Validity | Pass, Proposed scope | Cardinality, emptiness and cross-field agreement rejected at native admission; strict PyPI selection checks retained. |
| G4 — Hidden behavior | Pass, Proposed scope | No build, import or inferred replacement values. |
| G5 — Consistency and recovery | Pass, Proposed scope | Existing coherent immutable publication retained; no new recovery protocol. |
| G6 — Transformation and reuse | Pass, Proposed scope | Optional parsed values and exact identity retained through every named representation. |
| G7 — Truthful capability claims | Pass, Proposed scope | Unbuilt-source limitations explicit; completed distribution validation remains required. |

| Priority | Change | Acceptance evidence / regression protection |
|---|---|---|
| 1 — Contract, resolved | N1 amendment | ADR-0027:36–41 defines scalar-header cardinality, checked derivation and explicit invalid-state behavior |
| 2 — Evidence, recorded | Assigned upstream verification | Compatibility matrix:1136 and ADR-0027:76–80 contain dated primary-source entries; not independently re-fetched in this review |
| 3 — Implementation | Update Rust DTO/inventory/validation and nullable native projection; regenerate artifacts | Focused negative/round-trip tests plus real revision/offline/export checks in §9 |

This decision does not certify implementation, Plan 10 T6, or phases 4–6.
