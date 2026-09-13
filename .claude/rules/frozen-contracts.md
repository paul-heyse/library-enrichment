---
paths:
  - "contracts/**"
  - "tests/ACCEPTANCE_PLAN.md"
  - "tests/gates.toml"
  - "config/service.example.toml"
---

# Frozen contracts

These files are listed in `docs/provenance/bundle-2026-09-13/MANIFEST.sha256` at their current
paths. `just provenance-check` verifies their digests, so editing one breaks a gate. That is
deliberate: it makes the freeze mechanical rather than a convention.

`contracts/research-envelope.schema.json` is the **Phase-0 acceptance target**. Generated
schemas do not need to match it byte-for-byte — `$id`, `$defs` ordering, and serde-derived
naming will legitimately differ. They must match it *behaviourally*: accept the same documents
and reject the same documents. `just schema-conformance` is the oracle.

Acceptance gate IDs are stable identifiers. Never renumber them, never delete one, never reuse
a retired ID. Retire by marking the gate superseded and recording why. Downstream reports,
`tests/gates.toml`, and ADRs all reference these IDs.

`config/service.example.toml` documents the configuration contract with explicit defaults. Its
`enabled_profiles = ["static", "build"]` deliberately omits `runtime`: runtime execution is an
explicitly enabled local profile, never a default.

Changing any of this requires an ADR that re-freezes the bundle under a new dated directory in
`docs/provenance/`, with the old one retained.
