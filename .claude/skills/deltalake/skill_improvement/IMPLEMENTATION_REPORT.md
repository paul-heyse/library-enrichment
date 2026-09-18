# Delta Lake skill implementation — 2026-09-18

The portable Delta reference now routes tasks to built-in operations and explains the inputs,
outputs, state, effects and failure boundaries needed before coding. The active entrypoint is
[SKILL.md](../SKILL.md). All changes are contained in this skill; the supporting search/code-model
skills and host application are unchanged by this implementation.

The implemented release contains 18 reviewed capability families, routes for all 13 captured
packages, 54 operation families, 63 integration leads and 6,944 full contract records. The reader
works without the original repository, a sibling skill, a source cache or network access. Research
regeneration from empty generated output reproduces all 3,542 content files byte for byte.

## Exact scope and authority

| Identity | Qualified reference |
|---|---|
| delta-rs | `58f07cd62bfbce3649a7e1c87c696288068ae184` |
| Buoyant kernel fork | `8ba063f8f84fec222000f66d40d70911d7c79675` |
| DataFusion | 55.1.0 |
| Arrow / Parquet | 59.3.0 |
| object_store | 0.13.2 |
| Runtime consumer | Rust 1.98.1; standalone locked local filesystem/memory profile |
| Retained rustdoc producer | `nightly-2026-09-13`; format 61; public and private captures |
| Acquisition Cargo.lock SHA-256 | `374fe281aa9204f9cbf9fdfdb06749c008eb0ae32906d68f40f1b6837ad0e402` |

[Capture/runtime profiles](../content/profile.json) distinguish features used to document the API
from features actually exercised. `buoyant_kernel` and `buoyant_kernel_engine` are package
identities; delta-rs uses the Cargo aliases `delta_kernel` and `delta_kernel_default_engine`.
The inherited `1.0.0+58f07cd6` model label identifies the capture; it is not the published version
of every package. Immutable source URLs and hashes decide exact-version claims; Context7 is a
discovery source only.

The original Delta protocol corpus was captured from `master` without a commit identity. Its
retained archive digest is locked; no missing commit identity has been invented. Protocol text
provides normative context, while operation support needs implementation/profile evidence.

## Delivered stages

| Stage | Result and retained implementation |
|---|---|
| D0 — Preserve | Original reader archive and file hashes in `evidence/implementation/baseline.*`; exact acquisition lock retained alongside its capture; original planning probes and failed attempts preserved. |
| D1 — Contracts | Full raw docs/type trees/spans/links, modules, fields, variants, trait/impl members and associated outputs; public/private artifact identity; public versus returned-inferred/internal access; source-root-independent semantic IDs. |
| D2 — Routes | Task, crate, representation and effect routes; builders, explicit build methods, writers/flush/commit and non-builder operations; protocol vocabulary versus admission/execution; reviewed integration entry leads. |
| D3 — Decisions | Eighteen authored families, conditional alternatives, 85 distinct referenced operation paths, claim/evidence bindings, error/effect boundaries and explicit unknowns. Four pin-specific annotations supplement unchanged raw docs. |
| D4 — Probes | Twenty-four local runtime tests, two expected compiler failures, retained source queries/fixtures, exact runtime profile and source/lock/runner hashes. Assertions measure named outcomes rather than infer behavior from signatures. |
| D5 — Retrieval | Offline standard-library `find`, `show`, `compare`; aliases and facets; exact contracts/evidence on demand; byte-bounded lossless continuation. Legacy direct-file navigation remains available. |
| D6 — Evaluation | Independent baseline/candidate arms, 32 paired plus six nominal held-out responses each, eight caller tests per arm, independent judgment, separately identified revised-candidate follow-up. |
| D7 — Distribution | Deterministic reader/research archives, licenses, digest manifests, self-contained DataFusion/Arrow integration contracts, copied-reader tracing and clean offline research regeneration. |
| D8 — Maintenance | Replay versus deliberate refresh, locked offline build inputs, targeted invalidation including newly available alternatives, maintenance commands, this report and permanent receipts. |

Reviewed families cover open/history/read/session, write/replace/schema/merge/DML, replay/commit,
CDF/features, optimize/retention, storage/catalog/kernel. This is reviewed depth over a wider
captured API, not exhaustive behavioral certification. [Coverage](../content/capabilities/coverage.json)
and each brief state the distinction.

The generated layer has 1,447 contract pages, 330 module contracts, 3,602 records with upstream
docs, 804 canonical items and 1,168 aliases. Existing 211 module summaries, 42 trait pages and 15
mental-model topics remain. Broad foreign implementation inventory is retained separately from
63 integration leads; an implementation edge is not a verified call graph. The 30-name protocol
matrix does not convert feature recognition into universal read/write/maintenance support.

## Findings that materially change implementation choices

- A loaded provider retains its snapshot, including when another version is passed to the builder.
  An unloaded handle selects the requested version. Rebuild registrations when freshness is required.
- A partition-first layout exposed incorrect column selection by `scan_table().with_columns`.
  Provider/DataFrame projection by name returned the intended schema and values.
- Delta rows differed from raw Parquet because of removed files and deletion vectors. The retained
  DV fixture had eight logical rows versus ten raw rows; compaction preserved logical rows.
  Column-mapping scans worked while CDF rejected the mapped table.
- A normal DataFusion planner rejected the tested Delta write plan. Delta's planner worked.
  Session trait fallback could discard a caller UDF; explicit derive/reject policies distinguished
  preservation from rejection. A separate evaluator control verifies a caller-only store during write.
- Reusing the same transaction marker allowed sequential appends. Competing stale marker writers
  conflicted. A post-commit hook error occurred after a fresh observer could see the new version.
  Exception handling alone cannot decide whether retrying is safe.
- `SaveMode::Ignore` appended in the existing-table fixture. Invalid replacement input failed before
  publication. Safe casts produced nulls for invalid strings; strict casts returned an error.
- Low-level writer `write` can roll files and start background upload before explicit `flush`.
  `flush` returns staged actions; logical publication needs commit. A small-target control observes
  staged objects before flush while the Delta snapshot remains unchanged.
- CDF bounds were inclusive; an end beyond head clamped while a start beyond head needed explicit
  empty-range handling. Metadata is `_commit_version: UInt64`,
  `_commit_timestamp: Timestamp(Millisecond, None)`, `_change_type: Utf8` in the tested output.
  A small `ORDER BY` case reproduced a partitioning error; unordered execution plus caller sorting
  worked. These observations qualify this pin, not all future CDF implementations.
- Vacuum Full found an untracked orphan that Lite did not. Preview/retained-version controls were
  exercised. After actual vacuum an old log version loaded but its data stream failed. Restore
  published a later version; checkpointing and compaction retained expected logical rows.
- Nested decimal schema metadata survived the tested conversion, nanoseconds normalized to
  microseconds, and UInt64 schema round-tripped as Int64. Schema conversion success is not proof of
  value-range or timestamp fidelity. Memory-pool reservation tests do not establish process RSS.

These are documented upstream behaviors/defects. This implementation changes the reference and
its evidence, not upstream Delta code. Successful tests include expected error controls.

## Verification and limits

| Check | State | Evidence / scope |
|---|---|---|
| Full contract preservation | passed | `contracts-validation.json`: all 6,944 records compared to exact raw docs/types/spans/links; 13 packages |
| Reader contracts | passed | Alias equivalence, seven difficult task routes, facets/comparison, byte continuation and 1,096 resolved local links |
| Identity/access/replay | passed | Unit controls for rustdoc-ID renumbering, capture-root relocation, associated outputs, constructor disambiguation, claim binding, retained lock identity and no implicit lock regeneration |
| Local runtime | passed | `probe-results.json`: all 24 named tests; complete source/fixture/lock/profile/runner identities and logs |
| Negative caller access | passed | `compile-fail-results.json`: private LoadBuilder import E0603; private operation-trait method E0599 |
| Rust probe quality | passed | Locked offline Clippy, all targets, warnings denied; `clippy.log` |
| Existing navigation/rules | passed | Byte regeneration/integrity, 27 navigation probes and 15 ast-grep test groups; `legacy-validation.json` |
| Source/document checks | passed | Ruff and authored-link checks; `validation.json` records dated commands and hashes |
| Invalidation | passed | Changes to authoring, selected API/default contract, source, runtime/capture profile and newly available alternative reopen affected decisions; unrelated decisions remain stable in targeted controls |
| Portable reader | passed | Isolated Python from `/`, copied content/manifest, four lookup commands, no Internet calls or original-project/cache reads |
| Research rebuild | passed | Start with no generated content; reproduce all 3,542 files using only bundled inputs and declared tools; no Internet calls or original-project reads |
| Repeat packaging | passed | Two reader archives and two research archives are respectively byte-identical |
| Cloud/native services | not_run | No live Glue/Unity, AWS/GCP/Azure, HDFS, mount or other remote/native environment qualification |
| Crash durability / arbitrary concurrency | not_run | Local visibility and selected stale-snapshot conflicts do not certify crash recovery or distributed exactly-once |
| Workload memory/spill/performance | not_run | No process RSS, spill workload, throughput, query-speedup or exact file-size guarantee; only bounded allocation/store controls |
| Fresh rustdoc producer replay | not_run | Retained captures verified; replay implementation tested for lock/pin behavior, but all 13 captures were not redocumented in this implementation |
| Offline Cargo on a fresh machine | not_run | Bundles do not vendor the entire Cargo/toolchain graph; successful local `--offline` execution uses the existing dependency cache |

The 200 unresolved rustdoc-link diagnostics, 11 unresolved access paths and eight unnameable items
remain visible, not silently guessed or promoted to imports. Selected contracts remain usable via
raw type/doc data, source pointers and callable public entrypoints. Storage/catalog/kernel families
include source-backed guidance where a runtime environment is not available. Custom engine and
catalog adapter consumer implementations, all merge/schema/partition evolutions, all protocol
combinations, and every retention boundary remain outside the executed examples. No known
consequential error in a reviewed recommendation is being hidden by these inventory counts.

## Independent comparison and corrections

The [independent report](evidence/implementation/evaluation/REPORT.md) and machine-readable
judgment preserve the original frozen experiment. Both arms completed 38 responses and eventually
passed eight required caller tests: baseline after three attempts, candidate after four. Initial
private-import and CDF-type mistakes are retained; neither arm succeeded on its first attempt.

| Frozen comparison | Outcome |
|---|---|
| 32 paired prompts | Four candidate contract improvements, 28 ties |
| Six nominal held-out prompts | One improvement, four ties, one omission/regression |
| Consequential omission | H05 omitted early file staging during low-level writer `write` |
| Baseline source fallback | 21 cached dependency files; candidate original arm needed no dependency-source fallback |

The writer omission was corrected in the authored brief and annotated reset contract, with a
new early-roll/background-upload assertion. CDF metadata types and the observed ordering caveat
were made explicit. A [separate two-case review](evidence/implementation/evaluation/revision/REPORT.md)
of H05/E11B uses a newly frozen reader and inspects supplied executed evidence. It does not replace
the original 38-case scores or claim a second complete experiment. Both revised cases passed that
bounded reader-contract review; the evaluator did not execute new Rust tests.

E05B's original candidate test did not observe a restrictive caller capability during the write.
The separately retained attempt-06 control requires a caller-only store from inside source
planning, fails without that store before commit, and succeeds with it. The judge accepted this
bounded follow-up; it does not establish every resource policy.

Four nominal held-out cases overlap candidate examples, so they measure use of supplied evidence,
not unseen-capability generalization. H06 supplies no actual changed capture; the implementation's
invalidation controls establish that mechanism separately. Context-token totals and normalized
retrieval timing were not measured consistently. No general quality percentage, speed advantage,
context saving or unseen-task advantage is claimed.

Frozen identities (SHA-256):

- Baseline: `136b3da5e2b248df64d89750785050e77676d2b296b804d12f188774dce301ad`
- Original candidate: `f7c800efef46c61aba45909913a9b1bf2fb859bf19ea2b476f6fbba249f4d5e1`
- Revised two-case candidate: `55d26c6dab92462489c7a19cadd11a94f13c585c7038848fe5f87eb6eefcf2f4`

Later source-link identity and clean-rebuild corrections are qualified by the final release checks;
they are not retroactively part of either frozen evaluation.

## Distribution and maintenance

Archives are `evidence/implementation/bundles/reader.tar.gz` (about 4.8 MiB) and
`research.tar.gz` (about 32 MiB). Copy/extract the reader to use the skill in another repository.
Research additionally carries raw captures, acquisition lock, cached source corpora, authoring and
investigation/evaluation history. Compiler targets and extracted evaluation/qualification trees are
excluded. License texts and minimal exact-release DataFusion/Arrow seam contracts are bundled.

The adjacent `evidence/implementation/qualification.json` records final archive hashes, byte counts,
manifest counts and transfer/rebuild results. These packaging receipts are deliberately outside the
archives: an archive cannot include its own later digest/trace receipt. The report's receipt paths
refer to the retained implementation workspace; a reader-only copy includes the evidence needed
for reference claims, while research contains the broader implementation history.

Use [MAINTENANCE.md](../MAINTENANCE.md) and [command reference](../scripts/README.md).
Ordinary rebuild verifies locked inputs and never fetches missing content. Deliberate acquisition
refresh is a separate operation with a new identity. Re-run probes when source/fixture/lock/runner
or profile changes; review alternatives when a relevant candidate appears even if the chosen API
is unchanged. All current and historical receipts are indexed in
[evidence/implementation/README.md](evidence/implementation/README.md).
