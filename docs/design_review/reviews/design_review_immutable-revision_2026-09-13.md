# Immutable revision proposal review

## 1. Decision and scope

Codex, 2026-09-13. Review of ADR-0015 against blueprint §§3.1–3.3,4.3,13, identity/request,
archive and static worker contracts, and the current upstream matrix. This establishes a
specifiable proposal, not executed acceptance. Runtime and generic Git provider support excluded.

## 2. Authority and lifecycle

Core request and release key own repository/root/commit identity. HTTP metadata and acquired
bytes are immutable inputs. Manifest version does not override the commit identity.

## 3. Semantic contracts

Exact SHA plus canonical repository and explicit root; version/revision mutually exclusive.
Missing project identity fails. Revision snapshots never impersonate released API evidence.

## 4. Derivation and execution

Bounded HTTP and safe archive extraction feed existing storage/normalizers. Python declarations
remain static observations. Rust source-only output explicitly lacks compiled API coverage.

## 5. Journeys

Monorepo root selection resolves the named package; default root does not guess a nested package.
Wrong SHA fails before publication. Offline replays cannot cross repository identities. A release
with an identical manifest version stays separate. PAX path attacks fail before external writes.

## 6. Gates

| Gate | Proposal verdict | Evidence / obligation |
|---|---|---|
| G1 | pass | Rust canonical repository/root identity; generated request schema |
| G2 | pass | Separate revision and manifest version, explicit source incompleteness |
| G3 | pass | Exact SHA/root/project validation before producer admission |
| G4 | pass | Service-owned extraction only; static worker never imports target |
| G5 | pass | Actual archive digest and acquisition receipt; immutable publication |
| G6 | pass | Exact registry/revision/environment replay matching |
| G7 | pass | No released or compiled API claim without evidence |

Proposal verdicts require the named runtime tests before implementation acceptance.

## 7. Principle findings

| Finding | Principles | Evidence or gap | Consequence | Correction | Verification |
|---|---|---|---|---|---|
| Package-only replay could cross repositories | DM-08, DM-32 | Existing catalog find_release ignores registry | Wrong snapshot for identical package/revision | Registry-scoped lookup | just gate-phase 3 / revision_fixture |
| Source archives differ from built distributions | DM-20, DM-43 | Static tree lacks wheel/build metadata | False API completeness | Explicit source-tree qualification and gaps | revision_fixture |

Identity, authority, publication, reuse and absence principles apply. Numerical computation and
runtime containment are outside this static acquisition proposal and remain Phase 4 obligations.

## 8. Alternatives

Cloning and compiling every revision adds executable attack surface and cannot run in static
mode. Returning unsupported for every revision loses the blueprint's revision workflow. Bounded
archive acquisition with existing static producers supplies useful qualified evidence directly.

## 9. Verification

revision_identity, revision_fixture, PAX/hostile archive tests, schema conformance, Phase 3 gate
and independent acceptance reruns. External live availability does not replace deterministic
identity, replay and boundary oracles.

## 10. Exceptions

No binding exception. Public GitHub is the explicit initial provider. Other hosts/dynamic package
metadata remain unsupported with a reason rather than guessed identity.

## 11. Decision

| Priority | Verdict | Strength | Condition |
|---|---|---|---|
| 1 | Accept proposed contract | Proposed | Identity/replay/archive/boundary regressions must pass |
