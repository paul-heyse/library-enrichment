# Implementation handoff

Updated **2026-09-15**. **Plan 14 is complete and activated.**
[Plan](docs/plans/14-datafusion-catalog-policy-hard-pivot.md),
[final qualification](docs/reports/plan14-final-qualification-2026-09-15.md),
[physical decisions](docs/reports/plan14-physical-strategies-2026-09-15.md),
[ADR-0040](docs/adr/0040-immutable-native-catalog-policy.md).

## Delivered architecture

- One immutable DataFusion catalog/schema hierarchy binds exact snapshot evidence, native domain
  views, folded durable records, validation candidates and independent comparison pins. Every
  production reader uses it; procedural registration modes and legacy namespace aliases are gone.
- Finite codec-derived declarations supply semantic keys, native duplicate/reference checks and
  bounded nested metadata. Candidate/history bindings never inherit unsupported admitted facts.
- One validated immutable policy feeds read-only native settings, actual Parquet scans, writers
  and diagnostics. Operation work tables have their own controlled scope.
- Physical scans consume exact admitted row counts; count elimination retains source validation
  and leases. Native string coercion preserves Utf8/LargeUtf8/Utf8View scoring and metadata.
- Completed indexes carry exact counts, identity and established physical properties. Search and
  comparison avoid count-only IPC replay. Native views remain visible to the optimizer.
- Rule transitions and fingerprints are capped; managed-memory peaks have shared-runtime scope
  without resets. Independent storage layout, decoder/reorder, observation-ID Bloom and bounded
  whole-file groups are selected from recorded measurements. Unsorted IPC and the existing pool
  remain the sole selected production strategies.
- Rust retains policy, semantics, evidence and publication ownership; Python remains thin.
  Canonical research/2.0 and current evidence/catalog/job formats remain. Expanded derived query
  diagnostics use `query-failures-v2.json`; earlier history is inactive without a converter.

## Active installation

Verified **2026-09-15 21:37 UTC**, activated once at **21:36:31 UTC**:

- Installation: `/home/paul/.local/opt/library-enrichment/plan14-5b2640ca15e4`
- Daemon: `library-enrichment.service`, PID **2465957** at verification
- State: `/home/paul/.local/state/library-enrichment-v2`
- Config: `/home/paul/.config/library-enrichment/research-v2-plan14-5b2640ca15e4.toml`
- Production policy: **static-only**, with 32 GiB managed memory, 64 GiB spill, 2 GiB metadata
  cache, 16 query slots/partitions. No external execution profile was enabled.
- Both Codex/Claude registrations point to the same installed, locked, non-editable adapter.
  Existing interactive sessions may need to reconnect.

All installed inputs and the running executable match the manifest. Activation preserved the
existing data digest and prior configuration; retained compatible evidence serves the new runtime.
Old adapters are stopped and parent clients remain. The deployed smoke passed eight actual MCP
cases, including exact resolve, inspection, comparison and direct artifact reading. Final counters
show zero queued/running jobs, admitted queries, managed reservations and daemon children.
Receipts: `.dev-state/plan14-final/cutover/`.

## Verification and scope

Current phase: **6**. Source digest:
`5b2640ca15e4e1e6bb3355eda84edb89d4bcdab3e390897513648c0581d4026a`.
Source archive, build, quality and candidate receipts: `.dev-state/plan14-final/`.

| Executed boundary | Result |
|---|---|
| Ordinary native workspace, eight workers | 441 passed; 16 explicitly skipped manual/external cases |
| Ordinary Python, excluding live/client markers | 188 passed; 35 execution-image prerequisite skips; 17 deselected |
| Installed fresh-state raw MCP | Passed Rust/Python acquisition, discovery, inspection, comparison, paging/artifacts, offline reuse, empty/missing results |
| Installed capacity recovery | Passed typed budget error; post-failure permits/reservations return to zero |
| Actual installed Codex/Claude CLI | A01 and A02 passed; operator directories unchanged |
| P8 | All nine decisions complete, with independent source/executable measurements or explicit consumer-based not-applicability |

Formatting, workspace all-feature/all-target Clippy, doctests, Ruff, scoped ty, schemas, all eleven
rule fixtures, architecture, dependencies, ADRs, frozen provenance and state-leak checks passed.
`just doctor` and toolchain checks found all hard development requirements present.

Last machine-gate commands: **2026-09-15**, `just acceptance-report; just acceptance-check`:
**28 passed / 0 failed / 11 blocked / 9 not_run of 48**. Independent audit reproduced both
commands with identical outcomes and no source/native drift under `.dev-state/plan14-independent-audit/`.

Blocked IDs: C05, C16, C17, C18, C20, P08a, P08b, P09, P10, R09, R10. They require selected
qualified Rust/Python execution images and root; this campaign selects none. `execution-env.sh`
reports no receipt at `/nonexistent/podman/admitted-images.json`. The installed static-only
architecture qualification does not certify those external profiles. Machine not_run: A01–A06,
P01, P08, R02. A01/A02 actual CLI evidence passed separately; their registered pytest wrappers
were not run. P08 remains retired. Historical Plan 13 external gates are not current-source passes.

Earlier diagnostic failures remain preserved: two obsolete native fixture assumptions and a
32-worker cold-comparison deadline failure, corrected before all 441 final cases passed; a bare ty
invocation included out-of-scope development-skill build code; a one-byte capacity probe also
prevented status queries, so the final bounded recovery case uses 32,768 bytes. No product deadline
or safety gate was weakened. Separate pre-plan MCP/design findings retain their original scope.

## Remaining work and preservation

No required Plan 14 work remains. P0–P9, D01–D13 and J01–J17 are closed. No task-owned
qualification process remains running; the deployed daemon is intentionally active. Next work
requires a new scope or an observed R44/R45 trigger. Do not restart the old architecture or
rerun unrelated breadth to inflate the machine tally.

Baseline HEAD: `a78c74191d69ad82de7348c6503b58443f7f9c13`; source was clean at execution start.
Working-tree source edits implement Plan 14. The pre-existing untracked capability review and MCP
report remain inputs, not newly claimed implementation evidence. Additional untracked design
reference material is preserved without claiming ownership. No reset, clean, commit, merge,
push or evidence deletion was performed. Preserve the working tree, old installations and receipts.

Pins verified locally: DataFusion **55.1.0**, Arrow/Parquet **59.3.0**, object_store **0.13.2**,
FastMCP **4.0.3**, Python **3.14.7**, Rust **1.98.1**, ty **0.0.80**. Use locked/offline Cargo with
`CARGO_INCREMENTAL=0` and Python through uv. Exact source outranks Context7 discovery for release
claims. R44/R45 retain deliberate extension/workload triggers; they are not unfinished core work.
