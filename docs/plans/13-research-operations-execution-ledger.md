# Plan 13 execution ledger

**Complete and deployed — 2026-09-15 13:53 UTC.** Implementation was authorized on 2026-09-15.
Baseline HEAD: `41ac67219ea39e6b085dde6273c9df08c9937bba`. The user confirms the tree was clean
when execution began; accumulated working-tree edits are Plan 13 work. No reset, commit, merge,
push or evidence deletion was performed.

## Final outcome and operating authority

W0–W12, D01–D12 and J01–J20 are complete within the prescribed scope. See the
[final qualification and journey matrix](../reports/plan13-final-qualification-2026-09-15.md),
[closed removal inventory](../reports/plan13-removal-evidence-2026-09-15.md),
[accepted closing review](../design_review/reviews/design_review_plan13-implementation_2026-09-15.md),
[live outcomes](../reports/plan13-live-functional-outcomes-2026-09-15.md) and
[native measurements](../reports/plan13-native-operation-measurements-2026-09-15.md).

- Final source: `224dea96aca19e8482b6715f524ebe0b5948ba039e0c8fac00200aa933d72bf5`.
  DataFusion 55.1.0, Arrow/Parquet 59.3.0 and FastMCP 4.0.3 remain pinned.
- Independent final replay: **432 native, 223 Python, 12 execution, 2 live, 15 installed-client
  tests passed**. The client command includes fourteen actual Codex/Claude scenarios and operator
  preservation. Receipts and per-gate audit: `.dev-state/plan13-independent-audit/final/`.
- Byte-identical copies of those five log/receipt pairs feed the regenerated standard report.
  Acceptance and independent audit: **47 passed / 0 failed / 0 blocked / 1 not_run of 48**.
  Retired P08 is the sole not_run; its ADR-0005 successors are passed. The ordinary ignored and
  deselected external cases ran separately; the manual measurement has its own executed receipt.
- Source/static, dependency-policy and final state-leak checks passed. Earlier failed integrated
  CI/client receipts remain preserved as diagnostic runs. Corrected raw-frame and observer
  assertions were followed by independent full component replays; no older exit was relabeled.
- The verified installation `/home/paul/.local/opt/library-enrichment/plan13-3ae079f9718a` was
  activated once at **13:49:55 UTC**, with fresh state `/home/paul/.local/state/library-enrichment-v2`.
  The unit supervises verified daemon PID 818737; matching adapter, worker and both native helpers
  come from the same installed manifest. All installed hashes and effective limits match.
- The seven-case deployed raw-MCP smoke passed, including exact DataFusion resolve, useful
  SessionContext default, pending comparison completion, direct changes-section digest and
  packaging/Version. Old daemon/adapters are stopped; old state/configuration are unchanged.
  No owned worker, admitted query or pending job remained after the smoke.
- Both global Codex/Claude registrations and managed product-skill copies match the installation.
  Existing interactive sessions may need to reconnect for the current catalog. Their parent
  processes were preserved. Production retains its existing static-only policy; actual qualified
  execution passed in independently bound debug and installed-release roots.

No required implementation work remains. Full revision build closure, universal package/compatibility
claims, process RSS guarantees and statistically reliable tail latency remain outside the proven
scope. Native evidence retains missing/partial/unknown distinctions. Client prose limitations,
exact-version API findings and deferred capability/throughput triggers are explicit in the linked
reports and ADRs.

**Historical checkpoints follow.** Statements about running tests, pending scope or inactive
candidates below describe their timestamp only; this final section is the current handoff.

## Latest integration checkpoint — 2026-09-15 13:20 UTC

- The successor installation is `/home/paul/.local/opt/library-enrichment/plan13-3ae079f9718a`.
  Its three native hashes equal the release-qualified binaries. Its non-editable locked adapter
  includes the portable failed-job error projection. Focused installed receipt
  `.dev-state/p13-installed-live/focused-130513-receipt.json` passed with unchanged inputs.
- Release qualification `.dev-state/plan13-release-execution-qualification.log` passed both
  producer probes and 12 real containment/cleanup cases. The release image root is `.dev-state/p13r`;
  the debug root `.dev-state/p4p` has its independent qualification.
- The final native leg passed 431 tests, with 13 skips. The acceptance-report skill's required
  independent auditor replayed the same command: 431 passed, unchanged binaries/source for that
  replay. Receipt: `.dev-state/plan13-independent-audit/nextest.json.execution.json`.
- The long Python run exposed an outdated generic 160-byte error-summary assertion. Its corrected
  raw-wire regression checks the native recovery code/action and the unchanged 1024-byte total
  framing allowance; `.dev-state/plan13-final-portable-error-frame.log` passed.
- Installed actual Claude recovery now passes with an independent failed-job journal witness.
  Its original failure was real: Claude hid structured MCP error output. The adapter now supplies
  bounded native diagnostics and complete artifact-read actions in text. The corrected raw-boundary
  log records 12 passed cases. A failed artifact write supplies inline recovery without a read handle.
- A separate trace-reader correction distinguishes a pre-admission argument error from a terminal
  failed job even when the rejected request contains a job ID. Actual Codex research completed;
  the old observer rejected it. Ten parser regressions pass after correcting the distinction.
- The closing review required direct managed-memory pressure evidence. A real DataFusion sort
  under a 128-byte pool returns a correlated capacity error, releases admission/reservations and
  allows a subsequent query. `.dev-state/plan13-native-memory-pressure.log` passed. This adds an
  oracle without changing runtime behavior. Final native test count will therefore be 432.
- These observer/test changes altered source during the long runs. Original receipts remain
  diagnostic; final independent replay must bind source
  `224dea96aca19e8482b6715f524ebe0b5948ba039e0c8fac00200aa933d72bf5`.
  Installed runtime inputs remain unchanged. No new candidate assembly is needed for test-only edits.
- The current actual-client run is `.dev-state/logs/clients/run-snt4jm4k/`; full source/client
  terminal results and independent replay remain pending. The operations guide and closing
  implementation review are updated. Prepared activation files are under `.dev-state/plan13-cutover/`.
  Production has not switched: fresh target state will be `/home/paul/.local/state/library-enrichment-v2`,
  preserving current static-only policy and the prior root/configuration inactive.

Earlier checkpoints below are historical; they are not the current remaining-work list.

## Latest installed architecture checkpoint — 2026-09-15 12:46 UTC

The native replacements now have both development and installed-release live evidence:

- All **40 DataFusion 55.1.0 crates** completed resolve/overview/search/inspect/offline through
  raw MCP, first on debug and then on the installed release. Each family retained 201 answers.
- SessionContext/DataFrame/ParquetReadOptions/Expr defaults returned signature and documentation.
  Installed explicit traversal covered all **117 SessionContext relationships**, without duplicates.
- Revision/search/offline and five genuinely qualified Python runtime/ty/reuse regressions pass.
  The development executor was genuinely requalified; the earlier missing exports were a command error.
- Release binaries were built under `.dev-state/p13-release-target`, preserving the still-running
  old production binaries. The inactive installed candidate is
  `/home/paul/.local/opt/library-enrichment/plan13-3a540f4ab7eb`, with a non-editable locked adapter.
- W11 retains native per-operation indexes. The measured endpoint semi join outperformed the
  four-keyed-join union on equal bounded results. The installed one/eight-client experiment
  completed **135 equal-result tasks**, including pending comparison and full artifact delivery.
- Codex's real connection, discovery and failed-release recovery passed with independent daemon
  witnesses. Full client outcomes are still running. Client integration now admits complete
  direct data-section chains and host resource-link presentation; short citation identities must
  uniquely select observed evidence. Original failed client receipts are retained unchanged.
- Final integration is underway. Its first attempt stopped at formatting before tests. The next
  exposed three retrieval consumers assuming synchronous completion under concurrent load; the
  shared fixture helper now waits for pending jobs. Client-harness edits changed that run's source
  fingerprint, so it cannot be promoted as a stable final-source receipt.

The [live functional report](../reports/plan13-live-functional-outcomes-2026-09-15.md) and
[measurement report](../reports/plan13-native-operation-measurements-2026-09-15.md) record exact
receipt paths, observed values and limits. Release qualification, closing review, final acceptance
and the single production activation remain open. Earlier checkpoints below are historical.

## Current continuation checkpoint — 2026-09-15

The consolidated `.dev-state/plan13-architecture-integrated-ci.log` finished with **424 Rust
passes / 3 stack overflows / 13 skips** and **210 Python passes / 7 failures / 13 deselections**.
The three Rust failures occurred in real acquisition/publication before comparison assertions.
GDB recorded cumulative large async poll frames entering the native SQL planner; the shared
`publish_records` future and producer callback now allocate on the heap before caller polling.
No stack limit changed. All three formerly overflowing cases pass, alongside six admission and
semantic-scope cases in `.dev-state/plan13-native-publication-scope-outcomes.log`.
Rust edits began while the already-failed Python run was completing; that run is pre-fix
diagnostic evidence, not certification of the later source.

Native publication checks now use one bounded offending-identity projection with independent
Arrow field requirements and persistent operation-linked diagnostics. Attempt, publication and
unsigned-count decoders also declare their fixed query family. A separate native class-scope
assessment replaces the semantic producer's obsolete dependency on hydrated `Symbol.python`.
It uses qualified observation joins, list unnesting and counts to retain conservative behavior
for missing/conflicting declarations and unresolved bases. Its implementation digest participates
in producer identity. The actual ty end-to-end correction still needs execution qualification.

The first isolated live raw-MCP candidate under `.dev-state/p13-native-live/` returned usable
DataFusion 55.1.0 SessionContext, DataFrame and ParquetReadOptions defaults and a bounded explicit
relationship page. It then failed datafusion-expr overview at namespace consistency. Overview
now retains its chosen namespace relation before independently planned child/summary consumers.
A 300-namespace fixture with 32-row batches and a 256-namespace cutoff passes, including count,
sample and drop ownership assertions. The next live run determines the actual campaign outcome.

Revision search exposed malformed source citations built from registry identities containing
their own fragment selector. `Revision::source_uri` now creates percent-encoded immutable GitHub
file permalinks containing commit and package path; `revision-source` is **5**. The canonical
URL/path fixture passes. The adapter's URI validation remains intact. Its e2e search/offline
journey is being rechecked.

The selected Python diagnostic run passed three cases, failed revision citation delivery, and
skipped five execution cases because the focused command omitted `execution-env.sh` exports.
The original attribution to qualification was corrected. A subsequent read of the exact rebuilt
helper's containment identity also differs from `.dev-state/p4p/admitted-images.json`, so that
root does need genuine requalification before final execution outcomes can be recorded.

Current additional receipts:

- `.dev-state/plan13-native-admission-semantics-clippy.log` — passed all affected targets.
- `.dev-state/plan13-overview-revision-clippy.log` — passed all core/store/daemon targets.
- `.dev-state/plan13-overview-revision-native-outcomes.log` — three selected outcomes passed.
- `.dev-state/plan13-overview-revision-build.log` — all three workspace binaries rebuilt.
- `.dev-state/plan13-native-live-focused.log` — partial successful journey, then overview failure.
- `.dev-state/plan13-native-scope-python-outcomes.log` — one failed / three passed / five skipped.

Plan 13 remains in progress. These focused receipts close neither final-source acceptance nor
the complete live family, actual Codex/Claude, closing review or production cutover.

## Final work-package closure

| Package | Implemented boundary | Passed evidence |
|---|---|---|
| W0 | ADR-0036–0039, frozen historical provenance, accepted current scope and final review | Scoped contract review, ADR/provenance checks and accepted G1–G7 implementation review |
| W1 | Rust research/2.0 domains, authored MCP composition over generated DTOs, current frozen target | Schema generation/conformance, strict original JSON and actual output consumers |
| W2 | Captured operation identities/policy/leases, managed admission/deadlines, independent Arrow requirements and bounded diagnostics | Final native runtime, memory/spill/output pressure, blocking-worker and field/witness tests |
| W3 | One native requested-domain assessment for candidate/retained, discovery, inspection and comparison sides | Coverage-state fixtures and actual cold/offline agreement |
| W4 | Selection before hydration; independent observation/relationship/document/member/child/execution pages | Native navigation and 97 alternatives; installed four useful defaults and 117 complete relationships |
| W5 | Flat distinct comparison, changed-key index, independent side alternatives and complete large-value artifacts | Native high-variant/qualified-endpoint fixtures, installed 54.1→55.1 partial-scope comparison |
| W6 | Typed native cause/stage/actions, durable failure history, uniform readiness | Real job journal distinctions, transport failures, static guidance and qualified execution |
| W7 | Safe revision omissions, bounded declared Cargo input collection, native disposition assessment, immutable citations | Hostile archive and declared-input cases; pinned revision/search/offline canary |
| W8 | Indexed result sections, encoded-size fitting, compact terminal jobs, full closure before commit, read-only recovery | Unicode/direct reads, export, failed writes and all five SIGKILL publication barriers |
| W9 | Thin typed FastMCP adapter, precise transport errors, bounded native recovery preview, effects/progress/resources | 223-test Python replay, actual installed clients and deployed raw MCP |
| W10 | Current formats only; complete matching three-binary/non-editable Python installation | D01–D12 closed, obsolete-root rejection/preservation and installed identity verification |
| W11 | Native operation indexes and measured endpoint semi join; explicit R43 broader-throughput trigger | 135 equal-result one/eight-client tasks, native paired plans, lifecycle/drop measurements |
| W12 | Final independent qualification, all J01–J20, one supervised fresh-state activation | 47 active gates passed; accepted closing review; deployed smoke, registration/skill and preservation receipts |

This final table supersedes the earlier work-in-progress table. Exact commands, counts,
receipt selection and limitations are in the final qualification report.

## Boundary refinement — 2026-09-15

User asked to consider Pydantic, without preferring it in advance. Exact installed probes support
authored MCP presentation models over generated Rust domain DTOs. ADR-0037 and Plan 13 §2 record
the evaluated choice. Native semantic/result correctness work continues; a new native MCP schema
composition framework will not be implemented. No domain coverage or durable state moves to Python.

## Executed focused checks — 2026-09-15

These are implementation checks, not a promoted acceptance tally. The schema/adapter transition
is unfinished and the old frozen corpus is deliberately still unchanged.

- `CARGO_INCREMENTAL=0 cargo test -p enrichment-store --test repository requested_coverage -- --nocapture`: 1 passed.
- `CARGO_INCREMENTAL=0 cargo test -p enrichment-store --test repository native_comparison_preserves -- --nocapture`: 1 passed.
- `CARGO_INCREMENTAL=0 cargo test -p enrichment-store --test repository large_alternative_sets -- --nocapture`: 1 passed, 97 alternatives and duplicate input rows.
- `CARGO_INCREMENTAL=0 cargo test -p enrichment-store --test runtime operation_ -- --nocapture`: 2 passed.
- `CARGO_INCREMENTAL=0 cargo test -p enrichment-core --lib archive::tests -- --nocapture`: 16 passed, before final local-PAX framing refinement.
- `CARGO_INCREMENTAL=0 cargo clippy -p enrichment-store -p enrichment-daemon --all-targets -- -D warnings`: passed after current native changes.
- `uv run pytest tests/unit/test_daemon_transport.py -q`: 15 passed over actual Unix socket peers.
- Focused Ruff and `uv run ty check python/enrichment_mcp/daemon_client.py tests/unit/test_daemon_transport.py`: passed.

### Material implementation limits still to resolve

Native output charges are conservative cumulative materialization retained until operation end,
not a process RSS measurement. Spawned jobs now receive separate operation IDs and cumulative native output budgets; producer runners retain cancellation/cleanup ownership. Their existing timeout owners are not replaced by a future-dropping wrapper. This still needs job outcome qualification.
Inspection header selection no longer hydrates observations implicitly; remaining consumers and
contract tests must use explicit projections. Archive closure explicitly remains unknown for
unproven generated/submodule/LFS inputs; a missing declared or potentially required omitted input
makes it incomplete. This has not qualified revision compilation.

### Additional executed evidence — 2026-09-15

- Native navigation and text-projection fixture passed: lexical children and typed members,
  unchanged fragment/source identities, and bounded-vs-complete text equivalence.
- Store repository suite (15) and runtime suite (4) passed after the first native field checks.
- All-target affected Clippy passed after persistent failure-history and background-scope changes,
  before the latest discovery/source-window edits.
- New `put_stream` failure test passed with no published artifact or leaked staging file.
- `just schemas-generate` passed after discovery request/output types, before source-window label.
- Indexed Unicode section retrieval passed: exact roundtrip, every cursor advances, encoded
  pages stay within 4096 bytes and non-final pages use at least 4080 bytes.
- New discovery test passes with conflicting same-name feature declarations preserved on separate
  pages. The former blanket refusal assertion was replaced with source-preserving traversal.

The larger current focused run and adapter checks are still being completed; none of these
results promotes the Plan 13 acceptance journeys or authorizes a completion claim.

### Integration checkpoint — 2026-09-15 06:42 UTC

- Daemon library suite: **129 passed** after fixing the large routed-future stack allocation
  (boxed at admission, no stack-limit increase), retargeting job assertions and introducing
  `coverage.details` for complete retained limitation text without dropping scope/assessments.
- Store execution/publication/export suite: **7 passed** with the indexed 2.0 job result format.
- Runtime suite: **6 passed**, including outer-join role preservation, persistent failure history,
  and separate background-job correlation without admission deadlock.
- Affected all-target Clippy passed before the latest schema packaging/correlation edits.
- Schema generation/conformance passed with packaged `_schemas` copied mechanically from the
  same Rust emitter. Explicit Pydantic 2.13.5 runtime dependency added; offline uv lock resolved.
- Retrieval integration: **6 passed, 14 failed**. Log `.dev-state/plan13-retrieval-fixture.log`.
  Many assertions/requests still target removed fields (`depth`, root pagination, nested jobs,
  comparison source arrays); these must be retargeted. A real new failure in the generic native
  pre/post-optimizer `path` field check is under investigation; do not treat all failures as
  stale tests. The focused probe log is `.dev-state/plan13-search-probe.log`.
- Full Plan 13 completion, all deletion rows, J01–J20, fresh generation, installed candidate,
  performance evidence, live campaign, real clients, final reviews/ADRs and cutover remain open.

### Integration checkpoint — 2026-09-15 07:10 UTC

- Full Rust workspace command completed: **408 passed, 0 failed, 12 ignored**. This is an
  ordinary test run, not acceptance certification; ignored external execution tests remain
  unexecuted. Log: `.dev-state/plan13-workspace-tests.log`. This preceded the subsequent
  required-nullable-root-field deserialization refinement.
- The current native retrieval suite has **21 passing tests**, including discovery-facet
  failure isolation and cold/offline agreement of scope, assessments, indexed/missing and status.
  Daemon library tests remain **129 passed**.
- Store library tests: **21 passed**. Runtime tests: **8 passed**, including real scoring UDF
  scalar/empty/partitioned queries, nested native string coercion, blocking-context propagation,
  cumulative result charges, deadlines and retained failure history.
- Schema generation/conformance passed after closed native payload DTOs and independently
  qualified discovery outcomes. The emitter explicitly retains untagged domain DTO definitions
  for authored MCP composition; no generated file was manually edited.
- Python contract/socket checks: **102 passed** before the latest raw-stdio test additions;
  Ruff and ty passed. Authored output schemas now reject empty or job-shaped inline successes
  for ordinary research tools. The registered workflow examples pass input-schema validation.
- Raw stdio fixture journey: **passed** against the rebuilt native daemon and real adapter.
  It exercised pending resolution, compact terminal jobs, complete artifact digest verification,
  default inspection, partial facet failure, typed unknown-job error, retired input rejection,
  workflow resources and complete raw-frame caps (envelope + 1024 bytes).
  Log: `.dev-state/plan13-raw-stdio.log`. This is not the real Codex/Claude acceptance journey.
- Affected all-target Clippy passed. Full source/consumer deletion audit, typed I/O call-site
  replacement (44 blanket call sites plus the local resolve mapper remain), real DataFusion
  revision/campaign, terminal result/error invariants, installed packaging, performance evidence,
  qualified execution/client gates, final design/ADR review and production cutover remain open.

Native research results now remain typed through budget enforcement and are serialized once
at RPC projection. Minimum-page failures fit the minimum supported cap and carry actual
requested/effective/required bytes. Admission/provider mutation failures retain a native rule,
bounded witness and request correlation. Source lexical search is explicitly unsupported;
it no longer reports internally contradictory indexed/missing coverage.

### Live DataFusion probe — 2026-09-15 07:20 UTC

Using an isolated state/6 root under `.dev-state/plan13-live-probe`, the rebuilt development
daemon/native worker and real raw-stdio adapter, with workstation resources and static-only
execution permission:

- Fresh DataFusion 55.1.0 and datafusion-expr 55.1.0 acquisitions produced qualified partial
  library results. Default SessionContext, DataFrame, ParquetReadOptions and Expr inspections
  all returned `ok` with retained signature observations and useful documentation. The original
  relationship-alternative presentation failure did not recur.
- The pinned revision `7d3835c71f30cbd3c3ae4041732267f1f453097a`, package `datafusion/core`,
  acquired successfully as `partial`, with 22 source-document fragments. Offline reuse returned
  the same context, snapshot and structured scope assessments. It does not claim a compiled API:
  static-only execution leaves public_api missing, and potentially required omitted package
  LICENSE/NOTICE links leave source closure incomplete, explicitly recorded in the receipt.
- Raw frames and full reconstructed results are retained as `default-inspection-frames.ndjson`,
  `default-inspection-answers.json`, `revision-frames.ndjson` and `revision-answers.json` in that
  private probe directory. This is development evidence, not a sealed final-source campaign.
- The blanket mapper and the local resolve write/retry mapper have now been removed. Native
  errors preserve concrete origin classification; each former caller supplies its operation
  stage. Clippy passed; daemon library tests **130 passed** and retrieval tests **21 passed**
  after the change. Later provider missing-file refinement is being rechecked.
- Nullable root fields must be present even when null. The expanded real CLI/schema/Python
  parity corpus passed **24 cases**.

The 40-crate family probe is in progress. It does not replace performance, real Codex/Claude,
installed-candidate or final J01–J20 qualification. No production service or prior state has
been altered.

### Remaining-scope implementation checkpoint — 2026-09-15

Baseline remains `41ac67219ea39e6b085dde6273c9df08c9937bba`; the existing dirty Plan 13
implementation was preserved. The implementation now additionally includes:

- **W2/W6:** separate native DataFusion **55.1.0** analyzer and optimizer execution with
  default rules preserved; an early query identity survives analysis/planning failures;
  bounded analyzed/optimized/physical diagnostics and stage timings. Independent Arrow
  requirements are applied to coverage, search and comparison-key decoders. Contract defects
  report `internal`; retained-evidence invariant failures retain `corrupt_state`. Witnesses
  cover Utf8, LargeUtf8 and Utf8View with bounded UTF-8 text.
- **W4:** documentation alias folding uses definition, kind, text and the complete native
  source struct. Same text from another definition or producer remains a separate hit.
- **W5:** alternative values use closed inline/artifact variants. Values above 4096 encoded
  bytes are retained as complete JSON artifacts with digest, size and readable handles;
  source provenance remains attached to each alternative. Comparison wording describes
  representations and qualifies absence when either side's scope is incomplete.
- **W8:** recovery verifies the immutable artifact stream while retaining only indexed header
  sections. It neither parses the unrelated data payload nor creates scratch/artifact files.
  The verified prefix is checked against the index used for range selection.
- **W9:** failed optional progress notifications preserve completed tool receipts. Resource
  success/failure logs share a per-read correlation ID without echoing caller URIs. Generated
  Rust/Python schemas and packaged workflow guidance include the alternative value variants.

Executed focused evidence (implementation checks, **not** final journey certification):

| Command / log | Result |
|---|---|
| Store runtime/repository/views; `.dev-state/plan13-refinement-native-tests.log` | 11 runtime, 15 repository, 4 views passed before the subsequent alternative-value variant |
| Store witness unit; `.dev-state/plan13-native-witnesses.log` | 1 passed |
| Daemon retrieval; `.dev-state/plan13-refinement-retrieval.log` | 23 passed with alternative-value delivery |
| Daemon delivery; `.dev-state/plan13-indexed-recovery.log` | 3 passed, including no-scratch read-only recovery |
| Daemon comparison value; `.dev-state/plan13-comparison-values-test.log` | 1 passed; Unicode JSON, nulls, digest, size, identity and repeated delivery |
| Store/daemon all-target Clippy; `.dev-state/plan13-native-refinement-clippy.log` | passed with alternative-value delivery |
| Adapter catalog/socket/progress; `.dev-state/plan13-adapter-refinement.log` | 29 passed before the alternative-value variant |
| Adapter catalog/progress; `.dev-state/plan13-comparison-values-adapter.log` | 14 passed after schema regeneration |
| `just schemas-generate`; `.dev-state/plan13-comparison-values-schemas.log` | passed, including schema conformance and packaged guide generation |
| Focused Ruff/ty | passed; no suppressions |

During implementation, the planning-failure fixture initially failed in the builder before
reaching the runtime; it now deliberately supplies a malformed logical projection and exercises
actual analysis failure. Read-only recovery initially used the scratch-backed capture helper;
the no-scratch regression exposed that mismatch and the verified section-stream reader replaced
it. The stale string comparison against `ResultSectionName` was corrected to the enum variant.

**Still open:** complete independent requirements/operation inventories for remaining query
families; very-large comparison hydration and artifact publication/export closure qualification;
typed revision disposition lowering and additional hostile archive cases; lifecycle/client
qualification; ADR acceptance/living design/refreeze; D01–D12 evidence; installed candidate;
W11 measured execution choices; complete J01–J20, closing review and single production cutover.
The 40-crate acquisition log has finished with 40 partial acquisitions; full family
resolve/overview/search/inspect qualification remains open. No production cutover occurred.

### Architecture-first continuation — 2026-09-15

The user explicitly redirected effort from broad qualification to completing the hard pivot.
The earlier CI and 40-crate extended probe were stopped; no broad qualification was resumed.
The tree was clean at the start of Plan 13 execution, so its accumulated edits remain Plan 13
work. Nothing was reset or deployed to production.

#### Native ownership and result flow now implemented

- **W2:** every foreground RPC and durable job binds a request digest, method and canonical
  effective-policy digest before child work. Opened snapshots attach exact catalog generation,
  context/environment/snapshot IDs, the admitted manifest file digest, projection version and
  real retention leases to the operation. Child diagnostics copy this binding and its budgets.
  Actual optimized-plan inventories include bounded relations, scalar/aggregate/window functions
  and captured UDF field identities, including subqueries. These diagnostic identities are not
  a cache key or permission to execute code.
- **W2/W4:** independent Arrow requirements now cover symbol headers, projected/full API
  observations, relationships, execution observations, documentation/discovery pages, full
  retained relations, catalog attempts/artifacts, static input identities, coverage subjects,
  search payloads, counts, comparison keys and comparison alternatives. The Parquet inspection
  provider and decoder share one named schema projection. Native coercion/default rules remain
  active. `substring` is conservatively nullable in DataFusion 55.1.0; the output requirement
  accepts that declaration and the decoder still rejects an actual missing required value.
- **W5/W8:** comparison consumes Arrow batches through a bounded blocking fold. A selected
  alternative stays an Arrow cell until delivery; at most 4096 encoded bytes become owned inline
  JSON. Larger values stream through `BlobStore::put_stream` with canonical object ordering and
  explicit nested nulls. The previous whole-result Arrow JSON writer and daemon-side late
  promotion were removed. Native equality/order/provenance and the sentinel remain in DataFusion.
  Only artifacts reachable from the fitted final page appear in its outer artifact list.
- **W2/W8:** a blocking batch sink shares the existing query permit instead of acquiring a
  second permit. Its worker retains admission and operation/lease ownership after a caller timeout
  until it actually exits. Streamed artifact bytes have a separate cumulative 32 MiB operation
  bound and check the operation deadline; they are not charged as retained JSON allocations.
  Typed budget origin survives an I/O wrapper.
- **W7:** `RevisionInputs::collect` records filesystem/manifest observations. The daemon lowers
  those typed rows through native unions, component-aware joins and count aggregation before
  creating the extraction receipt. DataFusion now derives affected omissions and the known
  incomplete disposition. Unrelated/sibling paths stay outside the selected domain. Generated,
  submodule and LFS closure remains unknown; transitive/inherited manifest input discovery is
  still a limitation, not a claim of complete build closure.
- **W8:** stored results resolve and verify their indexed artifact dependencies. Catalog
  admission and its retained-file witnesses include that closure. Export carries both dependent
  bytes and their descriptors, making handles readable in a read-only offline bundle without
  producer-input rows. Missing dependency bytes cannot produce a valid recovered/exported result.

#### Evidence and corrections

These are focused implementation checks, not final Plan 13 acceptance:

- Native revision disposition: seven independent path/missing-input cases passed in one test;
  `.dev-state/plan13-revision-native-test.log`. The first `EXISTS` projection did not lower in
  DataFusion 55.1.0; native joins plus aggregation replaced it, with no Rust membership fallback.
- Native comparison fixture passed through the streaming path;
  `.dev-state/plan13-native-delivery-test.log`.
- Repository check: 13 passed, 2 failed due to the overly strong substring nullability requirement;
  `.dev-state/plan13-architecture-native-tests.log`. Both affected tests passed after correction:
  `.dev-state/plan13-family-projection-fix.log` and `.dev-state/plan13-discovery-family-fix.log`.
  The failed aggregate command remains failed; it is not relabeled as a full suite pass.
- Large value sink: passed with escaped JSON above 16 MiB, Unicode, nulls, exact digest/size,
  repeated content addressing and no staging residue;
  `.dev-state/plan13-streamed-value-test.log`.
- Export dependency closure: passed including independent offline reads and missing-dependent-
  value rejection; `.dev-state/plan13-artifact-closure-test.log`.
- Runtime check: 11 passed and the new sink-timeout assertion failed because it assumed only the
  outer timeout could report the deadline. The nested query can report it first. The assertion
  now accepts either typed deadline boundary; admission remains occupied until writer exit.
  Focused corrected test passed: `.dev-state/plan13-arrow-sink-cancellation.log`.
- Affected all-target Clippy passed before the final function-inventory addition;
  `.dev-state/plan13-architecture-final-clippy.log`. Final follow-up checks are recorded below.

Earlier stopped qualification remains visible:

- `.dev-state/plan13-refinement-ci.log`: interrupted Python tier, **12 failed / 148 passed /
  13 deselected**, exit 2 after SIGINT. Failures include execution readiness, resolution,
  provenance and retrieval cases; they are not all assumed to be stale tests.
- `.dev-state/plan13-family-current/`: extended family probe interrupted before completion;
  answers/frames/process logs retained. The earlier 40 acquisitions do not establish the full
  resolve/overview/search/inspect campaign.
- `.dev-state/p4p` execution qualification names an older executor checksum; the rebuilt helper
  correctly invalidates it. Requalify only after architectural edits settle. No receipt was forged.
- `just acceptance-check` reports a source-digest mismatch in the preserved report;
  `.dev-state/plan13-current-acceptance-check.log`. No current gate tally is promoted.

#### Governance correction and remaining boundary

ADRs 0036–0039 are accepted under the **scoped contract** review, not full implementation
certification. ADRs 0029/0030 were superseded through the ADR tool. The new research-v2 bundle is
sealed at `docs/provenance/bundle-2026-09-15-research-v2`; both old and new provenance checks passed
(15/15 each). The original bundle is unchanged and exact predecessor skill bytes remain sealed.
The living design was amended in the governed sections. Historical earlier paragraphs that call
this seal or ADR acceptance open are superseded by this checkpoint.

Still open: finish the remaining fixed-family/source-closure cases and evaluate comparison-job
catalog/export reachability; inspect real integration failures; qualify cleanup/disconnect/restart
ownership; complete D01–D12; build/install one candidate; perform W11 equal-result measurements
before choosing operation-local materialization and endpoint joins; then J01–J20, full family and
Rust/Python canaries, actual Codex/Claude clients, closing review and the single production cutover.
No persistent query-result cache, custom logical node or UDTF framework has been introduced.

Final follow-up for this architectural slice, 2026-09-15:

- Search's declared source role initially failed because native `CASE` drops top-level metadata.
  The projection now explicitly declares the selected qualified-source role using Arrow metadata;
  joins still determine actual provenance. The original four-view run remains 3 passed / 1 failed
  (`.dev-state/plan13-search-family-test.log`); corrected native case passed
  (`.dev-state/plan13-search-source-role.log`).
- The two default/workstation cold-to-offline all-tool journeys initially reproduced that same
  source-role failure (`.dev-state/plan13-operation-integration.log`). Both passed after correction
  (`.dev-state/plan13-operation-integration-fixed.log`). These are local native fixture journeys,
  not real Codex/Claude or the hosted 40-crate campaign.
- Store/daemon all-target Clippy passed with declared catalog schemas, complete search requirements,
  operation binding and native scalar/aggregate/window inventory:
  `.dev-state/plan13-native-contracts-clippy.log`.
- `STATUS.md` now reflects Plan 13 architecture and open qualification; its former Plan 12 source
  and receipt assertions were stale. The living design records actual streaming/ownership behavior.


### Durable comparison and declared source inputs — 2026-09-15

This continuation replaces two remaining architectural gaps rather than promoting the stopped
broad qualification runs.

- `ComparisonPublication` is a derived-result record with request digest and exact before/after
  context/snapshot identities. It has its own typed Arrow catalog relation; no fabricated producer
  attempt or result-as-producer-input is introduced. DataFusion validates unique job identity,
  both snapshot/context references and matching ecosystem/registry/package. Catalog format is /5.
- Comparison result/index and artifact dependency closure are admitted before catalog visibility.
  The catalog publication wins a cancellation racing a successful commit. Restart verifies the
  journal request and both snapshots and reads indexed retained bytes without query/producer replay.
  Terminal journal delivery checks existing artifacts instead of blindly accepting a descriptor.
- Export follows native-selected comparison inputs from each owning after snapshot, bounded to
  64 snapshots, and includes their exact catalog/physical/artifact closure. Bundle format is /5;
  independent verification admits every included input snapshot. Input artifact bytes deduplicate
  across snapshots. No public research wire type or sealed provenance bytes changed here.
- Revision input collection now follows explicit/ancestor workspace roots, inherited readme and
  license files, target-specific/transitive path dependencies, explicit target/build paths and
  selected Python file metadata. Package cycles terminate; unrelated workspace dependency entries
  and arbitrary metadata are not traversed. Collection is bounded to 512 manifests/packages,
  1 MiB per manifest and 16 MiB total. DataFusion joins omitted entries against these source roots
  and declared files. Receipt policy is `revision-extraction/3`; revision normalizer/producer is /4.
  This is a declared candidate input scope across targets/features, not Cargo resolution. Dynamic
  generated inputs, submodules/LFS, external Rust module attributes, patch activation and complete
  build closure remain unproven; there is no `complete` source assertion.
- Blocking producer callbacks now carry the operation into native normalization without holding
  an outer query permit. Finite catalog/result/blob work uses runtime blocking admission; indexed
  result writes reserve the exact encoded artifact size and check deadlines during serialization.
  Artifact output shares the operation's 32 MiB cumulative allowance; streamed input batches remain
  subject to their independent bounds. These are capacity bounds, not process-RSS measurements.
- Retired Python expectations were corrected: expected MCP errors use `raise_on_error=False`,
  resource equivalence follows artifact delivery, comparison additions retain their coverage
  qualification, and the bundle version assertion targets /5. Ruff passed on the four edited files.
  The prior execution-readiness failures still need a source-bound requalification.

Focused receipts:

- `.dev-state/plan13-source-input-test.log`: four archive/revision cases passed, including inherited
  workspace paths, transitive targets, cycles and path escape refusal.
- `.dev-state/plan13-source-roots-native.log`: native disposition test passed across nine omission
  scenarios, including an omitted dependency source file and a lexical sibling outside its root.
- `.dev-state/plan13-comparison-publication-fixed.log`: actual native catalog/publication/export
  case passed. Wrong input ownership is refused by both repository and native catalog validation;
  export includes both inputs and dependent artifacts; removing a dependent artifact fails closure.
  The original `.dev-state/plan13-comparison-publication-test.log` remains failed: moving lease
  initialization before creation of the staging data directory caused ENOENT; ordering was fixed.
- `.dev-state/plan13-comparison-recovery-test.log`: cold real fixture acquisition/comparison passed,
  then simulated the catalog-committed/terminal-journal-absent boundary and recovered offline after
  restart. This receipt predates the final propagation of operation ownership into producer I/O.
- `.dev-state/plan13-publication-ownership-clippy.log`: affected core/store/daemon all-target Clippy
  passed after final ownership/artifact charging changes. The matching recovery rerun is tracked
  in `.dev-state/plan13-operation-publication-recovery.log`; inspect its result before citing it.

W8/J17 now have this concrete comparison path, but complete cancellation/disk-fault/client
qualification and the Plan 13 terminal gates remain open. No full-suite or gate tally is promoted.


### Operation ownership and independent side pages — 2026-09-15

- `.dev-state/plan13-operation-publication-recovery.log` **failed** with a stack overflow after
  publication callbacks gained operation ownership. The task-owned fixture server was stopped.
  `QueryRuntime::blocking` now creates a boxed callback and boxed admission future before polling;
  no thread stack limit was raised. The worker still owns its permit and captured operation.
  `.dev-state/plan13-publication-owned-future.log` passed the same cold comparison/commit-interruption
  recovery case after this correction. Blocking admission now uses the operation's remaining deadline.
- Comparison artifact values are measured from borrowed Arrow before writing. A nonempty side
  page stops before a value that cannot fit its remaining artifact allowance and emits an advancing
  cursor. The operation retains its 32 MiB artifact bound, with 1 MiB reserved for result/index work.
  A minimum nonempty side page that cannot fit still returns a capacity failure; this is not an
  assertion that every pair of individually representable values fits one operation.
- A detail cursor hydrates only the requested side. The other side's native presence is retained
  separately from its values, with an unknown count and explanatory interpretation; no opposite-side
  artifacts are rewritten. All returned continuation pages remain nonempty. An initial implementation
  tried to emit a zero-entry continuation and the existing progress invariant correctly refused it:
  `.dev-state/plan13-independent-alternative-pages.log` remains failed. The corrected implementation
  preserves the invariant and the existing zero-offset cursor rejection.
- `.dev-state/plan13-independent-alternative-pages-fixed.log`: the 97-alternative native traversal
  passed with independent expected sets, source preservation, stable change identity and no repeated
  opposite-side hydration. `.dev-state/plan13-artifact-byte-deferral.log`: complete >16 MiB escaped
  value delivery, exact hashes/content addressing and deferral before writing passed.
- `.dev-state/plan13-architecture-continuation-clippy.log`: affected all-target Clippy passed.
  `.dev-state/plan13-architecture-continuation-removal.log`: `just architecture-check` passed.
  These are scoped source checks, not closure of D01–D12/J01–J20.
- The operation policy digest now covers the full serializable configured policy, including Arrow,
  delivery, freshness and producer settings, instead of only execution/network/profile subsets.
  Configuration origin is excluded. This diagnostic binding remains neither authorization nor a
  persistent cache key. The all-target candidate build with this binding is recorded separately in
  `.dev-state/plan13-candidate-bound-config-build.log`.
- Full-result raw MCP traversal now allows the declared 32 MiB result size using 64-KiB pages and
  a finite 4096-call ceiling; the previous helper's 256 small pages could stop below 1 MiB.
  Named-section retrieval remains the first useful result route; this helper verifies full roundtrip.


### Measured native reuse and qualification generation — 2026-09-15

The continuation followed the user's architectural priority. It added operation-local native
relations to eliminate repeated search ranking, overview child selection and comparison set
reconciliation, with DataFusion 55.1.0 disk/memory ownership and Arrow IPC streaming. Exact count
and page/facet consumers remain native plans. Per-operation metrics capture actual request/config
and snapshot bindings, child work and index bytes/reads; a bounded completion ring avoids global
counter attribution errors under concurrency. Full-fragment and remaining overview/count decoder
boundaries now have independent Arrow field requirements. No dependency pins or sealed wire bytes
changed. [Measurements and limits](../reports/plan13-native-operation-measurements-2026-09-15.md).

Concrete development evidence:

- `.dev-state/plan13-candidate-consumers.log`: 8 actual daemon/adapter consumer journeys passed
  before native reuse, covering raw stdio, resolve outcomes, resource equivalence, Python additions
  and offline export/tamper. The candidate is recorded at `.dev-state/p13candidate-dacbr5ip/`;
  its source fingerprint predates subsequent reuse changes and must be rebuilt before final use.
- Both early execution qualification attempts failed because the private probe sidecar still
  had the prior state generation. Fresh candidate environment variables do not select that
  sidecar. The operator now explicitly chooses `.ENGINE-qualification-state-6`; the prior
  directory is preserved, never adopted/reset. Two path/preservation cases passed in
  `.dev-state/plan13-qualification-generation.log`.
- `.dev-state/plan13-candidate-execution-qualification-generation.log`: real Python/Rust probes,
  7 execution-boundary and 5 cleanup cases passed. Receipt `.dev-state/p4p/admitted-images.json`
  binds the exact executor/controller/images/root/resources. Later store-only changes do not
  change that containment identity; a final candidate still requires identity verification.
- `.dev-state/plan13-search-index-native.log`: native multi-page search preserves independent
  totals/identities. `.dev-state/plan13-operation-index-lifecycle.log`: two replay/empty/failure
  lifecycle cases passed with zero retained native disk/memory reservations after final drop.
- `.dev-state/plan13-owned-operation-and-delivery-measurement.log`: completed manual experiment,
  three rounds with one/eight simultaneous native calls and equal data for known API, relations,
  search and overview. The small relationship fixture is not the large-library fanout gate.
- `.dev-state/plan13-comparison-owned-before.log` and `...-after.log`: the cold comparison/restart
  case passed both times. Native summed work was 3.129 s before and 2.853 s after; the changed-key
  index was 4488 bytes with two reads. Owned job lifetime was 7.358/6.927 s, including acquisition
  waits. These paired development samples do not establish a reliable general speedup.
- `.dev-state/plan13-reused-comparison-alternatives.log`: the existing independent 97-alternative
  traversal passed after changed-key reuse.
- `.dev-state/plan13-operation-index-typed-quota.log`: native quota/empty/successful replay cases
  passed after preserving typed quota witnesses through Arrow/native I/O wrappers.

Failures remain preserved: the first spill prototype referenced a private module; it was replaced
with public disk/IPC/streaming APIs. The overview synthetic-array case exposed an overly strong
child-nullability requirement; it was corrected to the native list contract. The measurement
harness initially used a retired field, then sampled operation history after artifact reads had
displaced the measured operations. It now captures the matching request's observation before
measuring artifact expansion. The native OS quota error is untyped; no error-string classifier
was added. Earlier failed logs remain failed.

W10 source/removal evidence still needs consolidation and one final rebuilt candidate. W11 still
needs large input/high-fanout, complete release/client/first-useful-section and resource measurements.
W12 full campaign, real Codex/Claude, final-source acceptance, closing review and production cutover
remain open. No production daemon, adapter or prior active state was stopped or modified.
