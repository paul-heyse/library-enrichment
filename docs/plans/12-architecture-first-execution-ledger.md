# Plan 12 execution ledger

Implementation authorized 2026-09-14. The shared working tree contains the uncommitted
Plan 09–11 work over `cd6d9490a6433ae355b04a465538062e1320544c`; preserve those changes.
The dependency order and scope are [Plan 12](12-architecture-first-completion.md).

## Current work

AR1–AR4, DC1/DC2 and FN1–FN4 are functionally complete on the target-only architecture.
Integrated checks and qualitative client-output review passed. The user stopped the redundant
registry-only independent replay; its incomplete certification is recorded below. No further
functional work or validation campaign is pending. Earlier sections are historical checkpoints.

## Executed checks

- 2026-09-14: `just toolchain-check` passed; active Rust matches stable 1.98.1.

## Implementation checkpoint — 2026-09-14

- AR1: DataFusion owns semantic-key distinct/order; incremental canonical hashing preserves
  the JSON-array preimage. Native joins resolve producer bindings, relationship endpoints,
  fragment subjects, input closure and actual acquisition attribution. Whole-input normalizer
  maps/sets and relation-wide semantic-key/blob collections are removed.
- AR2: native Parquet assembly uses a one-batch asynchronous handoff to a blocking writer
  retaining input plans and the store lease. Invalid fragment locator objects and public-api
  rendering failures are explicit. Raw Rustdoc extraction now runs in the mandatory native
  worker; the daemon consumes finite verified producer streams instead of retaining its AST.
- AR4: terminal delivery failures settle job status explicitly, preserve visible terminal
  records, retain unpersisted failures in memory and bound further admission until recovery.
- The native executable is renamed `library-enrichment-native-worker`; supported build and
  launch/measurement consumers use only that name. ADR-0028–ADR-0033 are accepted after the scoped native architecture review.

Executed focused evidence (not full Plan 12 or phase acceptance):

- `.dev-state/plan12-ar1-ingest-tests.log`: four migrated Rust/Python native ingestion cases passed.
- `.dev-state/plan12-ar1-store-tests.log`: four dataset and twelve repository cases passed.
- `.dev-state/plan12-ar2-native-writer-tests.log`: four ingestion and twelve repository cases passed
  after the blocking-writer change.
- `.dev-state/plan12-ar4-job-tests.log`: eight job tests passed, including failed terminal storage.
- `.dev-state/plan12-boundary-clippy.log`: core/store/daemon all-target Clippy passed.
- `.dev-state/plan12-native-parser-clippy.log`: all-target Clippy passed for native-parser integration.
- `.dev-state/plan12-native-parser-offline.log`: actual cold-to-offline Rust research journey passed
  through the new native extraction boundary. Later edits need their relevant rechecks.

## Architecture and removal checkpoint — 2026-09-14

- AR2: Python static worker limits are Rust-requested and installed before source parsing;
  docstrings are complete within the explicit response budget. Oversized declarations produce
  file gaps and retain source bytes. Worker transport decoding/preparation runs off async workers.
- AR3: artifact/provenance/log selection and deduplication now runs in DataFusion before hydration.
- AR4: C07 publishes a distinct snapshot in the same context using a later actual registry
  selection, without reacquiring the wheel; the old pinned snapshot stays byte immutable.
  Export assertions use the target bundle and verify complete closure independently. Recovery
  refuses unknown execution children and recognizes typed unpublished producer staging safely.
- DC1: native ownership/deletion rules and `just architecture-check` are active in CI.
  ADR-0028–ADR-0033 have one accepted scoped review; design, index and revisit register align.
  Actual client functionality remains FN3; old substring and shared prerequisite logic is deleted.
- DC2: exact cutover preview `plan12-cutover-preview.json` and applied receipt
  `plan12-cutover-applied.json` live under `.dev-state`. The locked inventory removed 340
  payload roots/files (2,818 entries) in 36 old physical fixture roots (formats 1.0–3.0),
  empty test-created real-XDG format-4 residue and retired binary products. No symlink aliases
  were traversed. Five exited private containers were matched by name/image/time/mount and
  removed; absence was confirmed for all 43 ownership records before deleting their journals.
  Original permanent locks, diagnostic logs, source, credentials and image storage remain.
  The one-off script's unused `.retention.lock` files were removed afterward; no application
  uses that filename. The final real-XDG baseline was recorded after this intentional cutover.

### D1–D13 deletion closure

| Obligation | Current implementation / boundary |
|---|---|
| D1 | Old catalog/snapshot/tables modules absent; current typed repository only |
| D2 | No production all_symbols/all_fragments or collection comparison/search entrypoints |
| D3 | parquet_io production module/export absent; corruption readers are test support |
| D4 | Dead contexts layout accessor absent; native catalog contexts remains required |
| D5 | Native browse/search/inspect selections precede bounded presentation |
| D6 | Shared delivery.rs bounds and stores complete result artifacts for inline/jobs |
| D7 | Source cache authority absent; verified retained archive bytes supply excerpts/documents |
| D8 | Stateless core kernels plus typed native staging/joins; test collectors are private/support |
| D9 | Exact owned physical cutover applied; no old reader or migration |
| D10 | Shell wrapper delegates to one locked generational installer; initialization/intent scratch now stays in its owned root, with actual interrupted-process recovery |
| D11 | Target manifest/export/same-context assertions; native client correlation replaces substrings and shared skips. Actual expanded scenarios remain FN2/FN3 |
| D12 | Original-byte coordinate scanner shared by retained admission and LSP documents |
| D13 | No AcquisitionEvent/record_acquisition sidecar path; old acquisition dirs removed |

### Additional executed focused checks

All on 2026-09-14; these logs are diagnostic evidence, not final source-bound gate receipts.

- `plan12-architecture-tests.log`: Rust library/integration suites passed; eleven real-container
  tests remained ignored and are still required explicitly.
- `plan12-provenance-selection.log`: 19 store unit and 12 repository tests passed.
- `plan12-schema-generation.log`: generated schemas/worker DTOs and schema conformance passed.
- `plan12-python-worker.log`: four worker checks passed, including full Unicode docs/oversize gap.
- `plan12-staging-recovery.log`: two staging recovery tests passed.
- `plan12-ownership-recovery.log`: unknown execution child preservation/refusal passed.
- `plan12-publication-export.log`: strengthened same-context C07 case passed.
- `plan12-export-e2e.log`: four portable export cases passed.
- `plan12-cutover-fresh.log`: thirteen fresh retrieval/retention/export cases passed.
- `plan12-client-parser.log`: three parser rejection/correlation cases passed; not client gates.
- `just architecture-check`, `ast-grep test` (eight rules), `just adr-lint` and real-XDG
  `just state-leak-check` passed. Initial stale e2e assertions and fixed type errors remain in
  earlier logs; they are not promoted to acceptance.

## Functional completion and integrated acceptance in progress

Implemented on 2026-09-14:

- Cold comparison one-side failure now reports VERSION_NOT_FOUND for an absent exact Python
  release. Shared comparison cancellation/restart keeps the surviving acquisition interest.
- Derived execution identity includes the source/coordinate/LSP/runtime helper closure and locked
  dependencies. Exact retained reuse still requires the qualified context and execution policy.
- Requested Rust target/default/features can trigger qualified local observation after hosted
  evidence is fetched. Hosted and local producer identities remain separate.
- Identical R09 consumer and lock now run on stable and the recorded dated nightly; actual
  containment qualification passed 12 tests. C20 now awaits successful Rust/Python operations
  across all six language/profile cases.
- Fixture execution settings come from the actual selected Rust qualification description.
  The cancellation-wrapper test performs its own qualification and restores the original one
  through actual probes. `.dev-state/plan12-broker.log`: one passed.
- Full runtime alternatives and large Unicode results use the current documentation aspect and
  paged artifact protocol. `.dev-state/plan12-runtime-final.log`: four passed. The previous
  functional run had 23 passed and two stale test consumers, both corrected by that rerun.
- Two-adapter Inspect/Verify survivor journeys now await usable results. Operational status
  exposes bounded native query/admission counters under accepted ADR-0034.
- Installer initialization and intent staging are recoverable inside the owned root; foreign
  bytes remain refused. `.dev-state/plan12-ops-final.log`: 23 operational/installer/parser tests
  passed. Clippy all targets/features, Ruff, ty and regenerated schema conformance passed.
- Actual Codex 0.154.0 and Claude 2.1.270 A01/A02 pilots passed with native event correlation,
  complete jobs/pages and independent daemon snapshot/artifact witnesses. Their output caught
  a real stale pre-publication summary, now corrected. The full ten-scenario client run,
  including upgrade/runtime in both clients, is in progress.

Current integrated commands: `CARGO_INCREMENTAL=0 just ci`, authenticated `just test-client`,
and optimized retained-workload preparation. Final qualified producer/ignored tiers, live
registries, performance assessment, acceptance audit and final status reconciliation remain.
These focused logs are not promoted to current-source gate receipts. No full-plan completion
is claimed until the integrated obligations settle.

## Final audit corrections and performance disposition — 2026-09-14

The independent preparatory acceptance audit identified and implementation corrected: separate
ignored Rust evidence was not joinable for R09; Python tests could proceed after a failed build;
client recipes omitted selected execution environment setup; receipts omitted exercised native
binary hashes; and A04 could pass after a resolver lookup with no actual documentation. The report
now joins regular and ignored native logs, setup failures stop execution, native binaries are bound
before/after, and A04 researches actual Context7 Serde documents against real exact 1.0.228 evidence.

Actual A01–A06 initially passed, and upgrade research in both clients plus Claude runtime passed.
The first Codex runtime attempt correctly reported its own client approval denial and did not
invent an observation. The supported per-tool approval setting is now applied in the isolated
home for this explicitly authorized scenario; the filesystem sandbox remains read-only.
The final corresponding actual checks are in progress. Real-user configuration digests remain
unchanged. The review also caught an invalid default-feature inference in an A03 brief; the live MCP
server instructions now explicitly state that empty cfg hints do not establish project
availability. The shipped skill remains byte-identical frozen provenance; its existing guidance
already requires separating hosted build configuration from project configuration.

The optimized existing cold/restart workload ran five successful output checks. Median wall time
was 2.20897 seconds (individual runs 2.12710–2.31651 seconds), above the historical 1.5-second
median target and below its 3-second per-run target. Receipt: `.dev-state/plan12-performance-final/`.
On 2026-09-14 the user explicitly deferred focus on the arbitrary timing target until correctness,
failure robustness and target functionality are complete. No budget was changed or performance
pass claimed; further performance work is deferred and does not block this plan's functional scope.


## Final client evidence integrity corrections — 2026-09-14

The independent auditor found a teardown false-pass: a successful scenario was appended before
daemon teardown, then a failure could append a duplicate which the consumer ignored. Each
scenario now publishes one outcome after teardown and cleanup. Summary validation rejects
duplicated/missing cases and inconsistent exit codes. Every client pytest row records the retained
trace root and file hashes; receipt validation rejects altered or missing linked outputs.
Both targeted failure regressions passed, and the auditor independently reran all ten parser/
receipt checks. The preceding broad producer run predates these final harness edits and cannot
be promoted as current-source acceptance; its behavior remains diagnostic evidence.

The living design's stale future-work labels were reconciled with implemented native storage,
all nine tools, worker/execution boundaries and durable jobs. Frozen specification, product skill
and accepted ADR arguments are unchanged. The fresh provenance check is 15/15 and real-XDG
state remains unchanged.


## Actionable inspection and completed artifact reads — 2026-09-14

Actual Serde research exposed an unselectable same-path ambiguity. Accepted ADR-0035 adds an
optional existing definition ID and typed candidates; native DataFusion filters by path and ID
before hydration. A real source-class/stub-function fixture verifies both selections, origin,
source excerpts, mismatch rejection and restart. The combined selection/parser/catalog run passed
23 tests (`.dev-state/plan12-selection-parser-current.log`); all-target/features Clippy and
regenerated schema conformance passed. No legacy candidate decoder was retained.

The artifact transcript validator now follows completed cursor chains even when the client
restarts with smaller pages. The actual retained Codex upgrade trace validates after this repair;
its old run is diagnostic, not current-source acceptance. Native Claude authentication failures
are distinguished from model text and reported as a named prerequisite. The user refreshed the
operator login before the final current client run.


## Final output-discovered overview correction — 2026-09-14

The refreshed ten-scenario run completed with 11 pytest checks passed, including both formerly
blocked Claude journeys (`.dev-state/plan12-client-current.log`, the retained run linked by its receipt). Actual
Serde inspection selected the macro by returned definition ID and completed its qualified derive
compile probe. Output assessment then found one service defect: overview selected an undocumented
stub by observation ID before a documented source, producing a false missing summary across
releases. Native ranking now prefers a deterministic nonnull summary within the selected public
binding; full inspection still preserves all alternatives. The deterministic native regression
passed (`plan12-overview-docs-regression.log`). A strengthened real Python journey checks summary
retention and post-extraction worker status.

The client's Griffe-status concern came from carrying startup status forward, not a failed
qualification update; Rust already shares and updates that process-scoped state. No health code
was changed. The brief's unsupported absence/MSRV/container-escape inferences are not adopted as
service evidence. The valid upgrade/runtime facts and explicit partial/environment limits remain
useful. Final gate receipts must be refreshed after the overview correction.


The strengthened real Python summary/status journey passed on 2026-09-14
(`plan12-overview-python-current.log`). Final CI initially stopped at a documentation spell-check
on an opaque temporary-run identifier; the prose was corrected, with no suppression. The current
CI rerun has passed lint/compilation and is preparing the final native test graph. Source remains
fixed during recorded execution.


Current final source digest is `4b434e3f7fd604068cf5e59205d7a0a54de09b442ce1a0d3267913babfd0db0a`.
The final Rust receipt records 390 passed with unchanged native hashes; the twelve real contained
cases remain the explicit ignored tier. Both live registry checks passed again on this source
(`plan12-live-final-source.log`). Final Python CI, client refresh and independent replay remain.


The independent read-only follow-up found no defect in the overview correction. It corrected an
unsupported integrated-review statement: query-result limits and decoder timeout/abort/reaping
are exercised, while spill limits are configured without a forced-spill/limit-failure campaign.
No such campaign is claimed and no speculative test expansion was added.


## Final report mapping correction and independent replay — 2026-09-14

Final `just ci` passed 390 Rust and 189 Python checks, with all schema/dependency/rule/provenance/
state checks passing (`plan12-ci-final.log`). The explicit contained tier then passed all twelve
cases (`plan12-execution-final.log`); both live cases passed. These commands had valid source and
unchanged native receipts before the following registry-only repair.

The generated join caught stale P03/C17 test references: P03 still named the removed legacy table
round-trip test; C17 omitted the actual `[1]`/`[2]` parameter nodes. P03 now names the target native
nested-observation round-trip alongside its existing real source/stub and corruption tests, and
C17 requires both actual parameter cases. Gate IDs/assertions are unchanged. This registry edit
changes the source fingerprint; no old receipt is rewritten or promoted. The mandatory independent
replay will run the recorded native, Python, contained, live and client commands on the corrected
registry and regenerate the final report. Source stays fixed through that replay.


The independent replay confirmed the corrected registry's 152 references (147 distinct) resolve
to real executed tests; only retired P08 intentionally has no current test. Its Rust replay
passed all 390 on source `6f5a3646a3fbb6ac5c9faca18a579b8364ac4cd506d317b3811e4a547f2886c8`,
with matching native hashes. The Python, explicit execution/live and client replays remain active.


## Final disposition and cleanup — 2026-09-14

The user explicitly stopped redundant replay after the metadata-only P03/C17 registry correction.
Functional Plan 12 scope is complete, supported by final-source CI (390 Rust, 189 Python), twelve
contained cases, two live cases, ten actual client scenarios (eleven checks), and targeted plus
full-CI validation of the final overview correction. The client run preceded that correction;
its receipt is preserved with its actual source identity. No receipt was rewritten or promoted.
The integrated review accepts the functional implementation with these validation boundaries.

Independent replay finished all 390 Rust cases on the corrected registry, then was interrupted
by the user during Python publication-recovery tests. This is an incomplete audit, not a source
failure. Later contained/live/client replay tiers did not start. Current report regeneration:
**7 passed / 0 failed / 0 blocked / 41 not_run** (48 IDs, including retired P08). This report
intentionally does not certify older-source receipts under the corrected registry.

`.dev-state/plan12-independent-audit/AUDIT.md` records the stop and its evidence. All nine recorded
processes exited, and read-only inspection confirmed zero containers and zero ownership records
in the selected execution root. No forced deletion or unrelated process termination was needed.
Logs and earlier receipts remain in the audit directory, including `prior-logs/`. No task-owned
execution or cleanup remains. Performance tuning and threshold enforcement remain deferred.

Final read-only checks passed: `just acceptance-check provenance-check state-leak-check`
(48 gates accounted for, 15/15 frozen files verified, real XDG paths unchanged), documentation
spelling and `git diff --check`. No tests or builds were started after the user stopped replay.
