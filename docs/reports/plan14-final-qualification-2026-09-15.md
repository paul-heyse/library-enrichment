# Plan 14 implementation and qualification — 2026-09-15

**Plan 14 is complete and activated: P0–P9, D01–D13 and J01–J17 are closed.**
[Plan 14](../plans/14-datafusion-catalog-policy-hard-pivot.md) is the scope authority.
The existing capability review is design evidence; the receipts below establish implementation.

## Candidate and reproducibility

- Starting HEAD: `a78c74191d69ad82de7348c6503b58443f7f9c13`.
- Final source digest: `5b2640ca15e4e1e6bb3355eda84edb89d4bcdab3e390897513648c0581d4026a`.
- Source archive and changed-input list: `.dev-state/plan14-final/source.json`, `source.zip`.
- Installation: `/home/paul/.local/opt/library-enrichment/plan14-5b2640ca15e4`.
- `candidate.json` binds all installed inputs and three release executable hashes; its SHA-256 is
  `35aa88fe32171e1e166e2acebe97e228f314b6d6edb7df1bdbb88775ced00a3f`.
- Locked optimized build: `.dev-state/plan14-final/release-build.log`; isolated Cargo output under
  `.dev-state/p13-release-target`. The installed Python environment was created with
  `uv sync --frozen --no-dev --no-editable` and has no working-tree package import.
- Exact pins: DataFusion 55.1.0, Arrow/Parquet 59.3.0, object_store 0.13.2, Rust 1.98.1,
  FastMCP 4.0.3 and Python 3.14.7. No dependency upgrade was required.

The final native/Python evidence runner records source before/after, executable hashes,
environment, commands and raw-log hashes. Documentation is outside the source fingerprint.
P8 measurements have their own historical source/executable receipts; they are not mislabeled
as executions of the final candidate. Their selected mechanisms are integrated and exercised
by the final correctness and installed consumers.

## Architecture and deletion closure

All rows below are **passed** against the final source. Paths are relative to
`crates/enrichment-store/src/` unless stated otherwise. This is a semantic replacement ledger,
not a count of removed names.

| ID | Implemented replacement and actual consumer |
|---|---|
| D01 | `admission.rs` builds one complete typed inventory; `native_catalog.rs` projects immutable schemas. Request leases are binding inputs. Old register/register_leased/register_views modes are deleted. |
| D02 | `catalog_generation.rs` uses the same hierarchy for exact history and folded records, with distinct binding kinds. Repeated mutable registration and raw aliases are removed. |
| D03 | `comparison.rs` binds independent `before`/`after` pins and qualifies native sources. Copied side tables and `SIDE_` substitution are removed. |
| D04 | `runtime.rs` installs a validated complete root once; immutable catalog/schema mutations fail. Only `native_catalog::work` registers approved operation-local tables. The new architecture rule rejects other registration owners. |
| D05 | `RelationContract` resolves semantic key names against the actual codec schema. Shared native duplicate/reference plans consume those declarations; explicit finite conditional rules drive both validation and metadata. No positional primary-key convention remains. |
| D06 | `native_policy.rs` captures one effective immutable policy. The read-only ConfigExtension and the actual Parquet source factory share it. Provider-local default paths are deleted. |
| D07 | `projection/contracts.rs` encodes bounded inventory, nested field roles and rule parameters from the installed objects/declarations. Runtime settings and diagnostics derive from the same policy. |
| D08 | `provider.rs` retains admitted rows and supplies physical `FileScanConfig` exact statistics. Candidate and unknown facts remain unknown. Transparent `LeasedProvider` delegates discovery statistics; retention survives scan elimination. |
| D09 | `scoring.rs` uses native logical-string coercion. Analyzed UDF arguments retain Utf8/LargeUtf8/Utf8View; matching semantics and result metadata remain equal. |
| D10 | `operation_index.rs` returns `CompletedIndex`, including exact rows, binding identity and established stream properties. Search/comparison use its count directly; count-only IPC replay is removed. |
| D11 | `query_diagnostics.rs` records capped rule transitions and shared managed-memory high-water observations. Rule hashing is bounded; peaks are not reset or labeled per-query/RSS. |
| D12 | Shared writer properties, explicit storage row groups and independent catalog file targets replace batch/delta coupling. Reservation-backed compaction combines processing batches within real byte/file/footer caps. |
| D13 | Only selected production strategies remain. Opaque derived providers were removed to preserve native logical visibility. Rejected alternatives are confined to explicitly manual measurement tests; no compatibility flags, namespace aliases or fallback provider engine remain. |

`rules/native-catalog-registration.yml`, its positive/negative fixtures, `just architecture-check`
and the actual store/daemon consumers verify the registration boundary. Namespaces are internal
query contracts; they do not create a new authorization or publication authority.

## Functional oracle closure

The ordinary final native run used:

```sh
CARGO_INCREMENTAL=0 NEXTEST_EXPERIMENTAL_LIBTEST_JSON=1 uv run python scripts/evidence_run.py \
  --log docs/reports/logs/nextest.json --stdout -- \
  cargo nextest run --locked --offline --workspace --test-threads 8 --message-format libtest-json-plus
```

Result: **441 passed, 16 skipped** in 58.584 seconds. The skipped tests are explicit manual
measurements and the external containment tier; P8's selected manual measurements have separate
executed receipts. They are not silently counted as ordinary passes. Eight workers avoid the
observed 32-worker fixture oversubscription without changing any product/test deadline.

| ID | State | Executable evidence on the selected architecture |
|---|---|---|
| J01 | passed | `native_catalog::tests::bound_inventory_is_immutable_consistent_and_drives_nested_metadata`; existing admission/catalog empty and exact-file lookup cases. |
| J02 | passed | `invalid_inventory_installs_nothing_and_discovery_reports_its_bound`; immutable mutation, duplicate/incorrect binding and work-shadow negatives. |
| J03 | passed | Admission, views, catalog_generation, dataset, ingest and repository integration tiers; offline bundle admission without the original store. |
| J04 | passed | Repository comparison publication, qualified endpoint comparison and daemon cold version-comparison completion. Installed 54.1→55.1 comparison also completes. |
| J05 | passed | Concurrent catalog commits retain old pins; compaction retains selections; runtime cancellation, operation-index readers/quotas and repository publication/bundle closure cases. Installed final ownership counters return to zero. |
| J06 | passed | `declared_key_position_and_native_duplicate_plan_follow_reordered_schema`; admission malformed/dangling subjects; `invalid_closure_leaves_root_unchanged_and_unreferenced_files_are_invisible`. |
| J07 | passed | `native_policy::tests::read_only_policy_is_the_actual_format_and_settings_authority`; installed effective settings include decoder/reorder/Bloom/layout/grouping selections. |
| J08 | passed | Bounded inventory tests cover nested metadata, duplicate/wrong-kind rejection and truncation on a 3,000-field input. Leased count test queries actual `operation.metadata.relations.verified_rows`. |
| J09 | passed | `physical_counts_eliminate_scans_without_losing_admission_or_retention` proves PlaceholderRowExec and no DataSourceExec; `exact_multi_file_counts_keep_nullable_filters_and_limits_correct` proves NULL/filter/LIMIT and multi-file results. |
| J10 | passed | The count-elimination case deletes an admitted source and proves both new binding and already-built execution reject it; source-witness/admission negatives remain required. |
| J11 | passed | `native_string_encodings_keep_scores_metadata_and_arguments_without_utf8_casts`, including sliced arrays, optional NULLs, Unicode, scalar broadcast and empty input; literal/pattern bonus semantics remain covered. |
| J12 | passed | `index_replays_exact_native_rows_and_drops_quota_after_last_reader`; `empty_and_failed_index_release_native_spill_reservations`; search/comparison page/total regression. |
| J13 | passed | `completed_ordering_has_real_ties_nulls_and_projection_behavior`; actual completed native properties are projected only when established, including unknown-order cases. |
| J14 | passed | `rule_history_and_shared_memory_peak_are_bounded_and_do_not_reset`; retained failure diagnostics and installed capacity failure with operation-correlated recovery and zero post-failure ownership. |
| J15 | passed | All nine decisions in the [physical strategy report](plan14-physical-strategies-2026-09-15.md), backed by matched outputs, actual consumers, counters and distinct source-bound raw receipts. |
| J16 | passed | Installed real raw-MCP campaign, empty/missing/capacity outcomes, plus real Codex A01 and Claude A02. Production activation smoke is recorded separately below. |
| J17 | passed | D01–D13 above, all eleven architecture-rule fixtures and actual consumer regression. |

## Installed MCP and real clients

`.dev-state/plan14-mcp/focused-212934-receipt.json` records a **passed** fresh-state campaign
against the installed release, with unchanged source and binary hashes. Raw requests/frames,
answers, process logs and the driver hash are retained beside it. Actual advertised output
schemas and MCP error flags are validated, including complete-frame delivery bounds.

- Exact DataFusion and datafusion-expr 55.1.0 resolve, overview, search and useful inspection.
- All 117 SessionContext relationships across independent cursor pages without duplicates.
- DataFrame, ParquetReadOptions and Expr inspection; evidence IDs close over returned support.
- DataFusion 54.1.0→55.1.0 documentation/release-note comparison with complete artifact expansion,
  chunk/content digest checks, and bounded delivery.
- Packaging 26.3 resolve and `packaging.version.Version` through the installed extraction worker.
- Exact retained/offline reuse preserving context and snapshot IDs.
- Successful empty search and explicit missing-symbol error, without an absence-of-capability claim.
- Final status reports zero queued/running jobs, admitted queries and reserved managed memory.

`.dev-state/plan14-mcp/capacity-receipt.json` records a **passed** real installed capacity case.
An isolated configuration limits native result bytes to 32,768; a real result needs 41,704 and
returns a typed capacity diagnostic. Subsequent status reports one incomplete native execution,
zero admitted queries and zero managed reservation. An exploratory one-byte limit also prevented
status queries; that failed harness assumption is retained under `capacity-too-small-for-status/`.
No production budget was reduced.

`.dev-state/plan14-final/clients/summary.json` records actual **Codex A01 and Claude A02 passed**,
with complete transcripts and unchanged real user directories. Each client exercised installed
status, resolve, search and artifact access. The bounded CLI campaign does not claim execution
of unrelated client gates; machine acceptance remains a separate evidence join.

## Quality, regression and honest limits

Formatting, workspace all-target/all-feature Clippy with warnings denied, Rust doctests, Ruff,
scoped ty, schema conformance, all eleven architecture rules, architecture checks, dependency
policy, ADR lint, frozen provenance and state-leak checks passed. Commands/results are retained
under `.dev-state/plan14-final/`; `quality.json` binds the batched quality checks to source.
Schema conformance validates all four fixtures and rejects all three negative examples.

The initial native regression had two stale fixture namespace/binding assumptions and one
oversubscribed 30-second cold-comparison fixture timeout. Target fixtures were corrected; the
product/test deadline was retained. All 441 final native cases pass. An earlier bare ty invocation
included development-skill build sources outside the project's supported check paths; scoped
`uv run ty check python tests scripts` passes. Those diagnostic failures remain in their logs.

The ordinary Python command was:

```sh
CARGO_INCREMENTAL=0 uv run python scripts/evidence_run.py --log docs/reports/logs/pytest.json -- \
  uv run pytest -m 'not live and not client' --json-report --json-report-file=docs/reports/logs/pytest.json
```

Result: **188 passed, 35 skipped, 17 deselected**, exit 0, in 425.76 seconds. The 35 skipped
cases explicitly require qualified Rust/Python execution images and an execution root not selected
for this campaign. Static canary, research raw-stdio, publication, retention, comparison, installed
launch and ordinary extraction cases ran. Native external-container cases and Python execution
profiles are not certified by this result. The current environment's `execution-env.sh` reports
no qualification receipt at `/nonexistent/podman/admitted-images.json`.

Independent machine-acceptance replay reproduced both ordinary commands on the same source. Unrelated external producer/runtime breadth is not replayed
for this native architecture change; earlier Plan 13 evidence is historical, not current-source
certification. Static production policy remains selected.

## Activation and final acceptance

The candidate activated **once at 21:36:31 UTC**. Receipts under
`.dev-state/plan14-final/cutover/` include the prepared launch, prior unit/registration, command
log, activation result, final identity, raw deployed requests/frames and post-activation smoke.

- Active daemon: PID **2465957**, executable under the identified installed release.
- Executable SHA-256: `98666f28d498471a05a15047ddc68ac2020c788b49992129f6593bd5e0e5a7e9`.
- Config: `/home/paul/.config/library-enrichment/research-v2-plan14-5b2640ca15e4.toml`.
- Config SHA-256: `eb318f8f58a3e19043c5dc77ec88df2144a71af9eced3fd8621cd1d6e8ad8fda`.
- State: existing target-compatible `/home/paul/.local/state/library-enrichment-v2`.
- Static-only execution permissions and workstation limits remain selected: 32 GiB managed
  memory, 64 GiB spill, 2 GiB metadata cache, 16 partitions/query slots.
- Both global Codex/Claude registrations match the same installed adapter and configuration.
- All installed manifest inputs and the running daemon hash match. The old four adapter processes
  are stopped; parent interactive clients were preserved. Existing sessions may need to reconnect.
- The activation receipt verifies the old configuration and existing data digest were unchanged
  across replacement. Production smoke then uses that compatible retained evidence normally.
- Query-failure history uses `query-failures-v2.json`; the old diagnostic history remains inactive,
  without a converter. Canonical snapshot/catalog/job and research/2.0 formats are retained.

The **passed** deployed smoke at 21:37:09 UTC completed eight cases: status, exact DataFusion
55.1.0 resolve, SessionContext inspection, 54.1→55.1 comparison, direct changes-section artifact
read/digest, packaging 26.3 resolve/Version inspection, and final ownership/settings status. The
final identity check found zero queued/running jobs, admitted queries, managed reservations and
daemon children. The final real-XDG state-leak comparison also passed.

Current machine acceptance: **28 passed / 0 failed / 11 blocked / 9 not_run of 48**.
`just acceptance-report` and `just acceptance-check` use only current-source valid receipts.
The blocked gates name the unselected qualified execution-image prerequisites. Unrun registered
client/live gates and retired P08 are not promoted from historical evidence. In particular, the
separate actual A01/A02 CLI campaign passed while the registry's pytest wrapper remains not_run;
this is a wiring/scope distinction, not a failed real-client connection.

The acceptance auditor independently replayed every distinct command behind the 28 claimed gate
passes, preserving the original logs. Native replay reproduced **441 passed / 16 skipped** in
53.336 seconds; Python replay reproduced **188 passed / 35 skipped / 17 deselected** in 420.51
seconds. Both original and replay receipts validate; source and native hashes remained unchanged.
The independent two-command join exactly matches the standard report, and acceptance-check passes.
All five actual publication barriers and the partial-API comparison case also reproduced.
No gate downgrade was required. The [per-gate audit](../../.dev-state/plan14-independent-audit/acceptance-audit-2026-09-15.md)
and [audit receipt](../../.dev-state/plan14-independent-audit/audit-receipt.json) bind the verdict
to preserved original and replay logs. The complete qualification receipt index is
`.dev-state/plan14-final/evidence-manifest.json`.

No required Plan 14 implementation, qualification or activation work remains. No source change
followed candidate qualification. Deferred R44/R45 triggers and unrelated pre-plan MCP findings
retain their stated scopes. Task-owned qualification processes have exited; the deployed service
is intentionally active. Working-tree edits and historical evidence are preserved without commit,
reset or cleanup.
