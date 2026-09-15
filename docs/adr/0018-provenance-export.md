---
id: ADR-0018
title: Portable, offline-verifiable provenance bundles
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-11, DM-12, DM-15, DM-42, DM-46, DM-48]
design: [§2.3, §3.1, §6.3, §8.2, §13]
review: not-required: A new read-only surface that copies already-published immutable evidence; it adds no producer, no policy path and no mutation. It is covered by the Phase 6 gate's test run rather than a scoped review.
evidence: Tested
supersedes: []
superseded-by: null
revisit: A bundle needs to carry more than one context, or a consumer asks to import one back into a store.
verification: enrichment_daemon::export::tests (three cases) and tests/e2e/test_provenance_export.py, both in `just test`; `library-enrichmentd verify-bundle` exits non-zero on a tampered bundle.

---

# ADR-0018: Portable, offline-verifiable provenance bundles

## Context

Design §13 names "provenance export" as Phase 6 scope in one phrase, and §2.3 gives it a
directory (`bundles/`) under the data home. Nothing else specifies what a bundle *is* — not its
file layout, not what travels with it, not what a recipient is supposed to be able to check. The
blueprint is frozen and will not gain that detail, so the shape has to be decided here or invented
silently by whoever writes the code.

The design already fixes everything a bundle depends on: snapshots are immutable directories
(§8.2), artifacts are content-addressed blobs, and identity is four distinct things that do not
collapse (§3.1). The open question is only what an export *emits* and what verifying one proves.

## Scope

This record binds the on-disk shape of an exported bundle, the guarantee `verify` makes, and the
two CLI subcommands that produce and check one. It deliberately leaves open: importing a bundle
back into a store, bundles spanning more than one context, signatures or any notion of an external
trust root, and compression or a single-file container format. A bundle is a *copy*, and this
record claims nothing beyond that.

## Drivers

- **Reproducibility.** An evidence service whose output cannot leave the machine it was produced
  on is not auditable by anyone but its operator.
- **Correctness of the claim.** The easy mistake is to let a bundle imply that the evidence was
  right. It can only establish that the copy is intact.
- **Availability when it matters.** An operator wants a bundle most when the service is wedged or
  stopped, so export must not require a running daemon.
- **The repository boundary.** Export reads the data home and writes an operator-named directory.
  It never reads or writes a repository under study.

## Options

- **Do nothing; tell operators to `tar` the data home.** Copies regenerable caches and every other
  context, carries no manifest, and gives a recipient nothing to check against. Rejected.
- **A single-file container (tar/zip, optionally compressed).** Convenient to move, but it puts an
  archive extractor between a recipient and their first integrity check — the same extractor class
  the service treats as hostile input elsewhere (gate C11). Rejected for the first version.
- **Sign the manifest.** Would let a recipient establish origin, not just integrity. Rejected for
  now: there is no key management story, and a signature over a manifest nobody yet verifies is
  ceremony. The manifest is designed so a signature can be added over it later without changing
  the layout.
- **A directory with a digest manifest (taken).** A recipient needs `sha256sum -c` and nothing
  else, including no copy of this service.

## Decision

`library-enrichmentd export <context-id> <dir>` writes a bundle; `library-enrichmentd
verify-bundle <dir>` checks one. Both read the store directly and contact no daemon, because a
bundle is a copy of evidence that is already immutable on disk.

A bundle is a directory containing:

- `MANIFEST.sha256` — one `<hex>  <relative path>` line per file, sorted, in `sha256sum` format.
  It is the only file not listed in itself.
- `bundle.json` — the bundle version, export time, schema version, and the service's own
  `release`, `environment`, `context` and `snapshot_id` records, verbatim.
- `snapshots/<snapshot_id>/…` — the context's current snapshot directory, copied file for file.
  Symlinks and device entries are not followed; a snapshot contains none, and following one is how
  a copy stops being a copy.
- `artifacts/<digest>` — the release's artifacts under the content-addressed names they already
  had, so a recipient can recompute the name from the bytes.

Four rules make the guarantee checkable rather than merely stated:

1. **Identities are the service's own.** A bundle carries the same `release_id`, `environment_id`,
   `context_id` and `snapshot_id`, never re-minted ones, so it can be related back to the store it
   came from — and so that §15's question about later graph evidence keeps a `yes` available.
2. **Export is all-or-nothing.** A non-empty destination is refused rather than merged: a manifest
   that does not describe everything in the directory is not verifiable.
3. **Verification is about the directory, not only the list.** A file present but unlisted is
   reported as a problem, exactly like a file whose digest does not match. Otherwise appending to a
   bundle and leaving the manifest alone would still verify.
4. **A bundle is not a second publication authority.** `bundle.json` says so in its own `note`
   field, and `verify` reports only what it can establish: that the copy is intact, not that the
   original was correct.

### Consequences

Easier: an operator can hand a colleague, an auditor or a bug report a self-checking copy of
exactly what the service concluded, and check it back with one command on a machine that has
never run this service.

Harder: a bundle is uncompressed and un-deduplicated, so a large snapshot travels at full size,
and two bundles of overlapping contexts duplicate their shared artifacts. Multi-context export
would need a layout decision this record does not take.

Foreclosed for now: importing a bundle. Nothing here reserves an import path, and adding one later
means deciding what happens when an incoming `context_id` already exists locally — a question
about authority, not about file format. Reversing this decision is cheap: no stored state depends
on the bundle layout, and no other subsystem reads it.

### Compensating controls

`verify` is the control: a bundle that has been edited, truncated, appended to, or had a file
swapped for a same-length replacement fails it, and the CLI exits non-zero and names every problem
rather than the first. `tests/e2e/test_provenance_export.py` runs export and verification end to
end against a real store, including a tamper case. The `bundle.json` `note` field keeps the
epistemic claim attached to the artifact itself, where a recipient reading it months later will
actually encounter it.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| `bundles/` is the design's home for exported provenance | `docs/design/DESIGN.md` §2.3 | 2026-09-13 | "retained evidence and job records under the data home (`blobs/sha256/`, `snapshots/`, `contexts/`, `jobs/`, `bundles/`, `logs/`)" |
| Provenance export is Phase 6 scope and is otherwise unspecified | `docs/design/DESIGN.md` §13 / blueprint §13 | 2026-09-13 | "6 operational hardening" — the phase text names export without giving it a shape |
| Snapshots are immutable, so copying one is the whole job | `docs/design/DESIGN.md` §8.2 | 2026-09-13 | "publish an immutable snapshot directory, then atomically update the context's current pointer" |
| `sha256sum` reads this manifest format without a bespoke tool | GNU coreutils `sha256sum(1)`, `--check` | 2026-09-13 | Each line of a checksum file is a digest, two spaces, and a file name — the format written here |

## Verification

`cargo nextest run -p enrichment-daemon export::tests` covers the three cases that matter: a
bundle verifies against its own manifest; a same-length byte change is caught (the case a size
check would miss); and an unlisted file added afterwards is a problem too. A missing manifest is an
error, not an empty pass.

`tests/e2e/test_provenance_export.py` exercises the operator path end to end against a real store:
resolve a library, export its context, verify the bundle, tamper with one snapshot byte, and
assert `verify-bundle` exits non-zero naming that file.

Export has **no acceptance gate ID**, and one is not invented for it. The frozen
`tests/ACCEPTANCE_PLAN.md` is the authority on which IDs exist, and `scripts/acceptance-check.py`
rejects any registry ID absent from it that is not a registered successor to a superseded gate.
Export is Phase 6 *scope* without a gate clause; it is verified by tests that run in `just test`,
and the Phase 6 gate fails if they fail.

## Boundaries preserved

§B1 (repository/state separation) — export reads the data home and writes an operator-named
directory; no repository under study is read or written. §B2 (Rust owns publication) — export is a
Rust surface and creates no new publication authority. §B3 (thin Python boundary) — the adapter is
unchanged; this is a CLI surface. §B4–§B13 are untouched: no new producer, no new policy path, no
network, no execution profile, no schema change to any evidence record. The four identities are
carried, not re-minted.

## More information

- Design §2.3 (filesystem layout), §3.1 (four identities), §8.2 (publication), §13 (Phase 6).
- Implementation: `crates/enrichment-daemon/src/export.rs`; CLI in
  `crates/enrichment-daemon/src/bin/library-enrichmentd.rs`.
- Plan: `docs/plans/08-phase-4-6-resumption.md`, Phase 6 "Provenance export".

## Status history

- 2026-09-13 — accepted; implemented with unit and end-to-end coverage.
