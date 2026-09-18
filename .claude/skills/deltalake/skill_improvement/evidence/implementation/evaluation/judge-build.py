"""Assemble the independent judgment from inspected, immutable arm evidence."""
from __future__ import annotations

import hashlib
import json
import re
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent


def read(path: Path):
    return json.loads(path.read_text())


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


tasks = read(HERE / "tasks.json")["tasks"]
baseline = {r["id"]: r for r in read(HERE / "baseline/responses.json")["responses"]}
candidate = read(HERE / "candidate/independent-responses.json")
candidate_map = read(HERE / "candidate/independent-task-map.json")
candidate_result = read(HERE / "candidate/independent-evaluation-result.json")
candidate_metrics = read(HERE / "candidate/metrics.json")
candidate_followup = read(HERE / "candidate/judge-followup-result.json")
baseline_metrics = read(HERE / "baseline/metrics.json")
expected_ids = [r["id"] for r in tasks]
code_ids = [r["id"] for r in tasks if r["code_required"]]
assert len(expected_ids) == len(set(expected_ids)) == 38
assert set(expected_ids) == set(baseline) == set(candidate)
assert set(candidate_map) == set(code_ids)
assert all(baseline[t["id"]]["code_required"] == t["code_required"] for t in tasks)
for source in (HERE / "baseline/consumer/tests/decisions.rs", HERE / "candidate/logs/attempt-04-source.rs"):
    assert source.read_text().count("#[tokio::test]") == 8
    assert not re.search(r"#\[ignore", source.read_text())
assert digest(HERE / "baseline/consumer/tests/decisions.rs") == baseline_metrics["final_source_sha256"]
assert digest(HERE / "baseline/cargo-test-03.log") == baseline_metrics["final_log_sha256"]
for attempt in candidate_result["cargo_attempts"]:
    assert digest(HERE / "candidate" / attempt["source"]) == attempt["source_hashes"]["tests/decisions.rs"]
    assert digest(HERE / "candidate" / attempt["log"]) == attempt["log_sha256"]
for attempt in candidate_followup["attempts"]:
    assert digest(HERE / "candidate" / attempt["source"]) == attempt["source_hashes"]["tests/decisions.rs"]
    assert digest(HERE / "candidate" / attempt["log"]) == attempt["log_sha256"]

# These are judgments of decisions and consequential contract details, not scores
# for length, symbol counts, or runtime coverage of every statement.
notes = {
    "E01A": "Both retain a report-start snapshot and identify retention as a separate requirement.",
    "E01B": "Both refresh, rebuild and replace registration in the reused context. Both test stale and fresh results; baseline additionally executes a query planned before replacement.",
    "E02A": "Both select ordinary Parquet for immutable non-Delta files. Baseline supplies an exact read_parquet entrypoint; candidate appropriately limits its plain-Parquet qualification.",
    "E02B": "Both require Delta log semantics when obsolete files remain. Candidate adds supported deletion-vector/mapping scope without claiming all features work.",
    "E03A": "Candidate uses provider/name projection and a partition-first fixture. Baseline's unqualified scan_table().with_columns recommendation has a known partition-order counterexample, although its unpartitioned multi-batch fixture passes.",
    "E03B": "Both choose a provider for relational composition in the existing query environment; neither claims a measured query speedup.",
    "E04A": "Both preserve concrete state subject to Delta planner compatibility. Neither claims arbitrary caller planners automatically compose.",
    "E04B": "Both select explicit derive-or-reject policy and distinguish trait-visible runtime/functions from nontransferable catalogs/custom planning semantics.",
    "E05A": "Both select high-level append for resident batches and distinguish lower-level writer publication control.",
    "E05B": "Both select logical-plan input with a compatible caller state and execute the expected UDF rows. Candidate's original runtime pointer assertion occurs before write and uses an embedded batch source; it does not independently observe runtime-policy preservation. Baseline adds a distinguishing plain-planner failure and store-backed source, but also lacks an in-operation restrictive resource-policy oracle.",
    "E06A": "Both select full overwrite, preserve schema-policy distinctions, and separate logical replacement from disk reclamation.",
    "E06B": "Both tests assert preservation outside the predicate and independently reload after invalid input to confirm no new version/rows.",
    "E07A": "Both require fixed-schema policy beyond automatic compatibility. Candidate adds the pin-specific false/error versus true/null cast-safety behavior with appropriately bounded evidence.",
    "E07B": "Both select additive schema merge while preserving old rows and reject treating it as unlimited coercion. Candidate explicitly bounds tested additions to nullable fields.",
    "E08A": "Both select qualified merge with explicit update/insert clauses and do not claim persistent uniqueness constraints.",
    "E08B": "Both require deterministic duplicate/null policy. Baseline repairs its frozen reader's blanket no-error statement through source inspection; candidate cites bounded duplicate-update rejection evidence. Neither incorrectly treats that check as universal uniqueness enforcement.",
    "E09A": "Both record a transaction marker with the rows and explicitly deny automatic replay suppression.",
    "E09B": "Both inspect refreshed marker/publication state before replay and state ownership/concurrency limits. Both caller tests demonstrate duplicate rows from unguarded sequential marker reuse.",
    "E10A": "Both condition correction/retry on a known pre-publication validation failure, independently observed state, and possible orphan files.",
    "E10B": "Both reconcile durable log state before replay after hook failure and separate repair of hook effects. Candidate correctly narrows local visibility evidence from crash/remote durability.",
    "E11A": "Both distinguish current snapshot rows from change history and make the refresh boundary explicit.",
    "E11B": "Both finally execute all four change kinds and exact versions. Both initially assume the wrong physical commit-version array type; candidate final log observes UInt64 and final source decodes it with checked conversion. No initial-success claim is justified.",
    "E12A": "Both restrict the interval to retained historically enabled CDF and delay downstream checkpointing until successful processing.",
    "E12B": "Both identify end clamping, optional empty out-of-range start handling, and missing pre-enablement history; neither checkpoints a requested future end.",
    "E13A": "Both choose compaction for small live files and make speedup conditional on workload measurement.",
    "E13B": "Both explicitly preview vacuum with retention. Candidate additionally exposes the consequential dry_run=false and Lite defaults in the answer.",
    "E14A": "Both use a separate historical handle. Candidate additionally warns that an already supplied snapshot overrides the provider builder version option at this pin.",
    "E14B": "Both publish a later restore version and test historical readability. Baseline checks RESTORE operation metadata and metrics; candidate checks rows/version/history length.",
    "E15A": "Both permit bounded ordinary mapped-column scans while requiring actual profile/mode compatibility; candidate has retained executed fixture evidence.",
    "E15B": "Both correctly reject mapped-table CDF for this pin independently of recognized feature names or ordinary scan support.",
    "E16A": "Both distinguish ensuring a missing mapping from replacing a wrong one. Baseline names the public update_datafusion_session helper; candidate leaves helper selection generic but offers valid explicit runtime registration.",
    "E16B": "Both execute read failure against a wrong store and success after explicit root-store replacement; baseline additionally asserts store identity. Both bound shared-runtime concurrency.",
    "H01": "Both choose the provider's exact residual filter or an explicit low-level FilterExec, separating pruning from row correctness. Candidate's worked fixture provides relevant support but this is not unseen-capability generalization.",
    "H02": "Both require nested/null/timestamp value tests rather than assuming lossless conversion. Candidate adds exact observed schema-only normalization boundaries (nanosecond UTC to microseconds; UInt64 to Int64) and explicitly declines value-losslessness claims.",
    "H03": "Both choose Full for untracked orphans, explicit keep_versions and preview/retention policy. Baseline explicitly separates log retention; candidate names required retained log/data. Neither claims cloud cleanup safety from local tests.",
    "H04": "Both route Glue/Unity identifiers through catalog resolution or Unity DataFusion catalogs, then separately load storage and qualify credentials. Neither reports live cloud success.",
    "H05": "Both correctly put logical visibility at log publication and distinguish flush from commit. Candidate's 'write buffers input' effect description omits that write can finalize/stage files on target-size rollover or schema widening; baseline explicitly covers this. The frozen visibility probe uses a small batch and does not test early rollover.",
    "H06": "Both reopen the alternative comparison after relevant surface changes. Candidate links a concrete invalidation helper but qualifies it as hints. No changed capture is provided to either arm, so only the decision procedure is evaluated.",
}
assert set(notes) == set(expected_ids)
improved = {"E03A", "E07A", "E13B", "E14A", "H02"}
regressed = {"H05"}

evidence_groups = {
    "E01": ["../probes/tests/contracts.rs:30", "baseline/consumer/tests/decisions.rs", "candidate/logs/attempt-04-source.rs"],
    "E02": ["../probes/tests/contracts.rs:157"],
    "E03": ["../probes/tests/behavior.rs (unloaded_handle_explicit_version_and_partition_reconstruction)", "../../sources/delta-rs/crates/core/src/operations/load.rs:75", "candidate/logs/attempt-04-source.rs"],
    "E04": ["../../sources/delta-rs/crates/core/src/delta_datafusion/session.rs:150", "../probes/tests/session.rs:155"],
    "E05": ["../probes/tests/behavior.rs (logical_plan_write_and_staged_writer_visibility)", "baseline/consumer/tests/decisions.rs", "candidate/logs/attempt-04-source.rs"],
    "E06": ["baseline/consumer/tests/decisions.rs", "candidate/logs/attempt-04-source.rs"],
    "E07": ["../probes/tests/behavior.rs (schema_merge_and_cast_error_policy)", "../../sources/delta-rs/crates/core/src/operations/write/mod.rs"],
    "E08": ["../probes/tests/behavior.rs (merge_clause_order_null_keys_and_duplicate_updates)", "../.evaluation/baseline/content/topics/dml.md:28"],
    "E09": ["baseline/consumer/tests/decisions.rs", "candidate/logs/attempt-04-source.rs", "../probes/tests/contracts.rs:72"],
    "E10": ["../probes/tests/contracts.rs:136", "../probes/tests/behavior.rs (save_modes_replace_where_and_validation_before_commit)"],
    "E11": ["baseline/cargo-test-02.log", "baseline/cargo-test-03.log", "candidate/logs/attempt-02.log", "candidate/logs/attempt-04.log"],
    "E12": ["../../sources/delta-rs/crates/core/src/operations/load_cdf.rs:186", "../probes/tests/behavior.rs (cdf_bounds_images_and_residual_filter; competing_marker_writers_conflict_and_cdf_enablement_is_historical)"],
    "E13": ["../../sources/delta-rs/crates/core/src/operations/vacuum.rs:264", "../probes/tests/behavior.rs (optimize_compact_and_zorder_preserve_rows; vacuum_full_finds_orphan_lite_does_not)"],
    "E14": ["../probes/tests/contracts.rs:30", "baseline/consumer/tests/decisions.rs", "candidate/logs/attempt-04-source.rs"],
    "E15": ["../probes/tests/fixtures.rs:72", "../../sources/delta-rs/crates/core/src/operations/load_cdf.rs (build_with_metrics)"],
    "E16": ["baseline/consumer/tests/decisions.rs", "candidate/logs/attempt-04-source.rs", "../../sources/delta-rs/crates/core/src/delta_datafusion/session.rs:105"],
    "H01": ["../../sources/delta-rs/crates/core/src/delta_datafusion/cdf/scan.rs:66", "../../sources/delta-rs/crates/core/src/operations/load_cdf.rs:140"],
    "H02": ["../probes/tests/boundaries.rs:7"],
    "H03": ["../../sources/delta-rs/crates/core/src/operations/vacuum.rs", "../probes/tests/behavior.rs (vacuum_preview_keep_versions_and_history_loss; vacuum_full_finds_orphan_lite_does_not)"],
    "H04": ["../../sources/delta-rs/crates/core/src/data_catalog/mod.rs", "/home/paul/.cargo/git/checkouts/delta-rs-dcb716bfdc369320/58f07cd/crates/catalog-glue/src/lib.rs:62", "/home/paul/.cargo/git/checkouts/delta-rs-dcb716bfdc369320/58f07cd/crates/catalog-unity/src/lib.rs:994", "/home/paul/.cargo/git/checkouts/delta-rs-dcb716bfdc369320/58f07cd/crates/catalog-unity/src/datafusion.rs"],
    "H05": ["../../sources/delta-rs/crates/core/src/writer/record_batch.rs:328", "../../sources/delta-rs/crates/core/src/writer/record_batch.rs:392", "../.evaluation/candidate/skill_improvement/evidence/implementation/probes/tests/behavior.rs (logical_plan_write_and_staged_writer_visibility)"],
    "H06": ["../.evaluation/candidate/MAINTENANCE.md", "../.evaluation/candidate/scripts/invalidation.py", "tasks.json"],
}

decisions = []
for t in tasks:
    key = t["id"]
    baseline_test = baseline[key]["verification"]["test"]
    candidate_test = candidate_map.get(key)
    if t["code_required"]:
        assert baseline_test and candidate_test
        for test, source, log in (
            (baseline_test, HERE / "baseline/consumer/tests/decisions.rs", HERE / "baseline/cargo-test-03.log"),
            (candidate_test, HERE / "candidate/logs/attempt-04-source.rs", HERE / "candidate/logs/attempt-04.log"),
        ):
            assert f"async fn {test}(" in source.read_text()
            assert f"test {test} ... ok" in log.read_text()
    decisions.append({
        **t,
        "group": "paired" if key.startswith("E") else "nominal_heldout",
        "baseline": {
            "decision_contract_verdict": "incomplete" if key == "E03A" else "supported",
            "incorrect_final_claims": [],
            "incomplete_claims": ["Generic projected-stream recommendation lacks the known partition-order qualification."] if key == "E03A" else [],
            "runtime_state": "passed" if t["code_required"] else "not_run",
            "test": baseline_test,
        },
        "candidate": {
            "decision_contract_verdict": "incomplete" if key == "H05" else "supported",
            "incorrect_final_claims": [],
            "incomplete_claims": ["write can stage files before flush, although publication still requires commit."] if key == "H05" else [],
            "runtime_state": "passed" if t["code_required"] else "not_run",
            "test": candidate_test,
        },
        "decision_contract_comparison": "improvement" if key in improved else "regression" if key in regressed else "tie",
        "integration_oracle_comparison": "regression_in_scope" if key == "E05B" else "different_strengths" if key in {"E01B", "E03A", "E14B", "E16B"} else "tie" if t["code_required"] else "not_applicable",
        "reason": notes[key],
        "judge_evidence": evidence_groups[key[:3] if key.startswith("E") else key],
    })

baseline_materials = read(HERE / "baseline/materials.json")["files"]
reader_materials = [r for r in baseline_materials if "/.evaluation/baseline/" in r["path"]]
cache_materials = [r for r in baseline_materials if "/home/paul/.cargo/" in r["path"]]
candidate_materials = read(HERE / "candidate/reader-input-hashes.json")
integrity_paths = [
    "tasks.json", "baseline/responses.json", "baseline/metrics.json",
    "baseline/consumer/Cargo.toml", "baseline/consumer/Cargo.lock",
    "baseline/consumer/tests/decisions.rs", "baseline/cargo-test-03.log",
    "candidate/independent-responses.json", "candidate/independent-task-map.json",
    "candidate/independent-evaluation-result.json", "candidate/logs/attempt-04-source.rs",
    "candidate/logs/attempt-04.log", "candidate/consumer/Cargo.toml", "candidate/consumer/Cargo.lock",
    "candidate/metrics.json", "candidate/judge-followup-result.json", "candidate/logs/attempt-06-source.rs",
    "candidate/logs/attempt-06.log",
    "candidate-bundle.json",
]
assert digest(HERE / "baseline/consumer/Cargo.lock") == digest(HERE / "candidate/consumer/Cargo.lock")
assert digest(HERE / "baseline/consumer/Cargo.toml") == digest(HERE / "candidate/consumer/Cargo.toml")

judgment = {
    "kind": "independent_nonblind_single_run_comparison",
    "verified_date": "2026-09-18",
    "written_at": datetime.now(timezone.utc).isoformat(),
    "scope": "38 responses per arm, eight original caller compositions per arm, source and root probes as judging authority. This is not project acceptance or a population estimate.",
    "profile": read(HERE / "tasks.json")["profile"],
    "method": {
        "judge_independent_of_arm_generation": True,
        "blinded_arm_labels": False,
        "runs_per_arm": 1,
        "new_runtime_executions_by_judge": 0,
        "verification": "Read all responses, original/final execution logs and final emitted sources; inspect exact-pin source and probe assertions; programmatically verify IDs, code flags, test mappings, manifests and final log/source hashes.",
        "shared_target_limit": "Both arms used the same authorized build target; tests were separately logged. Warm cache timing is not a cold-build benchmark.",
        "candidate_reference_only": True,
        "baseline_external_cache_source": True,
        "baseline_prior_source_snapshot_limit": "Earlier baseline attempts preserve logs/source hashes but not complete earlier source snapshots; final source and final passing log are retained. Candidate retains each attempt source.",
    },
    "mapping_validation": {"state": "passed", "task_count": 38, "paired_count": 32, "heldout_count": 6, "code_required": code_ids, "missing_or_extra_ids": [], "baseline_code_flags_match": True, "candidate_mapping_matches": True, "same_consumer_manifest_and_lock": True},
    "outcomes": {
        group: {
            "tasks": len(subset := [r for r in decisions if r["group"] == group]),
            "baseline": dict(Counter(r["baseline"]["decision_contract_verdict"] for r in subset)),
            "candidate": dict(Counter(r["candidate"]["decision_contract_verdict"] for r in subset)),
            "comparison": dict(Counter(r["decision_contract_comparison"] for r in subset)),
        }
        for group in ("paired", "nominal_heldout")
    },
    "integration": {
        "baseline_original_final": {"state": "passed", "passed_tests": 8, "failed_tests": 0, "attempts": 3, "log": "baseline/cargo-test-03.log"},
        "candidate_original_final": {"state": "passed", "passed_tests": 8, "failed_tests": 0, "attempts": 4, "log": "candidate/logs/attempt-04.log"},
        "runtime_policy_oracle": "Original candidate E05B checks runtime identity before operation and uses an embedded batch source. It verifies UDF-derived rows, not an observable restrictive runtime policy. Baseline has additional distinguishing planner/source checks but does not fully certify arbitrary runtime policy either.",
        "no_first_attempt_success": True,
        "source_supported_non_code_decisions_are_not_runtime_passes": True,
    },
    "corrections_required_or_found": [
        {"id": "J01", "arm": "baseline", "task": "E03A", "severity": "consequential", "finding": "Known projection-order counterexample missing from general recommendation.", "correction": "Qualify direct projection by a matching regression test or use provider/name projection; preserve the passing narrow fixture as narrow evidence."},
        {"id": "J02", "arm": "candidate", "task": "H05", "severity": "contract_incompleteness", "finding": "write is described as buffering without noting early physical I/O/finalization.", "correction": "Explicitly state target-size/schema-widening staging before flush, flush returning/resetting Add actions, and publication only by commit."},
        {"id": "J03", "arm": "candidate", "task": "E05B", "severity": "qualification_gap", "finding": "Passing original caller test does not prove a distinguishing runtime policy during write.", "correction": "Add a separate labeled post-judge control with an observable caller runtime policy; do not retroactively change the independent score."},
        {"id": "J04", "arm": "both_initial_compositions", "task": "E11B", "severity": "repaired_runtime_error", "finding": "Int64 commit-version assumption failed; candidate's first attempted repair also failed.", "correction": "Retain all failures and document actual schema/checked decoding. Original final tests already pass; reader annotations should expose the UInt64 observation."},
        {"id": "J05", "arm": "both_initial_compositions", "task": "imports", "severity": "repaired_compile_error", "finding": "Both initially attempted private paths. Baseline additionally double-wrapped an already-Arc provider; candidate repaired create_session through the alias index.", "correction": "Preserve public re-export routes and clear distinction between canonical identity and legal import; do not claim first-try compilation."},
    ],
    "frozen_reference_errors_repaired_by_baseline_agent": [
        {"claim": "Application transaction ID makes a retried append idempotent.", "source": "../.evaluation/baseline/content/topics/transactions.md:3; content/topics/writing.md:41", "verdict": "incorrect", "response": "E09A/E09B correctly reject this and execute duplicate-marker replay."},
        {"claim": "Multiple matched source rows never cause an error.", "source": "../.evaluation/baseline/content/topics/dml.md:28", "verdict": "incorrect", "response": "E08B corrects it from exact-pin source while retaining normalization requirements."},
    ],
    "discovery_and_cost": {
        "finding": "Candidate completed its original answers/compositions without opening dependency cache source; baseline used exact cached source to repair misleading guidance and resolve contracts. This supports improved self-contained retrieval for these tasks, not a measured speed/token advantage.",
        "baseline_reader_file_inventory": {"files": len(reader_materials), "bytes": sum(r["bytes"] for r in reader_materials)},
        "baseline_cache_file_inventory": {"files": len(cache_materials), "bytes": sum(r["bytes"] for r in cache_materials)},
        "candidate_reader_file_inventory": {"files": len(candidate_materials), "bytes": sum(r["bytes"] for r in candidate_materials.values())},
        "file_inventory_caveat": "Whole inspected-file sizes, including unshown portions; not delivered/context bytes. Query implementation reads and truncated output are not normalized. Do not derive context savings from these totals.",
        "baseline_total_elapsed_seconds": None,
        "baseline_observed_lower_bound_seconds": baseline_metrics["elapsed_seconds_from_first_instrumented_event"],
        "candidate_reported_elapsed_seconds": candidate_result["elapsed_seconds"],
        "elapsed_comparison": "unmeasured because baseline total is unknown and instrumentation boundaries differ",
        "context_tokens_both": None,
        "baseline_cargo_elapsed_seconds": baseline_metrics["cargo_total_elapsed_seconds"],
        "candidate_original_cargo_elapsed_seconds": sum(a["elapsed_seconds"] for a in candidate_result["cargo_attempts"]),
        "cargo_time_caveat": "Four versus three different repair attempts on a warm shared target; not an end-to-end efficiency measurement.",
        "tool_count_comparison": "No normalized complete original per-task retrieval-call metric; arm-level self-reported/instrumented counts remain supporting receipts only.",
        "baseline_observed_tool_calls": baseline_metrics["observed_tool_calls"],
        "candidate_independent_instrumentation": candidate_metrics["independent_phase"],
        "candidate_command_receipt": "candidate/metrics.json (records include both independent and subsequent follow-up phases; independent_phase isolates original counters)",
        "candidate_retrieval_instrumentation_limit": "Initial TextEncoder instrumentation failed, and complete byte/token counts were not measured. 176608 measured output characters include compiler/runtime output and are not pure reader retrieval or token counts.",
    },
    "heldout_limits": [
        "H01, H02, H03 and H05 directly overlap frozen candidate worked probes; they are nominal heldout task prompts, not unseen capability cases.",
        "H04 has dedicated frozen catalog guidance and no live-cloud oracle.",
        "H06 supplies no actual changed capture/new API; it tests an answer procedure. Root invalidation controls are separate implementation qualification.",
        "Only one arm agent run per reader; no repeat distribution or generalizable causal effect is established.",
    ],
    "evidence_boundaries": {
        "frozen_candidate_bundle": read(HERE / "candidate-bundle.json"),
        "frozen_candidate_probe_receipt_sha256": digest(ROOT / ".evaluation/candidate/skill_improvement/evidence/implementation/probe-results.json"),
        "current_root_probe_receipt_sha256_at_judgment": digest(ROOT / "probe-results.json"),
        "rule": "Post-freeze portable source/identity fixes, added root assertions, writer/CDF annotations, and judge-prompted E05B follow-up are final qualification only. They do not alter the independent arm comparison.",
    },
    "post_judgment_qualification": {
        "task": "E05B",
        "state": "passed",
        "evidence": ["candidate/judge-followup-result.json", "candidate/logs/attempt-05.log", "candidate/logs/attempt-06-source.rs", "candidate/logs/attempt-06.log"],
        "judge_verification": "Inspected RequiredStoreSource::scan and both caller states: source scan counters start at zero, a separate Delta-capable runtime lacks the required mapping and rejects without a commit, and the intended caller runtime lists actual source Parquet objects then appends the expected UDF row. Attempt 05's Url/AsRef compile failure was repaired in attempt 06; all eight tests pass.",
        "scope": "Closes the concrete operation-session propagation oracle for an observable caller-only store capability. It does not certify every resource policy, cloud storage, or arbitrary Session wrapper.",
        "changes_independent_comparison": False,
    },
    "tasks": decisions,
    "artifact_sha256": {p: digest(HERE / p) for p in integrity_paths},
}

(HERE / "judgment.json").write_text(json.dumps(judgment, indent=2) + "\n")

lines = [
    "# Independent Delta reader comparison — 2026-09-18",
    "",
    "Both arms produced all 38 responses and finished with all eight required caller tests passing. The candidate is stronger on the partition-order trap and several concrete contract details; the baseline already made most decisions correctly after inspecting exact dependency source. This single run does not establish a general quality or efficiency improvement.",
    "",
    "The judge read all responses, emitted sources, initial and final execution logs, relevant exact-pin source, and probe assertions. Labels were visible to the judge. No additional runtime test was executed by the judge. Task IDs, all eight `code_required` mappings, manifests/locks, final test names and result lines were checked programmatically. Detailed judgments and artifact hashes are in [judgment.json](judgment.json).",
    "",
    "| Scope | Baseline decisions/contracts | Candidate decisions/contracts | Candidate comparison |",
    "|---|---|---|---|",
    "| 32 paired prompts | 31 supported; E03A incomplete | 32 supported | 4 improvements; 28 ties |",
    "| 6 nominal held-out prompts | 6 supported | 5 supported; H05 incomplete | 1 improvement; 4 ties; 1 regression |",
    "| Required caller tests | 8 passed after 3 attempts | 8 passed after 4 attempts | Different fixture strengths; no blanket integration improvement |",
    "",
    "`Supported` means the stated decision/contract is backed within its declared scope; it does not mean every behavior was executed. Thirty non-code responses per arm have no new caller execution. These counts are not a quality percentage or project acceptance tally.",
    "",
    "## Consequential findings",
    "",
    "- **E03A:** baseline recommends `scan_table().with_columns` generally, but its successful test covers an unpartitioned layout. The retained source computes projection indices before the provider's reordered schema, and the root/frozen probe demonstrates a partition-first request for `id` returning `label`. Candidate selects provider/name projection and checks names/types/values on a partition-first layout. The direct API remains valid where a matching regression test proves it; the oracle does not demand a particular symbol.",
    "- **H05:** candidate correctly separates staging from log visibility, but 'write buffers input' omits early I/O/file finalization on target-size rollover or schema widening. Exact `RecordBatchWriter::write_with_mode` source states this. Baseline describes that effect accurately. This is a contract omission, not a claim that candidate publishes rows prematurely.",
    "- **E05B:** original candidate test proves appended UDF-derived rows. Its RuntimeEnv identity assertion occurs before write, and the source is an embedded batch. That does not independently prove a restrictive caller runtime policy during the operation. Baseline has stronger planner/source controls but likewise does not prove arbitrary policy preservation. Keep any judge-prompted stronger control separate from the original comparison.",
    "- **E11B:** both initially assumed an incorrect physical commit-version representation. Candidate attempt 04 prints `_commit_version: UInt64`; final checked scalar decoding passes exact row/version assertions. Baseline's final display-based decoding also passes. Candidate attempt 03 still failed after an incomplete repair. Preserve these failures; neither arm compiled and ran correctly on its first attempt.",
    "",
    "The remaining candidate contract improvements counted here are E07A's tested cast/null behavior, E13B's destructive vacuum defaults, E14A's loaded-snapshot/version-option trap, and H02's qualified schema-normalization results. E08B and E09B are **ties in final answers**: the baseline agent explicitly corrected frozen reader claims about duplicate merge matches and marker-only replay. The candidate's better source material must not be scored as an invented baseline answer failure.",
    "",
    "## Caller evidence",
    "",
    "[Baseline final log](baseline/cargo-test-03.log) and [candidate original final log](candidate/logs/attempt-04.log) each contain all eight expected tests with zero ignored tests. Both consumers use the same manifest/lock and exact Delta/kernel profile. Baseline additionally tests already planned stale queries, ordinary-planner rejection, restore operation metadata and store pointer identity. Candidate covers the partition-first projection layout. Candidate's original E05B evidence is narrower than its name suggests.",
    "",
    "Initial failures are retained: baseline attempted a private LogStoreExt import and redundant provider Arc wrappers, then hit planner/CDF errors; candidate attempted the private session module, then twice failed its CDF version downcast. Final repairs retain the behavioral assertions. Earlier baseline source versions were not fully retained, only their hashes and diagnostic logs; candidate retains each attempt's source.",
    "",
    "## Discovery and measured cost",
    "",
    f"Candidate original responses used the frozen reader without dependency-source fallback. Baseline inspected {len(cache_materials)} cached source files and repaired misleading reader guidance. Candidate's inventory covers {len(candidate_materials)} reader files ({sum(r['bytes'] for r in candidate_materials.values()):,} bytes); baseline's covers {len(reader_materials)} reader files ({sum(r['bytes'] for r in reader_materials):,} bytes) plus {sum(r['bytes'] for r in cache_materials):,} source bytes. These are whole file sizes, including unshown portions, **not context consumed**. They cannot establish token savings.",
    "",
    f"Baseline's measured interval is a {baseline_metrics['elapsed_seconds_from_first_instrumented_event']:.2f}-second lower bound; its total elapsed time is unknown. Candidate reports {candidate_result['elapsed_seconds']:.2f} seconds from its recorded start. Context tokens and normalized per-task retrieval counts are unmeasured. No total-time/token advantage is claimed. Cargo durations are warm-cache repair costs, not a reader efficiency benchmark.",
    "",
    "Original arm tool counters are also retained: baseline manually records 25 exec wrappers, 24 shell calls and 2 polls; candidate reconstructs 26 wrappers, 26 shell calls, 6 patches and 3 polls at the independent-phase boundary. Candidate measures 176,608 output characters after an initial telemetry failure. Different counting/output scopes prevent treating these as a complete normalized retrieval-cost comparison; full candidate command records are in [metrics.json](candidate/metrics.json).",
    "",
    "## Held-out and qualification boundaries",
    "",
    "H01/H02/H03/H05 overlap worked probes already in the frozen candidate. Their results show transfer from available evidence, not performance on previously unseen capabilities. H04 has catalog guidance but no live-cloud test. H06 provides no actual changed capture; both arms answer the procedure correctly, while changed-capture detection must be judged through separate implementation controls. No repeated stochastic run was performed.",
    "",
    "The frozen candidate receipt is distinct from the current root qualification receipt. Subsequent portable source/identity corrections, CDF/writer annotations, expanded root assertions and any judge-prompted E05B follow-up receive no retrospective credit here. Root qualification may resolve these findings under a new candidate identity. This report does not certify cloud authentication, crash durability, concurrent exactly-once delivery, all schemas/features, or workload performance.",
    "",
    "**Post-judgment E05B qualification:** the judge inspected the separately retained [follow-up source](candidate/logs/attempt-06-source.rs) and [passing log](candidate/logs/attempt-06.log). A source provider requires a caller-only runtime store during the operation: a separate runtime fails without committing, while the intended runtime lists real Parquet objects and appends the expected UDF row. This closes the concrete session-propagation oracle. The custom source does not certify cloud behavior or every resource policy. Attempt 05's compile failure remains visible; attempt 06 passes all eight tests. The original comparison above is unchanged.",
    "",
    "## Per-task comparison",
    "",
    "| Task | Baseline | Candidate | Comparison and reason |",
    "|---|---|---|---|",
]
for row in decisions:
    lines.append(f"| {row['id']} | {row['baseline']['decision_contract_verdict']} | {row['candidate']['decision_contract_verdict']} | {row['decision_contract_comparison']}: {row['reason']} |")
(HERE / "REPORT.md").write_text("\n".join(lines) + "\n")
print(json.dumps({"state": "passed", "tasks": len(decisions), "code_tasks": code_ids, "outcomes": judgment["outcomes"]}, indent=2))
