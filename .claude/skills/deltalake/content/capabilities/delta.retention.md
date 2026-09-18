# Preview and execute vacuum with explicit historical-read consequences

Vacuum removes physical files and can make historical snapshots unreadable. At this pin its default is Lite with dry_run=false and retention enforcement enabled; request preview explicitly.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| with_dry_run(true) | You need candidate paths before removal | Metrics naming does not mean preview actually deleted files |
| Lite | Candidates should come from tracked tombstones | Does not find every untracked orphan |
| Full | Untracked eligible objects must also be considered | Lists storage; path/age rules and backend costs matter |
| with_keep_versions | Selected historical snapshots must retain required files | Experimental option; does not create an unlimited history guarantee |

## Contract

**modes.** A controlled-clock orphan fixture was found by Full and not Lite. A kept historical version excluded its required file from the preview.
Claim `delta.retention.1`; runtime_observation; evidence: runtime.

**retention.** Zero retention was rejected with enforcement enabled. Historical v0 remained readable before actual removal and failed when consuming data after vacuum removed its files.
Claim `delta.retention.2`; runtime_observation; evidence: runtime.

**effects.** Loading an old log snapshot after vacuum can succeed while scanning its data fails. Restore cannot recreate bytes that are no longer present.
Claim `delta.retention.3`; source_observation; evidence: upstream, source.

## Implementation

- Specify preview/mode/retention deliberately and inspect candidate paths before choosing a production policy.
- Account for active readers, CDF consumers and recovery requirements when selecting retention; validate the exact backend behavior.

## Effects

- delete physical objects
- reduce historical/CDF recoverability

## Errors

- Retention guard and missing history/files are material conditions, not proof the current logical table is corrupt.

## Limits and unknowns

- Remote listing consistency, authorization and deletion behavior remain untested.

## Exact contracts

- [`deltalake_core::table::DeltaTable::vacuum`](../operations/deltalake_core.table.DeltaTable.md#op-f35c76fceb4dd387be47d4c5) — `fn vacuum(self) -> VacuumBuilder`
- [`deltalake_core::operations::vacuum::VacuumBuilder::with_dry_run`](../operations/deltalake_core.operations.vacuum.VacuumBuilder.md#op-64b36ce2d7cda9e7d12d98d1) — `fn with_dry_run(self, dry_run: bool) -> Self`
- [`deltalake_core::operations::vacuum::VacuumBuilder::with_mode`](../operations/deltalake_core.operations.vacuum.VacuumBuilder.md#op-1a50ecdb4534ce3068f563dc) — `fn with_mode(self, mode: VacuumMode) -> Self`
- [`deltalake_core::operations::vacuum::VacuumBuilder::with_keep_versions`](../operations/deltalake_core.operations.vacuum.VacuumBuilder.md#op-972e70574d351c468cfb45c7) — `fn with_keep_versions(self, versions: &[Version]) -> Self`
- [`deltalake_core::operations::vacuum::VacuumBuilder::Output`](../operations/deltalake_core.operations.vacuum.VacuumBuilder.md#op-9efeffefd9cbc54190d5ac4a) — `type Output = Result<(DeltaTable, VacuumMetrics), DeltaTableError>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/operations/vacuum.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: vacuum_preview_keep_versions_and_history_loss, vacuum_full_finds_orphan_lite_does_not
