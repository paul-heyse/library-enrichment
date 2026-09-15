# Implementation handoff

Updated **2026-09-15 14:02 UTC**. **Plan 13 is complete and deployed.**
[Plan](docs/plans/13-datafusion-research-operations-hard-pivot.md),
[execution ledger](docs/plans/13-research-operations-execution-ledger.md),
[final qualification](docs/reports/plan13-final-qualification-2026-09-15.md),
[accepted closing review](docs/design_review/reviews/design_review_plan13-implementation_2026-09-15.md).

## Architecture delivered

- One Rust research/2.0 contract owns scope, aspect selection, independent pages, diagnostics
  and delivery. DataFusion requested-domain assessment serves acquisition, retained resolution,
  discovery, inspection, execution and both comparison sides.
- Native selection precedes hydration. Observations, relationships, documents, members, children
  and execution results have independent bounded pages. Comparison reconciles flat distinct
  values before selecting changed keys and each side's alternatives.
- Explicit configured analysis, optimization and physical planning preserve DataFusion defaults.
  Independent Arrow family/role/nullability requirements and bounded offending-ID witnesses
  carry application validity through the native pipeline.
- Each foreground/job operation owns identities, leases, admission, deadlines and cumulative
  output/artifact charges. Blocking workers retain ownership until actual exit. Search, namespace
  and comparison indexes reuse operation-local native relations through managed spill, Arrow IPC
  and streaming providers, with no persistent cache or second query engine.
- Complete immutable result closure precedes publication. Restart reads committed indexed bytes
  without reacquisition, query execution or replacement writes. Direct sections avoid whole-result
  reparsing. Revision omissions and declared Cargo inputs receive native scope assessment;
  immutable citations retain exact commit/package paths. Qualified Python class scope feeds actual
  ty/runtime consumers while retaining separate source, stub and runtime observations.
- FastMCP validates original arguments and actual typed output. Bounded text preserves native
  failed-job recovery when a host hides structured errors. Optional progress/resources preserve
  ordinary tool behavior. Rust retains semantic, policy, job and publication ownership.
- Current formats only: research/2.0, state/6, snapshot/projection 6.0, catalog/5, jobs/5, bundle/5.
  Previous generations reject without translation. ADR-0036–0039 and all D01–D12/J01–J20 exits
  are closed under the recorded scope and limits.

## Active deployment

`library-enrichment.service` activated the verified installation at **13:49:55 UTC**:

- Installation: `/home/paul/.local/opt/library-enrichment/plan13-3ae079f9718a`
- Daemon PID at verification: **818737**
- State: `/home/paul/.local/state/library-enrichment-v2`
- Config: `/home/paul/.config/library-enrichment/research-v2-3ae079f9718a.toml`
- Python adapter/worker: the installation's non-editable locked environment
- Policy: existing **static-only** production policy, unchanged
- Native limits: 32 GiB managed memory, 64 GiB spill, 2 GiB metadata cache, 16 query slots

All installed inputs and three native executable hashes match the manifest. The deployed smoke
passed status, exact DataFusion 55.1.0 resolve, useful default SessionContext inspection, pending
comparison completion, direct changes-section digest verification, and packaging 26.3/Version.
The final status has no queued/running jobs, admitted queries or daemon children. Griffe completed
qualification using the installed worker. Receipts: `.dev-state/plan13-cutover/`.

Both global Codex/Claude registrations select the same installed adapter launch. Both managed
product-skill copies match the three shipped files. **Existing interactive clients may need to
reconnect to load the new catalog.** Their parent sessions were preserved.

Old daemon PID 1885544 and its four adapter processes are stopped. Old state
`/home/paul/.local/state/library-enrichment` and config
`/home/paul/.config/library-enrichment/service.toml` remain inactive and byte-unchanged.
No old journal/result/cursor migration, fallback or evidence deletion was performed. Old release
files in the working tree remain preserved; new builds used `.dev-state/p13-release-target`.

## Final verification

Source digest: `224dea96aca19e8482b6715f524ebe0b5948ba039e0c8fac00200aa933d72bf5`.
The acceptance skill's independent auditor replayed and verified all five command receipts:

| Boundary | Executed result |
|---|---|
| Ordinary native workspace | 432 passed; 13 skipped |
| Ordinary Python / raw stdio | 223 passed; 17 deselected |
| Containment, cleanup, stable/nightly compatibility | 12 passed |
| Live Rust/Python registries | 2 passed |
| Installed actual Codex/Claude | 15 passed: fourteen scenarios plus preservation |

The external native cases and Python deselections ran in their separate tiers; the remaining
manual native measurement has its own receipt. Formatting, Clippy, Ruff, ty, schemas, provenance,
architecture, ADRs, dependency policy and final state-leak checks passed. Regenerated acceptance
and independent audit: **47 passed / 0 failed / 0 blocked / 1 not_run of 48**. The sole not_run
is retired P08; P08a/P08b are its ADR-0005 successors. Per-gate audit:
`.dev-state/plan13-independent-audit/final/acceptance-audit-2026-09-15.md`.

The earlier integrated `just ci` failed an obsolete error-summary assertion; the earlier client
run failed observer correlation checks. Their original receipts remain diagnostic. The corrected
raw boundary and final independent full component replays establish the current source; those
older command exits are not relabeled. No runtime changes followed final qualification.

## Live evidence and deliberate limits

Both development and installed releases completed all forty exact DataFusion 55.1.0 crate
resolve/overview/search/inspect/offline journeys. The installed traversal returned all 117
SessionContext relationships; four high-fanout defaults remain useful. A 135-task one/eight-client
experiment returned equal data digests. Native measurements support operation indexes and the
selected endpoint semi join. See [live results](docs/reports/plan13-live-functional-outcomes-2026-09-15.md)
and [measurements](docs/reports/plan13-native-operation-measurements-2026-09-15.md).

Qualified execution passed in independent debug `.dev-state/p4p` and installed release
`.dev-state/p13r` roots. Production has no enabled execution-image profile. Full revision build
closure, universal package completeness and general compatibility are not established. Source,
stub and runtime evidence remain distinct. Managed quotas are not an RSS guarantee; measured
samples are not reliable p95 estimates. Broader throughput work has deferred-register trigger R43.
Two client-prose overstatements are recorded in the final report; native evidence preserves the
narrower registry and process-readiness observations.

## Workspace preservation and next work

No required Plan 13 implementation work remains. No task-owned campaign, test worker or adapter
remains running. The deployed service is intentionally active.

Baseline HEAD: `41ac67219ea39e6b085dde6273c9df08c9937bba`. The user states the original tree was
clean; accumulated edits are Plan 13 work. Preserve the dirty tree, historical installations,
logs and state. No reset, stash, clean, commit, merge or push was performed. Disk had about 5 GiB
free during closure; check capacity before any new large build or image assembly.

Pinned versions: DataFusion **55.1.0**, Arrow/Parquet **59.3.0**, FastMCP **4.0.3**, Python **3.14.7**,
stable Rust **1.98.1**. Use locked/offline Cargo with `CARGO_INCREMENTAL=0`, and Python through uv.
Exact pinned source and functional probes remain authority over Context7's main/older discovery.
