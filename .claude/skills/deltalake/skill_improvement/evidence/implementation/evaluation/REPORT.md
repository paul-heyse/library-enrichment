# Independent Delta reader comparison — 2026-09-18

Both arms produced all 38 responses and finished with all eight required caller tests passing. The candidate is stronger on the partition-order trap and several concrete contract details; the baseline already made most decisions correctly after inspecting exact dependency source. This single run does not establish a general quality or efficiency improvement.

The judge read all responses, emitted sources, initial and final execution logs, relevant exact-pin source, and probe assertions. Labels were visible to the judge. No additional runtime test was executed by the judge. Task IDs, all eight `code_required` mappings, manifests/locks, final test names and result lines were checked programmatically. Detailed judgments and artifact hashes are in [judgment.json](judgment.json).

| Scope | Baseline decisions/contracts | Candidate decisions/contracts | Candidate comparison |
|---|---|---|---|
| 32 paired prompts | 31 supported; E03A incomplete | 32 supported | 4 improvements; 28 ties |
| 6 nominal held-out prompts | 6 supported | 5 supported; H05 incomplete | 1 improvement; 4 ties; 1 regression |
| Required caller tests | 8 passed after 3 attempts | 8 passed after 4 attempts | Different fixture strengths; no blanket integration improvement |

`Supported` means the stated decision/contract is backed within its declared scope; it does not mean every behavior was executed. Thirty non-code responses per arm have no new caller execution. These counts are not a quality percentage or project acceptance tally.

## Consequential findings

- **E03A:** baseline recommends `scan_table().with_columns` generally, but its successful test covers an unpartitioned layout. The retained source computes projection indices before the provider's reordered schema, and the root/frozen probe demonstrates a partition-first request for `id` returning `label`. Candidate selects provider/name projection and checks names/types/values on a partition-first layout. The direct API remains valid where a matching regression test proves it; the oracle does not demand a particular symbol.
- **H05:** candidate correctly separates staging from log visibility, but 'write buffers input' omits early I/O/file finalization on target-size rollover or schema widening. Exact `RecordBatchWriter::write_with_mode` source states this. Baseline describes that effect accurately. This is a contract omission, not a claim that candidate publishes rows prematurely.
- **E05B:** original candidate test proves appended UDF-derived rows. Its RuntimeEnv identity assertion occurs before write, and the source is an embedded batch. That does not independently prove a restrictive caller runtime policy during the operation. Baseline has stronger planner/source controls but likewise does not prove arbitrary policy preservation. Keep any judge-prompted stronger control separate from the original comparison.
- **E11B:** both initially assumed an incorrect physical commit-version representation. Candidate attempt 04 prints `_commit_version: UInt64`; final checked scalar decoding passes exact row/version assertions. Baseline's final display-based decoding also passes. Candidate attempt 03 still failed after an incomplete repair. Preserve these failures; neither arm compiled and ran correctly on its first attempt.

The remaining candidate contract improvements counted here are E07A's tested cast/null behavior, E13B's destructive vacuum defaults, E14A's loaded-snapshot/version-option trap, and H02's qualified schema-normalization results. E08B and E09B are **ties in final answers**: the baseline agent explicitly corrected frozen reader claims about duplicate merge matches and marker-only replay. The candidate's better source material must not be scored as an invented baseline answer failure.

## Caller evidence

[Baseline final log](baseline/cargo-test-03.log) and [candidate original final log](candidate/logs/attempt-04.log) each contain all eight expected tests with zero ignored tests. Both consumers use the same manifest/lock and exact Delta/kernel profile. Baseline additionally tests already planned stale queries, ordinary-planner rejection, restore operation metadata and store pointer identity. Candidate covers the partition-first projection layout. Candidate's original E05B evidence is narrower than its name suggests.

Initial failures are retained: baseline attempted a private LogStoreExt import and redundant provider Arc wrappers, then hit planner/CDF errors; candidate attempted the private session module, then twice failed its CDF version downcast. Final repairs retain the behavioral assertions. Earlier baseline source versions were not fully retained, only their hashes and diagnostic logs; candidate retains each attempt's source.

## Discovery and measured cost

Candidate original responses used the frozen reader without dependency-source fallback. Baseline inspected 21 cached source files and repaired misleading reader guidance. Candidate's inventory covers 35 reader files (518,886 bytes); baseline's covers 39 reader files (294,601 bytes) plus 1,302,281 source bytes. These are whole file sizes, including unshown portions, **not context consumed**. They cannot establish token savings.

Baseline's measured interval is a 555.44-second lower bound; its total elapsed time is unknown. Candidate reports 666.51 seconds from its recorded start. Context tokens and normalized per-task retrieval counts are unmeasured. No total-time/token advantage is claimed. Cargo durations are warm-cache repair costs, not a reader efficiency benchmark.

Original arm tool counters are also retained: baseline manually records 25 exec wrappers, 24 shell calls and 2 polls; candidate reconstructs 26 wrappers, 26 shell calls, 6 patches and 3 polls at the independent-phase boundary. Candidate measures 176,608 output characters after an initial telemetry failure. Different counting/output scopes prevent treating these as a complete normalized retrieval-cost comparison; full candidate command records are in [metrics.json](candidate/metrics.json).

## Held-out and qualification boundaries

H01/H02/H03/H05 overlap worked probes already in the frozen candidate. Their results show transfer from available evidence, not performance on previously unseen capabilities. H04 has catalog guidance but no live-cloud test. H06 provides no actual changed capture; both arms answer the procedure correctly, while changed-capture detection must be judged through separate implementation controls. No repeated stochastic run was performed.

The frozen candidate receipt is distinct from the current root qualification receipt. Subsequent portable source/identity corrections, CDF/writer annotations, expanded root assertions and any judge-prompted E05B follow-up receive no retrospective credit here. Root qualification may resolve these findings under a new candidate identity. This report does not certify cloud authentication, crash durability, concurrent exactly-once delivery, all schemas/features, or workload performance.

**Post-judgment E05B qualification:** the judge inspected the separately retained [follow-up source](candidate/logs/attempt-06-source.rs) and [passing log](candidate/logs/attempt-06.log). A source provider requires a caller-only runtime store during the operation: a separate runtime fails without committing, while the intended runtime lists real Parquet objects and appends the expected UDF row. This closes the concrete session-propagation oracle. The custom source does not certify cloud behavior or every resource policy. Attempt 05's compile failure remains visible; attempt 06 passes all eight tests. The original comparison above is unchanged.

## Per-task comparison

| Task | Baseline | Candidate | Comparison and reason |
|---|---|---|---|
| E01A | supported | supported | tie: Both retain a report-start snapshot and identify retention as a separate requirement. |
| E01B | supported | supported | tie: Both refresh, rebuild and replace registration in the reused context. Both test stale and fresh results; baseline additionally executes a query planned before replacement. |
| E02A | supported | supported | tie: Both select ordinary Parquet for immutable non-Delta files. Baseline supplies an exact read_parquet entrypoint; candidate appropriately limits its plain-Parquet qualification. |
| E02B | supported | supported | tie: Both require Delta log semantics when obsolete files remain. Candidate adds supported deletion-vector/mapping scope without claiming all features work. |
| E03A | incomplete | supported | improvement: Candidate uses provider/name projection and a partition-first fixture. Baseline's unqualified scan_table().with_columns recommendation has a known partition-order counterexample, although its unpartitioned multi-batch fixture passes. |
| E03B | supported | supported | tie: Both choose a provider for relational composition in the existing query environment; neither claims a measured query speedup. |
| E04A | supported | supported | tie: Both preserve concrete state subject to Delta planner compatibility. Neither claims arbitrary caller planners automatically compose. |
| E04B | supported | supported | tie: Both select explicit derive-or-reject policy and distinguish trait-visible runtime/functions from nontransferable catalogs/custom planning semantics. |
| E05A | supported | supported | tie: Both select high-level append for resident batches and distinguish lower-level writer publication control. |
| E05B | supported | supported | tie: Both select logical-plan input with a compatible caller state and execute the expected UDF rows. Candidate's original runtime pointer assertion occurs before write and uses an embedded batch source; it does not independently observe runtime-policy preservation. Baseline adds a distinguishing plain-planner failure and store-backed source, but also lacks an in-operation restrictive resource-policy oracle. |
| E06A | supported | supported | tie: Both select full overwrite, preserve schema-policy distinctions, and separate logical replacement from disk reclamation. |
| E06B | supported | supported | tie: Both tests assert preservation outside the predicate and independently reload after invalid input to confirm no new version/rows. |
| E07A | supported | supported | improvement: Both require fixed-schema policy beyond automatic compatibility. Candidate adds the pin-specific false/error versus true/null cast-safety behavior with appropriately bounded evidence. |
| E07B | supported | supported | tie: Both select additive schema merge while preserving old rows and reject treating it as unlimited coercion. Candidate explicitly bounds tested additions to nullable fields. |
| E08A | supported | supported | tie: Both select qualified merge with explicit update/insert clauses and do not claim persistent uniqueness constraints. |
| E08B | supported | supported | tie: Both require deterministic duplicate/null policy. Baseline repairs its frozen reader's blanket no-error statement through source inspection; candidate cites bounded duplicate-update rejection evidence. Neither incorrectly treats that check as universal uniqueness enforcement. |
| E09A | supported | supported | tie: Both record a transaction marker with the rows and explicitly deny automatic replay suppression. |
| E09B | supported | supported | tie: Both inspect refreshed marker/publication state before replay and state ownership/concurrency limits. Both caller tests demonstrate duplicate rows from unguarded sequential marker reuse. |
| E10A | supported | supported | tie: Both condition correction/retry on a known pre-publication validation failure, independently observed state, and possible orphan files. |
| E10B | supported | supported | tie: Both reconcile durable log state before replay after hook failure and separate repair of hook effects. Candidate correctly narrows local visibility evidence from crash/remote durability. |
| E11A | supported | supported | tie: Both distinguish current snapshot rows from change history and make the refresh boundary explicit. |
| E11B | supported | supported | tie: Both finally execute all four change kinds and exact versions. Both initially assume the wrong physical commit-version array type; candidate final log observes UInt64 and final source decodes it with checked conversion. No initial-success claim is justified. |
| E12A | supported | supported | tie: Both restrict the interval to retained historically enabled CDF and delay downstream checkpointing until successful processing. |
| E12B | supported | supported | tie: Both identify end clamping, optional empty out-of-range start handling, and missing pre-enablement history; neither checkpoints a requested future end. |
| E13A | supported | supported | tie: Both choose compaction for small live files and make speedup conditional on workload measurement. |
| E13B | supported | supported | improvement: Both explicitly preview vacuum with retention. Candidate additionally exposes the consequential dry_run=false and Lite defaults in the answer. |
| E14A | supported | supported | improvement: Both use a separate historical handle. Candidate additionally warns that an already supplied snapshot overrides the provider builder version option at this pin. |
| E14B | supported | supported | tie: Both publish a later restore version and test historical readability. Baseline checks RESTORE operation metadata and metrics; candidate checks rows/version/history length. |
| E15A | supported | supported | tie: Both permit bounded ordinary mapped-column scans while requiring actual profile/mode compatibility; candidate has retained executed fixture evidence. |
| E15B | supported | supported | tie: Both correctly reject mapped-table CDF for this pin independently of recognized feature names or ordinary scan support. |
| E16A | supported | supported | tie: Both distinguish ensuring a missing mapping from replacing a wrong one. Baseline names the public update_datafusion_session helper; candidate leaves helper selection generic but offers valid explicit runtime registration. |
| E16B | supported | supported | tie: Both execute read failure against a wrong store and success after explicit root-store replacement; baseline additionally asserts store identity. Both bound shared-runtime concurrency. |
| H01 | supported | supported | tie: Both choose the provider's exact residual filter or an explicit low-level FilterExec, separating pruning from row correctness. Candidate's worked fixture provides relevant support but this is not unseen-capability generalization. |
| H02 | supported | supported | improvement: Both require nested/null/timestamp value tests rather than assuming lossless conversion. Candidate adds exact observed schema-only normalization boundaries (nanosecond UTC to microseconds; UInt64 to Int64) and explicitly declines value-losslessness claims. |
| H03 | supported | supported | tie: Both choose Full for untracked orphans, explicit keep_versions and preview/retention policy. Baseline explicitly separates log retention; candidate names required retained log/data. Neither claims cloud cleanup safety from local tests. |
| H04 | supported | supported | tie: Both route Glue/Unity identifiers through catalog resolution or Unity DataFusion catalogs, then separately load storage and qualify credentials. Neither reports live cloud success. |
| H05 | supported | incomplete | regression: Both correctly put logical visibility at log publication and distinguish flush from commit. Candidate's 'write buffers input' effect description omits that write can finalize/stage files on target-size rollover or schema widening; baseline explicitly covers this. The frozen visibility probe uses a small batch and does not test early rollover. |
| H06 | supported | supported | tie: Both reopen the alternative comparison after relevant surface changes. Candidate links a concrete invalidation helper but qualifies it as hints. No changed capture is provided to either arm, so only the decision procedure is evaluated. |
