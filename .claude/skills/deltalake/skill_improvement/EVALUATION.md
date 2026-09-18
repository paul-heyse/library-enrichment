# Evaluation specification

Proposed 2026-09-18; comparative Delta evaluation is **not_run**. The four Rust planning probes
support individual contract claims, not measured improvement in agents' decisions.

## Experimental setup

Freeze the current reader and proposed reader with hashes. Give each arm the same exact git/lock/
feature profile, tools, isolated Rust consumer scaffold and response budget. Keep task oracles and
other agents' answers out of the reader material. Use independent baseline/candidate agents and
independent judgment when running the implementation evaluation. Repeat stochastic cases rather
than attributing one agent's choices to the reference alone.

Require each response to identify candidates, choose under the task's constraints, explain the
input/output/effect contract, state uncertainties and produce a small compilable integration when
the task calls for code. Do not prescribe a single symbol as the only correct answer where several
built-ins satisfy the requirement. Score correct abstention separately from unsupported certainty.

Record task/profile/bundle identity, retrieved paths/commands, decisive evidence, selected and
rejected alternatives, unsupported claims, emitted source, compiler/runtime results, elapsed time,
tool calls and context consumption if instrumentation exposes it. Missing context data stays
unmeasured. A shared reference probe does not prove an evaluator's different composition compiles.

## Sixteen paired tasks

Each pair changes a material condition. Some require a choice reversal; others require preserving
the choice while changing configuration or handling. That distinction is part of the oracle.

| Pair | Case A | Changed condition in B | Decision distinction and executable oracle |
|---|---|---|---|
| E01 Snapshot | Repeatable reads of version N | Latest committed rows required each cycle | Retain version-bound provider versus refresh/rebuild; exact rows and reported version |
| E02 File source | Plain immutable Parquet dataset | Directory belongs to a Delta table with removed files | Plain reader versus Delta-aware snapshot/provider; tombstone control |
| E03 Read shape | Need selected columns as a stream | Need joins and filters in an existing DataFusion session | `scan_table` versus provider/query composition; schema and exact rows; no whole-query-memory claim |
| E04 Session | Caller supplies real SessionState with UDF/runtime policy | Caller supplies a different Session implementation | Preserve caller settings or explicitly select derive/reject fallback; actual UDF/runtime observation |
| E05 Write input | Batches already materialized | Source already represented by a logical plan | Existing write paths without needless collect; compile `with_input_plan(LogicalPlan)` and compare outputs/effects |
| E06 Overwrite | Replace entire table | Replace only one predicate-selected region | Full overwrite versus replaceWhere; preserve unmatched rows and reject invalid input where required |
| E07 Schema | Fixed contract rejects drift | New columns are intentionally admitted | Explicit schema/evolution policy; values, field types/nullability and version after error/success |
| E08 Merge | Unique non-null source keys | Source duplicates and null keys appear | Re-evaluate matching/clause multiplicity and source preparation; supported behavior at this pin, no implicit uniqueness assumption |
| E09 Replay | One append executed once | Same application batch retried after ambiguous result | Marker alone is insufficient; verify marker lookup, caller decision and publication state; separate concurrent protocol |
| E10 Failure | Error before log publication | Error injected after publication | Recovery depends on observed state/phase; exact log version, rows and absence/presence of effects |
| E11 Changes | Need current rows | Need incremental insert/delete/update images | Snapshot scan versus CDF with explicit interval/metadata; change rows, schema and resume semantics |
| E12 CDF bounds | Requested interval exists and CDF was enabled | Interval exceeds head or crosses enablement/retention boundary | Error/out-of-range policy and eligibility; no fabricated completeness |
| E13 Maintenance | Small files hurt scans | Need to remove obsolete physical objects | Optimize versus vacuum; row equality, file inventory, versions and dry-run effects |
| E14 History | Inspect an older version | Make its state current again | Time travel versus restore new commit; current version and old-file availability |
| E15 Protocol | A recognized feature has tested scan support | Another recognized feature is excluded/not admitted for requested operation | Recognized enum does not establish support; explicit matrix cell and feature fixture |
| E16 Store | Session has no mapping for table root | Existing mapping points to a different store/configuration | Ensure-registration versus explicit replacement/isolated runtime; table read and mapping identity |

E01/E02/E03/E09/E10 have partial planning controls. The rest, and all actual agent comparisons,
remain proposed. A single positive test does not cover every changed condition in its pair.

## Held-out tasks

Use at least six additional tasks not represented by a worked example: CDF residual filters that
mention non-partition columns; schema mapping/timestamp fidelity; vacuum lite versus full and kept
versions; catalog resolution versus direct table URL; low-level writer flush visibility; new
feature/default introduced in an intentionally changed capture. Vary wording and the representation
already available. Add real failed discovery cases from users rather than endlessly expanding a
synthetic benchmark.

## Separate outcome dimensions

| Dimension | Evidence to report |
|---|---|
| Candidate discovery | Relevant built-ins/alternatives found or missed; false internal APIs offered |
| Decision quality | Requirements met; material condition recognized; justified reversal or retention |
| Contract fidelity | Correct types, lifecycle, rows/schema, effects, error phase and feature constraints |
| Integration | Exact emitted composition compiles and behaves under positive/negative fixtures |
| Evidence use | Proper source/profile/probe attribution and bounded uncertainty |
| Efficiency | Tool calls, material read, context and wall time; independent of correctness |
| Generalization | Held-out and repeated-run outcomes; sensitivity to different representations |

Do not collapse these into a quality percentage. Block release claims for a consequential wrong
contract even when the candidate retrieves faster. If the candidate is accurate but cumbersome,
revise default retrieval summaries and the entrypoint rather than dropping evidence. If both arms
already choose well, report gains in contract completeness and lower unsupported certainty, not
invented baseline failures. Publish actual limitations and preserve raw responses/receipts.
