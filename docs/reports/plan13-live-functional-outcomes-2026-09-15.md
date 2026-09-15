# Plan 13 live DataFusion functional outcomes — 2026-09-15

## Result and evidence boundary

The complete 40-crate DataFusion **55.1.0** development campaign passed resolve, overview,
API search, selected default inspection and offline reuse through the actual Rust daemon and
FastMCP **4.0.3** stdio adapter. It retained **201 answers** plus raw requests and response frames.
The focused campaign also passed all four previously failing high-fanout defaults, a bounded
explicit relationship page, a two-release documentation comparison, pinned revision acquisition
and offline reuse, and Python `packaging.version.Version`.

These are observed functional outcomes on the rebuilt development binaries. The following
client-harness and release-qualification-script edits changed the overall source fingerprint;
the receipts retain their actual source identities and are not relabeled installed acceptance.
Installed release campaign, final integration, actual client qualification and activation are
recorded separately as they finish. This report does not assert whole-library completeness.

## Exact campaign receipts

- `.dev-state/p13-native-live/family-121449-receipt.json` — passed; 12:14:50–12:20:24 UTC.
- `.dev-state/p13-native-live/focused-121305-receipt.json` — passed; 12:13:06–12:13:51 UTC.
- Matching `*-answers.ndjson`, `*-requests.ndjson`, `*-frames.ndjson`, and `*-process.log` retain the observed values.
- Both receipts verify source and all three binary digests unchanged during their own run.
- The earlier `focused-120410-receipt.json` remains failed at the datafusion-expr overview consistency check.

## Family breadth

Every row completed all five operations; offline reuse retained the exact context and snapshot.
Every search returned a hit linked to an evidence citation. Inspection returned a qualified
observation or selectable definition. Counts below are observed snapshot counts, not coverage
percentages; partial research outcomes preserve unsupported or missing evidence scopes.

| Crate at 55.1.0 | Definitions | Fragments | Snapshot |
|---|---:|---:|---|
| datafusion | 732 | 2383 | `snap_eb367044d60af6f2` |
| datafusion-catalog | 205 | 320 | `snap_422ea97ac1dc95b7` |
| datafusion-catalog-listing | 81 | 126 | `snap_3239217b97ac50e4` |
| datafusion-cli | 235 | 257 | `snap_3b865009e99314be` |
| datafusion-common | 2096 | 3440 | `snap_cf658fe4eb41aea5` |
| datafusion-common-runtime | 43 | 87 | `snap_d926d8bd7832553f` |
| datafusion-datasource | 594 | 1015 | `snap_a1d9a12663a7ec92` |
| datafusion-datasource-arrow | 42 | 70 | `snap_d5a2c01a121bda40` |
| datafusion-datasource-avro | 35 | 60 | `snap_4539333bdabb5e5d` |
| datafusion-datasource-csv | 104 | 250 | `snap_d1f4413f94fca55f` |
| datafusion-datasource-json | 82 | 177 | `snap_f18558e6592e0082` |
| datafusion-datasource-parquet | 257 | 543 | `snap_0f822a13b2b199b5` |
| datafusion-doc | 66 | 79 | `snap_7da8e0d74d039a0b` |
| datafusion-execution | 541 | 889 | `snap_7d413b166789d476` |
| datafusion-expr | 2483 | 5385 | `snap_2e984ea581f36305` |
| datafusion-expr-common | 495 | 732 | `snap_0c2fb33c789a8c80` |
| datafusion-ffi | 830 | 1040 | `snap_ac2ea683e4834c15` |
| datafusion-functions | 1717 | 2047 | `snap_1c612e4aa4c32fa9` |
| datafusion-functions-aggregate | 787 | 913 | `snap_b097c8c69fce410d` |
| datafusion-functions-aggregate-common | 310 | 378 | `snap_1ed638e0114b8467` |
| datafusion-functions-nested | 825 | 969 | `snap_edf8cbc4f5805d61` |
| datafusion-functions-table | 70 | 76 | `snap_378db979a7dd784a` |
| datafusion-functions-window | 157 | 200 | `snap_05bfe47ef84c213d` |
| datafusion-functions-window-common | 22 | 35 | `snap_41fb3dd03db0791b` |
| datafusion-macros | 1 | 4 | `snap_09db8f4ca8427e6e` |
| datafusion-optimizer | 347 | 523 | `snap_aff07cc92d7834a4` |
| datafusion-physical-expr | 997 | 1526 | `snap_05ab21b5d4cb88d2` |
| datafusion-physical-expr-adapter | 28 | 72 | `snap_3293eea57337d676` |
| datafusion-physical-expr-common | 478 | 754 | `snap_ae41bd2039d621b5` |
| datafusion-physical-optimizer | 237 | 337 | `snap_ee78d9f6ea3b8ac4` |
| datafusion-physical-plan | 2146 | 3173 | `snap_13e8b54221085fee` |
| datafusion-proto | 224 | 246 | `snap_3460167baf766c5b` |
| datafusion-proto-common | 1470 | 4164 | `snap_5267278fb6157ecc` |
| datafusion-proto-models | 4451 | 8226 | `snap_19db41c4308a3b7d` |
| datafusion-pruning | 42 | 67 | `snap_02338233245e1de4` |
| datafusion-session | 130 | 461 | `snap_630a2134990f3249` |
| datafusion-spark | 1464 | 1684 | `snap_76fb807ad0d56dfc` |
| datafusion-sql | 477 | 627 | `snap_db9abff2c2e7216a` |
| datafusion-sqllogictest | 90 | 141 | `snap_e4f65b4b503f8627` |
| datafusion-substrait | 269 | 337 | `snap_5a5a19ce6bedd1a6` |

## Focused semantic results

- **SessionContext, DataFrame, ParquetReadOptions and Expr:** each default inspection returned one signature observation and one documentation fragment, with signature, availability and documentation aspects available. Default selection did not hydrate their relationship fanout.
- **SessionContext explicit relationships:** 32 relationships plus an advancing independent cursor. This development run read the first page only; installed traversal is a separate outcome.
- **54.1.0 → 55.1.0 documentation comparison:** 24 changed keys; the first bounded response contained two changes. Documentation is indexed on both sides. Release notes are missing on both sides with separate witnesses, so the result is partial. It does not establish no behavior changes.
- **Pinned repository revision:** commit `7d3835c71f30cbd3c3ae4041732267f1f453097a`, package `datafusion/core`, retained 22 fragments and zero compiled definitions. Offline reuse retained `snap_289f9b5f29d083a9`. Omitted potentially required LICENSE/NOTICE links leave closure incomplete; generated inputs, submodules, LFS and full build closure remain unproven.
- **Python:** live resolution selected packaging **26.3** and default inspection of `packaging.version.Version` returned useful qualified evidence. This static observation is not a runtime verification.

## Corrected integration outcomes

The consolidated diagnostic run exposed three native publication stack overflows and seven
Python failures. They remain recorded as failures in their original logs. Subsequent affected
checks passed after boxing the publication future, native Python class-scope assessment, stable
overview namespace selection, valid revision permalinks, and corrected retired consumer assumptions:

- `.dev-state/plan13-native-publication-scope-outcomes.log` — nine native outcomes passed, including all three prior stack-overflow cases.
- `.dev-state/plan13-revision-permalink-outcome.log` — actual revision/search/offline fixture passed.
- `.dev-state/plan13-qualified-native-semantic-outcomes.log` — five real qualified runtime/ty/reuse cases passed.
- `.dev-state/plan13-native-scope-execution-qualification.log` — real Python/Rust probes and 12 containment/cleanup cases passed for the recorded development executor.

These results address J01/J03/J09/J10/J18 and relevant failure regressions. They do not alone
close J01–J20, the removal ledger, the required client tasks or the single deployment cutover.

## Installed release campaign

The assembled candidate at
`/home/paul/.local/opt/library-enrichment/plan13-3a540f4ab7eb` contains all three release binaries,
a non-editable locked Python environment, packaged schemas and guidance, and the product skill.
Its `candidate.json` records installed input and binary hashes. It remains inactive in production.

- `focused-123818-receipt.json` — passed with unchanged source/binaries and installed manifest
  verification; full default/high-fanout/revision/comparison/Python journey. SessionContext's
  **117 relationships** were traversed completely without duplicate IDs.
- `family-123938-receipt.json` — passed with unchanged source/binaries; all 40 exact-release
  resolve/overview/search/inspect/offline journeys through the installed release and adapter.
- Both receipts and their raw answers/requests/frames are under `.dev-state/p13-installed-live/`.
- `.dev-state/plan13-installed-python.log` records `uv sync --locked --offline --no-editable --no-dev`
  and FastMCP 4.0.3. The adapter uses the absolute installed project/interpreter with Python
  isolated mode, from the independent campaign state directory.

These installed campaign outcomes pass. The production cutover, final acceptance receipts,
release executor qualification and full actual-client outcome set remain separate exits.

## Successor installed adapter and real recovery

`/home/paul/.local/opt/library-enrichment/plan13-3ae079f9718a` retains the same release native
binary hashes and adds portable error recovery. Its focused raw-MCP campaign passed in
`.dev-state/p13-installed-live/focused-130513-receipt.json`, with unchanged source/binaries during
that run. No previous candidate receipt was relabeled. The exact 117-relationship traversal,
high-fanout defaults, partial comparison, revision/offline and packaging canary all remain usable.

The subsequent actual installed Claude recovery trace and independent native failed-job witness
are under `.dev-state/logs/clients/run-snt4jm4k/recovery-claude/`. They show the missing 9.9.9 release,
native recovery diagnosis, then usable 0.2.0 signature evidence. The earlier trace's generic error
summary had hidden this diagnosis because the host dropped structured error content. Canonical
structured output remains unchanged; the portable error text is a bounded projection only.

This ongoing campaign uses explicit installed daemon/adapter/worker selection. Its early Codex
runs completed useful research but an observer incorrectly classified a rejected wait argument as
a failed-job identity mismatch. The observer is corrected and paired regressions pass; a new
independent live replay remains required before promoting those client gates.

## Independent installed-client replay — 2026-09-15

The required acceptance auditor replayed the recorded installed-client command against
`plan13-3ae079f9718a`: **15 passed / 225 deselected in 1001.66 seconds**. This is fourteen actual
Codex/Claude scenarios and one operator-directory preservation check. The unchanged source digest
is `224dea96aca19e8482b6715f524ebe0b5948ba039e0c8fac00200aa933d72bf5`; executable hashes remained
unchanged throughout the command. `valid_receipt()` accepted the source, binary, output-log and
client-trace bindings. Nothing from the earlier failed observer run was promoted.

Receipts: `.dev-state/plan13-independent-audit/final/client.json` and its `.execution.json`.
Actual transcripts and independent snapshot, artifact and failed-job witnesses:
`.dev-state/logs/clients/run-fofn83_3/`.

Both clients completed discovery, missing-release recovery, upgrade investigation and qualified
runtime inspection. Runtime evidence retains the observed `(value: 'int') -> 'str'` signature
and the disagreeing stub as separate observations. Failed-job recovery read the complete native
diagnosis before selecting an available exact release. Namespace upgrade evidence retains the
available import roots and explicitly partial package scope.

Manual transcript review found two limits in client prose. Claude described an unavailable
registry release as having “never existed,” which goes beyond the observed registry result.
It also repeated a process-scoped Griffe readiness observation taken concurrently with a pending
acquisition, without refreshing it after successful extraction. These are limitations of client
interpretation; the retained native evidence does not make either broader claim. The harness
does not purport to certify every sentence generated by a client.

This closes the installed-client campaign. Final native/Python/containment/live command replay
and the production activation remain separately tracked in the execution ledger.

## Final deployed outcome — 2026-09-15 13:50 UTC

The successor installation is now the sole selected production generation. All independent
final-source commands passed: 432 native, 223 Python, 12 execution, 2 live and 15 installed-client
checks. Earlier “pending” statements above describe their historical checkpoints. The installed
family/performance receipts remain attributed to their original unchanged-native candidate;
current-adapter qualification uses its focused, actual-client and deployed receipts.

The deployed seven-case raw-MCP smoke passed status, exact DataFusion 55.1.0 acquisition,
SessionContext default inspection, a pending 54.1→55.1 comparison, direct changes-section digest
verification, packaging 26.3 resolution and Version inspection. Both global registrations and
managed skill copies match the verified installation. Old daemon/adapters are stopped, old data
and configuration are unchanged, and no owned workers remain. The [final qualification report](plan13-final-qualification-2026-09-15.md)
closes J01–J20/D01–D12 and records the independent 47-active-gate audit and exact deployment hashes.
