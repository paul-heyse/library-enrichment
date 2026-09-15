# MCP functional campaign: Apache DataFusion 55.1.0 family over research/2.0

Executed 2026-09-15 between 18:13 and 18:32 UTC (14:13 to 14:32 EDT) against the Plan 13
production generation. Every functional assertion below came from a call to one of the nine
`library-enrichment` MCP tools or from an MCP resource read, made from a Claude Code session
whose working directory was this checkout with the development environment active. The only
out-of-band actions were environment control and observation, each labelled where it appears:
stopping and starting the daemon unit, digesting a scratch repository and the inactive old state
root, reading `git ls-remote` for commit hashes, hashing one decoded artifact slice, and one `df`
reading.

**This is not an acceptance-gate run.** Nothing in `tests/gates.toml`,
`docs/reports/acceptance.json` or `STATUS.md` was changed on the basis of this campaign, and
none of the results map onto gate identifiers. Results use the four states from `AGENTS.md`
only: `passed`, `failed`, `blocked`, `not_run`. The assertion rule is the one from the
2026-09-14 campaign, amended for the new contract: a `partial` answer with named assessments and
limitations is a pass; artifact delivery is a representation, never a failure; a typed refusal
passes only when its diagnostic cause, stage, retryable flag and first recovery action fit the
situation. A caller mistake reported as operator work or as a defect to report, or a transient
condition marked non-retryable, is a failed row with a finding.

This campaign builds on [the 2026-09-14 campaign](mcp-functional-datafusion-2026-09-14.md):
its row identifiers are reused so the two reports line up, rows added for research/2.0 take the
next number in their group, and G12 to G14 are new groups.

## Tally

| State | Count | Identifiers |
|---|---|---|
| passed | 114 | everything not listed below |
| failed | 7 | G1.3 (F9), G3.12 (F7), G4.15 (F10), G7.9 (F6), G10.2 and G10.6 (F5), G11.3 (F8) |
| blocked | 0 | |
| not_run | 4 | G5.7 (no multi-page alternative reached), G7.2 (cancel while running not reproducible from this client), G12.3 (adapter timeout not induced), G13.5 (progress not surfaced by this client) |

One hundred and twenty-five identifiers in total: P1.1 to P1.4 and groups G1 to G14 from the plan
at `~/.claude/plans/mutable-questing-parasol.md`.

## Deployment under test

| Item | Value |
|---|---|
| Installation | `/home/paul/.local/opt/library-enrichment/plan13-3ae079f9718a` (manifest `candidate.json`, source digest `3ae079f9718a704c2c6ee931ea24d8ae1874a43c86d15bc82f7159db8e70fd43`) |
| State root | `~/.local/state/library-enrichment-v2`, format `state/6`, fresh at the 13:49:55 UTC cutover |
| Socket | `~/.local/state/library-enrichment-v2/run/d.sock` |
| Service configuration | `~/.config/library-enrichment/research-v2-3ae079f9718a.toml`, sha256 `2b8ec51e6a07b0ff71dac3ed0648e356bc08192ec8eedb88d2557c00c3c47ae8`; no `[limits]` override, so `inline_result_bytes` 12288, `max_job_wait_seconds` 10, `expensive_worker_concurrency` 8 |
| Supervision | systemd user unit `library-enrichment.service`, `Restart=on-failure` |
| Execution profiles | `static` only; no producer image qualified |
| Registration scope | Claude Code user scope, top-level `mcpServers` |
| `library-enrichmentd` | sha256 `f64f58fa9128208e62bbee164eb203e0c54c37a32e2b92b729a3fb591a736d57` |
| `library-enrichment-executor` | sha256 `67fa43bb7233d91233e0f6c1c0ac9a7382c3a84fade17db19a78ebb1f3904c7e` |
| `library-enrichment-native-worker` | sha256 `85b27114cf2c72f4aae184513e1bfaef69038ccff0dec337aa76cc96272be222` |

The registration launches
`/home/paul/.local/bin/uv run --frozen --no-sync --project <installation> --python <installation>/.venv/bin/python python -I -B -m enrichment_mcp`
with `LIBENR_HOME`, `LIBENR_CACHE_HOME`, `LIBENR_DATA_HOME`, `LIBENR_SOCKET` and `LIBENR_CONFIG`
pinned to the paths above, plus `UV_PROJECT_ENVIRONMENT` and `UV_NO_SYNC=1`. The unit file,
verbatim, from `.dev-state/plan13-cutover/library-enrichment.service`:

```ini
[Unit]
Description=library-enrichment research/2.0 evidence daemon
After=default.target

[Service]
Type=simple
ExecStart=/home/paul/.local/opt/library-enrichment/plan13-3ae079f9718a/target/release/library-enrichmentd start
Environment=LIBENR_HOME=/home/paul/.local/state/library-enrichment-v2
Environment=LIBENR_CACHE_HOME=/home/paul/.local/state/library-enrichment-v2/cache
Environment=LIBENR_DATA_HOME=/home/paul/.local/state/library-enrichment-v2/data
Environment=LIBENR_SOCKET=/home/paul/.local/state/library-enrichment-v2/run/d.sock
Environment=LIBENR_CONFIG=/home/paul/.config/library-enrichment/research-v2-3ae079f9718a.toml
Environment=UV_PROJECT_ENVIRONMENT=/home/paul/.local/opt/library-enrichment/plan13-3ae079f9718a/.venv
Environment=UV_NO_SYNC=1
Restart=on-failure
RestartSec=2

[Install]
WantedBy=default.target
```

Verified 2026-09-15 before the campaign: the deployed binaries were built at 08:28 to 08:30 EDT
and HEAD `a78c741` was committed at 12:40 EDT, but every file modified after the build is a test
or a harness script, and the adapter modules in the installation match the checkout byte for
byte. The service has no build identity on the wire: `service_status.versions.daemon` is `0.0.0`
(observation O10), so evidence is bound to the build through the installation path and the three
binary digests above.

The client session that planned the campaign had started before the cutover and could not reach
the new socket until the operator ran `/mcp` and reconnected. That is a client fact, not a service
defect: MCP registrations are read once at client start.

## Delta from 2026-09-14

| Prior row | Prior verdict | New verdict | Note |
|---|---|---|---|
| P1.7 | passed | passed (P1.1 to P1.4) | catalog still nine tools; the plan's old `daemon.available` expectation is gone from the plan |
| G1.1 to G1.9 | passed | passed | context ids are identical across generations (O25); every crate again at format 61 |
| G2.1 to G2.6 | passed | passed | G2.6 now a typed `BUDGET_EXCEEDED` with a delivery rule, not a `result_artifact_id` |
| G3.1 to G3.8 | passed | passed | G3.4 changed by design: `kinds=["source"]` is now `UNSUPPORTED_CAPABILITY`, not `partial` |
| G4.1, G4.2, G4.4 | failed (F1) | passed | default inspection returns signature, availability and a documentation preview on every high-fanout symbol |
| G4.3, G4.5 to G4.7 | passed | passed | |
| G5.1 to G5.6 | passed | passed | every comparison is now `partial` because the docs.rs confounders are always disclosed; F3 fixed in G5.2 |
| G6.1 to G6.4 | passed | passed | slices now reach 9 to 10 KB of content within the 12288 cap; `max_bytes` is disclosed as requested versus effective |
| G7.1 to G7.4 | passed | passed (G7.4 F2 fixed) | G7.2 now `not_run`: cancellation is real and the jobs finished before a cancel could reach them |
| G8.1 to G8.3 | passed | passed | O2 fixed: refusals name the concrete setup commands |
| G9.1 to G9.4 | passed | passed | O8 (fetch hits 0) still present |
| G10.1, G10.2 | failed (F4) | G10.1 passed, G10.2 failed (F5) | the symlink no longer aborts acquisition; the workspace root without a package table is now a different, misclassified failure |
| G10.3 | not_run | passed | offline reuse of the revision snapshot works |
| G11.1 to G11.3 | passed | G11.1, G11.2 passed; G11.3 failed (F8) | the degraded and recovered shapes are right; the stopped daemon is reported non-retryable |

## Findings F1 to F4 from 2026-09-14

| Finding | Status | Evidence |
|---|---|---|
| F1 default `inspect_symbol` hit `BUDGET_EXCEEDED` on high-fanout symbols | fixed | `SessionContext` (`req_18c369c7bcfb4741bf7703ed9c60b885`), `DataFrame` (`req_ebda111b0b7f419fa69f4d274d072b51`), `Expr` in its home crate (`req_b25a8aa3db9b41ef962d4ef3b05d0f9c`) all `ok` with three aspects; explicit relationships paged 32, 32, 32, 21 to exactly 117 with `has_more: false` |
| F2 unknown `job_id` reported as retryable I/O failure | fixed on cause, action and retryable; message unchanged | `req_22a20fb275244a76a950f34086abef13`: `cause: not_found`, `stage: job_lookup`, `retryable: false`, next action "Use the job_id returned by the original submission. An unknown ID is not a storage-permission failure." The message is still the raw OS text (O11) |
| F3 silent `release_notes` scope | fixed | `req_b5d4c6cebe0349559d6443cc9700a5bc`: `partial`, both sides assess `release_notes` missing with witnesses, confounder "Requested evidence scopes are incomplete…", 24 docs changes |
| F4 symlink aborted revision acquisition | fixed | `job_b7e8cf4a9ec3479cb936b57c320abe52`: `partial`, 22 fragments, 0 compiled definitions, `crate_source` partial, closure-incomplete limitation naming the LICENSE and NOTICE links and the receipt `art_923e9311f7f16955c02416a2af063358`, whose omissions list the `CLAUDE.md` symlink to `AGENTS.md` |

## Observations O1 to O9 from 2026-09-14

| Observation | Status |
|---|---|
| O1 fragment hits not folded across re-export paths | fixed: `SessionState::window_functions` returned once with two `also_at` paths (`req_0cd6b7a13a1b467e9762279c9abbe25c`) |
| O2 `verify_usage` denial did not name setup commands | fixed: three `operator_setup` reasons name `just execution-images --apply` and `just execution-qualify --apply` |
| O3 struct namespaces list no children in overview | still present (`req_5f65f79359b847169d64d5891cd836b6`); members are reachable through the `members` aspect |
| O4 rendering change surfaced as potentially breaking | changed: each such change now carries "producer rendering, including Infallible versus never-type (!), can differ without a source-level compatibility change" (`art_0581d863dbe673cab58be58b20fabd48`) |
| O5 `ok` on first acquisition, `partial` when retained | fixed: both are `partial` with the same typed gaps |
| O6 eight concurrent resolves mostly went `pending` | changed: most crates now finish inside the 2 s inline wait; only the larger ones go `pending` |
| O7 source excerpt over-approximates the item | still present, now labelled `recorded_line_window` with `truncated: true` (`req_aa4a4fb437ce4e6fbaaf398f8000797c`, lines 1353 to 1432 for a seven-line method) |
| O8 `health.fetch.hits` stayed 0 | still present (hits 0 after 207 misses and 24 revalidations) |
| O9 documentation aspect on a doc-less item silently dropped | fixed: the aspect is reported `absent` or `unavailable` with a reason |

The two unnumbered observations from the previous report also changed: `job_control` no longer
needs paging at all because terminal results are compact, and `read_artifact` slices are bounded
by the disclosed cap rather than by a halving fit.

## New findings

**F5: caller-input mistakes in revision mode are classified as internal defects, after the
acquisition has been paid for.** Omitting `package_subdir` on the DataFusion workspace root
(`req_2854ca68819f488387578b40934955b2`), pointing it at a directory without a manifest
(`req_f3b66f1bb10a43af964d803638d7b350`, `req_b21a3430e2d7433191651d542fb0cdca`) and the same
on an uncached commit (`job_0934a34945c149ccb2065fcdde1dc78d`) all return `EXTRACTION_FAILED`
with `cause: internal` and a `report_defect` action, although the messages themselves say
exactly what the caller should change. The archive is downloaded and extracted first.

**F6: cancellation mistakes are diagnosed as operator or storage problems.** A cancel without
an interest token is `POLICY_DENIED` whose next action speaks of "your verification submission"
(`req_50ae266199a54732b5edb13863e34450`); a cancel with another job's token is `QUERY_FAILED`,
`cause: permission_denied`, with the action "Inspect service_status and the correlated native
failure before repairing storage or permissions" (`req_e7e5755a8f1a4ea89b5eddf11841ac41`). Both
are caller mistakes of the F2 shape.

**F7: a malformed `area` is reported as a defect to report.** `area="datafusion::"` returns
`QUERY_FAILED`, `cause: invalid_plan`, `stage: native_query`, "public path has empty,
control-bearing or excessive content", with a `report_defect` action
(`req_d9f198252ad74202bbd2528a2015cd69`).

**F8: a stopped daemon is reported as non-retryable.** With the unit stopped, every tool returned
`UPSTREAM_UNAVAILABLE`, `cause: transport`, `stage: daemon_transport`, `retryable: false`, next
action "Start the configured daemon with library-enrichmentd start." The 2026-09-14 generation
marked the same condition retryable, and it is: the identical requests succeeded seconds later.

**F9: `freshness="revalidate"` on a retained upstream-mode context republishes a snapshot with
duplicated evidence.** After `datafusion` unversioned resolved to `snap_082560ab2fea224f` with
2383 fragments (`req_278da576360542bfbe77029b1c2c2912`), the revalidate
(`req_d4165852f43744808bd2135b465e8a67`) published `snap_f02eb7db4d24c9e9` with 4766 fragments.
Its manifest (`req_ff6969bae4b64cae800ef74531ed9c10`) shows every evidence table doubled:
`api_observations` 2904 (was 1452), `relationships` 4502 (was 2251), `unresolved_reexports` 194
(was 97), `producer_runs` 8, `coverage` 16 rows, `release_metadata` 2 rows. A docs search on that
context returns 144 matches instead of 72 and lists the same fragment twice under different
evidence ids (`req_91d5d39c5def4a4bad7a8abbe1f0f82c`). The definitions and symbols tables are
not doubled. The version-pinned re-acquisition of `datafusion-expr` through its underscore
spelling did not reproduce this: it republished the same snapshot id with unchanged counts. This
is the most consequential finding of the campaign, because an agent following the skill's advice
to revalidate on "latest" questions would then search duplicated evidence.

**F10: a bare ambiguous name is reported as a defect to report.** `symbol_path="new"` on the
facade returns `QUERY_FAILED`, `cause: invalid_plan`, with details omitted and only
`action_kinds: ["report_defect"]` (`req_676a47f6edc6410e89e7ccaa96a897ff`). The schema admits a
bare name when unambiguous; an ambiguous one should return candidates or a bounded capacity
refusal.

## Results by group

Inputs are abbreviated; every request, job and artifact identifier is in the ledger named under
"Artifacts and reproduction". `ctx_fe173d42474fdf0b` and `snap_69c466e7240e78b5` are the
`datafusion` 55.1.0 context and snapshot from the cutover smoke unless stated otherwise.

### P1: catalog and identity

| ID | Observed | Verdict |
|---|---|---|
| P1.1 | nine tools; input schemas carry `selection`, `discovery` (at most four), `section` object, `alternative_cursor`, `wait_seconds` at most 10; no `depth`, `aspects`, `revalidate` or `run`; only `library-evidence://workflow` is listed as a resource | passed |
| P1.2 | `versions.daemon` `0.0.0`, schema emits 2.0 and accepts only 2.0, snapshot 6.0, v2 data root, `static` only, `execution_qualified: false`, four execution routes each with three `operator_setup` actions naming the commands | passed |
| P1.3 | `component="rustdoc-json"` narrows producers; an unknown component is `partial` with `missing` naming the filters and a limitation that absence is not proof | passed |
| P1.4 | baseline recorded (fetch misses 10, evidence requests 20, single-flight started 3) | passed |

### G1: identity and resolution (`resolve_library`)

| ID | Inputs | Observed | Verdict |
|---|---|---|---|
| G1.1 | `datafusion-expr` 55.1.0, cold | `pending` with job and interest token; `wait` gave `state: partial`, a compact result with `outcome: partial` and an artifact delivery whose effective cap is 1048576; `ctx_696229314162499e`, `snap_51056a6d7b913c63`, format 61 | passed |
| G1.2 | `datafusion`, no version | `mode: upstream`, 55.1.0, `latest_verified: true`, `ctx_1e4e7704d42d54b1` | passed |
| G1.3 | `revalidate` on that context | registry re-checked; new snapshot with doubled evidence (F9) | failed |
| G1.4 | `offline` on never-fetched `sqlparser` 0.58.0, daemon idle | `ARTIFACT_UNAVAILABLE`, `not_found`, "offline forbids acquisition", next action names `cache_ok`; fetch counters identical before and after | passed |
| G1.5 | `features=[parquet,avro]`, `default_features=false` | `ctx_60a23d6872250d0e`, environment `env_e6d8474a58b049da`; observed all-features build kept apart | passed |
| G1.6 | `datafusion_expr`; `datafusion-nonexistent-x` | the underscore spelling resolved `datafusion-expr` but re-acquired and republished the same snapshot (O19); the unknown crate is `VERSION_NOT_FOUND` naming both spellings checked | passed |
| G1.7 | `datafusion` 99.0.0 | `VERSION_NOT_FOUND` naming 54.1.0, 55.0.0, 55.1.0; finished inside the inline wait so no job id was disclosed | passed |
| G1.8 | `datafusion-cli` | 235 definitions, examples indexed | passed |
| G1.9 | all 40 crates | table below; 40 of 40 at format 61, no fetch failures | passed |
| G1.10 | `datafusion` 55.1.0 warm, first resolve of the campaign | `answered_from_cache: true`, the cutover ids | passed |
| G1.11 | `allow_prerelease=true` unversioned | still 55.1.0 | passed |
| G1.12 | old `revalidate=true` | adapter `UNSUPPORTED_FORMAT` "additionalProperties" | passed |

### G2: discovery (`library_overview`)

| ID | Inputs | Observed | Verdict |
|---|---|---|---|
| G2.1 | root, default discovery | artifact delivery (77909 bytes) with `coverage` and `data` sections; 101 namespaces, 15 discovery fragments | passed |
| G2.2 | `area=datafusion::dataframe` | 73 definitions, three whole namespaces | passed |
| G2.3 | `datafusion-doc` | 66 definitions, seven namespaces, nothing truncated | passed |
| G2.4 | `datafusion-macros` | one `proc_macro` | passed |
| G2.5 | unresolved re-exports | 97 with the limitation | passed |
| G2.6 | `max_bytes=1024` (and 2048) | `BUDGET_EXCEEDED`, `capacity`, `result_delivery` | passed |
| G2.7 | `discovery=[]` | no facets | passed |
| G2.8 | documentation facet preview, then its cursor | `text_complete: false`, `complete` action carrying the cursor | passed |
| G2.9 | `release_notes` and `examples` facets | `unavailable` with reason, page returned 0 | passed |
| G2.10 | O3 | struct namespaces still empty | passed |
| G2.11 | `max_bytes=4096` | artifact delivery with `effective_max_bytes` 4096 | passed |
| G2.12 | facet cursor under another kind | facet `state: failed` with an `invalid_input` diagnostic and a `change_request` | passed |
| G2.13 | `max_items:0` | core `UNSUPPORTED_FORMAT`; the schema admits 0 (O13) | passed |
| G2.14 | five entries | adapter refusal at `discovery` | passed |

### G3: search (`search_evidence`)

| ID | Inputs | Observed | Verdict |
|---|---|---|---|
| G3.1 | "register a parquet table" | 448 exact matches, four returned, `also_at` populated | passed |
| G3.2 | `kinds=[api]` "ScalarUDF" | 13 exact | passed |
| G3.3 | `kinds=[docs,examples]` "window function" | 72 exact | passed |
| G3.4 | `kinds=[source]` | `UNSUPPORTED_CAPABILITY` naming the `source` aspect (contract change) | passed |
| G3.5 | three per page to exhaustion | 3, 3, 3, 3, 1; count 13 on every page | passed |
| G3.6 | cursor with changed query | `INVALID_CURSOR` | passed |
| G3.7 | definition folding | once per page with `also_at` | passed |
| G3.8 | "kafka" | count exact 0, `public_api` indexed, absence caveat | passed |
| G3.9 | `area` restriction | 526 matches under `datafusion::execution`; zero under the re-export-only `datafusion::physical_plan` (O21) | passed |
| G3.10 | O1 | folded | passed |
| G3.11 | `max_bytes=4096` | artifact delivery, not a shorter page | passed |
| G3.12 | `area="datafusion::"` | `QUERY_FAILED` with `report_defect` (F7) | failed |
| G3.13 | `max_items=100` | four returned, bounded by bytes (O22) | passed |

### G4: symbol depth (`inspect_symbol`)

| ID | Inputs | Observed | Verdict |
|---|---|---|---|
| G4.1 | `SessionContext`, default | `ok`; signature, availability, documentation preview with a `complete` action | passed |
| G4.2 | `DataFrame`, default | `ok` | passed |
| G4.3 | `with_query_planner`, `source` | `recorded_line_window` 1353 to 1432, `truncated: true` (O7) | passed |
| G4.4 | `Expr` in `datafusion-expr`, default | `ok` | passed |
| G4.5 | ambiguous `register_udf`, then `definition_id` | two candidates; selection kept the `FunctionRegistry` qualifier | passed |
| G4.6 | bogus path | `ARTIFACT_UNAVAILABLE`, `not_found`, absence caveat | passed |
| G4.7 | `semantics` with `execute_on_miss`/`build`; then `retained` | `POLICY_DENIED` at `execution_readiness`; retained gives `unavailable` with reason | passed |
| G4.8 | relationships to exhaustion | 117 across four pages, `count.kind` unknown throughout (O15) | passed |
| G4.9 | `members` and `children` | both paged | passed |
| G4.10 | documentation 300 characters | preview and complete action | passed |
| G4.11 | `examples` aspect; examples search on `datafusion-cli` | `unavailable` with reason (O9 fixed); one example fragment found on the CLI crate | passed |
| G4.12 | old `depth` | adapter `UNSUPPORTED_FORMAT` | passed |
| G4.13 | cursor with changed limits; snapshot of another context | `INVALID_CURSOR`; `ARTIFACT_UNAVAILABLE` "Snapshot belongs to a different context" | passed |
| G4.14 | `ParquetReadOptions` availability with the feature context | requested `[avro, parquet]`, default features off, versus the observed all-features build; `project_availability_unverified` | passed |
| G4.15 | bare `new` | `QUERY_FAILED` with details omitted (F10) | failed |
| G4.16 | mismatched `definition_id` | `ARTIFACT_UNAVAILABLE` with the path guidance; search hits carry no `definition_id` (O12) | passed |
| G4.17 | `execute_on_miss` without profile | `UNSUPPORTED_FORMAT` naming the build profile | passed |
| G4.18 | duplicate aspects | `UNSUPPORTED_FORMAT` | passed |

### G5: change analysis (`compare_releases`)

| ID | Inputs | Observed | Verdict |
|---|---|---|---|
| G5.1 | 54.1.0 to 55.1.0, default scopes | `partial`, 183 changes, `api_complete: true`, artifact with a `changes` section | passed |
| G5.2 | `release_notes`, `docs` | F3 fixed | passed |
| G5.3 | `datafusion-common` 55.0.0 to 55.1.0 | four changes; the two `Err` rendering changes carry the Infallible/never caveat | passed |
| G5.4 | pinned same release | `same_release: true`, zero changes, confounders still listed | passed |
| G5.5 | both input forms | `UNSUPPORTED_FORMAT` | passed |
| G5.6 | 53.0.0 to 55.1.0, five per page, version form once | 376 changes; observed format 57 versus 61 | passed |
| G5.7 | alternatives paging | no multi-page alternative reached | not_run |
| G5.8 | artifact-valued alternative | digest and size match the declaration | passed |
| G5.9 | `configuration` scope | zero changes, difference carried in `configuration_differences` | passed |
| G5.10 | `relationships` on `datafusion-expr` 54.1.0 to 55.1.0 | 311 changes, no budget error | passed |
| G5.11 | `changes` section read | paged with digests | passed |
| G5.12 | version-form cursor into the pinned form | accepted, no new job | passed |
| G5.13 | both cursors together | `UNSUPPORTED_FORMAT` "cursors are independent" | passed |
| G5.14 | consistency of `api_complete` | consistent | passed |

### G6: bounded output (`read_artifact`)

| ID | Inputs | Observed | Verdict |
|---|---|---|---|
| G6.1 | `delivery.read` to completion | single-page artifacts match their sha256; the 13186-byte comparison result read in two contiguous slices to `remaining: 0` | passed |
| G6.2 | markdown heading; missing heading | slice returned; `ARTIFACT_UNAVAILABLE` without a heading list (O16) | passed |
| G6.3 | `max_bytes=1024` | `BUDGET_EXCEEDED` | passed |
| G6.4 | unknown id | "not stored on this service" with a misleading next action (O14) | passed |
| G6.5 | every advertised section; a non-advertised one | readable; typed refusal naming `delivery.sections` | passed |
| G6.6 | utf8 boundaries | no split code point | passed |
| G6.7 | `max_bytes=65536` | requested 65536, effective 12288 | passed |
| G6.8 | wrong-kind sections and old-generation ids | each refused with its own typed reason | passed |
| G6.9 | tarball slice | base64, digest of the decoded bytes matches | passed |

### G7: jobs (`job_control`)

| ID | Inputs | Observed | Verdict |
|---|---|---|---|
| G7.1 | normal path | `state: partial`, compact result | passed |
| G7.2 | cancel while running | job finished first; a later cancel only decremented the interest count (O28) | not_run |
| G7.3 | two identical `datafusion-proto-models` resolves | second joined the running job with `active_interests: 2` and the shared-run limitation; `single_flight.shared` 0 to 1 | passed |
| G7.4 | unknown job id | F2 fixed; raw OS message remains (O11) | passed |
| G7.5 | `not-a-job` | `not_found` "invalid service job identity" | passed |
| G7.6 | `wait_seconds=60` | adapter refusal at `wait_seconds` | passed |
| G7.7 | `max_bytes=1024` | artifact delivery at 1024 rather than a refusal (O23) | passed |
| G7.8 | failed job | `EXTRACTION_FAILED` with the job id and a read action; the stored terminal result carries the diagnostic with the job id as correlation id | passed |
| G7.9 | cancel without or with a foreign token | F6 | failed |

### G8: policy boundary (`verify_usage`)

| ID | Inputs | Observed | Verdict |
|---|---|---|---|
| G8.1 | `typecheck` / `build` | `POLICY_DENIED` at `execution_readiness` naming the setup commands | passed |
| G8.2 | `runtime` / `runtime` | same shape | passed |
| G8.3 | `allow_local_build=true` | retained answer, no build, `lsp.started` 0 | passed |
| G8.4 | `profile: static`; `compile` / `runtime` | adapter enum refusal; `POLICY_DENIED` "requires locally enabled build policy" | passed |
| G8.5 | old `mode=run` | adapter enum refusal | passed |

### G9: caching, freshness, reproducibility

| ID | Observed | Verdict |
|---|---|---|
| G9.1 | `answered_from_cache: true`, same ids | passed |
| G9.2 | pinned and unpinned reads identical | passed |
| G9.3 | offline after warm identical | passed |
| G9.4 | misses 207, revalidated 24, failures 0, hits 0 (O8), 49029887 bytes fetched for the whole campaign before the restart | passed |
| G9.5 | O5 fixed | passed |
| G9.6 | repeated unversioned `cache_ok` re-checked the registry inside the TTL and kept the snapshot id (O20) | passed |

### G10: revision mode

| ID | Inputs | Observed | Verdict |
|---|---|---|---|
| G10.1 | monorepo commit, `datafusion/core` | F4 fixed; `ctx_ce90d67f813fcc78`, `snap_5aaa24da2434ef6f` | passed |
| G10.2 | no `package_subdir` | `EXTRACTION_FAILED`, `cause: internal` (F5) | failed |
| G10.3 | offline | same snapshot | passed |
| G10.4 | search on the revision context | one README fragment with the commit permalink; `examples` and `release_notes` assessed `unknown` | passed |
| G10.5 | short SHA, non-canonical URL, `..` subdir, version with revision, repository without mode | all `VERSION_NOT_FOUND` before any acquisition (O18) | passed |
| G10.6 | subdir without a manifest, on three repositories | `EXTRACTION_FAILED`, `cause: internal` (F5); the uncached HEAD variant went `pending` and supplied the failed-job rows | failed |

### G11: cross-repository reachability, isolation, degradation

| ID | Observed | Verdict |
|---|---|---|
| G11.1 | three headless children (home directory, scratch repository, this checkout with the dev environment inherited) each saw nine tools, the v2 data root and the identical `ctx_09f8b3abe5f6b74b` and `snap_0001ef71316929c3` | passed |
| G11.2 | canary digest identical before and after, `git status --porcelain --ignored` empty (out-of-band) | passed |
| G11.3 | stopped: `service_status` `partial` with `daemon`, `producers`, `cache` missing and the start command; every other tool `UPSTREAM_UNAVAILABLE` at `daemon_transport`; the workflow resource still served. Restarted: `ok` at two seconds of uptime, counters reset, retained evidence, the job journal, a stored artifact and a pre-restart search cursor all served. Failed only on `retryable: false` (F8) | failed |
| G11.4 | old state root digest `585d7391f7ed35a20c1eb4781d7f5c168b6a3fd015a48405fe984a53aec9c060` over 2059 files and the old `service.toml` sha256 unchanged (out-of-band) | passed |

### G12: error presentation through this client

| ID | Observed | Verdict |
|---|---|---|
| G12.1 | the failed job's preview carried code, cause, stage, job id, read action, message and next action | passed |
| G12.2 | unknown field and a boolean for an integer refused at the adapter with the field named; no job created | passed |
| G12.3 | not induced | not_run |
| G12.4 | 65 terms refused by the daemon with the 64-term, 65536-byte limit; the megabyte variant not attempted | passed |

### G13: MCP resources

| ID | Observed | Verdict |
|---|---|---|
| G13.1 | only the workflow resource is listed; the templates are not surfaced but resolve by URI | passed |
| G13.2 | shipped contract text with a prepended "Registered tools" line (O27) | passed |
| G13.3 | artifact resource returns the same validated envelope as `read_artifact` | passed |
| G13.4 | manifest of the cutover snapshot: `is_current: true` before and after the campaign's republishing rows, catalog generation 4 to 23, tables with sha256 and row counts | passed |
| G13.5 | progress not surfaced | not_run |

### G14: dependency family extension

| ID | Observed | Verdict |
|---|---|---|
| G14.1 | `arrow` 59.3.0 (208 definitions) and `parquet` 59.3.0 (1980 definitions, examples indexed) resolved from hosted JSON at format 61 | passed |
| G14.2 | `datafusion::arrow` inspects as an import whose definition path and defining crate are `arrow`; the facade still reports 97 unresolved re-exports, so resolving the dependency does not cross-link snapshots | passed |

## Breadth sweep

All 40 crates at 55.1.0, resolved through `resolve_library` in batches of at most eight. Every
row: hosted rustdoc JSON `available` at format 61; `crate_source`, `documentation`,
`documentation_build_config`, `hosted_rustdoc_json`, `public_api` and `registry_metadata`
indexed; `release_notes` missing; `examples` missing unless noted. Definition counts equal the
2026-09-14 table and the Plan 13 live table; context ids equal the 2026-09-14 ids.

| Crate | Context | Snapshot | Definitions | Fragments | Notes |
|---|---|---|---:|---:|---|
| datafusion | `ctx_fe173d42474fdf0b` | `snap_69c466e7240e78b5` | 732 | 2383 | warm from the cutover smoke |
| datafusion-catalog | `ctx_66874c8875aa04d3` | `snap_6c19f0b815c4eb96` | 205 | 320 | |
| datafusion-catalog-listing | `ctx_5beca3efd7ef973e` | `snap_36f08f3b305a32b8` | 81 | 126 | |
| datafusion-cli | `ctx_74207232f4f575cd` | `snap_3b865009e99314be` | 235 | 257 | examples indexed |
| datafusion-common | `ctx_fa55a98da006f347` | `snap_502ac963136ef764` | 2096 | 3440 | |
| datafusion-common-runtime | `ctx_5962eb882c773016` | `snap_64927378547f94c9` | 43 | 87 | |
| datafusion-datasource | `ctx_7c23874bdd6f2a01` | `snap_812ec71282d61643` | 594 | 1015 | |
| datafusion-datasource-arrow | `ctx_de2f52a37b8eef28` | `snap_75f4f8c5cc0e8a5c` | 42 | 70 | |
| datafusion-datasource-avro | `ctx_b411f1ebf726a75f` | `snap_b9694578512047d7` | 35 | 60 | |
| datafusion-datasource-csv | `ctx_5e94639ab9871be9` | `snap_317ec5684f43ab56` | 104 | 250 | |
| datafusion-datasource-json | `ctx_c52cd4f25ebaec35` | `snap_407856dc7e849d03` | 82 | 177 | |
| datafusion-datasource-parquet | `ctx_2679c293135b4447` | `snap_0886bbb1dfc37c08` | 257 | 543 | |
| datafusion-doc | `ctx_09f8b3abe5f6b74b` | `snap_0001ef71316929c3` | 66 | 79 | complete overview; the G11 crate |
| datafusion-execution | `ctx_37ccc940653836f3` | `snap_db16ae7e9fe5cb12` | 541 | 889 | |
| datafusion-expr | `ctx_696229314162499e` | `snap_51056a6d7b913c63` | 2483 | 5385 | 10697 relationships |
| datafusion-expr-common | `ctx_663d9b0d8fc4dece` | `snap_ed175365ee097f3e` | 495 | 732 | |
| datafusion-ffi | `ctx_d984f13c7a92a155` | `snap_e0750a560ce4d62d` | 830 | 1040 | |
| datafusion-functions | `ctx_e305958021521179` | `snap_550756448f943ea6` | 1717 | 2047 | |
| datafusion-functions-aggregate | `ctx_f861fa3b1c0a5f6c` | `snap_9c5ad76f34f3f825` | 787 | 913 | |
| datafusion-functions-aggregate-common | `ctx_891e0e6e8f478cdd` | `snap_7d512f5fb9d36248` | 310 | 378 | |
| datafusion-functions-nested | `ctx_e92703720a79c268` | `snap_9bf243d2e5f48fbc` | 825 | 969 | |
| datafusion-functions-table | `ctx_773b98428b28b54e` | `snap_0ff3db468a36c5d5` | 70 | 76 | |
| datafusion-functions-window | `ctx_546cc255f082c933` | `snap_5e03571a9ea71708` | 157 | 200 | |
| datafusion-functions-window-common | `ctx_2826f6f75c28c5fa` | `snap_9ad7e0e3f00cfb1f` | 22 | 35 | |
| datafusion-macros | `ctx_15ea339377c91953` | `snap_74daad8dfcf1bf8f` | 1 | 4 | proc-macro |
| datafusion-optimizer | `ctx_a6af93cf17af903e` | `snap_aff07cc92d7834a4` | 347 | 523 | |
| datafusion-physical-expr | `ctx_f465d83c24e402a7` | `snap_019c655cda2c2e87` | 997 | 1526 | |
| datafusion-physical-expr-adapter | `ctx_dba57951fef4884d` | `snap_3293eea57337d676` | 28 | 72 | |
| datafusion-physical-expr-common | `ctx_14718dbb181e542c` | `snap_c5379a5913a0b535` | 478 | 754 | |
| datafusion-physical-optimizer | `ctx_5fc3314729ab0b96` | `snap_64b3ffdbe10676b8` | 237 | 337 | |
| datafusion-physical-plan | `ctx_cff971223534b056` | `snap_fa1dee851c346474` | 2146 | 3173 | already retained by another caller at 18:24:12Z (O26) |
| datafusion-proto | `ctx_6d6c8d98cfcac295` | `snap_39222544d110a50e` | 224 | 246 | |
| datafusion-proto-common | `ctx_dcff1aaeeea4eb16` | `snap_8ce1ddeb737906ac` | 1470 | 4164 | |
| datafusion-proto-models | `ctx_d91d3be157405f2c` | `snap_d31aff13319db12e` | 4451 | 8226 | the single-flight crate |
| datafusion-pruning | `ctx_888b706d3c6a674b` | `snap_0ca9f7fbf99b47f5` | 42 | 67 | |
| datafusion-session | `ctx_d064983874ce8ffc` | `snap_291f0afb27370f4e` | 130 | 461 | already retained by another caller at 18:24:10Z (O26) |
| datafusion-spark | `ctx_36fcd35d9b5b30a2` | `snap_f1865e64981f5ce2` | 1464 | 1684 | |
| datafusion-sql | `ctx_f6ad323dd086a4e1` | `snap_db9abff2c2e7216a` | 477 | 627 | examples indexed |
| datafusion-sqllogictest | `ctx_ac463f4a35e8afd2` | `snap_f6dacca64a9a9790` | 90 | 141 | |
| datafusion-substrait | `ctx_767b64e20515d9ba` | `snap_d0c25629b525522c` | 269 | 337 | |

Other releases acquired for the comparison, revision and job rows: `datafusion` 53.0.0
(`ctx_08ee096d33f92016`, `snap_7d7ae73b5009f405`), 54.1.0 (`ctx_1f490235809278dc`,
`snap_8272aa96648190b7`, from the smoke), 55.0.0 (`ctx_ade20e38ba83c4af`,
`snap_3644009b027fbdfc`); `datafusion-common` 55.0.0 (`snap_757c078fe0c5d9ca`);
`datafusion-expr` 54.1.0 (`snap_71e945398a1ed79e`); `arrow-schema` 59.3.0
(`ctx_cc04a30e8b927857`); `sqlparser` 0.57.0 (`ctx_f11cd3464c2c47d1`, hosted JSON missing, zero
definitions, 24 fragments, a correctly typed `hosted_json_missing` gap).

Upstream traffic for the whole campaign before the restart was 49,029,887 bytes over 207 cache
misses and 24 revalidations with zero failures.

## Correct behaviour that is not a failure

These results are by design on a static-only host with no qualified producer image, or by design
in the research/2.0 contract, and the campaign treats them as passes.

- `POLICY_DENIED` from `verify_usage` in every mode and from `inspect_symbol` with an execution
  intent, each naming the three operator prerequisites and the concrete setup commands.
- `UNSUPPORTED_CAPABILITY` from `search_evidence` with `kinds=["source"]`, pointing at the
  `source` aspect of `inspect_symbol`.
- `UNSUPPORTED_FORMAT` at `stage: adapter` for every unknown or old argument, with the offending
  field named and no daemon job created.
- `partial` on every acquisition and comparison: `release_notes` is missing family-wide, and
  comparisons of docs.rs builds always carry the unknown-toolchain confounders.
- Artifact delivery whenever an answer exceeds the 12288-byte inline cap; the descriptor keeps the
  research status and lists the readable sections, and the terminal result of every job is
  delivered this way.
- `BUDGET_EXCEEDED` only for a cap below the metadata floor, with the delivery rule named.
- Facet-level `failed` and `unavailable` states inside an otherwise usable overview.
- `VERSION_NOT_FOUND`, `INVALID_CURSOR` and `ARTIFACT_UNAVAILABLE` on the designed-refusal inputs.

## Observations

Non-defects worth knowing when driving the service from an agent. Numbering continues from the
2026-09-14 report.

- O10: `service_status.versions.daemon` is `0.0.0`; there is no build identity on the wire.
- O11: the unknown-job message is still the raw OS text; only the next action explains it.
- O12: search hits carry no `definition_id`, although the inspect request documents that handoff.
- O13: the published schema admits `max_items: 0` and `max_characters: 0`, which the core rejects.
- O14: a bogus artifact id suggests resolving the release again in case the cache was cleared.
- O15: every inspect aspect page reports `count.kind: unknown`, even after exhaustion.
- O16: the missing-heading error does not list the available headings.
- O17: this client renders error envelopes only through the `research-error-preview/1` text, so
  `diagnostic.observed`, `allowed`, `affected_ids` and action kinds are not visible to the agent
  except in the compact form's `action_kinds`.
- O18: revision input-shape mistakes are `VERSION_NOT_FOUND` with `cause: not_found` rather than
  `invalid_input`.
- O19: the underscore spelling of a retained crate re-acquires and republishes the same snapshot id
  instead of answering from cache.
- O20: an unversioned `cache_ok` resolve re-checks the registry on every call, inside the 900 s
  TTL, and reports "revalidated" without re-running extraction.
- O21: an `area` restricted to a re-export-only namespace such as `datafusion::physical_plan`
  matches nothing, because re-export subjects live under their source paths.
- O22: page sizes are byte-bounded; `max_items=100` returned four hits and the effective item cap
  is not disclosed beyond `returned`.
- O23: `job_control` with `max_bytes=1024` answers by artifact delivery where the other tools
  refuse with `BUDGET_EXCEEDED`.
- O24: result sections are advertised even when empty; the `signature` section of a
  relationships-only inspection is `[]`.
- O25: context identifiers are deterministic across generations; every 2026-09-14 context id
  recurred in the v2 root.
- O26: another client shared the daemon during the campaign; `datafusion-physical-plan` and
  `datafusion-session` were already retained when the sweep reached them, and the daemon journal
  after the restart shows inspections this session did not issue. No interference was observed.
- O27: the workflow resource prepends a "Registered tools" line to the shipped contract text.
- O28: this client serialises tool calls, so true concurrency exists only through jobs that
  outlive the two-second inline wait; single-flight sharing was observed that way, but cancelling a
  running job could not be timed.

## Artifacts and reproduction

The per-request ledger (every request, job and artifact identifier quoted above), the three
child-session `stream-json` transcripts under `g11/`, the child runner, and the before-and-after
digests of the inactive old state root are preserved under
`.dev-state/mcp-campaign-2026-09-15/`. That directory is gitignored development state; copy it
elsewhere if it needs to outlive the checkout.

The evidence itself is retained in the v2 state root without age expiry, so every context and
snapshot identifier in this report can be re-read with `freshness="offline"`, and every stored
result artifact with `read_artifact`. The 2026-09-14 report's identifiers are not readable
through this generation, as the fail-closed rows in G6.8 confirm.
