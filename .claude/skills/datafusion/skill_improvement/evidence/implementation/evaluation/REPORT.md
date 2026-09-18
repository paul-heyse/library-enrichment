Verified 2026-09-18 against retained DataFusion 55.1.0 and Arrow/Parquet 59.3.0 source, full upstream rustdoc contracts, and probe code/logs. This is review evidence, not an acceptance or completion gate. Detailed per-case discovery, selection, contract, evidence, uncertainty and source citations are in [judgment.json](judgment.json).

The candidate improves 13 cases and matches 11; no case is judged an overall regression. These are descriptive judgments, not a quality percentage. Both arms make supported core choices for all 24 tasks. Most gains replace baseline uncertainty with precise contracts; they do not correct wrong baseline results. No confirmed wrong task-specific candidate output was found.

| Task pair | A | B | Consequential difference |
|---|---|---|---|
| E01 filter / gather | improves | matches | Verified zero-column filtering; candidate loses `take_arrays` discovery in B. |
| E02 repeated / one-off filter | matches | matches | Both distinguish amortization from a guaranteed speedup. |
| E03 ordinary / zero-column gather | improves | improves | Source confirms unchecked batch helper and empty-column construction error. |
| E04 nullable / strict cast | improves | improves | Exact option semantics replace explicitly unverified interpretations. |
| E05 local / durable keys | improves | matches | Adds sort-indices alternative and conversion panic contracts; both choose typed durable interchange. |
| E06 logical / physical dictionary | improves | improves | Direct dictionary conversion avoids pre-cast; verified Utf8 decode clarifies information loss. |
| E07 stream / blocking sort budget | matches | matches | Both reject a whole-query memory guarantee from streaming. |
| E08 batches / object-store Parquet | improves | improves | Adds empty-partition recipe, file-only schema and partition-path obligations. |
| E09 exact / conservative pushdown | improves | matches | Candidate correctly detects contradictory upstream Exact prose. |
| E10 full / projected limited scan | matches | matches | Better limit contract in B is offset by a proposed-oracle ambiguity. |
| E11 ordinary / standalone expression | matches | improves | Shows why successful uncoerced simplification is not compatibility proof. |
| E12 all / retained row groups | matches | improves | Exact reader contract and fixture establish rebased coordinates. |

Two consequential corrections are justified:

1. Keep the upstream `Exact` text intact but place a separate conflict note beside it. Its claim to omit passing tuples contradicts the enum overview, [scan contract](../../sources/datafusion-session-55.1.0/src/table.rs) and [actual fixture](../probes/tests/engine_contracts.rs). The candidate chose the correct TRUE-row behavior; this is an upstream documentation defect, not a candidate mistake.
2. Clarify E10B's oracle: a scan limit is an at-least hint that permits overproduction, whereas final SQL `LIMIT` has exact capped cardinality. Adding `ORDER BY` can keep the limit above a sort, bypassing the provider branch being tested. Assert an observed `Some(limit)` scan call and test its contract separately from deterministic ordered SQL. The retained provider fixture already records this distinction.

E01B also omits the existing `take_arrays` helper and writes its per-column map. This is a minor discovery loss with equivalent behavior, not a wrong result.

Neither evaluator compiled its sketches or ran its proposed integration oracles. The frozen candidate bundle had 15 briefs and a 13-test receipt. The current working reference has 16 briefs and a [15-test receipt](../probe-results.json), including later coalesce and same-version IPC dictionary controls; current Parquet logging measures local `ChunkReader` requests. These later controls are not retroactive evidence of what the candidate read. All eight current receipt source hashes matched. The IPC control does not prove cross-version compatibility or arbitrary dictionary reconstruction; the Inexact fixture does not implement min/max pruning; the sort failure does not certify production RSS.

The recorded runs used 19 versus 33 nested shell-tool calls and 345.758 versus 522.635 seconds, baseline then candidate. These are descriptive observations with different exploration and retrieval affordances, not an isolated efficiency benchmark. Context consumption is unknown for both arms. There was one agent/run per arm, no variance estimate, and the tasks were seeded around known capabilities. The candidate had richer routes, full contracts and receipts. This pilot supports the named gains, not generalized superiority, reduced context use, or production qualification.
