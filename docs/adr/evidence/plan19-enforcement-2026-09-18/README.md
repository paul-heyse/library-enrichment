# Prepared Plan 19 enforcement update

Prepared **2026-09-18**; **not applied**. This is the CP00 review artifact for the protected
instruction update required by CP10/CP11. It does not close the deletion barrier.

The patch aligns five current files with accepted ADR-0042/0047 and the owner's execution
instructions. It removes stale snapshot-directory/current-pointer and ProducerSpec/ProducerPlan
requirements, identifies the native field and publication owners, separates historical contract
provenance from current wire acceptance, keeps the ty producer distinct from operator tooling,
and states the unit-versus-integration barrier. It does not modify hooks or frozen provenance.

- [Patch](target-enforcement.patch)
- [Exact base/post-image hashes](manifest.json)
- Authority: [ADR-0042](../../0042-delta-control-publication.md) and
  [ADR-0047](../../0047-native-schema-contract-authority.md).

`git apply --check docs/adr/evidence/plan19-enforcement-2026-09-18/target-enforcement.patch`
passed against the captured current bases. The manifest records both sides and the patch digest.
Recheck every base hash before applying; if protected files changed, regenerate the proposal from
those actual files. Do not force or partially apply a stale patch.

Actual application and `just guardrails-record` are operator actions outside an agent session,
under `.claude/rules/process-immutable.md`. After the operator applies it, run the protected
manifest/hook checks and record the installed disposition in Plan 19. Until then, the current
protected files remain installed and the required enforcement update remains open.
