# Plan 13 final qualification — 2026-09-15

## Current decision

**Plan 13 is complete.** Independent final-source qualification passed, and the verified
installation alone serves the new contract after the supervised production cutover at
**13:49:55 UTC**. Deployed raw-MCP smoke and identity/preservation verification passed at
13:50 UTC. All J01–J20 outcomes and D01–D12 replacement exits are closed by the evidence below.
Plan-local journey IDs remain separate from the stable acceptance registry.

Source: `224dea96aca19e8482b6715f524ebe0b5948ba039e0c8fac00200aa933d72bf5`.
Installed candidate: `/home/paul/.local/opt/library-enrichment/plan13-3ae079f9718a`.
DataFusion 55.1.0, Arrow/Parquet 59.3.0, FastMCP 4.0.3; no dependency upgrade is part of this pivot.

## Final command evidence

The acceptance auditor replays the recorded commands with their original environment and
arguments, changing only the output destination. Receipts retain source, executable, log and
client-trace bindings. The original diagnostic CI and client failures remain preserved.

| Boundary | Result | Receipt or log |
|---|---|---|
| Formatting, all-feature Clippy, Ruff, ty, schemas, provenance, architecture, ADRs, state leak | passed | `.dev-state/plan13-final-current-source-checks.log` |
| Independent ordinary native workspace | passed: 432; 13 skipped | `.dev-state/plan13-independent-audit/final/nextest.json` |
| Independent non-live/non-client Python | passed: 223; 17 deselected | `.dev-state/plan13-independent-audit/final/pytest.json` |
| Independent containment, cleanup and stable/nightly compatibility | passed: all 12 selected ignored cases | `.dev-state/plan13-independent-audit/final/nextest-execution.json` |
| Independent live registries | passed: 2 | `.dev-state/plan13-independent-audit/final/live.json` |
| Installed actual Codex/Claude plus operator preservation | passed: 15 tests, fourteen actual scenarios | `.dev-state/plan13-independent-audit/final/client.json` |
| Matching installed release execution qualification | passed: Python/Rust probes and 12 containment/cleanup cases | `.dev-state/plan13-release-execution-qualification.log` |
| Dependency policy | passed | `.dev-state/plan13-final-dependency-policy.log` |
| Regenerated acceptance and independent audit | passed: 47 active gates; retired P08 remains not_run | `.dev-state/plan13-final-acceptance.log` |
| Deployed fresh-generation raw-MCP smoke | passed: seven cases, including two pending completions | `.dev-state/plan13-cutover/post-activation-receipt.json` |
| Deployed identities, preservation and final state leak | passed | `.dev-state/plan13-cutover/deployment-verification.json`; `.dev-state/plan13-final-state-leak.log` |
| Final document consistency | passed: ADR/provenance checks, ten rule fixtures, spelling, whitespace and 135 local links | `.dev-state/plan13-final-doc-checks.log`; `.dev-state/plan13-final-doc-spelling.log` |

The thirteen ordinary native skips comprise the twelve separately executed external cases and
one manual operation-measurement case whose executed receipt is in the measurement report.
The seventeen ordinary Python deselections are the fifteen client and two live cases executed
separately. No skipped test is relabeled as a pass.

The five independent logs and receipts were selected byte-identically into `docs/reports/logs/`
for `just acceptance-report` and `just acceptance-check`. Original standard diagnostics remain
under `.dev-state/plan13-independent-audit/previous-standard-logs/`; selection hashes are in
`final/standard-log-selection.json`. The auditor independently verified the generated report:
**47 passed / 0 failed / 0 blocked / 1 not_run of 48**. All 46 frozen IDs remain present once;
P08a/P08b are the ADR-0005 successors of retired P08. The per-gate audit is
`.dev-state/plan13-independent-audit/final/acceptance-audit-2026-09-15.md`.

The installed manifest predates the last observer and test-only corrections. Every installed
runtime input still matches its recorded hash; those corrections do not require a new runtime
candidate. The preceding installed candidate's family and performance receipts remain attributed
to that candidate, with identical native binaries. The current adapter has its own focused and
actual-client receipts. This is affected-change qualification, not relabeling old evidence.

## Journey evidence map

**All twenty journeys passed within their declared scopes.** Test names refer to actual executed
native or Python cases; campaign payloads are retained with their receipts. The distinction
between useful partial evidence and complete knowledge remains part of the passing assertion.

| Journey | Concrete oracle and observed meaning |
|---|---|
| J01 | Current installed focused campaign: SessionContext, DataFrame, ParquetReadOptions and Expr each return signature and useful bounded documentation by default. |
| J02 | `large_alternative_sets_remain_reachable_for_inspection_and_comparison`, foreign-cursor fixtures and installed traversal of all 117 SessionContext relationship IDs without duplicates. |
| J03 | Installed 54.1.0→55.1.0 comparison retains 24 changed documentation keys and independent missing release-note assessments on both sides; result remains partial. |
| J04 | `requested_coverage_distinguishes_unknown_missing_partial_and_recovered_scopes` and cold/offline retrieval exercise the shared requested-domain relation without merging incompatible qualifications. |
| J05 | Native flat comparison fixtures preserve independent expected values, qualified endpoints, nulls, duplicates and 97 alternatives. Live representation differences are qualified observations, not an asserted compatibility break. |
| J06 | `job_lookup_distinguishes_unknown_permission_and_corrupt_journals`; actual installed recovery independently reads the failed native journal and artifact before resolving an available release. |
| J07 | Cold comparison and acquisition, `test_a_long_operation_returns_pending_and_finishes_through_ordinary_tools`, and 135 installed one/eight-client tasks reconstruct results through compact terminal jobs and artifact actions. |
| J08 | Indexed Unicode section tests, escaped MCP frame tests, direct data-section cursor/digest checks and installed named-section reads preserve complete bounded content and advancing continuation. |
| J09 | Revision fixture plus pinned commit `7d3835c71f30cbd3c3ae4041732267f1f453097a` yields useful source evidence, explicit LICENSE/NOTICE omissions and stable offline snapshot. Hostile archive entries still fail closed. |
| J10 | `test_every_tool_answers_cold_then_offline_from_the_same_snapshot`, native retained/candidate coverage tests and the installed 40-crate campaign preserve scope and evidence identities. |
| J11 | `native_navigation_and_text_projection_preserve_retained_identities`, qualified alias/document folds, conflicting-feature pages and discovery facet isolation preserve source distinctions and actual excerpt windows. |
| J12 | Real stdio contract journey and complete-frame tests cover native outcome/error flag, strict actual output, complete recovery actions, effects, optional progress and resource links. |
| J13 | Original-JSON MCP/RPC rejection corpus, fifteen real Unix-peer transport cases and offline catalog/status checks distinguish connection, timeout, framing and protocol failures. |
| J14 | `static_service_reports_the_same_execution_prerequisites_at_every_route`, actual qualified Rust/Python execution, independent release qualification and installed runtime client tasks preserve policy and scope. |
| J15 | Native runtime tests cover real managed-memory exhaustion, cumulative output, disk quota, deadlines, independent field requirements, bounded witnesses and blocking-worker ownership. A 128-byte pool fails with correlated capacity, releases admission, then admits subsequent work. |
| J16 | Two-client shared-work/cancellation, durable interests, cold-comparison restart, client timeout teardown and blocking-reader/sink tests preserve surviving interests and wait for actual owned cleanup. |
| J17 | `comparison_publication_exports_both_inputs_and_verified_result_closure`, failed stream/index writes, read-only recovery, five SIGKILL publication boundaries and offline bundle corruption checks require complete closure without producer reruns. |
| J18 | Forty exact DataFusion 55.1.0 crates, each resolve/overview/search/inspect/offline; representative real Rust/Python fixtures and live registry canaries. Source-supported useful partial evidence is accepted without inventing complete-library coverage. |
| J19 | Fourteen actual installed Codex/Claude scenarios plus preservation, independently assessed through native snapshot/artifact/failed-job witnesses; exact API, discovery, upgrade, recovery and runtime tasks pass. Client prose limits are recorded below. |
| J20 | Fresh state/6 and obsolete-format rejection fixtures; deployed current schema/binaries/worker and both global registrations match. Old daemon/adapters are stopped; old evidence/config bytes are unchanged. All enabled-profile canary digests pass; no task workers remain after the deployed smoke. |

## Deployed generation and preservation

`library-enrichment.service` now supervises PID **818737** from the installed candidate.
The fresh active state is `/home/paul/.local/state/library-enrichment-v2`; the selected configuration
is `/home/paul/.config/library-enrichment/research-v2-3ae079f9718a.toml`.
Its SHA-256 is `2b8ec51e6a07b0ff71dac3ed0648e356bc08192ec8eedb88d2557c00c3c47ae8`.
The native executable hashes are:

| Component | SHA-256 |
|---|---|
| daemon | `f64f58fa9128208e62bbee164eb203e0c54c37a32e2b92b729a3fb591a736d57` |
| executor | `67fa43bb7233d91233e0f6c1c0ac9a7382c3a84fade17db19a78ebb1f3904c7e` |
| native worker | `85b27114cf2c72f4aae184513e1bfaef69038ccff0dec337aa76cc96272be222` |

Every installed input still matches `candidate.json`. Actual status admits research/2.0 and
snapshot/6.0 only. Effective managed memory is 32 GiB, spill ceiling 64 GiB, metadata cache 2 GiB,
and query admission concurrency 16. The existing **static-only production policy** is preserved;
qualified Rust/Python execution was proven separately in isolated roots. Successful production
packaging extraction qualified the installed Griffe worker; there are no daemon children,
queued/running jobs or admitted queries after the smoke.

The deployed smoke resolved exact DataFusion 55.1.0 to `snap_69c466e7240e78b5`, returned useful
SessionContext default inspection, completed a pending documentation/notes comparison and read
its changes section with a verified content digest. Packaging 26.3 resolved to
`snap_1db11d3767046c09` and returned Version observations. Acquisition/comparison partial outcomes
retain their native evidence limits and are not converted to complete claims.

Old daemon PID 1885544 and its four adapter processes were stopped before routing changed.
The old state `/home/paul/.local/state/library-enrichment` and old `service.toml` remain inactive
and unchanged. No migration, fallback reader or deletion was performed. Both global Codex/Claude
registrations match the same installed adapter launch. Both managed product-skill copies match
all three shipped files. Existing interactive clients may need to reconnect to load the new catalog;
their parent sessions were preserved. Activation, smoke, deployment, registration and skill
receipts are under `.dev-state/plan13-cutover/`.

At 14:00 UTC, a final process/source check found only the intended deployed daemon among service,
adapter and helper processes, with the final source digest unchanged. Receipt:
`.dev-state/plan13-cutover/final-process-source-verification.json`.

## Evidence limits and preserved failures

- The original integrated `just ci` returned failure: one obsolete error-summary assertion
  remained after the new portable diagnostic projection. Its corrected raw-frame oracle passes.
  The original client command had five observer failures after completed native work; the trace
  reader now distinguishes pre-admission argument rejection from a terminal failed job. Those
  commands are retained as diagnostic runs, not reported as successful final-source commands.
- The final source checks and independent component replays qualify these last corrections.
  No repeated whole-suite run is required for the subsequent documentation-only closure.
- The namespace registry assertion has a narrow fixture; the independent auditor also inspected
  actual installed upgrade evidence for both import roots and explicitly partial package coverage.
  The registered C06 test is supplemented by all five executed SIGKILL publication boundaries.
- Actual Claude prose once inferred “never existed” from an unavailable registry result and once
  repeated a readiness observation taken before extraction completed. Native evidence retains
  the narrower observations. Client acceptance does not certify every generated sentence.
- Revision source/build closure, universal package completeness and general compatibility are
  not established by these observations. Static and runtime/stub evidence remain distinct.
- Managed Arrow/output/spill budgets are not a total process RSS cap. Measurements report their
  sample counts and limitations; they do not establish reliable p95 latency or a universal speedup.
  Broader throughput work has the observable trigger in deferred-register R43.

## Related evidence

- [Live functional results](plan13-live-functional-outcomes-2026-09-15.md)
- [Native execution measurements](plan13-native-operation-measurements-2026-09-15.md)
- [Removal and replacement inventory](plan13-removal-evidence-2026-09-15.md)
- [Closing design review](../design_review/reviews/design_review_plan13-implementation_2026-09-15.md)
- [Execution ledger](../plans/13-research-operations-execution-ledger.md)
