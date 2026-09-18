# D6 independent revision follow-up: H05 and E11B

Verified 2026-09-18. Both affected cases are **passed for this bounded reader-contract review**. The revised reader supports the distinctions required by H05 and the exact CDF metadata types and query caveats required by E11B. No consequential contract error was found in these two answers. This result does not replace the original scores, certify a new 38-case evaluation, or establish broader improvement.

The independent answers were frozen in [responses.json](responses.json) before inspecting the supplied Rust receipt, logs, assertions, or captured source. Answer SHA-256: `787f893686c016db4c91e5221ec2f22381c20b9127112c7e3a664e2b36829616`. Its digest remains unchanged. Answer generation used only the revised reader's `SKILL.md`, `scripts/reference.py`, and `content`, plus task/instruction/identity metadata. No original acquisition/cache or earlier baseline/candidate response was consulted. After freezing, only evidence inside the same frozen revised reader was inspected for the judgments below.

## Candidate identity and scope

- Archive: `revised-candidate.tar.gz`.
- Archive SHA-256: `55d26c6dab92462489c7a19cadd11a94f13c585c7038848fe5f87eb6eefcf2f4`.
- Archive size: 4,962,755 bytes; 3,771 payload files plus `BUNDLE_MANIFEST.json`.
- Every archived file was compared byte-for-byte with the supplied extracted reader, with no mismatch.
- Profile: delta-rs `58f07cd62bfbce3649a7e1c87c696288068ae184`, kernel `8ba063f8f84fec222000f66d40d70911d7c79675`, DataFusion 55.1.0, Arrow 59.3.0.
- Evaluator: `/root/delta_revision_evaluation`; tasks: H05 and E11B only.

The archive digest matches the [supplied receipt](../revised-candidate-bundle.json). [verification.json](verification.json) records archive, answer, supplied-probe, log and source-file hashes. An initial verification assertion incorrectly compared the total archive count to the payload count and failed; inspecting the manifest established the one additional member. The corrected check verifies both counts explicitly. This was an evaluator count assumption, not a candidate mismatch.

The report retains only its 11 cited frozen-reader files under `frozen-evidence/`, preserving their reader-relative paths and exact bytes. Each retained file was verified against the frozen archive; copied-file hashes are recorded in [verification.json](verification.json). Report links resolve to these permanent copies, so navigation does not depend on extracted scratch trees or recursively bundled archives. This packaging follow-up changed no answers, scores, archive identity or runtime evidence.

## Findings

| Case | Outcome | Independent finding | Evidence boundary |
|---|---|---|---|
| H05 | passed | The answer distinguishes write-time object I/O, target-size/schema rolling and background upload completion, explicit flush returning staged Add actions, log publication by commit, and reset aborting pending uploads while leaving finalized files unreferenced. It also distinguishes a previously drained flush from data still pending in the writer. | The supplied test directly covers a completed size-roll upload before explicit flush and row visibility before/after commit. Reset, schema rotation and the explicit flush/manual-commit contract are supported by captured public contracts/source; this review does not claim separate runtime tests for all of them. |
| E11B | passed | The answer chooses the built-in CDF provider with explicit inclusive bounds; includes a Rust query fragment; specifies nullable `Utf8`, `UInt64`, and `Timestamp(Millisecond, None)` metadata; preserves all change kinds; and explains filtering, ordering, retention, enablement, mapping and checkpoint caveats. | The supplied assertion confirms direct-query `UInt64` commit versions and the update images/filter result. The captured source defines all three metadata fields. The fragment was not newly compiled or executed by this evaluator. |

H05's detailed contract is in the [write brief](frozen-evidence/content/capabilities/delta.write.json), especially claims `delta.write.2` and `delta.write.5`, and the [RecordBatchWriter contracts](frozen-evidence/content/operations/deltalake_core.writer.record_batch.RecordBatchWriter.md), especially `write_with_mode`, `with_target_file_size`, `flush`, `flush_and_commit`, and `reset`. The `reset` annotation explicitly corrects ambiguous upstream wording about flush “committing” data. The [DeltaWriter contract](frozen-evidence/content/operations/deltalake_core.writer.DeltaWriter.md) states that returned Add actions still require a transaction.

After answer freeze, inspection of the supplied [behavior assertions](frozen-evidence/skill_improvement/evidence/implementation/probes/tests/behavior.rs), lines 124–152, confirmed a target size of 1, one-row groups, a bounded wait for the background upload to appear in storage, unchanged rows `[4, 5]` after observer refresh, and rows `[4, 5, 6]` only after `flush_and_commit`. This establishes the intended distinction more precisely than a buffered-write-only example. The captured [writer source](frozen-evidence/skill_improvement/evidence/sources/delta-rs/crates/core/src/writer/record_batch.rs), lines 246–263 and 391–420, supports reset, rolling, drain and staging/publication semantics.

E11B's reader-level evidence is the [CDF brief](frozen-evidence/content/capabilities/delta.cdf.md), lines 16–55, and the [provider contract](frozen-evidence/content/operations/deltalake_core.delta_datafusion.cdf.scan.DeltaCdfTableProvider.md). Its exact public facade import is preserved in [aliases.tsv](frozen-evidence/content/index/aliases.tsv), line 572. The captured [CDF schema source](frozen-evidence/skill_improvement/evidence/sources/delta-rs/crates/core/src/delta_datafusion/cdf/mod.rs), lines 22–40, specifies nullable `UInt64` commit versions, millisecond timestamps without a timezone annotation, and nullable `Utf8` change kinds. The [behavior assertions](frozen-evidence/skill_improvement/evidence/implementation/probes/tests/behavior.rs), lines 294–443, confirm the returned version type, preimage/postimage values, exact provider filter, range controls, and the observed `ORDER BY id` error containing `UnknownPartitioning(0)`. Unordered collection followed by caller sorting is the demonstrated path; this is not a claim that all ordered CDF queries fail.

## Commands and results

[commands.json](commands.json) retains four exact reader command argument lists, working directories, return codes, output filenames and output hashes. All four returned exit code 0:

```text
reference.py find --task "low-level RecordBatchWriter write roll background upload flush commit reset file staging transaction visibility" --limit 3
reference.py find --task "row level inserts updates deletes CDF explicit versions incremental consumer metadata Arrow types exact row filtering" --limit 3
reference.py show delta.write --view evidence
reference.py show delta.cdf --view evidence
```

The reader was invoked using the project's `uv run python` interpreter. The first queries ranked `delta.write` and `delta.cdf` first respectively. Complete outputs are [reader-01.json](reader-01.json), [reader-02.json](reader-02.json), [reader-03.json](reader-03.json), and [reader-04.json](reader-04.json). These retained calls repeat discovery calls for an auditable record; they are not a retrieval-cost benchmark. Additional direct reader-file inspection supplied full member contracts and the answer citations.

Archive/source consistency inspection used standard-library Python (`hashlib`, `tarfile`, and JSON) under `uv run python`. It verified all archive members against the extracted reader, all 17 source/fixture hashes declared by the supplied probe receipt, both named tests in the receipt and log, and the unchanged response digest. [verification.json](verification.json) records the result and the initial count-check correction. Final artifact checks parsed the response JSON, confirmed the exact two-case scope and unchanged answer digest, and resolved every report link successfully.

The **supplied** [probe receipt](frozen-evidence/skill_improvement/evidence/implementation/probe-results.json) records execution from `2026-09-18T21:58:39.124057+00:00` through `2026-09-18T21:58:40.032579+00:00`, with these commands returning 0:

```text
cargo +1.98.1 metadata --offline --format-version 1 --locked
rustc +1.98.1 -Vv
cargo +1.98.1 test --offline --locked -j 4 -- --show-output --test-threads=1
```

The supplied [runtime log](frozen-evidence/skill_improvement/evidence/implementation/logs/runtime-2.log), lines 3 and 6, records `cdf_bounds_images_and_residual_filter ... ok` and `logical_plan_write_and_staged_writer_visibility ... ok`. The probe source and lock hashes match the receipt. These are independently inspected execution records supplied with the candidate; **this evaluator did not execute Rust tests**.

## Remaining limits

New Rust compilation/execution of the E11B response fragment is **not_run**. Full service gates and a new 38-case suite are **not_run**. The original evaluation scores remain untouched. The candidate's local memory/filesystem evidence does not certify remote cloud behavior, crash durability, throughput, memory bounds, every CDF timestamp/enablement transition, or downstream exactly-once processing. Within the two requested reader-contract corrections, no further revision is required by this review.
